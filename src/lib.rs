use std::error::Error;
use std::{env, fs, io};
use std::io::{Read};
use atty::Stream;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = read_from_source(&config.source)?;
    let results = search(&file_content, &config);
    for line in results {
        println!("{line}");
    }
    Ok(())
}

fn read_from_source(source_type: &TextSource) -> Result<String, Box<dyn Error>> {
    let contents = match source_type {
        TextSource::Stdin => {
            let mut contents = String::new();
            io::stdin().read_to_string(& mut contents).map(|_size| {contents})?
        }
        TextSource::File(path) => {
            fs::read_to_string(path)?
        }
    };
    Ok(contents)
}


fn search<'a>(contents: &'a str, config: &Config) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = if config.ignore_case {
        &config.query.to_lowercase()
    } else {
        &config.query
    };
    for line in contents.lines() {
        let tmp_line = if config.ignore_case {
            &line.to_lowercase()
        } else {
            line
        };
        if tmp_line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub struct Config {
    pub query: String,
    pub ignore_case: bool,
    pub source: TextSource,
}

#[derive(Debug)]
pub enum TextSource {
    Stdin, File(String)
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        let source = if atty::isnt(Stream::Stdin) {
            Self::check_args_count(args, 2)?;
            TextSource::Stdin
        } else {
            Self::check_args_count(args, 3)?;
            TextSource::File( args[2].clone())
        };
        let query = args[1].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {query, ignore_case, source})
    }

    fn check_args_count(args: &[String], required: usize) -> Result<(), String> {
        if args.len() < required {
            return Err(format!("not enough arguments. Required {} but got {}", &required, args.len()) );
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(contents, &Config{
            query: query.to_string(), ignore_case: false, source: TextSource::Stdin
        }));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(contents, &Config {
                query: query.to_string(), ignore_case: true, source: TextSource::Stdin
            })
        );
    }

}
