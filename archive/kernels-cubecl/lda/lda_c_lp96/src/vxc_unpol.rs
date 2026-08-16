//! LDA_C_LP96 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_lp96.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_C_LP96 vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_lp96_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3::<f64>(rho[ip]);
        let t3 = param_C2 / t1;
        let t4 = t1 * t1;
        let t6 = param_C3 / t4;
        let tzk0 = param_C1 + t3 + t6;
        zk[ip] += tzk0;
        let t9 = param_C2 / t1 / rho[ip];
        let t13 = param_C3 / t4 / rho[ip];
        let tvrho0 = param_C1 + t3 + t6 + rho[ip] * (-t9 / 3.0 - 2.0 / 3.0 * t13);
        vrho[ip] += tvrho0;
    }
}
