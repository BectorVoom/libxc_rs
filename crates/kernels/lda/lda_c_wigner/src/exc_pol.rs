//! LDA_C_WIGNER exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_wigner.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_C_WIGNER exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_wigner_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 - rho1;
        let t2 = t1 * t1;
        let t3 = rho0 + rho1;
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = -t2 * t5 + 1.0;
        let t8 = t7 * param_a;
        let t9 = M_CBRT3;
        let t10 = 1.0 / M_PI;
        let t11 = pow_1_3::<f64>(t10);
        let t12 = t9 * t11;
        let t13 = M_CBRT4;
        let t14 = t13 * t13;
        let t15 = pow_1_3::<f64>(t3);
        let t16 = 1.0 / t15;
        let t20 = param_b + t12 * t14 * t16 / 4.0;
        let t21 = 1.0 / t20;
        let tzk0 = t8 * t21;
        zk[ip] += tzk0;
    }
}
