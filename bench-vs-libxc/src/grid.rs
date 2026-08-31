//! Physically-representative grid generation.
//!
//! Timing a functional on unphysical inputs measures the wrong thing: most XC
//! kernels branch on density/zeta thresholds, and a grid of garbage spends its
//! time in the cutoff arm rather than in the formula. These grids sit where a
//! real molecular quadrature sits — density log-uniform over the range a
//! Becke/Lebedev grid actually samples, reduced gradient in [0, 3], and (for
//! MGGA) `tau` at or above the von Weizsaecker bound so the point is inside the
//! functional's domain.

/// Deterministic splitmix64 — no `rand` dependency, and identical grids across
/// runs so two legs are never compared on different data.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed)
    }
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform in [0, 1).
    pub fn f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
    /// Uniform in [lo, hi).
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.f64()
    }
    /// Log-uniform in [lo, hi) — the right distribution for a density, which
    /// spans orders of magnitude across a molecular grid.
    pub fn log_range(&mut self, lo: f64, hi: f64) -> f64 {
        (lo.ln() + (hi.ln() - lo.ln()) * self.f64()).exp()
    }
}

/// Density range a molecular quadrature samples in its *chemically active*
/// region, in atomic units.
const RHO_LO: f64 = 1e-6;
const RHO_HI: f64 = 1e1;

/// Fraction of grid points placed in the far tail, below every threshold.
///
/// A real Becke/Lebedev grid puts a large share of its points in the outer
/// radial shells, where the density is numerically zero. This matters for the
/// comparison and not only for realism: libxc's `work_*_inc.c` loop `continue`s
/// on `dens < dens_threshold` and does no arithmetic at all for such a point,
/// whereas the kernels here evaluate the whole formula and then select 0
/// through a branch-free `piecewise3`. So the tail fraction is exactly the knob
/// that exposes the screening gap. Set with `XCVS_TAIL` (0.0 by default, so the
/// headline numbers describe the active region only).
pub fn tail_fraction() -> f64 {
    std::env::var("XCVS_TAIL")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0)
}

/// Density for a tail point: far below both libraries' thresholds.
const RHO_TAIL: f64 = 1e-30;
/// Reduced gradient `s`. 0 is uniform-gas, ~3 is a typical valence maximum.
const S_HI: f64 = 3.0;
/// `alpha = (tau - tau_W) / tau_unif`. 0 is a single orbital / covalent bond,
/// 1 is the uniform gas, >1 is overlap / vdW region.
const ALPHA_HI: f64 = 3.0;

fn kf(rho: f64) -> f64 {
    (3.0 * std::f64::consts::PI * std::f64::consts::PI * rho).cbrt()
}

/// Uniform-gas kinetic energy density.
fn tau_unif(rho: f64) -> f64 {
    0.3 * kf(rho) * kf(rho) * rho
}

pub struct LdaGrid {
    pub rho: Vec<f64>,
}

pub struct GgaGrid {
    pub rho: Vec<f64>,
    pub sigma: Vec<f64>,
}

pub struct MggaGrid {
    pub rho: Vec<f64>,
    pub sigma: Vec<f64>,
    pub lapl: Vec<f64>,
    pub tau: Vec<f64>,
}

/// How the tail points are laid out.
///
/// `block` (the default) is what a real quadrature looks like: points come
/// ordered by radial shell, so the numerically-empty ones arrive in contiguous
/// stretches. `scatter` sprinkles them individually, which no real grid does
/// but which is the worst case for anything that screens by splitting the grid
/// into runs -- so it is the one to check for a regression, not the one to
/// quote. Set with `XCVS_TAIL_LAYOUT=scatter`.
fn scattered() -> bool {
    std::env::var("XCVS_TAIL_LAYOUT")
        .map(|v| v == "scatter")
        .unwrap_or(false)
}

/// Length of one simulated radial batch in the `block` layout.
const BATCH: usize = 1024;

/// Is grid point `ip` in the tail?
fn is_tail(r: &mut Rng, ip: usize, tail: f64, scatter: bool) -> bool {
    if tail <= 0.0 {
        return false;
    }
    if scatter {
        r.f64() < tail
    } else {
        // Last `tail` fraction of each batch: the outer shells.
        (ip % BATCH) as f64 >= BATCH as f64 * (1.0 - tail)
    }
}

