//! LDA_C_PW vxc pol kernel (rayon backend).
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
pub fn lda_c_pw_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
        let t147 = t1 * t2 * t3;
        let t149 = 1.0 / t11 / t10;
        let t152 = t147 * t9 * t149 * t45;
        let t153 = t152 / 6.0;
        let t154 = t40 * t40;
        let t155 = 1.0 / t154;
        let t156 = t16 * t155;
        let t157 = 1.0 / t23;
        let t159 = t19 * t157 * t3;
        let t160 = t9 * t149;
        let t165 = rmath::sqrt(t22);
        let t167 = t30 * t165 * t3;
        let t173 = -t159 * t160 / 12.0 - t27 * t160 / 12.0 - 0.0625 * t167 * t160 - t39 * t37 * t55 / 3.0;
        let t174 = 1.0 / t44;
        let t175 = t173 * t174;
        let t176 = t156 * t175;
        let t177 = t49 * t48;
        let t178 = t177 * t53;
        let t179 = t178 * t139;
        let t180 = 4.0 * t179;
        let t181 = t52 * t10;
        let t182 = 1.0 / t181;
        let t183 = t50 * t182;
        let t184 = t183 * t139;
        let t185 = 4.0 * t184;
        let t186 = 1.0 / t51;
        let t187 = t48 * t186;
        let t188 = t55 - t187;
        let t191 = piecewise3(t58, 0.0, 4.0 / 3.0 * t61 * t188);
        let t192 = -t188;
        let t195 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t192);
        let t197 = (t191 + t195) * t73;
        let t198 = t197 * t138;
        let t199 = t54 * t198;
        let t201 = t75 * t76 * t3;
        let t206 = t98 * t98;
        let t207 = 1.0 / t206;
        let t208 = t80 * t207;
        let t210 = t83 * t157 * t3;
        let t216 = t90 * t165 * t3;
        let t222 = -t210 * t160 / 12.0 - t87 * t160 / 12.0 - 0.0625 * t216 * t160 - t97 * t95 * t55 / 3.0;
        let t223 = 1.0 / t102;
        let t224 = t222 * t223;
        let t226 = t105 * t106;
        let t227 = t226 * t20;
        let t228 = t8 * t149;
        let t232 = t128 * t128;
        let t233 = 1.0 / t232;
        let t234 = t110 * t233;
        let t236 = t113 * t157 * t3;
        let t242 = t120 * t165 * t3;
        let t248 = -t236 * t160 / 12.0 - t117 * t160 / 12.0 - 0.0625 * t242 * t160 - t127 * t125 * t55 / 3.0;
        let t249 = 1.0 / t132;
        let t251 = t248 * t249 * t134;
        let t253 = t201 * t9 * t149 * t103 / 6.0 + t208 * t224 - t153 - t176 + t227 * t228 * t135 / 6.0 + t234 * t251;
        let t254 = t74 * t253;
        let t255 = t54 * t254;
        let t257 = t197 * t105 * t143;
        let t258 = 2.0 * t257;
        let t259 = t226 * t3;
        let t260 = t74 * t259;
        let t263 = t9 * t149 * t133 * t134;
        let t264 = t260 * t263;
        let t265 = t264 / 6.0;
        let t266 = t74 * t110;
        let t268 = t249 * t134;
        let t269 = t233 * t248 * t268;
        let t270 = t266 * t269;
        let tvrho0 = -t47 + t140 + t145 + t10 * (t153 + t176 + t180 - t185 + t199 + t255 + t258 - t265 - t270);
        vrho[ip * 2] += tvrho0;
        let t273 = -t55 - t187;
        let t276 = piecewise3(t58, 0.0, 4.0 / 3.0 * t61 * t273);
        let t277 = -t273;
        let t280 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t277);
        let t282 = (t276 + t280) * t73;
        let t283 = t282 * t138;
        let t284 = t54 * t283;
        let t286 = t282 * t105 * t143;
        let t287 = 2.0 * t286;
        let tvrho1 = -t47 + t140 + t145 + t10 * (t153 + t176 - t180 - t185 + t284 + t255 + t287 - t265 - t270);
        vrho[ip * 2 + 1] += tvrho1;
    }
}
