//! Head-to-head speed and memory benchmark: rayon backend vs C libxc 7.0.0.
//!
//! Four legs per case, and the second is the one that matters:
//!
//! * `libxc-1t` — one `xc_*_exc_vxc` call. libxc's own API is serial.
//! * `libxc-Nt` — the grid cut into one chunk per core, each chunk its own
//!   `xc_*` call, run across a rayon pool. This is what a DFT code does with
//!   an OpenMP loop over grid batches, and it is the honest bar: beating
//!   serial libxc with 16 threads would prove nothing.
//! * `rust-1t`  — the same work with splitting disabled (`min_chunk = MAX`),
//!   so it runs entirely on the calling thread. Same core count, same libm as
//!   libxc, no parallelism: this isolates *kernel quality*.
//! * `rust-Nt`  — the library's own stride-aware parallel sweep.
//!
//! Every case also cross-checks `libxc-1t` against `rust-Nt` elementwise, so a
//! leg that silently skipped work cannot post a good time.

mod grid;
mod harness;

use harness::{Leg, report, run_interleaved};
use libxc_core::dims::Dimensions;
use libxc_core::input::{GgaInput, LdaInput, MggaInput};
use libxc_core::model::{DerivativeOrder, Spin, Thresholds};
use libxc_core::output::{GgaOutput, LdaOutput, MggaOutput};
use libxc_core::registry::lookup_by_name;
use libxc_reval::routing;
use rayon::prelude::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// ---------------------------------------------------------------------------
// Allocation counter
// ---------------------------------------------------------------------------

/// Counts Rust-side allocations, so "does an evaluation allocate?" is answered
/// by measurement rather than by reading the code. `malloc` inside libxc does
/// not pass through here; that side shows up in peak RSS instead.
struct Counting;

static ALLOC_CALLS: AtomicU64 = AtomicU64::new(0);
static ALLOC_BYTES: AtomicU64 = AtomicU64::new(0);
static LIVE_BYTES: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        ALLOC_CALLS.fetch_add(1, Ordering::Relaxed);
        ALLOC_BYTES.fetch_add(l.size() as u64, Ordering::Relaxed);
        LIVE_BYTES.fetch_add(l.size(), Ordering::Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        LIVE_BYTES.fetch_sub(l.size(), Ordering::Relaxed);
        unsafe { System.dealloc(p, l) }
    }
}

#[global_allocator]
static A: Counting = Counting;

fn alloc_snapshot() -> (u64, u64) {
    (
        ALLOC_CALLS.load(Ordering::Relaxed),
        ALLOC_BYTES.load(Ordering::Relaxed),
    )
}

// glibc's malloc arena totals. The Rust counter above only sees Rust's own
// allocations, which would let libxc allocate freely and still look clean --
// libxc is statically linked into this binary and reaches `malloc` directly.
// `mallinfo2` counts both sides, so it is the number that can fairly be
// compared between the two libraries.
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct MallInfo2 {
    arena: usize,
    ordblks: usize,
    smblks: usize,
    hblks: usize,
    hblkhd: usize,
    usmblks: usize,
    fsmblks: usize,
    uordblks: usize,
    fordblks: usize,
    keepcost: usize,
}
unsafe extern "C" {
    fn mallinfo2() -> MallInfo2;
}

/// Bytes currently handed out by malloc (`uordblks` + mmap'd `hblkhd`).
fn heap_in_use() -> usize {
    let m = unsafe { mallinfo2() };
    m.uordblks + m.hblkhd
}

/// Measure what a *single* evaluation costs in allocations, on both sides of
/// the allocator. Each closure is run once first: rayon builds its worker pool
/// and its per-thread state lazily, and libxc touches its own statics, so an
/// unwarmed first call would report one-time setup as if it were per-call cost.
fn alloc_probe(label: &str, mut once: impl FnMut()) {
    once();
    let (c0, b0) = alloc_snapshot();
    let h0 = heap_in_use();
    once();
    let (c1, b1) = alloc_snapshot();
    let h1 = heap_in_use();
    println!(
        "        one {label} evaluation: {} rust allocs / {} B, malloc in-use delta {:+} B",
        c1 - c0,
        b1 - b0,
        h1 as i64 - h0 as i64
    );
}

/// Peak resident set size in bytes. Monotonic — the kernel never lowers it.
fn vm_hwm() -> u64 {
    proc_status_kb("VmHWM:") * 1024
}
fn vm_rss() -> u64 {
    proc_status_kb("VmRSS:") * 1024
}
fn proc_status_kb(key: &str) -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with(key))
                .and_then(|l| l.split_whitespace().nth(1).and_then(|v| v.parse().ok()))
        })
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Raw base pointers for the chunk-parallel libxc leg
// ---------------------------------------------------------------------------

/// Chunks are disjoint by construction (each worker owns `[off, off+len)` at
/// every array's own stride), so handing each worker a base pointer plus an
/// offset is sound. Slices would need a 15-deep `par_chunks_mut().zip()` nest
/// for MGGA to express the same thing.
#[derive(Clone, Copy)]
struct P(*mut f64);
unsafe impl Send for P {}
unsafe impl Sync for P {}
#[derive(Clone, Copy)]
struct CP(*const f64);
unsafe impl Send for CP {}
unsafe impl Sync for CP {}

