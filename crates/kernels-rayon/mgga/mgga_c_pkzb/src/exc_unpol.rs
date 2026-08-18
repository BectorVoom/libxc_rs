//! MGGA_C_PKZB exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_pkzb.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_pkzb_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = sigma[ip] * sigma[ip];
        let t3 = rho[ip] * rho[ip];
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = tau[ip] * tau[ip];
        let t7 = 1.0 / t6;
        let t10 = 1.0 + 0.00828125 * t5 * t7;
        let t11 = M_CBRT3;
        let t12 = 1.0 / M_PI;
        let t13 = pow_1_3(t12);
        let t14 = t11 * t13;
        let t15 = M_CBRT4;
        let t16 = t15 * t15;
        let t17 = pow_1_3(rho[ip]);
        let t18 = 1.0 / t17;
        let t20 = t14 * t16 * t18;
        let t22 = 1.0 + 0.053425 * t20;
        let t23 = f64::sqrt(t20);
        let t26 = pow_3_2(t20);
        let t28 = t11 * t11;
        let t29 = t13 * t13;
        let t30 = t28 * t29;
        let t31 = t17 * t17;
        let t32 = 1.0 / t31;
        let t34 = t30 * t15 * t32;
        let t36 = 3.79785 * t23 + 0.8969 * t20 + 0.204775 * t26 + 0.123235 * t34;
        let t39 = 1.0 + 16.081979498692537 / t36;
        let t40 = f64::ln(t39);
        let t42 = 0.0621814 * t22 * t40;
        let t43 = 1.0 <= zeta_threshold;
        let t44 = pow_1_3(zeta_threshold);
        let t45 = t44 * zeta_threshold;
        let t46 = piecewise3(t43, t45, 1.0);
        let t49 = M_CBRT2;
        let t52 = 1.0 / (2.0 * t49 - 2.0);
        let t53 = (2.0 * t46 - 2.0) * t52;
        let t55 = 1.0 + 0.0278125 * t20;
        let t60 = 5.1785 * t23 + 0.905775 * t20 + 0.1100325 * t26 + 0.1241775 * t34;
        let t63 = 1.0 + 29.608749977793437 / t60;
        let t64 = f64::ln(t63);
        let t67 = 0.0197516734986138 * t53 * t55 * t64;
        let t68 = f64::ln(2.0);
        let t69 = 1.0 - t68;
        let t70 = M_PI * M_PI;
        let t71 = 1.0 / t70;
        let t72 = t69 * t71;
        let t73 = t44 * t44;
        let t74 = piecewise3(t43, t73, 1.0);
        let t75 = t74 * t74;
        let t76 = t75 * t74;
        let t78 = 1.0 / t17 / t3;
        let t79 = sigma[ip] * t78;
        let t81 = 1.0 / t75;
        let t83 = 1.0 / t13;
        let t84 = t83 * t15;
        let t85 = t81 * t28 * t84;
        let t88 = 1.0 / t69;
        let t91 = 1.0 / t76;
        let t92 = t70 * t91;
        let t94 = f64::exp(-(-t42 + t67) * t88 * t92);
        let t95 = t94 - 1.0;
        let t96 = 1.0 / t95;
        let t97 = t88 * t96;
        let t98 = t3 * t3;
        let t100 = 1.0 / t31 / t98;
        let t101 = t2 * t100;
        let t103 = t49 * t49;
        let t104 = t75 * t75;
        let t105 = 1.0 / t104;
        let t106 = t103 * t105;
        let t107 = 1.0 / t29;
        let t108 = t11 * t107;
        let t109 = t108 * t16;
        let t110 = t106 * t109;
        let t113 = t79 * t49 * t85 / 96.0 + 0.0002143700905903487 * t97 * t101 * t110;
        let t114 = t113 * t88;
        let t117 = 1.0 + 0.6585449182935511 * t97 * t113;
        let t118 = 1.0 / t117;
        let t121 = 1.0 + 0.6585449182935511 * t114 * t118;
        let t122 = f64::ln(t121);
        let t125 = t72 * t76 * t122 - t42 + t67;
        let t126 = t10 * t125;
        let t129 = rho[ip] / 2.0 <= dens_threshold || t43;
        let t130 = t14 * t16;
        let t133 = piecewise3(t43, 1.0 / t44, 1.0);
        let t135 = t130 * t18 * t49 * t133;
        let t137 = 1.0 + 0.053425 * t135;
        let t138 = f64::sqrt(t135);
        let t141 = pow_3_2(t135);
        let t143 = t30 * t15;
        let t145 = t133 * t133;
        let t147 = t143 * t32 * t103 * t145;
        let t149 = 3.79785 * t138 + 0.8969 * t135 + 0.204775 * t141 + 0.123235 * t147;
        let t152 = 1.0 + 16.081979498692537 / t149;
        let t153 = f64::ln(t152);
        let t155 = 0.0621814 * t137 * t153;
        let t156 = 2.0 <= zeta_threshold;
        let t158 = piecewise3(t156, t45, 2.0 * t49);
        let t159 = 0.0 <= zeta_threshold;
        let t160 = piecewise3(t159, t45, 0.0);
        let t162 = (t158 + t160 - 2.0) * t52;
        let t164 = 1.0 + 0.05137 * t135;
        let t169 = 7.05945 * t138 + 1.549425 * t135 + 0.420775 * t141 + 0.1562925 * t147;
        let t172 = 1.0 + 32.16395899738507 / t169;
        let t173 = f64::ln(t172);
        let t177 = 1.0 + 0.0278125 * t135;
        let t182 = 5.1785 * t138 + 0.905775 * t135 + 0.1100325 * t141 + 0.1241775 * t147;
        let t185 = 1.0 + 29.608749977793437 / t182;
        let t186 = f64::ln(t185);
        let t187 = t177 * t186;
        let t190 = t162 * (-0.0310907 * t164 * t173 + t155 - 0.0197516734986138 * t187);
        let t192 = 0.0197516734986138 * t162 * t187;
        let t193 = piecewise3(t156, t73, t103);
        let t194 = piecewise3(t159, t73, 0.0);
        let t196 = t193 / 2.0 + t194 / 2.0;
        let t197 = t196 * t196;
        let t198 = t197 * t196;
        let t199 = 1.0 / t197;
        let t200 = t199 * t28;
        let t204 = t84 * t103 / t133;
        let t209 = 1.0 / t198;
        let t210 = t70 * t209;
        let t212 = f64::exp(-(-t155 + t190 + t192) * t88 * t210);
        let t213 = t212 - 1.0;
        let t214 = 1.0 / t213;
        let t215 = t88 * t214;
        let t216 = t197 * t197;
        let t217 = 1.0 / t216;
        let t220 = t16 * t49;
        let t221 = 1.0 / t145;
        let t222 = t220 * t221;
        let t223 = t108 * t222;
        let t226 = t79 * t200 * t204 / 96.0 + 0.0004287401811806974 * t215 * t101 * t217 * t223;
        let t227 = t226 * t88;
        let t230 = 1.0 + 0.6585449182935511 * t215 * t226;
        let t231 = 1.0 / t230;
        let t234 = 1.0 + 0.6585449182935511 * t227 * t231;
        let t235 = f64::ln(t234);
        let t239 = piecewise3(t43, zeta_threshold, 1.0);
        let t242 = piecewise3(t129, 0.0, (t72 * t198 * t235 - t155 + t190 + t192) * t239 / 2.0);
        let t243 = t7 * t242;
        let t245 = 0.0478125 * t5 * t243;
        let tzk0 = t126 - t245;
        zk[ip] += tzk0;
    }
}
