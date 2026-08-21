//! LDA_C_PW exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pw.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_pw_exc_unpol(
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
        let t1 = param_a_0;
        let t2 = param_alpha1_0;
        let t3 = M_CBRT3;
        let t4 = t2 * t3;
        let t5 = 1.0 / M_PI;
        let t6 = pow_1_3(t5);
        let t7 = M_CBRT4;
        let t8 = t7 * t7;
        let t9 = t6 * t8;
        let t10 = pow_1_3(rho[ip]);
        let t11 = 1.0 / t10;
        let t12 = t9 * t11;
        let t15 = 1.0 + t4 * t12 / 4.0;
        let t17 = 1.0 / t1;
        let t18 = param_beta1_0;
        let t19 = t3 * t6;
        let t21 = t19 * t8 * t11;
        let t22 = rmath::sqrt(t21);
        let t26 = param_beta2_0 * t3;
        let t29 = param_beta3_0;
        let t30 = pow_3_2(t21);
        let t34 = t21 / 4.0;
        let t36 = param_pp_0 + 1.0;
        let t37 = rmath::pow(t34, t36);
        let t38 = param_beta4_0 * t37;
        let t39 = t18 * t22 / 2.0 + t26 * t12 / 4.0 + 0.125 * t29 * t30 + t38;
        let t43 = 1.0 + t17 / t39 / 2.0;
        let t44 = rmath::ln(t43);
        let t45 = t1 * t15 * t44;
        let t47 = pow_1_3(zeta_threshold);
        let t49 = piecewise3(1.0 <= zeta_threshold, t47 * zeta_threshold, 1.0);
        let t52 = M_CBRT2;
        let t56 = (2.0 * t49 - 2.0) / (2.0 * t52 - 2.0);
        let t57 = param_a_2;
        let t59 = param_alpha1_2;
        let t60 = t59 * t3;
        let t63 = 1.0 + t60 * t12 / 4.0;
        let t64 = 1.0 / t57;
        let t65 = param_beta1_2;
        let t69 = param_beta2_2 * t3;
        let t72 = param_beta3_2;
        let t77 = param_pp_2 + 1.0;
        let t78 = rmath::pow(t34, t77);
        let t79 = param_beta4_2 * t78;
        let t80 = t65 * t22 / 2.0 + t69 * t12 / 4.0 + 0.125 * t72 * t30 + t79;
        let t84 = 1.0 + t64 / t80 / 2.0;
        let t85 = rmath::ln(t84);
        let t87 = 1.0 / param_fz20;
        let t89 = t56 * t57 * t63 * t85 * t87;
        let tzk0 = -2.0 * t45 + 2.0 * t89;
        zk[ip] += tzk0;
    }
}