impl P {
    fn at(self, i: usize) -> *mut f64 {
        unsafe { self.0.add(i) }
    }
}
impl CP {
    fn at(self, i: usize) -> *const f64 {
        unsafe { self.0.add(i) }
    }
}

// ---------------------------------------------------------------------------
// Case description
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
enum Fam {
    Lda,
    Gga,
    Mgga,
    /// A composite (`xc_mix_init`) GGA such as HSE06. The Rust leg has to go
    /// through `Functional::evaluate_gga`, because the reval dispatch layer
    /// only knows individual kernels -- mixing lives one layer up.
    HybGga,
}

#[derive(Clone, Copy)]
struct Case {
    fam: Fam,
    name: &'static str,
    order: DerivativeOrder,
    spin: Spin,
}

fn nc(spin: Spin) -> usize {
    match spin {
        Spin::Unpolarized => 1,
        Spin::Polarized => 2,
    }
}

fn func_id(name: &str) -> i32 {
    lookup_by_name(&format!("XC_{name}"))
        .unwrap_or_else(|e| panic!("no libxc id for {name}: {e}"))
        .raw() as i32
}

/// An initialised C `xc_func_type`, freed on drop.
struct CFunc(*mut libxc_sys::xc_func_type);

impl CFunc {
    fn new(name: &str, spin: Spin) -> CFunc {
        unsafe {
            let p = libxc_sys::xc_func_alloc();
            assert!(!p.is_null(), "xc_func_alloc failed");
            let r = libxc_sys::xc_func_init(p, func_id(name), nc(spin) as i32);
            assert_eq!(r, 0, "xc_func_init failed for {name}");
            CFunc(p)
        }
    }
}
impl Drop for CFunc {
    fn drop(&mut self) {
        unsafe {
            libxc_sys::xc_func_end(self.0);
            libxc_sys::xc_func_free(self.0);
        }
    }
}
// libxc takes the functional `const` during evaluation and does not mutate it,
// so sharing one across the chunk-parallel leg is sound.
unsafe impl Sync for CFunc {}
unsafe impl Send for CFunc {}

// ---------------------------------------------------------------------------
// Driver
// ---------------------------------------------------------------------------

fn arg_val(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let np: usize = arg_val(&args, "--np")
        .and_then(|s| s.parse().ok())
        .unwrap_or(100_000);
    let reps: usize = arg_val(&args, "--reps")
        .and_then(|s| s.parse().ok())
        .unwrap_or(9);
    let only = arg_val(&args, "--only");

    let cases = [
        Case {
            fam: Fam::Lda,
            name: "lda_c_vwn",
            order: DerivativeOrder::Vxc,
            spin: Spin::Unpolarized,
        },
        Case {
            fam: Fam::Gga,
            name: "gga_x_b88",
            order: DerivativeOrder::Vxc,
            spin: Spin::Unpolarized,
        },
        Case {
            fam: Fam::Gga,
            name: "gga_c_lyp",
            order: DerivativeOrder::Vxc,
            spin: Spin::Unpolarized,
        },
        // PBE. The two halves are separate libxc functionals and separate
        // kernels here, so they are timed separately -- a DFT code asking for
        // "PBE" calls both. `gga_x_pbe` is the pure-arithmetic case (no
        // transcendental at all, 5 cbrt); `gga_c_pbe` carries 1 exp + 3 ln and
        // is the one the SIMD allowlist has a pending verdict on.
        Case {
            fam: Fam::Gga,
            name: "gga_x_pbe",
            order: DerivativeOrder::Vxc,
            spin: Spin::Unpolarized,
        },
        Case {
            fam: Fam::Gga,
            name: "gga_c_pbe",
            order: DerivativeOrder::Vxc,
            spin: Spin::Unpolarized,
        },
        Case {
            fam: Fam::Gga,
            name: "gga_x_pbe",
            order: DerivativeOrder::Vxc,
            spin: Spin::Polarized,
        },
        Case {
            fam: Fam::Gga,
            name: "gga_c_pbe",
            order: DerivativeOrder::Vxc,
            spin: Spin::Polarized,
        },
        Case {
            fam: Fam::Gga,
            name: "gga_x_pbe",
            order: DerivativeOrder::Fxc,
            spin: Spin::Unpolarized,
        },
        Case {
            fam: Fam::Gga,
            name: "gga_c_pbe",
            order: DerivativeOrder::Fxc,
            spin: Spin::Unpolarized,
        },
        // HSE06 itself: three auxiliary sweeps (two `gga_x_wpbeh` at different
        // screening parameters, plus `gga_c_pbe`) accumulated by the mix layer.
        Case {
            fam: Fam::HybGga,
            name: "hyb_gga_xc_hse06",
            order: DerivativeOrder::Vxc,
            spin: Spin::Unpolarized,
        },
        Case {
            fam: Fam::HybGga,
            name: "hyb_gga_xc_hse06",
            order: DerivativeOrder::Vxc,
            spin: Spin::Polarized,
        },
        // The screened-exchange leg HSE06 is built from. Timed on its own so
        // the composite's cost can be attributed: HSE06 evaluates this twice
        // (once per omega) plus `gga_c_pbe`.
        Case {
            fam: Fam::Gga,
            name: "gga_x_wpbeh",
            order: DerivativeOrder::Vxc,
            spin: Spin::Unpolarized,
        },
        Case {
            fam: Fam::Gga,
            name: "gga_x_b88",
            order: DerivativeOrder::Fxc,
            spin: Spin::Unpolarized,
        },
        Case {
            fam: Fam::Gga,
            name: "gga_x_b88",
            order: DerivativeOrder::Vxc,
            spin: Spin::Polarized,
        },
        Case {
            fam: Fam::Mgga,
            name: "mgga_x_scan",
            order: DerivativeOrder::Vxc,
            spin: Spin::Unpolarized,
        },
        Case {
            fam: Fam::Mgga,
            name: "mgga_c_r2scan",
            order: DerivativeOrder::Vxc,
            spin: Spin::Unpolarized,
        },
        Case {
            fam: Fam::Mgga,
            name: "mgga_x_scan",
            order: DerivativeOrder::Vxc,
            spin: Spin::Polarized,
        },
        // SIMD-allowlist candidates (docs/perf/simd-kernels.md): the most
        // libm-heavy routed kernels, where LLVM declines to vectorise the
        // grid loop. Their transcendentals are exp/ln only, so the SIMD form
        // is bit-identical to the scalar form and the fingerprint must not
        // move when one is added to SIMD_FUNCS.
        Case {
            fam: Fam::Mgga,
            name: "mgga_c_tpssloc",
            order: DerivativeOrder::Vxc,
            spin: Spin::Unpolarized,
        },
        Case {
            fam: Fam::Mgga,
            name: "mgga_c_scan",
            order: DerivativeOrder::Vxc,
            spin: Spin::Unpolarized,
        },
        Case {
            fam: Fam::Mgga,
            name: "mgga_c_rregtm",
            order: DerivativeOrder::Vxc,
            spin: Spin::Unpolarized,
        },
    ];

    let threads = rayon::current_num_threads();
    println!("grid points: {np}   rayon threads: {threads}   reps: {reps}");
    println!("load average at start: {:.2}", harness::loadavg());
    println!("min_chunk (rust-Nt): {}", default_min_chunk());
    println!(
        "tail fraction (points below every threshold): {:.0}%\n",
        grid::tail_fraction() * 100.0
    );

    for c in cases {
        if let Some(f) = &only {
            if !c.name.contains(f.as_str()) {
                continue;
            }
        }
        match c.fam {
            Fam::Lda => bench_lda(c, np, reps, threads),
            Fam::Gga => bench_gga(c, np, reps, threads),
            Fam::Mgga => bench_mgga(c, np, reps, threads),
            Fam::HybGga => bench_hyb_gga(c, np, reps, threads),
        }
    }
}

