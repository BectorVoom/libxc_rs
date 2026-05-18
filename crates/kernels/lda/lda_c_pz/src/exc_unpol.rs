//! LDA_C_PZ exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pz.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_PZ exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_pz_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_a_0: f64,
    param_a_1: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_beta1_0: f64,
    param_beta1_1: f64,
    param_beta2_0: f64,
    param_beta2_1: f64,
    param_c_0: f64,
    param_c_1: f64,
    param_d_0: f64,
    param_d_1: f64,
    param_gamma_0: f64,
    param_gamma_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3::<f64>(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3::<f64>(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t1 * t3 * t9;
        let t11 = t10 / 4.0;
        let t12 = 1.0 <= t11;
        let t13 = param_gamma_0;
        let t14 = param_beta1_0;
        let t15 = f64::sqrt(t10);
        let t19 = param_beta2_0 * t1;
        let t20 = t3 * t6;
        let t21 = t20 * t8;
        let t24 = 1.0 + t14 * t15 / 2.0 + t19 * t21 / 4.0;
        let t27 = param_a_0;
        let t28 = f64::ln(t11);
        let t32 = param_c_0 * t1;
        let t33 = t32 * t3;
        let t34 = t9 * t28;
        let t38 = param_d_0 * t1;
        let t42 = piecewise3::<f64>(t12, t13 / t24, t27 * t28 + param_b_0 + t33 * t34 / 4.0 + t38 * t21 / 4.0);
        let t43 = param_gamma_1;
        let t44 = param_beta1_1;
        let t48 = param_beta2_1 * t1;
        let t51 = 1.0 + t44 * t15 / 2.0 + t48 * t21 / 4.0;
        let t54 = param_a_1;
        let t58 = param_c_1 * t1;
        let t59 = t58 * t3;
        let t63 = param_d_1 * t1;
        let t67 = piecewise3::<f64>(t12, t43 / t51, t54 * t28 + param_b_1 + t59 * t34 / 4.0 + t63 * t21 / 4.0);
        let t70 = pow_1_3::<f64>(zeta_threshold);
        let t72 = piecewise3::<f64>(1.0 <= zeta_threshold, t70 * zeta_threshold, 1.0);
        let t74 = 2.0 * t72 - 2.0;
        let t76 = M_CBRT2;
        let t79 = 1.0 / (2.0 * t76 - 2.0);
        let t80 = (t67 - t42) * t74 * t79;
        let tzk0 = t42 + t80;
        zk[ip] += tzk0;
    }
}
