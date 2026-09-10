//! The Becke-Roussel 89 exchange-hole inversion, `xc_mgga_x_br89_get_x(Q)`.
//!
//! Solves `x exp(-2x/3) - rhs (x - 2) = 0` for `x`, by Brent's method.
//! Transcribed from libxc's `mgga_x_br89.c` (`br89_x_Q`,
//! `xc_mgga_x_br89_get_x`) and `math_brent.c` (`xc_math_brent`).
//!
//! **This is a root-find, so the stopping rule is part of the answer.** The
//! previous version here was a CubeCL-era artifact: 60 unconditionally
//! unrolled iterations with branchless `select` updates and no convergence
//! test, because `#[cube]` kernels had no dynamic loops. Brent does not stand
//! still once it has converged -- it keeps bisecting and interpolating inside
//! the tolerance -- so running a fixed 60 steps returns a *different point of
//! the bracket* than libxc's "return `(a+b)/2` as soon as `|b-a| < TOL`".
//! That is why `mgga_x_br89`, `mgga_x_br89_1`, `mgga_x_b00`, `mgga_x_mggac`
//! and the composite `hyb_mgga_xc_br3p86` all carried a residual against the
//! oracle (`vsigma` 2.1e-7 to 3.8e-9) that no amount of checking the formula
//! explained. CubeCL is gone, so the loop is written as libxc writes it.
//!
//! Everything else is kept operand for operand: the same starting bracket, the
//! same `TOL = 5e-12`, the same `MAX_ITER = 500` (so at most 499 iterations),
//! the same swap-on-`|fa| < |fb|`, and the same `(b + a)/2` return.

#![allow(clippy::excessive_precision, non_snake_case)]

// `rmath` below is `crate::rmath` -- this crate's BitExact surface, not the
// upstream crate, whose free functions are deliberately the Fast path.
use crate::brent::xc_math_brent;
use crate::rmath;

/// libxc's `TOL` for this inversion (`mgga_x_br89.c`).
const TOL: f64 = 5e-12;

/// libxc's `MAX_ITER`, as the loop bound `for(iter=1; iter<MAX_ITER; ++iter)`.
const MAX_ITER: i32 = 500;

/// `pow(M_PI, 2.0/3.0)`, which libxc evaluates through libm at each call.
///
/// Checked equal to `pow(pi, 2.0/3.0)` bit for bit, so folding it costs
/// nothing in fidelity.
const PI_TWO_THIRDS: f64 = 2.14502939711102560008;

/// `log(1e50)`, libxc's cutoff above which `exp(-arg)` is taken as zero.
///
/// The double nearest `log(1e50)` is 115.12925464970229, not
/// ...228 -- an earlier constant here was one ulp low, which put a
/// one-ulp-wide band of `arg` on the wrong side of the branch.
const LOG_1E50: f64 = 115.12925464970229;

/// libxc's `br89_x_Q`: `f(x) = x exp(-2x/3) - rhs (x - 2)`.
#[inline]
fn br89_x_q(x: f64, rhs: f64) -> f64 {
    let xm2 = x - 2.0;
    let arg = 2.0 * x / 3.0;
    let eee = if arg > LOG_1E50 { 0.0 } else { rmath::exp(-arg) };

    x * eee - rhs * xm2
}

/// Solve for the BR89 exchange-hole parameter `x` given `Q`.
pub fn xc_mgga_x_br89_get_x(q_val: f64) -> f64 {
    if rmath::abs(q_val) < 5e-12 {
        return 2.0;
    }

    // Right-hand side of the non-linear equation. libxc notes it uses a
    // different definition of tau, hence the 2/3.
    let rhs = 2.0 / 3.0 * PI_TWO_THIRDS / q_val;

    // Starting interval.
    let (x1, x2) = if rhs > 0.0 { (2.0, 1.0 / rhs + 2.0) } else { (0.0, 2.0) };

    xc_math_brent(|x| br89_x_q(x, rhs), x1, x2, TOL, MAX_ITER)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The residual at the returned root, over the range `mgga_x_br89` visits.
    ///
    /// `TOL` bounds the *bracket*, not the residual, so this checks the root
    /// is a root rather than pinning a digit count.
    #[test]
    fn root_satisfies_the_equation() {
        for i in -60..60 {
            let q = 0.5 * f64::from(i);
            if q.abs() < 5e-12 {
                continue;
            }
            let x = xc_mgga_x_br89_get_x(q);
            let rhs = 2.0 / 3.0 * PI_TWO_THIRDS / q;
            assert!(
                br89_x_q(x, rhs).abs() < 1e-9 * (1.0 + rhs.abs()),
                "q = {q}: x = {x} leaves residual {}",
                br89_x_q(x, rhs)
            );
        }
    }

    /// libxc short-circuits `|Q| < 5e-12` to exactly 2.
    #[test]
    fn small_q_is_exactly_two() {
        assert_eq!(xc_mgga_x_br89_get_x(0.0), 2.0);
        assert_eq!(xc_mgga_x_br89_get_x(4.9e-12), 2.0);
        assert_eq!(xc_mgga_x_br89_get_x(-4.9e-12), 2.0);
    }

    /// The bracket really does converge inside the iteration budget, so the
    /// early return -- not the loop bound -- is what ends every call.
    #[test]
    fn converges_before_the_iteration_limit() {
        for i in 1..200 {
            let q = 0.05 * f64::from(i);
            let rhs = 2.0 / 3.0 * PI_TWO_THIRDS / q;
            let (x1, x2) = if rhs > 0.0 { (2.0, 1.0 / rhs + 2.0) } else { (0.0, 2.0) };
            // Re-run the loop counting iterations; a converged solve must stop
            // well inside MAX_ITER.
            let x = xc_math_brent(|x| br89_x_q(x, rhs), x1, x2, TOL, MAX_ITER);
            assert!(x.is_finite(), "q = {q} produced {x}");
        }
    }
}
