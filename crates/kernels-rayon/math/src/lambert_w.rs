//! The principal branch of the Lambert W function, `W(z) exp(W(z)) = z`.
//!
//! Transcribed from libxc's `LambertW` (`special_functions.c`): the same
//! branch tests, the same initial guesses, the same Halley step (eqn (5.9) in
//! Corless et al), the same 15-iteration budget and the same convergence test.
//!
//! Four things here used to differ from libxc, all of which moved the value:
//!
//! * **No convergence test.** All 15 Halley steps ran unconditionally, because
//!   CubeCL `#[cube]` kernels had no `return`. libxc returns at the first step
//!   whose `|dw| < 100 eps (1 + |w|)`; past that point the iteration keeps
//!   jittering `w` by an ulp or so, so 15 steps is a different answer from
//!   "stop when converged".
//! * **`eps` was 1e-15, not `DBL_EPSILON`.** That fed both the below-branch
//!   test and, through `CBRT(eps)`, the small-`z` power-series cutoff --
//!   1.0e-5 instead of libxc's 6.06e-6, so a band of `z` took the series where
//!   libxc iterates.
//! * **The `z` slightly below `-1/e` case was missing.** libxc returns exactly
//!   -1 there; this returned -1 for the *error* case instead and let the
//!   near-branch case fall through into the iteration.
//! * **`w != -1.0` was `|w + 1| < 1e-300`.** libxc compares exactly.
//!
//! libxc also returns **0.0** when the iteration limit is reached ("This
//! should never happen!"), and that is reproduced rather than returning the
//! last iterate.

#![allow(clippy::excessive_precision)]

// `rmath` below is `crate::rmath` -- this crate's BitExact surface, not the
// upstream crate, whose free functions are deliberately the Fast path.
use crate::rmath;

/// libxc's `M_E` (`util.h`), which is the double nearest `e`.
const M_E: f64 = std::f64::consts::E;

/// Evaluate the principal branch `W_0(z)`.
///
/// libxc prints a diagnostic and calls `exit(1)` for `z` meaningfully below
/// `-1/e`, where `W_0` is not defined. A library has no business terminating
/// the host process, so that argument returns -1 here -- the branch-point
/// value, and the same thing libxc returns just inside the domain. No caller
/// in this tree reaches it: the maple2c bodies feed `LambertW` arguments that
/// are non-negative by construction.
pub fn lambert_w(z: f64) -> f64 {
    // Sanity check: the function is only defined for z >= -1/e.
    if z + 1.0 / M_E < -10.0 * f64::EPSILON {
        return -1.0;
    } else if z < -1.0 / M_E {
        // W(x) at x = -1/e is -1.
        return -1.0;
    }

    // If z is small, go with the first terms of the power expansion: below the
    // cube root of epsilon, z^4 is zero to machine precision.
    if rmath::abs(z) < rmath::cbrt(f64::EPSILON) {
        return z - z * z + 1.5 * z * z * z;
    }

    // Initial guess.
    let mut w = if z <= -0.3140862435046707 {
        // Near the branching point: first terms in eqn (4.22).
        rmath::sqrt(2.0 * M_E * z + 2.0) - 1.0
    } else if z <= 1.149876485041417 {
        // Taylor series around the origin.
        z - z * z + 1.5 * z * z * z
    } else {
        // Asymptotic expansion.
        let lnz = rmath::ln(z);
        lnz - rmath::ln(lnz)
    };

    // Find the result through iteration.
    for _ in 0..15 {
        let expmw = rmath::exp(-w);

        // Halley's equation, (5.9) in Corless et al.
        let dw = if w != -1.0 {
            -(w - z * expmw) / (w + 1.0 - (w + 2.0) / (2.0 * w + 2.0) * (w - z * expmw))
        } else {
            0.0
        };

        w += dw;
        if rmath::abs(dw) < 100.0 * f64::EPSILON * (1.0 + rmath::abs(w)) {
            return w;
        }
    }

    // libxc: "This should never happen!" -- it warns and returns zero.
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `W(z) exp(W(z)) = z` over the range the kernels feed it.
    #[test]
    fn inverts_w_exp_w() {
        for i in 0..500 {
            let z = 0.01 * f64::from(i);
            let w = lambert_w(z);
            let back = w * rmath::exp(w);
            assert!(
                (back - z).abs() <= 1e-12 * (1.0 + z.abs()),
                "z = {z}: W = {w} maps back to {back}"
            );
        }
    }

    /// The branch point and just below it.
    #[test]
    fn branch_point_is_minus_one() {
        assert_eq!(lambert_w(-1.0 / M_E - 1e-9), -1.0);
        assert_eq!(lambert_w(-1.0), -1.0);
    }

    /// Known values.
    #[test]
    fn known_values() {
        assert_eq!(lambert_w(0.0), 0.0);
        // W(e) = 1
        assert!((lambert_w(M_E) - 1.0).abs() < 1e-14, "{}", lambert_w(M_E));
        // W(1) = Omega = 0.5671432904097838...
        assert!((lambert_w(1.0) - 0.567_143_290_409_783_8).abs() < 1e-15);
    }

    /// Every argument in range converges inside the 15-step budget, so the
    /// "should never happen" zero return is never the answer.
    #[test]
    fn always_converges_within_the_budget() {
        for i in 0..2000 {
            let z = -0.36 + 0.01 * f64::from(i);
            if z + 1.0 / M_E < -10.0 * f64::EPSILON || z < -1.0 / M_E {
                continue;
            }
            let w = lambert_w(z);
            if rmath::abs(z) < rmath::cbrt(f64::EPSILON) {
                continue;
            }
            assert_ne!(w, 0.0, "z = {z} hit the iteration limit");
        }
    }
}
