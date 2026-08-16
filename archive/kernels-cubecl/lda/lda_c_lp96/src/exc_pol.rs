//! LDA_C_LP96 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_lp96.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_C_LP96 exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_lp96_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3::<f64>(t1);
        let t4 = param_C2 / t2;
        let t5 = t2 * t2;
        let t7 = param_C3 / t5;
        let tzk0 = param_C1 + t4 + t7;
        zk[ip] += tzk0;
    }
}
