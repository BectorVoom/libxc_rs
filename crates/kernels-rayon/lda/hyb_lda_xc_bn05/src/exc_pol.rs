//! HYB_LDA_XC_BN05 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/hyb_lda_xc_bn05.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_lda_xc_bn05_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t3 * t1;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = t6 * t4;
        let t8 = M_CBRT2;
        let t9 = t8 * t8;
        let t10 = rho0 - rho1;
        let t11 = rho0 + rho1;
        let t12 = 1.0 / t11;
        let t13 = t12 * t10;
        let t14 = 1.0 + t13;
        let t15 = t14 <= zeta_threshold;
        let t16 = pow_1_3(zeta_threshold);
        let t17 = t16 * zeta_threshold;
        let t18 = pow_1_3(t14);
        let t20 = piecewise3(t15, t17, t18 * t14);
        let t21 = t20 * t9;
        let t22 = pow_1_3(t11);
        let t23 = pow_1_3(9.0);
        let t24 = t23 * t23;
        let t25 = t3 * t3;
        let t26 = t25 * t24;
        let t27 = param_hyb_omega_0 * t26;
        let t28 = 1.0 / t22;
        let t29 = t28 * t1;
        let t30 = piecewise3(t15, t16, t18);
        let t31 = 1.0 / t30;
        let t34 = t31 * t29 * t27 / 18.0;
        let t35 = 1.92 <= t34;
        let t36 = 1.92 < t34;
        let t37 = piecewise3(t36, t34, 1.92);
        let t38 = t37 * t37;
        let t41 = t38 * t38;
        let t42 = 1.0 / t41;
        let t44 = t41 * t38;
        let t45 = 1.0 / t44;
        let t47 = t41 * t41;
        let t48 = 1.0 / t47;
        let t50 = t47 * t38;
        let t51 = 1.0 / t50;
        let t53 = t47 * t41;
        let t54 = 1.0 / t53;
        let t56 = t47 * t44;
        let t57 = 1.0 / t56;
        let t59 = t47 * t47;
        let t60 = 1.0 / t59;
        let t63 = 1.0 / t59 / t38;
        let t66 = 1.0 / t59 / t41;
        let t69 = 1.0 / t59 / t44;
        let t72 = 1.0 / t59 / t47;
        let t75 = 1.0 / t59 / t50;
        let t78 = 1.0 / t59 / t53;
        let t81 = 1.0 / t59 / t56;
        let t83 = t59 * t59;
        let t84 = 1.0 / t83;
        let t87 = 1.0 / t83 / t38;
        let t90 = 1.0 / t83 / t41;
        let t92 = 1.0 / t38 / 9.0 - t42 / 30.0 + t45 / 70.0 - t48 / 135.0 + t51 / 231.0 - t54 / 364.0 + t57 / 540.0 - t60 / 765.0 + t63 / 1045.0 - t66 / 1386.0 + t69 / 1794.0 - t72 / 2275.0 + t75 / 2835.0 - t78 / 3480.0 + t81 / 4216.0 - t84 / 5049.0 + t87 / 5985.0 - t90 / 7030.0;
        let t93 = piecewise3(t36, 1.92, t34);
        let t94 = f64::atan2(1.0, t93);
        let t95 = t93 * t93;
        let t96 = t95 + 3.0;
        let t97 = 1.0 / t95;
        let t98 = 1.0 + t97;
        let t99 = f64::ln(t98);
        let t101 = -t96 * t99 + 1.0;
        let t104 = t94 + t101 * t93 / 4.0;
        let t108 = piecewise3(t35, t92, 1.0 - 8.0 / 3.0 * t104 * t93);
        let t109 = t108 * t22;
        let t112 = 3.0 / 32.0 * t109 * t21 * t7;
        let t113 = 1.0 - t13;
        let t114 = t113 <= zeta_threshold;
        let t115 = pow_1_3(t113);
        let t117 = piecewise3(t114, t17, t115 * t113);
        let t118 = t117 * t9;
        let t119 = piecewise3(t114, t16, t115);
        let t120 = 1.0 / t119;
        let t123 = t120 * t29 * t27 / 18.0;
        let t124 = 1.92 <= t123;
        let t125 = 1.92 < t123;
        let t126 = piecewise3(t125, t123, 1.92);
        let t127 = t126 * t126;
        let t130 = t127 * t127;
        let t131 = 1.0 / t130;
        let t133 = t130 * t127;
        let t134 = 1.0 / t133;
        let t136 = t130 * t130;
        let t137 = 1.0 / t136;
        let t139 = t136 * t127;
        let t140 = 1.0 / t139;
        let t142 = t136 * t130;
        let t143 = 1.0 / t142;
        let t145 = t136 * t133;
        let t146 = 1.0 / t145;
        let t148 = t136 * t136;
        let t149 = 1.0 / t148;
        let t152 = 1.0 / t148 / t127;
        let t155 = 1.0 / t148 / t130;
        let t158 = 1.0 / t148 / t133;
        let t161 = 1.0 / t148 / t136;
        let t164 = 1.0 / t148 / t139;
        let t167 = 1.0 / t148 / t142;
        let t170 = 1.0 / t148 / t145;
        let t172 = t148 * t148;
        let t173 = 1.0 / t172;
        let t176 = 1.0 / t172 / t127;
        let t179 = 1.0 / t172 / t130;
        let t181 = 1.0 / t127 / 9.0 - t131 / 30.0 + t134 / 70.0 - t137 / 135.0 + t140 / 231.0 - t143 / 364.0 + t146 / 540.0 - t149 / 765.0 + t152 / 1045.0 - t155 / 1386.0 + t158 / 1794.0 - t161 / 2275.0 + t164 / 2835.0 - t167 / 3480.0 + t170 / 4216.0 - t173 / 5049.0 + t176 / 5985.0 - t179 / 7030.0;
        let t182 = piecewise3(t125, 1.92, t123);
        let t183 = f64::atan2(1.0, t182);
        let t184 = t182 * t182;
        let t185 = t184 + 3.0;
        let t186 = 1.0 / t184;
        let t187 = 1.0 + t186;
        let t188 = f64::ln(t187);
        let t190 = -t185 * t188 + 1.0;
        let t193 = t183 + t190 * t182 / 4.0;
        let t197 = piecewise3(t124, t181, 1.0 - 8.0 / 3.0 * t193 * t182);
        let t198 = t197 * t22;
        let t201 = 3.0 / 32.0 * t198 * t118 * t7;
        let t203 = t28 * t6 * t4;
        let t205 = 1.0 + 0.053425 * t203;
        let t206 = f64::sqrt(t203);
        let t209 = pow_3_2(t203);
        let t211 = t1 * t1;
        let t212 = t25 * t211;
        let t213 = t22 * t22;
        let t214 = 1.0 / t213;
        let t216 = t214 * t5 * t212;
        let t218 = 3.79785 * t206 + 0.8969 * t203 + 0.204775 * t209 + 0.123235 * t216;
        let t221 = 1.0 + 16.081979498692537 / t218;
        let t222 = f64::ln(t221);
        let t224 = 0.0621814 * t222 * t205;
        let t225 = t10 * t10;
        let t226 = t225 * t225;
        let t227 = t11 * t11;
        let t228 = t227 * t227;
        let t229 = 1.0 / t228;
        let t230 = t229 * t226;
        let t231 = t20 + t117 - 2.0;
        let t234 = 1.0 / (2.0 * t8 - 2.0);
        let t235 = t234 * t231;
        let t237 = 1.0 + 0.05137 * t203;
        let t242 = 7.05945 * t206 + 1.549425 * t203 + 0.420775 * t209 + 0.1562925 * t216;
        let t245 = 1.0 + 32.16395899738507 / t242;
        let t246 = f64::ln(t245);
        let t250 = 1.0 + 0.0278125 * t203;
        let t255 = 5.1785 * t206 + 0.905775 * t203 + 0.1100325 * t209 + 0.1241775 * t216;
        let t258 = 1.0 + 29.608749977793437 / t255;
        let t259 = f64::ln(t258);
        let t260 = t259 * t250;
        let t262 = -0.0310907 * t246 * t237 + t224 - 0.0197516734986138 * t260;
        let t263 = t262 * t235;
        let t267 = -t224 + t263 * t230 + 0.0197516734986138 * t260 * t235;
        let t270 = 3.2 - 0.225 * t203 + t216 / 4.0;
        let t271 = 1.0 / t270;
        let t273 = 3.4602 * t271 * t267;
        let tzk0 = -t112 - t201 + t273;
        zk[ip] += tzk0;
    }
}
