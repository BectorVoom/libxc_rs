//! LDA_K_GDS08_WORKER vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_gds08_worker.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

/// LDA_K_GDS08_WORKER vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_k_gds08_worker_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = 1.0 <= zeta_threshold;
        let t4 = zeta_threshold - 1.0;
        let t6 = piecewise5::<f64>(t3, t4, t3, -t4, 0.0);
        let t7 = 1.0 + t6;
        let t9 = f64::ln(t7 * rho[ip]);
        let t11 = t9 * t9;
        let t16 = piecewise3::<f64>(t2, 0.0, t7 * (param_C * t11 + param_B * t9 + param_A) / 2.0);
        let tzk0 = 2.0 * t16;
        zk[ip] += tzk0;
        let t17 = 1.0 / rho[ip];
        let t19 = param_C * t9;
        let t25 = piecewise3::<f64>(t2, 0.0, t7 * (2.0 * t19 * t17 + param_B * t17) / 2.0);
        let tvrho0 = 2.0 * rho[ip] * t25 + 2.0 * t16;
        vrho[ip] += tvrho0;
    }
}