fn default_min_chunk() -> usize {
    std::env::var("XCVS_MIN_CHUNK")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2048)
}

fn set_min_chunk(fam: Fam, n: usize) {
    match fam {
        Fam::Lda => libxc_reval::sweep_lda::set_min_chunk(n),
        Fam::Gga => libxc_reval::sweep_gga::set_min_chunk(n),
        Fam::Mgga => libxc_reval::sweep_mgga::set_min_chunk(n),
        Fam::HybGga => libxc_reval::sweep_gga::set_min_chunk(n),
    }
}

fn order_name(o: DerivativeOrder) -> &'static str {
    match o {
        DerivativeOrder::Exc => "exc",
        DerivativeOrder::Vxc => "exc+vxc",
        DerivativeOrder::Fxc => "exc+vxc+fxc",
        DerivativeOrder::Kxc => "kxc",
        DerivativeOrder::Lxc => "lxc",
    }
}

fn header(c: &Case, np: usize) {
    println!("=========================================================================");
    println!(
        "{}  [{}]  {}  np={}",
        c.name,
        order_name(c.order),
        if c.spin == Spin::Unpolarized {
            "unpolarized"
        } else {
            "polarized"
        },
        np
    );
}

/// Order-sensitive fingerprint over the raw bits of every output.
///
/// This is what makes a codegen change (wider SIMD, a different `min_chunk`,
/// moving where the buffers are zeroed) checkable rather than merely plausible:
/// run the bench before and after, and the fingerprints must be identical. It
/// hashes `to_bits()`, so it separates `0.0` from `-0.0` and distinguishes NaN
/// payloads — exactly the differences a "looks the same" comparison hides.
fn fingerprint(bufs: &[(&str, &[f64])]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for (name, b) in bufs {
        for byte in name.as_bytes() {
            h = (h ^ *byte as u64).wrapping_mul(0x1000_0000_01b3);
        }
        for v in b.iter() {
            h = (h ^ v.to_bits()).wrapping_mul(0x1000_0000_01b3);
            h ^= h >> 29;
        }
    }
    h
}

