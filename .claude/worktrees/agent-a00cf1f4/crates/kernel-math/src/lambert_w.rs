//! Lambert W function for CubeCL kernels.
//!
//! W(z) is defined as the inverse of f(w) = w * exp(w), i.e., W(z) * exp(W(z)) = z.
//! Uses Halley's method with 15 iterations, matching libxc's `LambertW` implementation
//! in `special_functions.c`.
//!
//! Matches the libxc C original's control flow using `if/else` guards for
//! below-branch, small-z, and initial guess region selection. CubeCL 0.9.0
//! does not support `return`, so we use mutable result + `if/else` instead
//! of early returns. The 15 Halley iterations remain unrolled (libxc's
//! convergence-detected early return inside the loop cannot be replicated
//! without `return` support).

use cubecl::prelude::*;

/// Evaluate the principal branch of the Lambert W function.
///
/// For z >= -1/e, returns W_0(z). Uses `if/else` guards for special cases
/// and initial guess selection, with 15 unrolled Halley iteration steps.
#[cube]
pub fn lambert_w(z: f64) -> f64 {
    let inv_e = 1.0 / f64::exp(1.0);
    let eps = f64::EPSILON;
    let cbrt_eps = f64::powf(eps, 1.0 / 3.0);

    let mut result = 0.0f64;

    if z + inv_e < -10.0 * eps {
        // Below branch: z < -1/e
        result = -1.0;
    } else if f64::abs(z) < cbrt_eps {
        // Small z: power expansion
        result = z - z * z + 1.5 * z * z * z;
    } else {
        // Initial guess based on region
        let mut w0 = 0.0f64;
        if z <= -0.3140862435046707 {
            // Near branch point
            w0 = f64::sqrt(2.0 * f64::exp(1.0) * z + 2.0) - 1.0;
        } else if z <= 1.149876485041417 {
            // Taylor around origin
            w0 = z - z * z + 1.5 * z * z * z;
        } else {
            // Asymptotic expansion
            let lnz = f64::ln(z);
            w0 = lnz - f64::ln(lnz);
        }

        // Halley's iteration: 15 steps (unrolled)
        let w1 = halley_step(w0, z);
        let w2 = halley_step(w1, z);
        let w3 = halley_step(w2, z);
        let w4 = halley_step(w3, z);
        let w5 = halley_step(w4, z);
        let w6 = halley_step(w5, z);
        let w7 = halley_step(w6, z);
        let w8 = halley_step(w7, z);
        let w9 = halley_step(w8, z);
        let w10 = halley_step(w9, z);
        let w11 = halley_step(w10, z);
        let w12 = halley_step(w11, z);
        let w13 = halley_step(w12, z);
        let w14 = halley_step(w13, z);
        result = halley_step(w14, z);
    }

    result
}

/// Single Halley iteration step for Lambert W.
#[cube]
fn halley_step(w: f64, z: f64) -> f64 {
    let expmw = f64::exp(-w);
    let residual = w - z * expmw;
    let denom = w + 1.0 - (w + 2.0) / (2.0 * w + 2.0) * residual;
    // Guard against w == -1 (denom would be 0)
    let dw = select(f64::abs(w + 1.0) < 1.0e-300, 0.0f64, -residual / denom);
    w + dw
}
