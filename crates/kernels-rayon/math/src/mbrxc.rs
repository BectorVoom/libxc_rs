//! The modified Becke-Roussel cuspless-hole inversion,
//! `xc_mgga_x_mbrxc_get_x(Q)`.
//!
//! Solves `(1+x)^(5/3) exp(-2x/3) - rhs (x - 3) = 0` for `x`. Transcribed from
//! libxc's `mgga_x_mbrxc_bg.c` (`mbrxc_x_Q`, `xc_mgga_x_mbrxc_get_x`).
//! Called by `mgga_x_mbrxc_bg` and `mgga_x_mggac`.
//!
//! Two fidelity fixes went in with the rewrite off CubeCL, both of which moved
//! the answer:
//!
//! * The iteration is [`crate::brent::xc_math_brent`], which stops the moment
//!   `|b - a| < TOL`. It used to be 60 unconditionally unrolled steps with no
//!   convergence test, which lands on a different point of the same bracket --
//!   see the module docs there.
//! * `mbrxc_x_Q` no longer guards `exp(-2x/3)`. libxc's `br89_x_Q` does carry
//!   such a guard (`arg > log(1e50) ? 0 : exp(-arg)`); libxc's `mbrxc_x_Q`
//!   does **not**, and the guard copied across from `br89.rs` zeroed
//!   `exp(-arg)` from `arg > 115` upward, where the true value is still
//!   ~1e-50 rather than 0.

#![allow(clippy::excessive_precision, non_snake_case)]

use crate::brent::xc_math_brent;
use crate::rmath;

/// libxc's `TOL` for this inversion.
const TOL: f64 = 5e-12;

/// libxc's `MAX_ITER`, as the loop bound `for(iter=1; iter<MAX_ITER; ++iter)`.
const MAX_ITER: i32 = 500;

/// `pow(32.0*M_PI, 2.0/3.0)`, which libxc evaluates through libm at each call.
///
/// Checked equal to it bit for bit, so folding it costs nothing in fidelity.
const POW_32PI_TWO_THIRDS: f64 = 21.620541520507917;

/// libxc's `mbrxc_x_Q`: `f(x) = (1+x)^(5/3) exp(-2x/3) - rhs (x - 3)`.
#[inline]
fn mbrxc_x_q(x: f64, rhs: f64) -> f64 {
    rmath::pow(1.0 + x, 5.0 / 3.0) * rmath::exp(-2.0 * x / 3.0) - rhs * (x - 3.0)
}

/// Solve for the cuspless-hole parameter `x` given `Q`.
pub fn xc_mgga_x_mbrxc_get_x(q_val: f64) -> f64 {
    if rmath::abs(q_val) < 5e-12 {
        return 3.0;
    }

    // Right-hand side of the non-linear equation. libxc notes it uses a
    // different definition of tau, hence the 6.
    let rhs = POW_32PI_TWO_THIRDS / (6.0 * q_val);

    // Starting interval. libxc's comment: "I checked that the solution is
    // always in this interval".
    let (x1, x2) = if rhs > 0.0 { (3.0, 2.0 / rhs + 3.0) } else { (-1.0, 3.0) };

    xc_math_brent(|x| mbrxc_x_q(x, rhs), x1, x2, TOL, MAX_ITER)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The residual at the returned root. `TOL` bounds the bracket, not the
    /// residual, so this checks the root is a root rather than pinning digits.
    #[test]
    fn root_satisfies_the_equation() {
        for i in -60..60 {
            let q = 0.5 * f64::from(i);
            if q.abs() < 5e-12 {
                continue;
            }
            let x = xc_mgga_x_mbrxc_get_x(q);
            let rhs = POW_32PI_TWO_THIRDS / (6.0 * q);
            let r = mbrxc_x_q(x, rhs);
            assert!(r.abs() < 1e-8 * (1.0 + rhs.abs()), "q = {q}: x = {x} leaves residual {r}");
        }
    }

    /// libxc short-circuits `|Q| < 5e-12` to exactly 3.
    #[test]
    fn small_q_is_exactly_three() {
        assert_eq!(xc_mgga_x_mbrxc_get_x(0.0), 3.0);
        assert_eq!(xc_mgga_x_mbrxc_get_x(4.9e-12), 3.0);
    }

    /// libxc's `mbrxc_x_Q` has no underflow guard on `exp(-2x/3)`, and the
    /// guard this file used to carry (zero above `arg > log(1e50)`) is visible
    /// well before `exp` actually underflows.
    #[test]
    fn no_exp_underflow_guard() {
        // arg = 2x/3 = 120 > log(1e50) = 115.13, but exp(-120) is 7.7e-53.
        let x = 180.0;
        let with_guard = rmath::pow(1.0 + x, 5.0 / 3.0) * 0.0;
        assert_ne!(mbrxc_x_q(x, 0.0), with_guard);
    }
}
