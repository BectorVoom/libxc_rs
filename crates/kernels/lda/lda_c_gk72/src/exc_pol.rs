//! LDA_C_GK72 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gk72.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise5};

/// LDA_C_GK72 exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t12 = t11 / 4.0;
        let t13 = t12 < 0.7;
        let t14 = f64::ln(t12);
        let t21 = t12 < 10.0;
        let t24 = t1 * t1;
        let t26 = t24 / t3;
        let t30 = f64::sqrt(4.0);
        let t31 = f64::sqrt(t11);
        let t36 = t3 * t3;
        let t38 = t1 / t36;
        let t39 = t8 * t8;
        let t43 = t24 * t36;
        let t45 = t5 / t39;
        let t49 = 1.0 / t31 / t43 / t45 / 4.0;
        let tzk0 = piecewise5(t13, 0.0311 * t14 - 0.048 + 0.00225 * t4 * t10 * t14 - 0.00425 * t11, t21, -0.06156 + 0.01898 * t14, 0.146 * t26 * t5 * t8 + 5.3 * t30 / t31 / t11 - 0.49 * t38 * t6 * t39 - 6.4 * t30 * t49);
        zk[ip] += tzk0;
    }
}
