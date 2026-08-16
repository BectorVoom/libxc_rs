//! LDA_K_GDS08_WORKER fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_gds08_worker.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

/// LDA_K_GDS08_WORKER fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_k_gds08_worker_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = 1.0 <= zeta_threshold;
        let t4 = zeta_threshold - 1.0;
        let t6 = piecewise5(t3, t4, t3, -t4, 0.0);
        let t7 = 1.0 + t6;
        let t9 = f64::ln(t7 * rho[ip]);
        let t11 = t9 * t9;
        let t16 = piecewise3(t2, 0.0, t7 * (param_C * t11 + param_B * t9 + param_A) / 2.0);
        let tzk0 = 2.0 * t16;
        zk[ip] += tzk0;
        let t17 = 1.0 / rho[ip];
        let t19 = param_C * t9;
        let t25 = piecewise3(t2, 0.0, t7 * (2.0 * t19 * t17 + param_B * t17) / 2.0);
        let tvrho0 = 2.0 * rho[ip] * t25 + 2.0 * t16;
        vrho[ip] += tvrho0;
        let t29 = rho[ip] * rho[ip];
        let t30 = 1.0 / t29;
        let t39 = piecewise3(t2, 0.0, t7 * (-2.0 * t19 * t30 - param_B * t30 + 2.0 * param_C * t30) / 2.0);
        let tv2rho20 = 2.0 * rho[ip] * t39 + 4.0 * t25;
        v2rho2[ip] += tv2rho20;
    }
}