/// Elementwise cross-check that both sides computed the same thing on the same
/// grid. Not the accuracy test — that is `crates/kernels-rayon/oracle` — but it
/// catches a leg that skipped work.
fn check(pairs: &[(&str, &[f64], &[f64])], name: &str) {
    // Scale for the cancellation guard below: the largest |zk| on this grid,
    // which is the natural magnitude of the functional here.
    let scale = pairs
        .iter()
        .find(|(f, _, _)| *f == "zk")
        .map(|(_, a, _)| a.iter().fold(0.0f64, |m, v| m.max(v.abs())))
        .unwrap_or(1.0);

    let mut worst = 0.0f64;
    let mut worst_field = "";
    for (f, a, b) in pairs {
        for (x, y) in a.iter().zip(b.iter()) {
            if x == y {
                continue;
            }
            if !x.is_finite() || !y.is_finite() {
                continue;
            }
            // Skip an element only when BOTH sides are negligible against the
            // functional's own scale. Without this the relative comparison
            // reports cancellation dust as signal: a `vsigma` that is 1e-20 on
            // both sides because the point has essentially no gradient can
            // differ in its last digits and show up as a 1e-7 "disagreement".
            // `gga_x_wpbeh` was flagged at 4.5e-7 this way while a direct
            // sweep over the same (rho, s) domain agrees to 2e-13 everywhere
            // and the rayon oracle reports 0 of 1221 fields out of tolerance.
            // Same rule the oracle harness already uses (AGENTS.md records the
            // `gga_k_tfvw` case that put it there).
            if x.abs() < scale * 1e-12 && y.abs() < scale * 1e-12 {
                continue;
            }
            let e = if y.abs() < 1e-280 {
                (x - y).abs()
            } else {
                ((x - y) / y).abs()
            };
            if e > worst {
                worst = e;
                worst_field = f;
            }
        }
    }
    let rust: Vec<(&str, &[f64])> = pairs.iter().map(|(f, _, b)| (*f, *b)).collect();
    let c: Vec<(&str, &[f64])> = pairs.iter().map(|(f, a, _)| (*f, *a)).collect();
    if worst > 1e-10 {
        println!("        !! {name}: legs disagree, worst {worst:.3e} on {worst_field}");
    } else {
        println!("        agreement libxc vs rust: worst rel {worst:.2e} ({worst_field})");
    }
    println!(
        "        fingerprint  rust {:016x}  libxc {:016x}",
        fingerprint(&rust),
        fingerprint(&c)
    );
}

fn mem_report(buf_mb: f64, in_mb: f64, hwm0: u64, rss0: u64, a0: (u64, u64)) {
    let a1 = alloc_snapshot();
    println!();
    println!(
        "memory: caller buffers {buf_mb:.1} MB/leg (x4 legs) | inputs {in_mb:.1} MB | RSS floor {:.1} MB",
        rss0 as f64 / 1e6
    );
    println!(
        "        peak RSS {:.1} MB (was {:.1} MB before timing)  => timing added {:.1} MB",
        vm_hwm() as f64 / 1e6,
        hwm0 as f64 / 1e6,
        (vm_hwm().saturating_sub(hwm0)) as f64 / 1e6
    );
    println!(
        "        rust-side allocations during timing: {} calls / {:.3} MB",
        a1.0 - a0.0,
        (a1.1 - a0.1) as f64 / 1e6
    );
}

/// Run `f(chunk_index, offset, len)` over one chunk per core.
fn par_chunks<F: Fn(usize, usize) + Sync>(np: usize, cn: usize, f: F) {
    (0..np.div_ceil(cn)).into_par_iter().for_each(|i| {
        let off = i * cn;
        f(off, cn.min(np - off));
    });
}

// ---------------------------------------------------------------------------
// LDA
// ---------------------------------------------------------------------------

fn bench_lda(c: Case, np: usize, reps: usize, threads: usize) {
    header(&c, np);
    let n = nc(c.spin);
    let d = Dimensions::lda(c.spin);
    let g = grid::lda(np, n, 0x1234);
    let cf = CFunc::new(c.name, c.spin);
    let th = Thresholds::default();
    let fxc = c.order >= DerivativeOrder::Fxc;

    let mk = || {
        (
            vec![0.0f64; np * d.zk as usize],
            vec![0.0f64; np * d.vrho as usize],
            vec![0.0f64; np * if fxc { d.v2rho2 as usize } else { 1 }],
        )
    };
    let (mut zk1, mut vr1, mut v21) = mk();
    let (mut zkn, mut vrn, mut v2n) = mk();
    let (mut zkr1, mut vrr1, mut v2r1) = mk();
    let (mut zkrn, mut vrrn, mut v2rn) = mk();

    let buf_mb = (zk1.len() + vr1.len() + v21.len()) as f64 * 8.0 / 1e6;
    let in_mb = g.rho.len() as f64 * 8.0 / 1e6;

    let rho = CP(g.rho.as_ptr());
    let (p_zk1, p_vr1, p_v21) = (
        P(zk1.as_mut_ptr()),
        P(vr1.as_mut_ptr()),
        P(v21.as_mut_ptr()),
    );
    let (p_zkn, p_vrn, p_v2n) = (
        P(zkn.as_mut_ptr()),
        P(vrn.as_mut_ptr()),
        P(v2n.as_mut_ptr()),
    );
    let cn = np.div_ceil(threads);
    let (dr, dz, dv, d2) = (
        d.rho as usize,
        d.zk as usize,
        d.vrho as usize,
        d.v2rho2 as usize,
    );
    let f = &cf;

    let call_c = move |p: (P, P, P), off: usize, len: usize| unsafe {
        if fxc {
            libxc_sys::xc_lda_exc_vxc_fxc(
                f.0,
                len,
                rho.at(off * dr),
                p.0.at(off * dz),
                p.1.at(off * dv),
                p.2.at(off * d2),
            );
        } else {
            libxc_sys::xc_lda_exc_vxc(
                f.0,
                len,
                rho.at(off * dr),
                p.0.at(off * dz),
                p.1.at(off * dv),
            );
        }
    };

    println!("per-call allocation probe (before timing):");
    alloc_probe("libxc", || call_c((p_zk1, p_vr1, p_v21), 0, np));
    alloc_probe("rust ", || {
        set_min_chunk(Fam::Lda, default_min_chunk());
        run_lda(&c, &g.rho, np, &th, &mut zkr1, &mut vrr1, &mut v2r1, fxc);
    });

    let rss0 = vm_rss();
    let hwm0 = vm_hwm();
    let a0 = alloc_snapshot();

    let rho_s: &[f64] = &g.rho;
    let mut legs = vec![
        Leg::new(
            "libxc-1t",
            Box::new(move || call_c((p_zk1, p_vr1, p_v21), 0, np)),
        ),
        Leg::new(
            "libxc-Nt",
            Box::new(move || {
                par_chunks(np, cn, |off, len| call_c((p_zkn, p_vrn, p_v2n), off, len))
            }),
        ),
        Leg::new(
            "rust-1t",
            Box::new(|| {
                set_min_chunk(Fam::Lda, usize::MAX);
                run_lda(&c, rho_s, np, &th, &mut zkr1, &mut vrr1, &mut v2r1, fxc);
            }),
        ),
        Leg::new(
            "rust-Nt",
            Box::new(|| {
                set_min_chunk(Fam::Lda, default_min_chunk());
                run_lda(&c, rho_s, np, &th, &mut zkrn, &mut vrrn, &mut v2rn, fxc);
            }),
        ),
    ];
    run_interleaved(&mut legs, 2, reps);
    report(&legs, np, "libxc-1t");
    drop(legs);

    mem_report(buf_mb, in_mb, hwm0, rss0, a0);
    let mut pairs: Vec<(&str, &[f64], &[f64])> = vec![("zk", &zk1, &zkrn), ("vrho", &vr1, &vrrn)];
    if fxc {
        pairs.push(("v2rho2", &v21, &v2rn));
    }
    check(&pairs, c.name);
    println!();
}

