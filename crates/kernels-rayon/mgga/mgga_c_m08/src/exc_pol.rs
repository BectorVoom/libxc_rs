//! MGGA_C_M08 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_m08.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_m08_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_m08_a_1: f64,
    param_m08_a_2: f64,
    param_m08_a_3: f64,
    param_m08_a_4: f64,
    param_m08_a_5: f64,
    param_m08_a_6: f64,
    param_m08_a_7: f64,
    param_m08_a_8: f64,
    param_m08_a_9: f64,
    param_m08_a_10: f64,
    param_m08_a_11: f64,
    param_m08_a_0: f64,
    param_m08_b_1: f64,
    param_m08_b_2: f64,
    param_m08_b_3: f64,
    param_m08_b_4: f64,
    param_m08_b_5: f64,
    param_m08_b_6: f64,
    param_m08_b_7: f64,
    param_m08_b_8: f64,
    param_m08_b_9: f64,
    param_m08_b_10: f64,
    param_m08_b_11: f64,
    param_m08_b_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t3 = param_m08_a_1;
        let t4 = M_CBRT6;
        let t5 = t4 * t4;
        let t6 = M_PI * M_PI;
        let t7 = pow_1_3(t6);
        let t8 = t7 * t7;
        let t10 = 3.0 / 10.0 * t5 * t8;
        let t11 = M_CBRT2;
        let t12 = t11 * t11;
        let t13 = pow_1_3(rho0);
        let t14 = t13 * t13;
        let t16 = 1.0 / t14 / rho0;
        let t17 = tau0 * t16;
        let t18 = rho0 - rho1;
        let t19 = rho0 + rho1;
        let t20 = 1.0 / t19;
        let t21 = t18 * t20;
        let t22 = 1.0 + t21;
        let t23 = t22 / 2.0;
        let t24 = pow_1_3(t23);
        let t25 = t24 * t24;
        let t26 = t25 * t23;
        let t28 = pow_1_3(rho1);
        let t29 = t28 * t28;
        let t31 = 1.0 / t29 / rho1;
        let t32 = tau1 * t31;
        let t33 = 1.0 - t21;
        let t34 = t33 / 2.0;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = t36 * t34;
        let t40 = t12 * (t17 * t26 + t32 * t37);
        let t41 = t10 - t40;
        let t42 = t3 * t41;
        let t43 = t10 + t40;
        let t44 = 1.0 / t43;
        let t46 = param_m08_a_2;
        let t47 = t41 * t41;
        let t48 = t46 * t47;
        let t49 = t43 * t43;
        let t50 = 1.0 / t49;
        let t52 = param_m08_a_3;
        let t53 = t47 * t41;
        let t54 = t52 * t53;
        let t55 = t49 * t43;
        let t56 = 1.0 / t55;
        let t58 = param_m08_a_4;
        let t59 = t47 * t47;
        let t60 = t58 * t59;
        let t61 = t49 * t49;
        let t62 = 1.0 / t61;
        let t64 = param_m08_a_5;
        let t65 = t59 * t41;
        let t66 = t64 * t65;
        let t67 = t61 * t43;
        let t68 = 1.0 / t67;
        let t70 = param_m08_a_6;
        let t71 = t59 * t47;
        let t72 = t70 * t71;
        let t73 = t61 * t49;
        let t74 = 1.0 / t73;
        let t76 = param_m08_a_7;
        let t77 = t59 * t53;
        let t78 = t76 * t77;
        let t79 = t61 * t55;
        let t80 = 1.0 / t79;
        let t82 = param_m08_a_8;
        let t83 = t59 * t59;
        let t84 = t82 * t83;
        let t85 = t61 * t61;
        let t86 = 1.0 / t85;
        let t88 = param_m08_a_9;
        let t89 = t83 * t41;
        let t90 = t88 * t89;
        let t92 = 1.0 / t85 / t43;
        let t94 = param_m08_a_10;
        let t95 = t83 * t47;
        let t96 = t94 * t95;
        let t98 = 1.0 / t85 / t49;
        let t100 = param_m08_a_11;
        let t101 = t83 * t53;
        let t102 = t100 * t101;
        let t104 = 1.0 / t85 / t55;
        let t106 = t102 * t104 + t42 * t44 + t48 * t50 + t54 * t56 + t60 * t62 + t66 * t68 + t72 * t74 + t78 * t80 + t84 * t86 + t90 * t92 + t96 * t98 + param_m08_a_0;
        let t107 = M_CBRT3;
        let t108 = 1.0 / M_PI;
        let t109 = pow_1_3(t108);
        let t110 = t107 * t109;
        let t111 = M_CBRT4;
        let t112 = t111 * t111;
        let t113 = pow_1_3(t19);
        let t116 = t110 * t112 / t113;
        let t118 = 1.0 + 0.053425 * t116;
        let t119 = f64::sqrt(t116);
        let t122 = pow_3_2(t116);
        let t124 = t107 * t107;
        let t125 = t109 * t109;
        let t126 = t124 * t125;
        let t127 = t113 * t113;
        let t130 = t126 * t111 / t127;
        let t132 = 3.79785 * t119 + 0.8969 * t116 + 0.204775 * t122 + 0.123235 * t130;
        let t135 = 1.0 + 16.081979498692537 / t132;
        let t136 = f64::ln(t135);
        let t138 = 0.0621814 * t118 * t136;
        let t139 = t18 * t18;
        let t140 = t139 * t139;
        let t141 = t19 * t19;
        let t142 = t141 * t141;
        let t143 = 1.0 / t142;
        let t144 = t140 * t143;
        let t145 = t22 <= zeta_threshold;
        let t146 = pow_1_3(zeta_threshold);
        let t147 = t146 * zeta_threshold;
        let t148 = pow_1_3(t22);
        let t149 = t148 * t22;
        let t150 = piecewise3(t145, t147, t149);
        let t151 = t33 <= zeta_threshold;
        let t152 = pow_1_3(t33);
        let t153 = t152 * t33;
        let t154 = piecewise3(t151, t147, t153);
        let t155 = t150 + t154 - 2.0;
        let t158 = 1.0 / (2.0 * t11 - 2.0);
        let t159 = t155 * t158;
        let t161 = 1.0 + 0.05137 * t116;
        let t166 = 7.05945 * t119 + 1.549425 * t116 + 0.420775 * t122 + 0.1562925 * t130;
        let t169 = 1.0 + 32.16395899738507 / t166;
        let t170 = f64::ln(t169);
        let t174 = 1.0 + 0.0278125 * t116;
        let t179 = 5.1785 * t119 + 0.905775 * t116 + 0.1100325 * t122 + 0.1241775 * t130;
        let t182 = 1.0 + 29.608749977793437 / t179;
        let t183 = f64::ln(t182);
        let t184 = t174 * t183;
        let t186 = -0.0310907 * t161 * t170 + t138 - 0.0197516734986138 * t184;
        let t187 = t159 * t186;
        let t191 = -t138 + t144 * t187 + 0.0197516734986138 * t159 * t184;
        let t192 = t106 * t191;
        let t194 = param_m08_b_1;
        let t195 = t194 * t41;
        let t197 = param_m08_b_2;
        let t198 = t197 * t47;
        let t200 = param_m08_b_3;
        let t201 = t200 * t53;
        let t203 = param_m08_b_4;
        let t204 = t203 * t59;
        let t206 = param_m08_b_5;
        let t207 = t206 * t65;
        let t209 = param_m08_b_6;
        let t210 = t209 * t71;
        let t212 = param_m08_b_7;
        let t213 = t212 * t77;
        let t215 = param_m08_b_8;
        let t216 = t215 * t83;
        let t218 = param_m08_b_9;
        let t219 = t218 * t89;
        let t221 = param_m08_b_10;
        let t222 = t221 * t95;
        let t224 = param_m08_b_11;
        let t225 = t224 * t101;
        let t227 = t225 * t104 + t195 * t44 + t198 * t50 + t201 * t56 + t204 * t62 + t207 * t68 + t210 * t74 + t213 * t80 + t216 * t86 + t219 * t92 + t222 * t98 + param_m08_b_0;
        let t228 = f64::ln(2.0);
        let t229 = 1.0 - t228;
        let t230 = t227 * t229;
        let t231 = 1.0 / t6;
        let t232 = t146 * t146;
        let t233 = t148 * t148;
        let t234 = piecewise3(t145, t232, t233);
        let t235 = t152 * t152;
        let t236 = piecewise3(t151, t232, t235);
        let t238 = t234 / 2.0 + t236 / 2.0;
        let t239 = t238 * t238;
        let t240 = t239 * t238;
        let t241 = t231 * t240;
        let t243 = sigma0 + 2.0 * sigma1 + sigma2;
        let t245 = 1.0 / t113 / t141;
        let t246 = t243 * t245;
        let t248 = 1.0 / t239;
        let t250 = 1.0 / t109;
        let t251 = t250 * t111;
        let t252 = t248 * t124 * t251;
        let t255 = 1.0 / t229;
        let t256 = t191 * t255;
        let t257 = 1.0 / t240;
        let t258 = t6 * t257;
        let t260 = f64::exp(-t256 * t258);
        let t261 = t260 - 1.0;
        let t262 = 1.0 / t261;
        let t263 = t255 * t262;
        let t264 = t243 * t243;
        let t266 = 1.0 / t127 / t142;
        let t267 = t264 * t266;
        let t269 = t239 * t239;
        let t270 = 1.0 / t269;
        let t271 = t12 * t270;
        let t272 = 1.0 / t125;
        let t273 = t107 * t272;
        let t274 = t273 * t112;
        let t275 = t271 * t274;
        let t278 = t246 * t11 * t252 / 96.0 + 0.0002143700905903487 * t263 * t267 * t275;
        let t279 = t278 * t255;
        let t282 = 1.0 + 0.6585449182935511 * t263 * t278;
        let t283 = 1.0 / t282;
        let t286 = 1.0 + 0.6585449182935511 * t279 * t283;
        let t287 = f64::ln(t286);
        let t288 = t241 * t287;
        let t289 = t230 * t288;
        let tzk0 = t192 + t289;
        zk[ip] += tzk0;
    }
}
