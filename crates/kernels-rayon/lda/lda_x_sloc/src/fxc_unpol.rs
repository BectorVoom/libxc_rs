//! LDA_X_SLOC fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_sloc.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::piecewise::{piecewise3};

/// LDA_X_SLOC fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_x_sloc_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = param_b + 1.0;
        let t4 = param_a / t1 / 2.0;
        let t5 = f64::powf(rho[ip], param_b);
        let t7 = f64::powf(zeta_threshold, t1);
        let t8 = piecewise3(1.0 <= zeta_threshold, t7, 1.0);
        let t10 = t4 * t5 * t8;
        let tzk0 = -2.0 * t10;
        zk[ip] += tzk0;
        let tvrho0 = -2.0 * t4 * t5 * param_b * t8 - 2.0 * t10;
        vrho[ip] += tvrho0;
        let t16 = t4 * t5;
        let t17 = 1.0 / rho[ip];
        let t21 = param_b * param_b;
        let tv2rho20 = -2.0 * t16 * t21 * t17 * t8 - 2.0 * t16 * param_b * t17 * t8;
        v2rho2[ip] += tv2rho20;
    }
}