#[allow(clippy::too_many_arguments)]
fn run_lda(
    c: &Case,
    rho: &[f64],
    np: usize,
    th: &Thresholds,
    zk: &mut [f64],
    vrho: &mut [f64],
    v2: &mut [f64],
    fxc: bool,
) {
    let input = LdaInput::new(rho, np, c.spin).expect("lda input");
    let mut out = LdaOutput {
        zk: Some(&mut *zk),
        vrho: Some(&mut *vrho),
        v2rho2: if fxc { Some(&mut *v2) } else { None },
        ..Default::default()
    };
    routing::dispatch_lda_by_name(c.name, &input, &mut out, c.order, c.spin, th)
        .expect("routed")
        .expect("evaluated");
}

// ---------------------------------------------------------------------------
// GGA
// ---------------------------------------------------------------------------

struct GgaBufs {
    zk: Vec<f64>,
    vrho: Vec<f64>,
    vsigma: Vec<f64>,
    v2rho2: Vec<f64>,
    v2rhosigma: Vec<f64>,
    v2sigma2: Vec<f64>,
}

impl GgaBufs {
    fn new(np: usize, d: &Dimensions, fxc: bool) -> Self {
        let s = |k: usize| vec![0.0f64; if fxc { np * k } else { 1 }];
        GgaBufs {
            zk: vec![0.0; np * d.zk as usize],
            vrho: vec![0.0; np * d.vrho as usize],
            vsigma: vec![0.0; np * d.vsigma as usize],
            v2rho2: s(d.v2rho2 as usize),
            v2rhosigma: s(d.v2rhosigma as usize),
            v2sigma2: s(d.v2sigma2 as usize),
        }
    }
    fn mb(&self) -> f64 {
        (self.zk.len()
            + self.vrho.len()
            + self.vsigma.len()
            + self.v2rho2.len()
            + self.v2rhosigma.len()
            + self.v2sigma2.len()) as f64
            * 8.0
            / 1e6
    }
    fn ptrs(&mut self) -> [P; 6] {
        [
            P(self.zk.as_mut_ptr()),
            P(self.vrho.as_mut_ptr()),
            P(self.vsigma.as_mut_ptr()),
            P(self.v2rho2.as_mut_ptr()),
            P(self.v2rhosigma.as_mut_ptr()),
            P(self.v2sigma2.as_mut_ptr()),
        ]
    }
}

