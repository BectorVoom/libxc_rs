//! LDA_C_PZ exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pz.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_pz_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    param_gamma_0: f64,
    param_beta1_0: f64,
    param_beta2_0: f64,
    param_a_0: f64,
    param_c_0: f64,
    param_d_0: f64,
    param_b_0: f64,
    param_gamma_1: f64,
    param_beta1_1: f64,
    param_beta2_1: f64,
    param_a_1: f64,
    param_c_1: f64,
    param_d_1: f64,
    param_b_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t9 = 1.0 / t8;
        let t10 = t6 * t9;
        let t11 = t1 * t3 * t10;
        let t12 = t11 / 4.0;
        let t13 = 1.0 <= t12;
        let t14 = param_gamma_0;
        let t15 = param_beta1_0;
        let t16 = f64::sqrt(t11);
        let t20 = param_beta2_0 * t1;
        let t21 = t3 * t6;
        let t22 = t21 * t9;
        let t25 = 1.0 + t15 * t16 / 2.0 + t20 * t22 / 4.0;
        let t28 = param_a_0;
        let t29 = f64::ln(t12);
        let t33 = param_c_0 * t1;
        let t34 = t33 * t3;
        let t35 = t10 * t29;
        let t39 = param_d_0 * t1;
        let t43 = piecewise3(t13, t14 / t25, t28 * t29 + param_b_0 + t34 * t35 / 4.0 + t39 * t22 / 4.0);
        let t44 = param_gamma_1;
        let t45 = param_beta1_1;
        let t49 = param_beta2_1 * t1;
        let t52 = 1.0 + t45 * t16 / 2.0 + t49 * t22 / 4.0;
        let t55 = param_a_1;
        let t59 = param_c_1 * t1;
        let t60 = t59 * t3;
        let t64 = param_d_1 * t1;
        let t68 = piecewise3(t13, t44 / t52, t55 * t29 + param_b_1 + t60 * t35 / 4.0 + t64 * t22 / 4.0);
        let t69 = t68 - t43;
        let t70 = rho0 - rho1;
        let t71 = 1.0 / t7;
        let t72 = t70 * t71;
        let t73 = 1.0 + t72;
        let t74 = t73 <= zeta_threshold;
        let t75 = pow_1_3(zeta_threshold);
        let t76 = t75 * zeta_threshold;
        let t77 = pow_1_3(t73);
        let t79 = piecewise3(t74, t76, t77 * t73);
        let t80 = 1.0 - t72;
        let t81 = t80 <= zeta_threshold;
        let t82 = pow_1_3(t80);
        let t84 = piecewise3(t81, t76, t82 * t80);
        let t85 = t79 + t84 - 2.0;
        let t87 = M_CBRT2;
        let t90 = 1.0 / (2.0 * t87 - 2.0);
        let t91 = t69 * t85 * t90;
        let tzk0 = t43 + t91;
        zk[ip] += tzk0;
    }
}
