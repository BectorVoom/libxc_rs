//! LDA_X_SLOC kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_sloc.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_X_SLOC kxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_x_sloc_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t26 = rho[ip] * rho[ip];
        let t27 = 1.0 / t26;
        let t31 = t21 * param_b;
        let tv3rho30 = -2.0 * t16 * t31 * t27 * t8 + 2.0 * t16 * param_b * t27 * t8;
        v3rho3[ip] += tv3rho30;
    }
}