fn bench_gga(c: Case, np: usize, reps: usize, threads: usize) {
    header(&c, np);
    let n = nc(c.spin);
    let d = Dimensions::gga(c.spin);
    let g = grid::gga(np, n, 0x1234);
    let cf = CFunc::new(c.name, c.spin);
    let th = Thresholds::default();
    let fxc = c.order >= DerivativeOrder::Fxc;

    let mut b1 = GgaBufs::new(np, &d, fxc);
    let mut bn = GgaBufs::new(np, &d, fxc);
    let mut r1 = GgaBufs::new(np, &d, fxc);
    let mut rn = GgaBufs::new(np, &d, fxc);
    let buf_mb = b1.mb();
    let in_mb = (g.rho.len() + g.sigma.len()) as f64 * 8.0 / 1e6;

    let rho = CP(g.rho.as_ptr());
    let sigma = CP(g.sigma.as_ptr());
    let q1 = b1.ptrs();
    let qn = bn.ptrs();
    let st = [
        d.zk as usize,
        d.vrho as usize,
        d.vsigma as usize,
        d.v2rho2 as usize,
        d.v2rhosigma as usize,
        d.v2sigma2 as usize,
    ];
    let (dr, ds) = (d.rho as usize, d.sigma as usize);
    let cn = np.div_ceil(threads);
    let f = &cf;

    let call_c = move |q: [P; 6], off: usize, len: usize| unsafe {
        if fxc {
            libxc_sys::xc_gga_exc_vxc_fxc(
                f.0,
                len,
                rho.at(off * dr),
                sigma.at(off * ds),
                q[0].at(off * st[0]),
                q[1].at(off * st[1]),
                q[2].at(off * st[2]),
                q[3].at(off * st[3]),
                q[4].at(off * st[4]),
                q[5].at(off * st[5]),
            );
        } else {
            libxc_sys::xc_gga_exc_vxc(
                f.0,
                len,
                rho.at(off * dr),
                sigma.at(off * ds),
                q[0].at(off * st[0]),
                q[1].at(off * st[1]),
                q[2].at(off * st[2]),
            );
        }
    };

    println!("per-call allocation probe (before timing):");
    alloc_probe("libxc", || call_c(q1, 0, np));
    alloc_probe("rust ", || {
        set_min_chunk(Fam::Gga, default_min_chunk());
        run_gga(&c, &g.rho, &g.sigma, np, &th, &mut r1, fxc);
    });

    let rss0 = vm_rss();
    let hwm0 = vm_hwm();
    let a0 = alloc_snapshot();

    let (rho_s, sig_s): (&[f64], &[f64]) = (&g.rho, &g.sigma);
    let mut legs = vec![
        Leg::new("libxc-1t", Box::new(move || call_c(q1, 0, np))),
        Leg::new(
            "libxc-Nt",
            Box::new(move || par_chunks(np, cn, |off, len| call_c(qn, off, len))),
        ),
        Leg::new(
            "rust-1t",
            Box::new(|| {
                set_min_chunk(Fam::Gga, usize::MAX);
                run_gga(&c, rho_s, sig_s, np, &th, &mut r1, fxc);
            }),
        ),
        Leg::new(
            "rust-Nt",
            Box::new(|| {
                set_min_chunk(Fam::Gga, default_min_chunk());
                run_gga(&c, rho_s, sig_s, np, &th, &mut rn, fxc);
            }),
        ),
    ];
    run_interleaved(&mut legs, 2, reps);
    report(&legs, np, "libxc-1t");
    drop(legs);

    mem_report(buf_mb, in_mb, hwm0, rss0, a0);
    let mut pairs: Vec<(&str, &[f64], &[f64])> = vec![
        ("zk", &b1.zk, &rn.zk),
        ("vrho", &b1.vrho, &rn.vrho),
        ("vsigma", &b1.vsigma, &rn.vsigma),
    ];
    if fxc {
        pairs.push(("v2rho2", &b1.v2rho2, &rn.v2rho2));
        pairs.push(("v2rhosigma", &b1.v2rhosigma, &rn.v2rhosigma));
        pairs.push(("v2sigma2", &b1.v2sigma2, &rn.v2sigma2));
    }
    check(&pairs, c.name);
    println!();
}

fn run_gga(
    c: &Case,
    rho: &[f64],
    sigma: &[f64],
    np: usize,
    th: &Thresholds,
    b: &mut GgaBufs,
    fxc: bool,
) {
    let input = GgaInput::new(rho, sigma, np, c.spin).expect("gga input");
    let mut out = GgaOutput {
        zk: Some(&mut b.zk),
        vrho: Some(&mut b.vrho),
        vsigma: Some(&mut b.vsigma),
        v2rho2: if fxc { Some(&mut b.v2rho2) } else { None },
        v2rhosigma: if fxc { Some(&mut b.v2rhosigma) } else { None },
        v2sigma2: if fxc { Some(&mut b.v2sigma2) } else { None },
        ..Default::default()
    };
    routing::dispatch_gga_by_name(c.name, &input, &mut out, c.order, c.spin, th)
        .expect("routed")
        .expect("evaluated");
}

// ---------------------------------------------------------------------------
// Composite (mixed) GGA — HSE06 and friends
// ---------------------------------------------------------------------------

