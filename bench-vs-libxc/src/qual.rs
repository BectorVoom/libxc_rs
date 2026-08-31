//! `xcqual` -- the SIMD-allowlist qualification harness.
//!
//! `xcvs` answers "how do we compare to C libxc". This answers a narrower and
//! much cheaper question: for one `(functional, order, spin)` triple, what does
//! *this* library cost, and what are the exact bits it produces? That is all the
//! SIMD allowlist gate needs (docs/perf/simd-kernels.md):
//!
//!   * the fingerprint must not move when the triple is added to
//!     `SIMD_EXACT_FUNCS` -- an exact check, not a tolerance, because every
//!     transcendental now has a bit-exact `libxc_rkernel_math::simd` form; and
//!   * the timing must improve.
//!
//! Dropping the C legs is what makes a sweep over hundreds of candidates
//! affordable: no `xc_func_type`, no second set of output buffers, and -- unlike
//! `xcvs` -- no per-family restriction on the derivative order, because nothing
//! here has to match a C entry point that may not exist. It shares `grid` and
//! `harness` with `xcvs` so the numbers sit on the same grid and the same
//! contention filter.
//!
//! Output is one machine-readable `QUAL` line per case, for
//! `tools/translate_rayon/simd_qualify.py` to parse.

use libxc_core::input::{GgaInput, LdaInput, MggaInput};
use libxc_core::model::{DerivativeOrder, Spin, Thresholds};
use libxc_core::output::{GgaOutput, LdaOutput, MggaOutput};
use libxc_reval::routing;

#[path = "grid.rs"]
mod grid;
#[path = "harness.rs"]
mod harness;

use harness::{Leg, report, run_interleaved};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Fam {
    Lda,
    Gga,
    Mgga,
}

struct Case {
    fam: Fam,
    name: String,
    order: DerivativeOrder,
    spin: Spin,
}

/// `fam:name:order:spin`, e.g. `gga:gga_c_lyp:vxc:unpol`.
fn parse_case(s: &str) -> Result<Case, String> {
    let p: Vec<&str> = s.split(':').collect();
    if p.len() != 4 {
        return Err(format!("expected fam:name:order:spin, got {s:?}"));
    }
    let fam = match p[0] {
        "lda" => Fam::Lda,
        "gga" => Fam::Gga,
        "mgga" => Fam::Mgga,
        o => return Err(format!("unknown family {o:?}")),
    };
    let order = match p[2] {
        "exc" => DerivativeOrder::Exc,
        "vxc" => DerivativeOrder::Vxc,
        "fxc" => DerivativeOrder::Fxc,
        "kxc" => DerivativeOrder::Kxc,
        "lxc" => DerivativeOrder::Lxc,
        o => return Err(format!("unknown order {o:?}")),
    };
    let spin = match p[3] {
        "unpol" => Spin::Unpolarized,
        "pol" => Spin::Polarized,
        o => return Err(format!("unknown spin {o:?}")),
    };
    Ok(Case { fam, name: p[1].to_string(), order, spin })
}

fn nc(spin: Spin) -> usize {
    match spin {
        Spin::Unpolarized => 1,
        Spin::Polarized => 2,
    }
}

/// Order-sensitive fingerprint over the raw bits of every output.
///
/// Byte-for-byte the function `xcvs` uses (`main.rs`), so a value printed here
/// is comparable with one printed there. It hashes `to_bits()`, so it separates
/// `0.0` from `-0.0` and distinguishes NaN payloads.
fn fingerprint(bufs: &[Vec<f64>]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bufs {
        for v in b.iter() {
            h = (h ^ v.to_bits()).wrapping_mul(0x1000_0000_01b3);
            h ^= h >> 29;
        }
    }
    h
}

