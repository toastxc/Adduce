use crate::structs::toml_conf::{Conf, Object};

use super::rfs::fs_to_str;

impl Conf {
    pub fn to_html(self) -> String {
        let mut divs = String::new();
        for x in self.main.unwrap().block.iter() {
            divs += &compile_html(x);
        }

        println!("{}", self.style.is_some());
        let styling = match self.style {
            None => String::new(),
            Some(a) => fs_to_str(&a),
        };

        format!("<!DOCTYPE html>\n<head>\n<style>\n{styling}\n</style>\n</head>\n<body>\n{divs}\n</body>")
    }
}

fn compile_html(conf: &Object) -> String {
    // import configuration and own it
    let conf: Object = conf.to_owned();

    // define basic style
    let style = conf.style.unwrap_or_default();

    // define text
    let text = match conf.from_str {
        None => conf.text.unwrap_or_else(|| String::from("PLACEHOLDER")),
        Some(a) => fs_to_str(&a),
    };

    let pt_text = pretty_text(&text);

    // defines id
    let id = match conf.id {
        None => String::new(),
        Some(a) => format!(" id=\"{}\"", a),
    };

    // in case there is a special style
    match &style as &str {
        "br" => String::from("<br>"),
        "html" => pt_text,
        "md" => markdown(&text),
        // standard
        _ => format!("\n<{style}{id}>{pt_text}\n</{style}>"),
    }
}

fn markdown(text: &str) -> String {
    let mut fin = String::new();
    for x in text.split('\n') {
        let mut c = x.chars();

        let (style, popper, at_end) =
            match (c.next(), c.next(), c.next(), c.next(), c.next(), c.next()) {
                // h6 -> h1
                (Some('#'), Some('#'), Some('#'), Some('#'), Some('#'), Some('#')) => {
                    ("h6", 6, false)
                }
                (Some('#'), Some('#'), Some('#'), Some('#'), Some('#'), _) => ("h5", 5, false),
                (Some('#'), Some('#'), Some('#'), Some('#'), _, _) => ("h4", 4, false),
                (Some('#'), Some('#'), Some('#'), _, _, _) => ("h3", 3, false),
                (Some('#'), Some('#'), _, _, _, _) => ("h2", 2, false),
                (Some('#'), _, _, _, _, _) => ("h1", 1, false),

                // bold italics
                (Some('*'), Some('*'), _, _, _, _) => ("bold", 2, true),
                (Some('*'), _, _, _, _, _) => ("italic", 1, true),

                (Some('>'), Some('>'), _, _, _, _) => ("nestquote", 2, false),
                (Some('>'), _, _, _, _, _) => ("quote", 1, false),

                (Some('<'), _, _, _, _, _) => ("html", 0, false),

                (Some('-'), _, _, _, _, _) => ("li", 1, false),

                // (Some(_), _, _, _, _, _) => ("p", 0, false),
                _ => ("no", 0, false),
            };
        // text minus the md formating
        let mut text_min: String = x.to_owned();

        for _ in 0..popper {
            text_min.remove(0);
            if at_end {
                text_min.pop();
            };
        }

        fin += match style {
            "br" => String::from("\n<br>"),
            "no" => String::new(),
            "html" => text_min,
            _ => format!("\n<{style}>\n    {text_min}\n</{style}>"),
        }
        .as_ref();
    }
    fin
}

fn pretty_text(text: &str) -> String {
    let mut fin = String::new();
    for x in text.split('\n') {
        fin += &format!("\n    {x}");
    }
    fin
}
