use reqwest;
use scraper;
use std::collections::HashMap;

fn main() {
    // Get the HTML DOM for Premier League Table page
    let res = get_html();
    if res.is_ok() {
    let html = res.unwrap();
    // Scrap the table content for the DOM
    // Get the Club position in the table after last Gameweek
    let result = scrap_table(html);
    println!("{:#?}", result);
}
}

fn get_html() -> Result<String, reqwest::Error>{
    let url = "https://www.premierleague.com/tables";
    match reqwest::blocking::get(url.to_string())
    .expect("Response not found")
    .text(){
        Ok(response) => Ok(response),
        Err(e) => Err(e),
    }
}

fn scrap_table(html: String) -> HashMap<i8,String> {
    let document = scraper::Html::parse_document(&html);
    let team_selector = scraper::Selector::parse(r#"span[class="long"]"#).unwrap();
    let mut team_position:i8 = 0;
    let mut teams: HashMap<i8, String> = HashMap::new();
    for element in document.select(&team_selector){
        team_position +=1;
        teams.insert(team_position, element.inner_html());
        if team_position == 20{
            break;
        }
    }
    teams
}