/// Time a composite functional, which neither `xcvs` nor the reval layer could
/// reach before.
///
/// The C side is unchanged: `xc_gga_exc_vxc` on a functional libxc built with
/// `xc_mix_init`, so libxc does its own mixing in `xc_mix_func`. The Rust side
/// has to go through `Functional::evaluate_gga`, which is the only path that
/// knows about auxiliaries; `routing::dispatch_gga_by_name` would evaluate one
/// kernel, not a mix.
///
/// The allocation probe matters more here than anywhere else in this harness.
/// libxc's `xc_mix_func` mallocs per call, and this library used to build a
/// fresh `EvaluationWorkspace` per call sized for the MGGA all-orders superset
/// -- 767 doubles per grid point polarized, against the 6 a GGA Vxc evaluation
/// writes. Both numbers show up below.
fn bench_hyb_gga(c: Case, np: usize, reps: usize, threads: usize) {
    use libxc_eval::eval::workspace::EvaluationWorkspace;
    use libxc_eval::functional::Functional;

    header(&c, np);
    let n = nc(c.spin);
    let d = Dimensions::gga(c.spin);
    let g = grid::gga(np, n, 0x1234);
    let cf = CFunc::new(c.name, c.spin);
    let fxc = c.order >= DerivativeOrder::Fxc;
    assert!(!fxc, "composite bench is exc+vxc only");

    let id = lookup_by_name(&format!("XC_{}", c.name)).expect("registry");
    let f = Functional::new(id, c.spin).expect("Functional::new");

    let mut b1 = GgaBufs::new(np, &d, fxc);
    let mut bn = GgaBufs::new(np, &d, fxc);
    let mut r1 = GgaBufs::new(np, &d, fxc);
    let mut rn = GgaBufs::new(np, &d, fxc);
    let buf_mb = b1.mb();
    let in_mb = (g.rho.len() + g.sigma.len()) as f64 * 8.0 / 1e6;

    let rho = CP(g.rho.as_ptr());
    let sigma = CP(g.sigma.as_ptr());
    let q1 = b1.ptrs();
    let qn = bn.ptrs();
    let st = [d.zk as usize, d.vrho as usize, d.vsigma as usize];
    let (dr, ds) = (d.rho as usize, d.sigma as usize);
    let cn = np.div_ceil(threads);
    let cfr = &cf;

    let call_c = move |q: [P; 6], off: usize, len: usize| unsafe {
        libxc_sys::xc_gga_exc_vxc(
            cfr.0,
            len,
            rho.at(off * dr),
            sigma.at(off * ds),
            q[0].at(off * st[0]),
            q[1].at(off * st[1]),
            q[2].at(off * st[2]),
        );
    };

    // One workspace, reused across every call -- which is the point. It starts
    // at `Exc` and `evaluate_mixed_gga` grows it once to the order in use.
    let mut ws1 = EvaluationWorkspace::with_order(np, c.spin, DerivativeOrder::Exc);
    let mut wsn = EvaluationWorkspace::with_order(np, c.spin, DerivativeOrder::Exc);

    let (rho_s, sig_s): (&[f64], &[f64]) = (&g.rho, &g.sigma);
    let run = |f: &Functional,
               ws: &mut EvaluationWorkspace,
               b: &mut GgaBufs| {
        let input = GgaInput::new(rho_s, sig_s, np, c.spin).expect("gga input");
        let mut out = GgaOutput {
            zk: Some(&mut b.zk),
            vrho: Some(&mut b.vrho),
            vsigma: Some(&mut b.vsigma),
            ..Default::default()
        };
        f.evaluate_gga(&input, c.order, &mut out, ws).expect("evaluated");
    };

    println!("per-call allocation probe (before timing):");
    alloc_probe("libxc", || call_c(q1, 0, np));
    alloc_probe("rust ", || {
        set_min_chunk(Fam::Gga, default_min_chunk());
        run(&f, &mut ws1, &mut r1);
    });
    println!(
        "        rust scratch: {} elems ({:.2} MB) for {} grid points, order {:?}",
        ws1.scratch_len(),
        ws1.scratch_len() as f64 * 8.0 / 1e6,
        np,
        ws1.alloc_order()
    );
    println!(
        "        (the all-orders MGGA superset this used to allocate: {} elems, {:.1} MB)",
        Dimensions::mgga(c.spin).total_output_components() * np,
        Dimensions::mgga(c.spin).total_output_components() as f64 * np as f64 * 8.0 / 1e6
    );

    let rss0 = vm_rss();
    let hwm0 = vm_hwm();
    let a0 = alloc_snapshot();

    let mut legs = vec![
        Leg::new("libxc-1t", Box::new(move || call_c(q1, 0, np))),
        Leg::new(
            "libxc-Nt",
            Box::new(move || par_chunks(np, cn, |off, len| call_c(qn, off, len))),
        ),
        Leg::new(
            "rust-1t",
            Box::new(|| {
                set_min_chunk(Fam::Gga, usize::MAX);
                run(&f, &mut ws1, &mut r1);
            }),
        ),
        Leg::new(
            "rust-Nt",
            Box::new(|| {
                set_min_chunk(Fam::Gga, default_min_chunk());
                run(&f, &mut wsn, &mut rn);
            }),
        ),
    ];
    run_interleaved(&mut legs, 2, reps);
    report(&legs, np, "libxc-1t");
    drop(legs);

    mem_report(buf_mb, in_mb, hwm0, rss0, a0);
    check(
        &[
            ("zk", &b1.zk, &rn.zk),
            ("vrho", &b1.vrho, &rn.vrho),
            ("vsigma", &b1.vsigma, &rn.vsigma),
        ],
        c.name,
    );
    println!();
}

// ---------------------------------------------------------------------------
// MGGA
// ---------------------------------------------------------------------------

/// Order-1 MGGA outputs only. Order 2 would need 10 more buffers; the Vxc case
/// is what an SCF actually spends its time in.
struct MggaBufs {
    zk: Vec<f64>,
    vrho: Vec<f64>,
    vsigma: Vec<f64>,
    vlapl: Vec<f64>,
    vtau: Vec<f64>,
}

