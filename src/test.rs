#[cfg(test)]


mod tests {
    use crate::get::get_all::get_all;
    use crate::types::Difficulty;
    #[test]
    fn print_all_example() {
        let levels = get_all();
    
        for level in levels {
            println!("ID: {}", level.id);
            println!("Name: {}", level.name);
            println!("Creators: {:?}", level.creators);
            println!("Verifier: {}", level.verifier);
            println!("Verification: {}", level.verification);
            println!("Percent to Qualify: {}", level.percent_to_qualify);
            println!("Song: {}", level.song_name);
            if let Some(song_link) = level.song_link {
                println!("Song Link: {}", song_link);
            }
            println!("Difficulty: {:?}", level.difficulty);
            println!("Records: ");
    
            let mut i = 0;
            for record in level.records {
                i += 1;
                println!("    {i}. ");
                println!("    User: {}", record.user);
                println!("    Link: {}", record.link);
                println!("    Percent: {}", record.percent);
                println!("    Hz: {}", record.hz);
                println!("    Mobile: {}", record.mobile);
                if let Some(enjoyment) = record.enjoyment {
                    println!("    Enjoyment: {}", enjoyment);
                }
            }
        }
    }

    #[test]
    fn check_diff_range() {
        let diff_index: Option<u64> = Some(15);
        let result = Difficulty::map_index(diff_index);
        assert_eq!(result, Difficulty::None);
    }
}
