//! The `xc_integrate` calls that appear inside maple2c kernel bodies.
//!
//! Three functionals integrate at runtime: `gga_x_fd_lb94` (with its
//! `revlb94` sibling, which shares the kernel), `lda_x_1d_soft` and
//! `lda_x_1d_exponential`. Each wrapper below reproduces the corresponding
//! `xc_integrate(func, NULL, a, b)` call in libxc -- integrand and limits alike.
//!
//! # Why these now call QUADPACK
//!
//! This module used to implement composite Gauss-Legendre by hand, because
//! QUADPACK "uses malloc and function pointers, which are not available in
//! `#[cube]` kernels". That constraint died with the CubeCL backend; these are
//! ordinary Rust functions over `f64` now, and a `&dyn Fn` costs nothing here.
//!
//! It also could not have worked. libxc runs `dqagse` to
//! `epsabs = epsrel = 1e-10`, so libxc's own answer carries ~1e-10 of error.
//! Matching it to this project's 1e-12 contract means making the *same*
//! approximation, not a better one: the hand-written scheme was accurate to
//! ~1e-12 of the true integral and still missed libxc by 7.8e-8 on
//! `lda_x_1d_exponential`. [`crate::quadpack`] is a transcription of the
//! `dqagse` libxc ships, which is what closes that gap.

use crate::quadpack::xc_integrate;

// ---------------------------------------------------------------------------
// gga_x_fd_lb94
// ---------------------------------------------------------------------------

/// `xc_integrate(func0, NULL, 0.0, b)` **as libxc 7.0.0 actually computes it**.
///
/// Always `+0.0`. Both `gga_x_fd_lb94` integrands come from one helper in
/// `libxc-master/src/gga_x_fd_lb94.c`:
///
/// ```c
/// static inline double FT_inter(int n, double x)
/// {
///   static double fd_beta = 0.05, fd_csi = M_CBRT2;
///   double mlog = (n == 0) ? 1 : log(x);
///   return -3/4 * fd_beta*fd_csi*mlog / (1 + 3*fd_beta*fd_csi*x*log(...));
/// }
/// ```
///
/// `-3/4` divides two **integer** literals, so C evaluates it with integer
/// division: it is `0`, not `-0.75`. Every remaining factor is multiplied by
/// that zero, so the integrand is identically zero, both integrals vanish, and
/// the correction factor they feed (`t44 = 1 - ...`) collapses to `1`.
///
/// This is a defect in libxc -- the author meant `-3.0/4.0` -- but libxc is the
/// oracle this project is measured against, so reproducing it is the contract.
/// Integrating the *intended* function instead is what this module used to do,
/// and it is why `gga_x_fd_lb94` and `gga_x_fd_revlb94` were the two largest
/// non-hybrid entries in the oracle's offender list (188x on `vsigma`,
/// percent-level on `zk`/`vrho`).
///
/// `FT_inter` also hardcodes `fd_beta = 0.05`, so libxc ignores the
/// functional's own `_beta` external parameter here -- `gga_x_fd_revlb94` sets
/// `_beta = 0.004` and it never reaches the integrand. Both details only start
/// to matter if the `-3/4` is fixed upstream, at which point the integrand is
/// `-0.75 * beta * csi * mlog / (1 + 3*beta*csi*x*asinh(csi*x))` over `[0, b]`,
/// with a `log(x)` singularity at the lower limit for `func1`;
/// [`crate::quadpack::xc_integrate`] handles that shape directly.
#[inline(always)]
pub fn xc_integrate_func0(_b: f64, _beta: f64) -> f64 {
    0.0
}

/// `xc_integrate(func1, NULL, 0.0, b)` **as libxc 7.0.0 actually computes it**.
///
/// Always `+0.0`, for the reason given on [`xc_integrate_func0`]: `func1`
/// differs from `func0` only by a `log(x)` factor, which is still multiplied by
/// the zero that C's integer `-3/4` produces.
#[inline(always)]
pub fn xc_integrate_func1(_b: f64, _beta: f64) -> f64 {
    0.0
}

