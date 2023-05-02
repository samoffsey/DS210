use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

struct EdgeWeightedDigraph {
    graph: Vec<Vec<i32>>,
}

impl EdgeWeightedDigraph {
    // Initializes the graph with n nodes, with all weights set to 0
    fn new(n: usize) -> Self {
        Self {
            graph: vec![vec![0; n]; n],
        }
    }

    // Adds 1 to the weight between the start and end node.
    fn add_weight(&mut self, start: usize, end: usize) {
        self.graph[start][end] += 1;
    }

    fn get_weight(&self, start:usize, end: usize) -> i32{
        return self.graph[start][end];

    }

    fn degree_centrality(&self, type_str: &str) -> Vec<(usize, usize)> {
        let mut degrees = vec![];
    
        for i in 0..self.graph.len() {
            let in_degree: i32 = self.graph.iter().map(|row| row[i]).sum();
            let out_degree: i32 = self.graph[i].iter().sum();
            let degree = match type_str {
                "in-degree" => in_degree as usize,
                "out-degree" => out_degree as usize,
                "combined" => (in_degree + out_degree) as usize,
                _ => panic!("Invalid type '{}', must be 'in-degree', 'out-degree', or 'combined'", type_str),
            };
    
            degrees.push((i, degree));
        }
    
        return degrees
    }

    // Returns a vector of tuples containing the node ID and its simple centrality.
    fn simple_betweenness_centrality(&self) -> Vec<(usize, usize)> {
        let mut centrality = HashMap::new(); // Store the number of shortest paths that pass through each node

        for i in 0..self.graph.len() {
            let mut visited = HashSet::new(); // Track visited nodes during BFS
            let mut queue = vec![i]; // Initialize queue for BFS with current node
            visited.insert(i);

            while !queue.is_empty() {
                let node = queue.remove(0);
                for j in 0..self.graph[node].len() {
                    if self.graph[node][j] > 0 && !visited.contains(&j) {
                        visited.insert(j);
                        queue.push(j);
                        *centrality.entry(j).or_insert(0) += 1; // Increment centrality for each visited node
                    }
                }
            }
        }

        // Convert the HashMap to a vector of tuples and sort by centrality in descending order
        let mut centrality_vec: Vec<(usize, usize)> = centrality.into_iter().collect();
        centrality_vec.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

        return centrality_vec;
    }
    
}

fn main() {
    // Initialize the graph with 167 nodes
    let mut graph = EdgeWeightedDigraph::new(168);

    // Open the CSV file
    let file = File::open("communication.csv").expect("Failed to open file");

    // Create a BufReader to read the file line by line
    let reader = BufReader::new(file);

    // Loop through the lines in the file
    for line in reader.lines().skip(1) { //skip header row
        // Split the line by semicolons
        let s = line.unwrap();
        let values: Vec<&str> = s.split(';').collect();

        // Extract the start and end nodes and convert them to integers
        let start = values[0].parse::<usize>().unwrap();
        let end = values[1].parse::<usize>().unwrap();

        // Add 1 to the weight between the start and end nodes
        graph.add_weight(start, end);

            }
    // Compute the degree centrality of each node in the graph for all three types
    let types = ["in-degree", "out-degree", "combined"];
    for &type_str in types.iter() {
        // Compute the degree centrality of each node for the current type
        let degree_centralities = graph.degree_centrality(type_str);

        // Sort the nodes by degree centrality in descending order
        let mut sorted_nodes = degree_centralities.clone();
        sorted_nodes.sort_by_key(|&(_, degree)| std::cmp::Reverse(degree));

        // Print the top 5 nodes by degree centrality for the current type
        println!("Top 5 nodes by number of emails ({})", type_str);
        for i in 0..5 {
            let (node, degree) = sorted_nodes[i];
            println!("Node {}: {}", node, degree);
        }
        println!();

    }

    let simple_centralities = graph.simple_betweenness_centrality();

    println!("Top 5 nodes by simple betweenness centrality:");
    for i in 0..5 {
        let (node, centrality) = simple_centralities[i];
        println!("Node {}: {}", node, centrality);
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let graph = EdgeWeightedDigraph::new(3);
        assert_eq!(graph.graph, vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]);
    }

    #[test]
    fn test_add_weight() {
        let mut graph = EdgeWeightedDigraph::new(3);
        graph.add_weight(0, 1);
        graph.add_weight(1, 2);
        assert_eq!(graph.graph, vec![vec![0, 1, 0], vec![0, 0, 1], vec![0, 0, 0]]);
    }

    #[test]
    fn test_get_weight() {
        let mut graph = EdgeWeightedDigraph::new(3);
        graph.add_weight(0, 1);
        graph.add_weight(1, 2);
        assert_eq!(graph.get_weight(0, 1), 1);
        assert_eq!(graph.get_weight(1, 2), 1);
        assert_eq!(graph.get_weight(0, 2), 0);
    }

    #[test]
    fn test_degree_centrality() {
        let mut graph = EdgeWeightedDigraph::new(3);
        graph.add_weight(0, 1);
        graph.add_weight(1, 2);
        let degree_centralities = graph.degree_centrality("in-degree");
        assert_eq!(degree_centralities, vec![(0, 0), (1, 1), (2, 1)]);
    }

    #[test]
    fn test_simple_betweenness_centrality() {
        let mut graph = EdgeWeightedDigraph::new(4);
        graph.add_weight(0, 1);
        graph.add_weight(1, 2);
        graph.add_weight(2, 3);
        let simple_centralities = graph.simple_betweenness_centrality();
        assert_eq!(simple_centralities, vec![(3, 3), (2, 2), (1, 1)]);
    }
}