fn draw_rho(r: &mut Rng, ip: usize, tail: f64, scatter: bool) -> f64 {
    if is_tail(r, ip, tail, scatter) {
        RHO_TAIL
    } else {
        r.log_range(RHO_LO, RHO_HI)
    }
}

/// `nc` is the number of density channels: 1 unpolarized, 2 polarized.
pub fn lda(np: usize, nc: usize, seed: u64) -> LdaGrid {
    let mut r = Rng::new(seed);
    let (tail, scatter) = (tail_fraction(), scattered());
    let mut rho = Vec::with_capacity(np * nc);
    for ip in 0..np {
        for _ in 0..nc {
            rho.push(draw_rho(&mut r, ip, tail, scatter));
        }
    }
    LdaGrid { rho }
}

pub fn gga(np: usize, nc: usize, seed: u64) -> GgaGrid {
    let mut r = Rng::new(seed);
    let (tail, scatter) = (tail_fraction(), scattered());
    let ns = if nc == 1 { 1 } else { 3 };
    let mut rho = Vec::with_capacity(np * nc);
    let mut sigma = Vec::with_capacity(np * ns);
    for ip in 0..np {
        // Per-channel densities, then sigma consistent with them. A tail point
        // is a tail point in every channel — a half-empty point would still be
        // above the screening threshold and so would not test what this is for.
        let is_tail = is_tail(&mut r, ip, tail, scatter);
        let mut ch = [0.0f64; 2];
        for c in 0..nc {
            ch[c] = if is_tail {
                RHO_TAIL
            } else {
                r.log_range(RHO_LO, RHO_HI)
            };
            rho.push(ch[c]);
        }
        if nc == 1 {
            let g = r.range(0.0, S_HI) * 2.0 * kf(ch[0]) * ch[0];
            sigma.push(g * g);
        } else {
            // sigma_aa, sigma_ab, sigma_bb. The off-diagonal is bounded by
            // Cauchy-Schwarz; violating it puts the point outside the domain.
            let ga = r.range(0.0, S_HI) * 2.0 * kf(ch[0]) * ch[0];
            let gb = r.range(0.0, S_HI) * 2.0 * kf(ch[1]) * ch[1];
            let cos = r.range(-1.0, 1.0);
            sigma.push(ga * ga);
            sigma.push(ga * gb * cos);
            sigma.push(gb * gb);
        }
    }
    GgaGrid { rho, sigma }
}

pub fn mgga(np: usize, nc: usize, seed: u64) -> MggaGrid {
    let g = gga(np, nc, seed);
    let mut r = Rng::new(seed ^ 0xA5A5_5A5A);
    let mut lapl = Vec::with_capacity(np * nc);
    let mut tau = Vec::with_capacity(np * nc);
    for ip in 0..np {
        for c in 0..nc {
            let rho_c = g.rho[ip * nc + c];
            // Per-channel sigma for the von Weizsaecker bound. Polarized
            // kernels use the spin-scaled form, so take the diagonal element.
            let sig_c = if nc == 1 {
                g.sigma[ip]
            } else if c == 0 {
                g.sigma[ip * 3]
            } else {
                g.sigma[ip * 3 + 2]
            };
            // Polarized channels carry half the uniform-gas density each.
            let (rs, ss) = if nc == 1 {
                (rho_c, sig_c)
            } else {
                (2.0 * rho_c, 4.0 * sig_c)
            };
            let tw = ss / (8.0 * rs);
            // tau >= tau_W is the exact constraint; staying above it keeps the
            // point inside the domain, so the kernel runs its real branch
            // rather than producing NaN.
            let t = tw + r.range(0.0, ALPHA_HI) * tau_unif(rs);
            tau.push(if nc == 1 { t } else { t / 2.0 });
            lapl.push(r.range(-1.0, 1.0) * tau_unif(rs) * 4.0);
        }
    }
    MggaGrid {
        rho: g.rho,
        sigma: g.sigma,
        lapl,
        tau,
    }
}
