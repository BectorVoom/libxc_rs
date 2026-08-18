//! MGGA_C_M08 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_m08.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_m08_exc_unpol(
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
        let t3 = param_m08_a_1;
        let t4 = M_CBRT6;
        let t5 = t4 * t4;
        let t6 = M_PI * M_PI;
        let t7 = pow_1_3(t6);
        let t8 = t7 * t7;
        let t10 = 3.0 / 10.0 * t5 * t8;
        let t11 = M_CBRT2;
        let t12 = t11 * t11;
        let t13 = tau[ip] * t12;
        let t14 = pow_1_3(rho[ip]);
        let t15 = t14 * t14;
        let t17 = 1.0 / t15 / rho[ip];
        let t18 = t13 * t17;
        let t19 = t10 - t18;
        let t20 = t3 * t19;
        let t21 = t10 + t18;
        let t22 = 1.0 / t21;
        let t24 = param_m08_a_2;
        let t25 = t19 * t19;
        let t26 = t24 * t25;
        let t27 = t21 * t21;
        let t28 = 1.0 / t27;
        let t30 = param_m08_a_3;
        let t31 = t25 * t19;
        let t32 = t30 * t31;
        let t33 = t27 * t21;
        let t34 = 1.0 / t33;
        let t36 = param_m08_a_4;
        let t37 = t25 * t25;
        let t38 = t36 * t37;
        let t39 = t27 * t27;
        let t40 = 1.0 / t39;
        let t42 = param_m08_a_5;
        let t43 = t37 * t19;
        let t44 = t42 * t43;
        let t45 = t39 * t21;
        let t46 = 1.0 / t45;
        let t48 = param_m08_a_6;
        let t49 = t37 * t25;
        let t50 = t48 * t49;
        let t51 = t39 * t27;
        let t52 = 1.0 / t51;
        let t54 = param_m08_a_7;
        let t55 = t37 * t31;
        let t56 = t54 * t55;
        let t57 = t39 * t33;
        let t58 = 1.0 / t57;
        let t60 = param_m08_a_8;
        let t61 = t37 * t37;
        let t62 = t60 * t61;
        let t63 = t39 * t39;
        let t64 = 1.0 / t63;
        let t66 = param_m08_a_9;
        let t67 = t61 * t19;
        let t68 = t66 * t67;
        let t70 = 1.0 / t63 / t21;
        let t72 = param_m08_a_10;
        let t73 = t61 * t25;
        let t74 = t72 * t73;
        let t76 = 1.0 / t63 / t27;
        let t78 = param_m08_a_11;
        let t79 = t61 * t31;
        let t80 = t78 * t79;
        let t82 = 1.0 / t63 / t33;
        let t84 = t20 * t22 + t26 * t28 + t32 * t34 + t38 * t40 + t44 * t46 + t50 * t52 + t56 * t58 + t62 * t64 + t68 * t70 + t74 * t76 + t80 * t82 + param_m08_a_0;
        let t85 = M_CBRT3;
        let t86 = 1.0 / M_PI;
        let t87 = pow_1_3(t86);
        let t88 = t85 * t87;
        let t89 = M_CBRT4;
        let t90 = t89 * t89;
        let t93 = t88 * t90 / t14;
        let t95 = 1.0 + 0.053425 * t93;
        let t96 = f64::sqrt(t93);
        let t99 = pow_3_2(t93);
        let t101 = t85 * t85;
        let t102 = t87 * t87;
        let t103 = t101 * t102;
        let t106 = t103 * t89 / t15;
        let t108 = 3.79785 * t96 + 0.8969 * t93 + 0.204775 * t99 + 0.123235 * t106;
        let t111 = 1.0 + 16.081979498692537 / t108;
        let t112 = f64::ln(t111);
        let t115 = 1.0 <= zeta_threshold;
        let t116 = pow_1_3(zeta_threshold);
        let t118 = piecewise3(t115, t116 * zeta_threshold, 1.0);
        let t124 = (2.0 * t118 - 2.0) / (2.0 * t11 - 2.0);
        let t126 = 1.0 + 0.0278125 * t93;
        let t131 = 5.1785 * t96 + 0.905775 * t93 + 0.1100325 * t99 + 0.1241775 * t106;
        let t134 = 1.0 + 29.608749977793437 / t131;
        let t135 = f64::ln(t134);
        let t139 = -0.0621814 * t95 * t112 + 0.0197516734986138 * t124 * t126 * t135;
        let t140 = t84 * t139;
        let t142 = param_m08_b_1;
        let t143 = t142 * t19;
        let t145 = param_m08_b_2;
        let t146 = t145 * t25;
        let t148 = param_m08_b_3;
        let t149 = t148 * t31;
        let t151 = param_m08_b_4;
        let t152 = t151 * t37;
        let t154 = param_m08_b_5;
        let t155 = t154 * t43;
        let t157 = param_m08_b_6;
        let t158 = t157 * t49;
        let t160 = param_m08_b_7;
        let t161 = t160 * t55;
        let t163 = param_m08_b_8;
        let t164 = t163 * t61;
        let t166 = param_m08_b_9;
        let t167 = t166 * t67;
        let t169 = param_m08_b_10;
        let t170 = t169 * t73;
        let t172 = param_m08_b_11;
        let t173 = t172 * t79;
        let t175 = t143 * t22 + t146 * t28 + t149 * t34 + t152 * t40 + t155 * t46 + t158 * t52 + t161 * t58 + t164 * t64 + t167 * t70 + t170 * t76 + t173 * t82 + param_m08_b_0;
        let t176 = f64::ln(2.0);
        let t177 = 1.0 - t176;
        let t178 = t175 * t177;
        let t179 = 1.0 / t6;
        let t180 = t116 * t116;
        let t181 = piecewise3(t115, t180, 1.0);
        let t182 = t181 * t181;
        let t183 = t182 * t181;
        let t184 = t179 * t183;
        let t185 = rho[ip] * rho[ip];
        let t187 = 1.0 / t14 / t185;
        let t190 = 1.0 / t182;
        let t192 = 1.0 / t87;
        let t194 = t190 * t101 * t192 * t89;
        let t197 = 1.0 / t177;
        let t199 = 1.0 / t183;
        let t200 = t6 * t199;
        let t202 = f64::exp(-t139 * t197 * t200);
        let t203 = t202 - 1.0;
        let t204 = 1.0 / t203;
        let t205 = t197 * t204;
        let t206 = sigma[ip] * sigma[ip];
        let t207 = t185 * t185;
        let t209 = 1.0 / t15 / t207;
        let t212 = t182 * t182;
        let t213 = 1.0 / t212;
        let t214 = t12 * t213;
        let t215 = 1.0 / t102;
        let t216 = t85 * t215;
        let t217 = t216 * t90;
        let t218 = t214 * t217;
        let t221 = sigma[ip] * t187 * t11 * t194 / 96.0 + 0.0002143700905903487 * t205 * t206 * t209 * t218;
        let t222 = t221 * t197;
        let t225 = 1.0 + 0.6585449182935511 * t205 * t221;
        let t226 = 1.0 / t225;
        let t229 = 1.0 + 0.6585449182935511 * t222 * t226;
        let t230 = f64::ln(t229);
        let t231 = t184 * t230;
        let t232 = t178 * t231;
        let tzk0 = t140 + t232;
        zk[ip] += tzk0;
    }
}
