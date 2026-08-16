//! LDA_C_PZ fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pz.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_PZ fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_pz_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
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
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3::<f64>(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3::<f64>(t7);
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
        let t43 = piecewise3::<f64>(t13, t14 / t25, t28 * t29 + param_b_0 + t34 * t35 / 4.0 + t39 * t22 / 4.0);
        let t44 = param_gamma_1;
        let t45 = param_beta1_1;
        let t49 = param_beta2_1 * t1;
        let t52 = 1.0 + t45 * t16 / 2.0 + t49 * t22 / 4.0;
        let t55 = param_a_1;
        let t59 = param_c_1 * t1;
        let t60 = t59 * t3;
        let t64 = param_d_1 * t1;
        let t68 = piecewise3::<f64>(t13, t44 / t52, t55 * t29 + param_b_1 + t60 * t35 / 4.0 + t64 * t22 / 4.0);
        let t69 = t68 - t43;
        let t70 = rho0 - rho1;
        let t71 = 1.0 / t7;
        let t72 = t70 * t71;
        let t73 = 1.0 + t72;
        let t74 = t73 <= zeta_threshold;
        let t75 = pow_1_3::<f64>(zeta_threshold);
        let t76 = t75 * zeta_threshold;
        let t77 = pow_1_3::<f64>(t73);
        let t79 = piecewise3::<f64>(t74, t76, t77 * t73);
        let t80 = 1.0 - t72;
        let t81 = t80 <= zeta_threshold;
        let t82 = pow_1_3::<f64>(t80);
        let t84 = piecewise3::<f64>(t81, t76, t82 * t80);
        let t85 = t79 + t84 - 2.0;
        let t87 = M_CBRT2;
        let t90 = 1.0 / (2.0 * t87 - 2.0);
        let t91 = t69 * t85 * t90;
        let tzk0 = t43 + t91;
        zk[ip] += tzk0;
        let t92 = t25 * t25;
        let t94 = t14 / t92;
        let t95 = 1.0 / t16;
        let t97 = t15 * t95 * t1;
        let t99 = 1.0 / t8 / t7;
        let t100 = t21 * t99;
        let t104 = -t20 * t100 / 12.0 - t97 * t100 / 12.0;
        let t109 = t6 * t99 * t29;
        let t117 = piecewise3::<f64>(t13, -t94 * t104, -t28 * t71 / 3.0 - t34 * t109 / 12.0 - t33 * t100 / 12.0 - t39 * t100 / 12.0);
        let t118 = t52 * t52;
        let t120 = t44 / t118;
        let t122 = t45 * t95 * t1;
        let t126 = -t122 * t100 / 12.0 - t49 * t100 / 12.0;
        let t137 = piecewise3::<f64>(t13, -t120 * t126, -t55 * t71 / 3.0 - t60 * t109 / 12.0 - t59 * t100 / 12.0 - t64 * t100 / 12.0);
        let t138 = t137 - t117;
        let t140 = t138 * t85 * t90;
        let t141 = t7 * t7;
        let t142 = 1.0 / t141;
        let t143 = t70 * t142;
        let t144 = t71 - t143;
        let t147 = piecewise3::<f64>(t74, 0.0, 4.0 / 3.0 * t77 * t144);
        let t148 = -t144;
        let t151 = piecewise3::<f64>(t81, 0.0, 4.0 / 3.0 * t82 * t148);
        let t152 = t147 + t151;
        let t154 = t69 * t152 * t90;
        let tvrho0 = t43 + t91 + t7 * (t117 + t140 + t154);
        vrho[ip * 2] += tvrho0;
        let t157 = -t71 - t143;
        let t160 = piecewise3::<f64>(t74, 0.0, 4.0 / 3.0 * t77 * t157);
        let t161 = -t157;
        let t164 = piecewise3::<f64>(t81, 0.0, 4.0 / 3.0 * t82 * t161);
        let t165 = t160 + t164;
        let t167 = t69 * t165 * t90;
        let tvrho1 = t43 + t91 + t7 * (t117 + t140 + t167);
        vrho[ip * 2 + 1] += tvrho1;
        let t170 = 2.0 * t117;
        let t171 = 2.0 * t140;
        let t175 = t14 / t92 / t25;
        let t176 = t104 * t104;
        let t180 = 1.0 / t16 / t11;
        let t182 = t1 * t1;
        let t183 = t15 * t180 * t182;
        let t184 = t3 * t3;
        let t185 = t184 * t5;
        let t186 = t8 * t8;
        let t189 = t185 / t186 / t141;
        let t193 = 1.0 / t8 / t141;
        let t194 = t21 * t193;
        let t199 = -t183 * t189 / 18.0 + t97 * t194 / 9.0 + t20 * t194 / 9.0;
        let t205 = t6 * t193 * t29;
        let t213 = piecewise3::<f64>(t13, 2.0 * t175 * t176 - t94 * t199, t28 * t142 / 3.0 + t34 * t205 / 9.0 + 5.0 / 36.0 * t33 * t194 + t39 * t194 / 9.0);
        let t216 = t44 / t118 / t52;
        let t217 = t126 * t126;
        let t221 = t45 * t180 * t182;
        let t228 = -t221 * t189 / 18.0 + t122 * t194 / 9.0 + t49 * t194 / 9.0;
        let t240 = piecewise3::<f64>(t13, -t120 * t228 + 2.0 * t216 * t217, t55 * t142 / 3.0 + t60 * t205 / 9.0 + 5.0 / 36.0 * t59 * t194 + t64 * t194 / 9.0);
        let t241 = t240 - t213;
        let t243 = t241 * t85 * t90;
        let t245 = t138 * t152 * t90;
        let t246 = 2.0 * t245;
        let t247 = t77 * t77;
        let t248 = 1.0 / t247;
        let t249 = t144 * t144;
        let t252 = t141 * t7;
        let t253 = 1.0 / t252;
        let t254 = t70 * t253;
        let t256 = -2.0 * t142 + 2.0 * t254;
        let t260 = piecewise3::<f64>(t74, 0.0, 4.0 / 9.0 * t248 * t249 + 4.0 / 3.0 * t77 * t256);
        let t261 = t82 * t82;
        let t262 = 1.0 / t261;
        let t263 = t148 * t148;
        let t266 = -t256;
        let t270 = piecewise3::<f64>(t81, 0.0, 4.0 / 9.0 * t262 * t263 + 4.0 / 3.0 * t82 * t266);
        let t271 = t260 + t270;
        let t273 = t69 * t271 * t90;
        let tv2rho20 = t170 + t171 + 2.0 * t154 + t7 * (t213 + t243 + t246 + t273);
        v2rho2[ip * 3] += tv2rho20;
        let t277 = t138 * t165 * t90;
        let t278 = t248 * t157;
        let t281 = t77 * t70;
        let t285 = piecewise3::<f64>(t74, 0.0, 4.0 / 9.0 * t278 * t144 + 8.0 / 3.0 * t281 * t253);
        let t286 = t262 * t161;
        let t289 = t82 * t70;
        let t293 = piecewise3::<f64>(t81, 0.0, 4.0 / 9.0 * t286 * t148 - 8.0 / 3.0 * t289 * t253);
        let t294 = t285 + t293;
        let t296 = t69 * t294 * t90;
        let tv2rho21 = t170 + t171 + t154 + t167 + t7 * (t213 + t243 + t245 + t277 + t296);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t300 = 2.0 * t277;
        let t301 = t157 * t157;
        let t305 = 2.0 * t142 + 2.0 * t254;
        let t309 = piecewise3::<f64>(t74, 0.0, 4.0 / 9.0 * t248 * t301 + 4.0 / 3.0 * t77 * t305);
        let t310 = t161 * t161;
        let t313 = -t305;
        let t317 = piecewise3::<f64>(t81, 0.0, 4.0 / 9.0 * t262 * t310 + 4.0 / 3.0 * t82 * t313);
        let t318 = t309 + t317;
        let t320 = t69 * t318 * t90;
        let tv2rho22 = t170 + t171 + 2.0 * t167 + t7 * (t213 + t243 + t300 + t320);
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
