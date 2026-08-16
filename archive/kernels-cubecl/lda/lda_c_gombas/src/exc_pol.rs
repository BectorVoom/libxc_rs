//! LDA_C_GOMBAS exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gombas.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_C_GOMBAS exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_gombas_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3::<f64>(t1);
        let t3 = 1.0 / t2;
        let t5 = 1.0 + 0.0562 * t3;
        let t7 = 0.0357 / t5;
        let t8 = t3 + 2.39;
        let t10 = f64::ln(t8 * t2);
        let t11 = 0.0311 * t10;
        let tzk0 = -t7 - t11;
        zk[ip] += tzk0;
    }
}
