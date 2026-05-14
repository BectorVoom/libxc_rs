//! LDA_C_WIGNER fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_wigner.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_C_WIGNER fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_wigner_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t12 = param_b + t4 * t6 * t8 / 4.0;
        let tzk0 = param_a / t12;
        zk[ip] += tzk0;
        let t15 = t12 * t12;
        let t16 = 1.0 / t15;
        let tvrho0 = tzk0 + t8 * param_a * t16 * t4 * t6 / 12.0;
        vrho[ip] += tvrho0;
        let t22 = param_a * t16 * t1;
        let t23 = t3 * t6;
        let t28 = t7 * t7;
        let t33 = 1.0 / t15 / t12;
        let t35 = t1 * t1;
        let t36 = t3 * t3;
        let tv2rho20 = t22 * t23 / t7 / rho[ip] / 18.0 + 1.0 / t28 / rho[ip] * param_a * t33 * t35 * t36 * t5 / 18.0;
        v2rho2[ip] += tv2rho20;
    }
}