// ---------------------------------------------------------------------------
// lda_x_1d_soft
// ---------------------------------------------------------------------------

/// `FT_inter` for `lda_x_1d_soft`: `2 * K0(x)`.
#[inline]
fn soft_ft_inter(x: f64) -> f64 {
    2.0 * crate::bessel::xc_bessel_K0(x)
}

/// `xc_integrate(func1, NULL, 0.0, b)` for `lda_x_1d_soft`.
///
/// `K0` has a logarithmic singularity at the origin, which is what the adaptive
/// bisection and epsilon extrapolation in `dqagse` exist for.
pub fn xc_integrate_lda_soft_func1(b: f64) -> f64 {
    xc_integrate(&soft_ft_inter, 0.0, b)
}

/// `xc_integrate(func2, NULL, 0.0, b)` for `lda_x_1d_soft`: `x * 2 * K0(x)`.
pub fn xc_integrate_lda_soft_func2(b: f64) -> f64 {
    xc_integrate(&|x: f64| x * soft_ft_inter(x), 0.0, b)
}

// ---------------------------------------------------------------------------
// lda_x_1d_exponential
// ---------------------------------------------------------------------------

/// `FT_inter` for `lda_x_1d_exponential`: `E1_scaled(x^2)`.
#[inline]
fn exponential_ft_inter(x: f64) -> f64 {
    crate::expint_e1::xc_e1_scaled(x * x)
}

/// `xc_integrate(func1, NULL, 1e-20, b)` for `lda_x_1d_exponential`.
///
/// The lower limit is `1e-20`, not `0.0`. libxc's maple body writes it that way
/// and the two are not interchangeable: `dqagse` bisects the interval it is
/// handed, so a different lower limit yields a different sequence of panels and
/// a different answer at the 1e-10 level it works to.
pub fn xc_integrate_lda_exponential_func1(b: f64) -> f64 {
    xc_integrate(&exponential_ft_inter, 1e-20, b)
}

/// `xc_integrate(func2, NULL, 1e-20, b)` for `lda_x_1d_exponential`:
/// `x * E1_scaled(x^2)`.
pub fn xc_integrate_lda_exponential_func2(b: f64) -> f64 {
    xc_integrate(&|x: f64| x * exponential_ft_inter(x), 1e-20, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Guards the reading of libxc's integer `-3/4`: if someone "fixes" these
    /// to compute the mathematically intended integral, `gga_x_fd_lb94` stops
    /// matching libxc and the oracle fails on four fields per functional.
    #[test]
    fn fd_lb94_integrals_are_zero_for_every_argument() {
        for &b in &[0.0, 1e-12, 1e-3, 0.5, 1.0, 12.75, 1e6] {
            for &beta in &[0.004, 0.05, 1.0] {
                assert_eq!(xc_integrate_func0(b, beta).to_bits(), 0.0f64.to_bits());
                assert_eq!(xc_integrate_func1(b, beta).to_bits(), 0.0f64.to_bits());
            }
        }
    }

    /// `func2`'s integrand is `x` times `func1`'s, so the two must differ --
    /// a cheap guard against both wrappers being wired to the same closure.
    #[test]
    fn one_dimensional_integrals_are_distinct_and_finite() {
        for &b in &[1e-3, 0.25, 1.0, 8.0] {
            for (name, f1, f2) in [
                ("soft", xc_integrate_lda_soft_func1(b), xc_integrate_lda_soft_func2(b)),
                (
                    "exponential",
                    xc_integrate_lda_exponential_func1(b),
                    xc_integrate_lda_exponential_func2(b),
                ),
            ] {
                assert!(f1.is_finite(), "{name} func1 not finite at b={b}");
                assert!(f2.is_finite(), "{name} func2 not finite at b={b}");
                assert!(f1 != f2, "{name} func1 == func2 at b={b}");
            }
        }
    }
}