impl MggaBufs {
    fn new(np: usize, d: &Dimensions) -> Self {
        MggaBufs {
            zk: vec![0.0; np * d.zk as usize],
            vrho: vec![0.0; np * d.vrho as usize],
            vsigma: vec![0.0; np * d.vsigma as usize],
            vlapl: vec![0.0; np * d.vlapl as usize],
            vtau: vec![0.0; np * d.vtau as usize],
        }
    }
    fn mb(&self) -> f64 {
        (self.zk.len() + self.vrho.len() + self.vsigma.len() + self.vlapl.len() + self.vtau.len())
            as f64
            * 8.0
            / 1e6
    }
    fn ptrs(&mut self) -> [P; 5] {
        [
            P(self.zk.as_mut_ptr()),
            P(self.vrho.as_mut_ptr()),
            P(self.vsigma.as_mut_ptr()),
            P(self.vlapl.as_mut_ptr()),
            P(self.vtau.as_mut_ptr()),
        ]
    }
}

fn bench_mgga(c: Case, np: usize, reps: usize, threads: usize) {
    header(&c, np);
    assert_eq!(c.order, DerivativeOrder::Vxc, "mgga bench covers order 1");
    let n = nc(c.spin);
    let d = Dimensions::mgga(c.spin);
    let g = grid::mgga(np, n, 0x1234);
    let cf = CFunc::new(c.name, c.spin);
    let th = Thresholds::default();

    let mut b1 = MggaBufs::new(np, &d);
    let mut bn = MggaBufs::new(np, &d);
    let mut r1 = MggaBufs::new(np, &d);
    let mut rn = MggaBufs::new(np, &d);
    let buf_mb = b1.mb();
    let in_mb = (g.rho.len() + g.sigma.len() + g.lapl.len() + g.tau.len()) as f64 * 8.0 / 1e6;

    let (rho, sigma) = (CP(g.rho.as_ptr()), CP(g.sigma.as_ptr()));
    let (lapl, tau) = (CP(g.lapl.as_ptr()), CP(g.tau.as_ptr()));
    let q1 = b1.ptrs();
    let qn = bn.ptrs();
    let st = [
        d.zk as usize,
        d.vrho as usize,
        d.vsigma as usize,
        d.vlapl as usize,
        d.vtau as usize,
    ];
    let (dr, ds, dl, dt) = (
        d.rho as usize,
        d.sigma as usize,
        d.lapl as usize,
        d.tau as usize,
    );
    let cn = np.div_ceil(threads);
    let f = &cf;

    let call_c = move |q: [P; 5], off: usize, len: usize| unsafe {
        libxc_sys::xc_mgga_exc_vxc(
            f.0,
            len,
            rho.at(off * dr),
            sigma.at(off * ds),
            lapl.at(off * dl),
            tau.at(off * dt),
            q[0].at(off * st[0]),
            q[1].at(off * st[1]),
            q[2].at(off * st[2]),
            q[3].at(off * st[3]),
            q[4].at(off * st[4]),
        );
    };

    println!("per-call allocation probe (before timing):");
    alloc_probe("libxc", || call_c(q1, 0, np));
    alloc_probe("rust ", || {
        set_min_chunk(Fam::Mgga, default_min_chunk());
        run_mgga(&c, &g.rho, &g.sigma, &g.lapl, &g.tau, np, &th, &mut r1);
    });

    let rss0 = vm_rss();
    let hwm0 = vm_hwm();
    let a0 = alloc_snapshot();

    let (rho_s, sig_s, lap_s, tau_s): (&[f64], &[f64], &[f64], &[f64]) =
        (&g.rho, &g.sigma, &g.lapl, &g.tau);
    let mut legs = vec![
        Leg::new("libxc-1t", Box::new(move || call_c(q1, 0, np))),
        Leg::new(
            "libxc-Nt",
            Box::new(move || par_chunks(np, cn, |off, len| call_c(qn, off, len))),
        ),
        Leg::new(
            "rust-1t",
            Box::new(|| {
                set_min_chunk(Fam::Mgga, usize::MAX);
                run_mgga(&c, rho_s, sig_s, lap_s, tau_s, np, &th, &mut r1);
            }),
        ),
        Leg::new(
            "rust-Nt",
            Box::new(|| {
                set_min_chunk(Fam::Mgga, default_min_chunk());
                run_mgga(&c, rho_s, sig_s, lap_s, tau_s, np, &th, &mut rn);
            }),
        ),
    ];
    run_interleaved(&mut legs, 2, reps);
    report(&legs, np, "libxc-1t");
    drop(legs);

    mem_report(buf_mb, in_mb, hwm0, rss0, a0);
    check(
        &[
            ("zk", &b1.zk, &rn.zk),
            ("vrho", &b1.vrho, &rn.vrho),
            ("vsigma", &b1.vsigma, &rn.vsigma),
            ("vtau", &b1.vtau, &rn.vtau),
        ],
        c.name,
    );
    println!();
}

#[allow(clippy::too_many_arguments)]
fn run_mgga(
    c: &Case,
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    np: usize,
    th: &Thresholds,
    b: &mut MggaBufs,
) {
    let input = MggaInput::new(rho, sigma, lapl, tau, np, c.spin).expect("mgga input");
    let mut out = MggaOutput {
        zk: Some(&mut b.zk),
        vrho: Some(&mut b.vrho),
        vsigma: Some(&mut b.vsigma),
        vlapl: Some(&mut b.vlapl),
        vtau: Some(&mut b.vtau),
        ..Default::default()
    };
    routing::dispatch_mgga_by_name(c.name, &input, &mut out, c.order, c.spin, th)
        .expect("routed")
        .expect("evaluated");
}
