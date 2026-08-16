//! LDA_C_GOMBAS exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gombas.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_C_GOMBAS exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_gombas_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3::<f64>(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = 1.0 + 0.0562 * t2;
        let t6 = 0.0357 / t4;
        let t7 = t2 + 2.39;
        let t9 = f64::ln(t7 * t1);
        let t10 = 0.0311 * t9;
        let tzk0 = -t6 - t10;
        zk[ip] += tzk0;
    }
}
