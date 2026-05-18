//! Lambert W function for CubeCL kernels.
//!
//! W(z) is defined as the inverse of f(w) = w * exp(w), i.e., W(z) * exp(W(z)) = z.
//! Uses Halley's method with 15 iterations, matching libxc's `LambertW` implementation
//! in `special_functions.c`.
//! Generic over `<F: Float>` to support both f64 and f32.
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
pub fn lambert_w<F: Float>(z: F) -> F {
    let inv_e = F::new(1.0) / F::exp(F::new(1.0));
    let eps = F::new(1e-15);
    let cbrt_eps = F::powf(eps, F::new(1.0) / F::new(3.0));

    let mut result = F::new(0.0);

    if z + inv_e < F::new(-10.0) * eps {
        // Below branch: z < -1/e
        result = F::new(-1.0);
    } else if F::abs(z) < cbrt_eps {
        // Small z: power expansion
        result = z - z * z + F::new(1.5) * z * z * z;
    } else {
        // Initial guess based on region
        let mut w0 = F::new(0.0);
        if z <= F::new(-0.3140862435046707) {
            // Near branch point
            w0 = F::sqrt(F::new(2.0) * F::exp(F::new(1.0)) * z + F::new(2.0)) - F::new(1.0);
        } else if z <= F::new(1.149876485041417) {
            // Taylor around origin
            w0 = z - z * z + F::new(1.5) * z * z * z;
        } else {
            // Asymptotic expansion
            let lnz = F::ln(z);
            w0 = lnz - F::ln(lnz);
        }

        // Halley's iteration: 15 steps (unrolled)
        let w1 = halley_step::<F>(w0, z);
        let w2 = halley_step::<F>(w1, z);
        let w3 = halley_step::<F>(w2, z);
        let w4 = halley_step::<F>(w3, z);
        let w5 = halley_step::<F>(w4, z);
        let w6 = halley_step::<F>(w5, z);
        let w7 = halley_step::<F>(w6, z);
        let w8 = halley_step::<F>(w7, z);
        let w9 = halley_step::<F>(w8, z);
        let w10 = halley_step::<F>(w9, z);
        let w11 = halley_step::<F>(w10, z);
        let w12 = halley_step::<F>(w11, z);
        let w13 = halley_step::<F>(w12, z);
        let w14 = halley_step::<F>(w13, z);
        result = halley_step::<F>(w14, z);
    }

    result
}

/// Single Halley iteration step for Lambert W.
#[cube]
fn halley_step<F: Float>(w: F, z: F) -> F {
    let expmw = F::exp(-w);
    let residual = w - z * expmw;
    let denom = w + F::new(1.0) - (w + F::new(2.0)) / (F::new(2.0) * w + F::new(2.0)) * residual;
    // Guard against w == -1 (denom would be 0)
    let dw = select(F::abs(w + F::new(1.0)) < F::new(1.0e-300), F::new(0.0), -residual / denom);
    w + dw
}