fn arg_val(args: &[String], key: &str) -> Option<String> {
    args.iter().position(|a| a == key).and_then(|i| args.get(i + 1)).cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let np: usize = arg_val(&args, "--np").and_then(|s| s.parse().ok()).unwrap_or(100_000);
    let reps: usize = arg_val(&args, "--reps").and_then(|s| s.parse().ok()).unwrap_or(5);
    let quiet = args.iter().any(|a| a == "--quiet");
    let min_chunk: usize = arg_val(&args, "--min-chunk")
        .and_then(|s| s.parse().ok())
        .unwrap_or(2048);

    // Everything that is not a flag or a flag's value is a case.
    let mut cases = Vec::new();
    let mut i = 1;
    while i < args.len() {
        let a = &args[i];
        if a.starts_with("--") {
            // Flags that take a value consume the next argument.
            if matches!(a.as_str(), "--np" | "--reps" | "--min-chunk") {
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        match parse_case(a) {
            Ok(c) => cases.push(c),
            Err(e) => {
                eprintln!("xcqual: {e}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    if cases.is_empty() {
        eprintln!("usage: xcqual <fam:name:order:spin>... [--np N] [--reps R] [--quiet]");
        std::process::exit(2);
    }

    if !quiet {
        println!(
            "grid points: {np}   rayon threads: {}   reps: {reps}   min_chunk: {min_chunk}",
            rayon::current_num_threads()
        );
        println!("load average at start: {:.2}\n", harness::loadavg());
    }

    for c in &cases {
        match c.fam {
            Fam::Lda => qual_lda(c, np, reps, min_chunk, quiet),
            Fam::Gga => qual_gga(c, np, reps, min_chunk, quiet),
            Fam::Mgga => qual_mgga(c, np, reps, min_chunk, quiet),
        }
    }
}

/// Emit the one line the driver parses, plus (unless `--quiet`) the usual table.
fn emit(c: &Case, np: usize, legs: &[Leg<'_>], fp: u64, quiet: bool) {
    if !quiet {
        println!(
            "=== {} [{:?}] {} np={np}",
            c.name,
            c.order,
            if c.spin == Spin::Unpolarized { "unpol" } else { "pol" }
        );
        report(legs, np, "rust-1t");
        println!();
    }
    // A leg whose every rep was rejected has `best == INFINITY`. On a busy box
    // that is every leg, which would make the whole sweep unusable, so fall
    // back to the unfiltered best and report the contention alongside it: the
    // driver records `minforeign` in the ledger so a verdict taken on a loaded
    // machine can be told apart from one taken on a quiet one, rather than
    // being silently trusted.
    let ns = |n: &str| {
        legs.iter()
            .find(|l| l.name == n)
            .map(|l| if l.best.is_finite() { l.best } else { l.best_any } * 1e9 / np as f64)
            .unwrap_or(f64::NAN)
    };
    let rej: usize = legs.iter().map(|l| l.rejected).sum();
    let minforeign = legs
        .iter()
        .map(|l| l.min_contention)
        .fold(f64::INFINITY, f64::min);
    println!(
        "QUAL {}:{}:{}:{} np={np} ns1t={:.3} nsNt={:.3} fp={:016x} rejected={rej} \
         minforeign={minforeign:.2}",
        match c.fam {
            Fam::Lda => "lda",
            Fam::Gga => "gga",
            Fam::Mgga => "mgga",
        },
        c.name,
        match c.order {
            DerivativeOrder::Exc => "exc",
            DerivativeOrder::Vxc => "vxc",
            DerivativeOrder::Fxc => "fxc",
            DerivativeOrder::Kxc => "kxc",
            DerivativeOrder::Lxc => "lxc",
        },
        if c.spin == Spin::Unpolarized { "unpol" } else { "pol" },
        ns("rust-1t"),
        ns("rust-Nt"),
        fp,
    );
}

/// Report a case the eval layer does not route, and move on to the next one.
///
/// Returning rather than exiting matters: the driver hands this binary a whole
/// batch of cases at once, so exiting on the first unrouted one would silently
/// skip every case after it -- and the driver, seeing no `QUAL` line for them,
/// would mark them decided without ever having measured them.
fn unrouted(c: &Case) {
    println!("QUALSKIP {} reason=unrouted", c.name);
}

fn lda_strides(d: &libxc_core::dims::Dimensions) -> Vec<usize> {
    vec![
        d.zk as usize,
        d.vrho as usize,
        d.v2rho2 as usize,
        d.v3rho3 as usize,
        d.v4rho4 as usize,
    ]
}

fn qual_lda(c: &Case, np: usize, reps: usize, min_chunk: usize, quiet: bool) {
    use libxc_core::dims::Dimensions;
    let d = Dimensions::lda(c.spin);
    let g = grid::lda(np, nc(c.spin), 0x1234);
    let th = Thresholds::default();
    let strides = lda_strides(&d);
    let mk = || -> Vec<Vec<f64>> { strides.iter().map(|s| vec![0f64; np * s]).collect() };

    let run = |bufs: &mut Vec<Vec<f64>>| -> bool {
        let input = LdaInput::new(&g.rho, np, c.spin).expect("lda input");
        let mut it = bufs.iter_mut();
        let mut nx = || Some(it.next().unwrap().as_mut_slice());
        let mut out =
            LdaOutput { zk: nx(), vrho: nx(), v2rho2: nx(), v3rho3: nx(), v4rho4: nx() };
        matches!(
            routing::dispatch_lda_by_name(&c.name, &input, &mut out, c.order, c.spin, &th),
            Some(Ok(()))
        )
    };

    let mut probe = mk();
    libxc_reval::sweep_lda::set_min_chunk(min_chunk);
    if !run(&mut probe) {
        unrouted(c);
        return;
    }
    let fp = fingerprint(&probe);

    let mut b1 = mk();
    let mut bn = mk();
    let mut legs = vec![
        Leg::new(
            "rust-1t",
            Box::new(|| {
                libxc_reval::sweep_lda::set_min_chunk(usize::MAX);
                run(&mut b1);
            }),
        ),
        Leg::new(
            "rust-Nt",
            Box::new(|| {
                libxc_reval::sweep_lda::set_min_chunk(min_chunk);
                run(&mut bn);
            }),
        ),
    ];
    run_interleaved(&mut legs, 2, reps);
    emit(c, np, &legs, fp, quiet);
}

fn gga_strides(d: &libxc_core::dims::Dimensions) -> Vec<usize> {
    vec![
        d.zk as usize,
        d.vrho as usize,
        d.vsigma as usize,
        d.v2rho2 as usize,
        d.v2rhosigma as usize,
        d.v2sigma2 as usize,
        d.v3rho3 as usize,
        d.v3rho2sigma as usize,
        d.v3rhosigma2 as usize,
        d.v3sigma3 as usize,
        d.v4rho4 as usize,
        d.v4rho3sigma as usize,
        d.v4rho2sigma2 as usize,
        d.v4rhosigma3 as usize,
        d.v4sigma4 as usize,
    ]
}

fn qual_gga(c: &Case, np: usize, reps: usize, min_chunk: usize, quiet: bool) {
    use libxc_core::dims::Dimensions;
    let d = Dimensions::gga(c.spin);
    let g = grid::gga(np, nc(c.spin), 0x1234);
    let th = Thresholds::default();
    let strides = gga_strides(&d);
    let mk = || -> Vec<Vec<f64>> { strides.iter().map(|s| vec![0f64; np * s]).collect() };

    let run = |bufs: &mut Vec<Vec<f64>>| -> bool {
        let input = GgaInput::new(&g.rho, &g.sigma, np, c.spin).expect("gga input");
        let mut it = bufs.iter_mut();
        let mut nx = || Some(it.next().unwrap().as_mut_slice());
        let mut out = GgaOutput {
            zk: nx(),
            vrho: nx(),
            vsigma: nx(),
            v2rho2: nx(),
            v2rhosigma: nx(),
            v2sigma2: nx(),
            v3rho3: nx(),
            v3rho2sigma: nx(),
            v3rhosigma2: nx(),
            v3sigma3: nx(),
            v4rho4: nx(),
            v4rho3sigma: nx(),
            v4rho2sigma2: nx(),
            v4rhosigma3: nx(),
            v4sigma4: nx(),
        };
        matches!(
            routing::dispatch_gga_by_name(&c.name, &input, &mut out, c.order, c.spin, &th),
            Some(Ok(()))
        )
    };

    let mut probe = mk();
    libxc_reval::sweep_gga::set_min_chunk(min_chunk);
    if !run(&mut probe) {
        unrouted(c);
        return;
    }
    let fp = fingerprint(&probe);

    let mut b1 = mk();
    let mut bn = mk();
    let mut legs = vec![
        Leg::new(
            "rust-1t",
            Box::new(|| {
                libxc_reval::sweep_gga::set_min_chunk(usize::MAX);
                run(&mut b1);
            }),
        ),
        Leg::new(
            "rust-Nt",
            Box::new(|| {
                libxc_reval::sweep_gga::set_min_chunk(min_chunk);
                run(&mut bn);
            }),
        ),
    ];
    run_interleaved(&mut legs, 2, reps);
    emit(c, np, &legs, fp, quiet);
}

/// MGGA up to order 2. Orders 3 and 4 would need the other 55 of the 70 output
/// fields; the allowlist work reaches order 2 (`docs/perf/simd-ledger.md`), and
/// the high-order register-pressure question is answered on GGA, where the
/// bodies are just as large and all 15 fields are already wired.
fn mgga_strides(d: &libxc_core::dims::Dimensions) -> Vec<usize> {
    vec![
        d.zk as usize,
        d.vrho as usize,
        d.vsigma as usize,
        d.vlapl as usize,
        d.vtau as usize,
        d.v2rho2 as usize,
        d.v2rhosigma as usize,
        d.v2rholapl as usize,
        d.v2rhotau as usize,
        d.v2sigma2 as usize,
        d.v2sigmalapl as usize,
        d.v2sigmatau as usize,
        d.v2lapl2 as usize,
        d.v2lapltau as usize,
        d.v2tau2 as usize,
    ]
}

fn qual_mgga(c: &Case, np: usize, reps: usize, min_chunk: usize, quiet: bool) {
    use libxc_core::dims::Dimensions;
    if c.order > DerivativeOrder::Fxc {
        println!("QUALSKIP {} reason=mgga-order-above-fxc", c.name);
        return;
    }
    let d = Dimensions::mgga(c.spin);
    let g = grid::mgga(np, nc(c.spin), 0x1234);
    let th = Thresholds::default();
    let strides = mgga_strides(&d);
    let mk = || -> Vec<Vec<f64>> { strides.iter().map(|s| vec![0f64; np * s]).collect() };

    let run = |bufs: &mut Vec<Vec<f64>>| -> bool {
        let input =
            MggaInput::new(&g.rho, &g.sigma, &g.lapl, &g.tau, np, c.spin).expect("mgga input");
        let mut it = bufs.iter_mut();
        let mut nx = || Some(it.next().unwrap().as_mut_slice());
        let mut out = MggaOutput {
            zk: nx(),
            vrho: nx(),
            vsigma: nx(),
            vlapl: nx(),
            vtau: nx(),
            v2rho2: nx(),
            v2rhosigma: nx(),
            v2rholapl: nx(),
            v2rhotau: nx(),
            v2sigma2: nx(),
            v2sigmalapl: nx(),
            v2sigmatau: nx(),
            v2lapl2: nx(),
            v2lapltau: nx(),
            v2tau2: nx(),
            ..Default::default()
        };
        matches!(
            routing::dispatch_mgga_by_name(&c.name, &input, &mut out, c.order, c.spin, &th),
            Some(Ok(()))
        )
    };

    let mut probe = mk();
    libxc_reval::sweep_mgga::set_min_chunk(min_chunk);
    if !run(&mut probe) {
        unrouted(c);
        return;
    }
    let fp = fingerprint(&probe);

    let mut b1 = mk();
    let mut bn = mk();
    let mut legs = vec![
        Leg::new(
            "rust-1t",
            Box::new(|| {
                libxc_reval::sweep_mgga::set_min_chunk(usize::MAX);
                run(&mut b1);
            }),
        ),
        Leg::new(
            "rust-Nt",
            Box::new(|| {
                libxc_reval::sweep_mgga::set_min_chunk(min_chunk);
                run(&mut bn);
            }),
        ),
    ];
    run_interleaved(&mut legs, 2, reps);
    emit(c, np, &legs, fp, quiet);
}
