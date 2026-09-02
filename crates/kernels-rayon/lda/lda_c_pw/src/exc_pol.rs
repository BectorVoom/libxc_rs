//! LDA_C_PW exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pw.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_pw_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    param_a_0: f64,
    param_alpha1_0: f64,
    param_beta1_0: f64,
    param_beta2_0: f64,
    param_beta3_0: f64,
    param_pp_0: f64,
    param_beta4_0: f64,
    param_a_2: f64,
    param_alpha1_2: f64,
    param_beta1_2: f64,
    param_beta2_2: f64,
    param_beta3_2: f64,
    param_pp_2: f64,
    param_beta4_2: f64,
    param_fz20: f64,
    param_a_1: f64,
    param_alpha1_1: f64,
    param_beta1_1: f64,
    param_beta2_1: f64,
    param_beta3_1: f64,
    param_pp_1: f64,
    param_beta4_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = param_a_0;
        let t2 = param_alpha1_0;
        let t3 = M_CBRT3;
        let t4 = t2 * t3;
        let t5 = 1.0 / M_PI;
        let t6 = pow_1_3(t5);
        let t7 = M_CBRT4;
        let t8 = t7 * t7;
        let t9 = t6 * t8;
        let t10 = rho0 + rho1;
        let t11 = pow_1_3(t10);
        let t12 = 1.0 / t11;
        let t13 = t9 * t12;
        let t16 = 1.0 + t4 * t13 / 4.0;
        let t18 = 1.0 / t1;
        let t19 = param_beta1_0;
        let t20 = t3 * t6;
        let t22 = t20 * t8 * t12;
        let t23 = rmath::sqrt(t22);
        let t27 = param_beta2_0 * t3;
        let t30 = param_beta3_0;
        let t31 = pow_3_2(t22);
        let t35 = t22 / 4.0;
        let t37 = param_pp_0 + 1.0;
        let t38 = rmath::pow(t35, t37);
        let t39 = param_beta4_0 * t38;
        let t40 = t19 * t23 / 2.0 + t27 * t13 / 4.0 + 0.125 * t30 * t31 + t39;
        let t44 = 1.0 + t18 / t40 / 2.0;
        let t45 = rmath::ln(t44);
        let t46 = t1 * t16 * t45;
        let t47 = 2.0 * t46;
        let t48 = rho0 - rho1;
        let t49 = t48 * t48;
        let t50 = t49 * t49;
        let t51 = t10 * t10;
        let t52 = t51 * t51;
        let t53 = 1.0 / t52;
        let t54 = t50 * t53;
        let t55 = 1.0 / t10;
        let t56 = t48 * t55;
        let t57 = 1.0 + t56;
        let t58 = t57 <= zeta_threshold;
        let t59 = pow_1_3(zeta_threshold);
        let t60 = t59 * zeta_threshold;
        let t61 = pow_1_3(t57);
        let t63 = piecewise3(t58, t60, t61 * t57);
        let t64 = 1.0 - t56;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3(t64);
        let t68 = piecewise3(t65, t60, t66 * t64);
        let t69 = t63 + t68 - 2.0;
        let t70 = M_CBRT2;
        let t73 = 1.0 / (2.0 * t70 - 2.0);
        let t74 = t69 * t73;
        let t75 = param_a_1;
        let t76 = param_alpha1_1;
        let t77 = t76 * t3;
        let t80 = 1.0 + t77 * t13 / 4.0;
        let t82 = 1.0 / t75;
        let t83 = param_beta1_1;
        let t87 = param_beta2_1 * t3;
        let t90 = param_beta3_1;
        let t95 = param_pp_1 + 1.0;
        let t96 = rmath::pow(t35, t95);
        let t97 = param_beta4_1 * t96;
        let t98 = t83 * t23 / 2.0 + t87 * t13 / 4.0 + 0.125 * t90 * t31 + t97;
        let t102 = 1.0 + t82 / t98 / 2.0;
        let t103 = rmath::ln(t102);
        let t105 = param_a_2;
        let t106 = param_alpha1_2;
        let t107 = t106 * t3;
        let t110 = 1.0 + t107 * t13 / 4.0;
        let t112 = 1.0 / t105;
        let t113 = param_beta1_2;
        let t117 = param_beta2_2 * t3;
        let t120 = param_beta3_2;
        let t125 = param_pp_2 + 1.0;
        let t126 = rmath::pow(t35, t125);
        let t127 = param_beta4_2 * t126;
        let t128 = t113 * t23 / 2.0 + t117 * t13 / 4.0 + 0.125 * t120 * t31 + t127;
        let t132 = 1.0 + t112 / t128 / 2.0;
        let t133 = rmath::ln(t132);
        let t134 = 1.0 / param_fz20;
        let t135 = t133 * t134;
        let t138 = -2.0 * t75 * t80 * t103 - 2.0 * t105 * t110 * t135 + 2.0 * t46;
        let t139 = t74 * t138;
        let t140 = t54 * t139;
        let t143 = t110 * t133 * t134;
        let t145 = 2.0 * t74 * t105 * t143;
        let tzk0 = -t47 + t140 + t145;
        zk[ip] += tzk0;
    }
}
