//! Interleaved, contention-filtered timing harness.
//!
//! Two problems this solves on the measurement box:
//!
//! 1. **Thermal drift.** The Ryzen AI 7 350 boosts to ~5 GHz on one core but
//!    drops far below that all-core. Running leg A to completion then leg B
//!    systematically favours whichever ran while the package was cool. So every
//!    rep runs *every* leg once, and the starting leg rotates each rep.
//!
//! 2. **Foreign CPU load.** An unrelated rust-analyzer loop on this machine
//!    saturates all 16 threads in ~2-minute bursts. Each rep measures how much
//!    CPU time processes *other than us* consumed while it ran (system busy
//!    jiffies from /proc/stat, minus our own from /proc/self/stat); reps above
//!    `CONTENTION_LIMIT` are discarded rather than averaged in.
//!
//! Reported figure is best-of-accepted-reps, the standard robust estimator when
//! noise is one-sided (contention can only ever make a run slower).

use std::time::Instant;

pub struct Leg<'a> {
    pub name: &'static str,
    pub run: Box<dyn FnMut() + 'a>,
    pub best: f64,
    pub total: f64,
    pub count: usize,
    pub rejected: usize,
    pub inner: usize,
    pub best_any: f64,
    pub min_contention: f64,
}

impl<'a> Leg<'a> {
    pub fn new(name: &'static str, run: Box<dyn FnMut() + 'a>) -> Self {
        Leg {
            name,
            run,
            best: f64::INFINITY,
            total: 0.0,
            count: 0,
            rejected: 0,
            inner: 1,
            best_any: f64::INFINITY,
            min_contention: f64::INFINITY,
        }
    }
}

/// System-wide busy jiffies (all CPUs) from /proc/stat.
fn sys_busy_ticks() -> f64 {
    let s = std::fs::read_to_string("/proc/stat").unwrap_or_default();
    let line = s.lines().next().unwrap_or("");
    let v: Vec<f64> = line
        .split_whitespace()
        .skip(1)
        .filter_map(|x| x.parse().ok())
        .collect();
    if v.len() < 5 {
        return 0.0;
    }
    let total: f64 = v.iter().sum();
    let idle = v[3] + v[4]; // idle + iowait
    total - idle
}

/// This process's own CPU jiffies (utime + stime), summed over all threads.
fn own_busy_ticks() -> f64 {
    let s = std::fs::read_to_string("/proc/self/stat").unwrap_or_default();
    let after = match s.rfind(')') {
        Some(i) => &s[i + 1..],
        None => return 0.0,
    };
    let f: Vec<&str> = after.split_whitespace().collect();
    // f[0] is state (field 3), so utime is index 11 and stime index 12.
    if f.len() < 13 {
        return 0.0;
    }
    f[11].parse().unwrap_or(0.0) + f[12].parse().unwrap_or(0.0)
}

const TICKS_PER_SEC: f64 = 100.0;
/// Discard a rep if foreign processes used more than this many core-seconds
/// per wall-second while it ran.
///
/// The 0.6 this started at assumed an otherwise-idle box. A background code
/// indexer holding ~0.5 cores is enough to reject *every* rep, which turns the
/// filtered column into `inf` and silently pushes the reader onto the
/// unfiltered fallback. Raise it with `XCVS_CONTENTION` when something else is
/// running: the estimator is best-of-accepted, and contention can only ever
/// make a run slower, so a looser gate costs accuracy far more gently than an
/// empty sample does.
pub fn contention_limit() -> f64 {
    std::env::var("XCVS_CONTENTION")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.5)
}
/// Target duration of one timed rep. Jiffy resolution is 10 ms, so reps need to
/// be long enough that the contention estimate is meaningful.
const TARGET_REP_SECS: f64 = 0.15;

pub fn loadavg() -> f64 {
    std::fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|s| s.split_whitespace().next().and_then(|x| x.parse().ok()))
        .unwrap_or(-1.0)
}

pub fn run_interleaved(legs: &mut [Leg<'_>], warmup: usize, reps: usize) {
    for _ in 0..warmup {
        for l in legs.iter_mut() {
            (l.run)();
        }
    }

    // Pilot pass: size each leg's inner loop so a timed rep lasts ~TARGET_REP_SECS.
    for l in legs.iter_mut() {
        let t0 = Instant::now();
        (l.run)();
        let est = t0.elapsed().as_secs_f64().max(1e-6);
        l.inner = ((TARGET_REP_SECS / est).ceil() as usize).max(1);
    }

    let limit = contention_limit();
    let nlegs = legs.len();
    for rep in 0..reps {
        for k in 0..nlegs {
            let i = (rep + k) % nlegs;
            let inner = legs[i].inner;

            let sb0 = sys_busy_ticks();
            let ob0 = own_busy_ticks();
            let t0 = Instant::now();
            for _ in 0..inner {
                (legs[i].run)();
            }
            let elapsed = t0.elapsed().as_secs_f64();
            let foreign = ((sys_busy_ticks() - sb0) - (own_busy_ticks() - ob0)) / TICKS_PER_SEC;
            let contention = (foreign / elapsed).max(0.0);

            let dt = elapsed / inner as f64;
            let l = &mut legs[i];
            if dt < l.best_any {
                l.best_any = dt;
            }
            if contention < l.min_contention {
                l.min_contention = contention;
            }
            if contention > limit {
                l.rejected += 1;
                continue;
            }
            if dt < l.best {
                l.best = dt;
            }
            l.total += dt;
            l.count += 1;
        }
    }
}

/// Print the standard results table. `unit` names what one "point" is.
pub fn report(legs: &[Leg<'_>], n: usize, baseline: &str) {
    let base = legs
        .iter()
        .find(|l| l.name == baseline)
        .map(|l| l.best)
        .unwrap_or(f64::NAN);

    println!(
        "{:<24} {:>10} {:>11} {:>13} {:>10} {:>9} {:>12}",
        "leg", "best", "per point", "throughput", "mean", "vs base", "reps ok/rej"
    );
    println!("{}", "-".repeat(97));
    for l in legs {
        println!(
            "{:<24} {:>7.2} ms {:>8.2} ns/pt {:>8.1} Mpts/s {:>7.2} ms {:>8.2}x {:>5}/{:<5}",
            l.name,
            l.best * 1e3,
            l.best * 1e9 / n as f64,
            n as f64 / l.best / 1e6,
            l.total / l.count.max(1) as f64 * 1e3,
            base / l.best,
            l.count,
            l.rejected,
        );
    }
    println!();
    println!("--- unfiltered fallback (best over ALL reps, incl. contended) ---");
    for l in legs {
        println!(
            "{:<24} {:>7.2} ms {:>8.2} ns/pt   (min foreign-CPU seen: {:.2} cores)",
            l.name,
            l.best_any * 1e3,
            l.best_any * 1e9 / n as f64,
            l.min_contention,
        );
    }
}
