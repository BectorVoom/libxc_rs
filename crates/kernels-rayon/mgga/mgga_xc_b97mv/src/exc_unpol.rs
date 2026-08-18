//! MGGA_XC_B97MV exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_b97mv.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_b97mv_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c_x_1: f64,
    param_c_x_2: f64,
    param_c_x_3: f64,
    param_c_x_4: f64,
    param_c_x_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_ss_0: f64,
    param_c_os_1: f64,
    param_c_os_2: f64,
    param_c_os_3: f64,
    param_c_os_4: f64,
    param_c_os_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = 1.0 <= zeta_threshold;
        let t3 = piecewise3(t2, zeta_threshold, 1.0);
        let t5 = rho[ip] / 2.0 <= dens_threshold;
        let t6 = M_CBRT3;
        let t7 = M_CBRTPI;
        let t9 = t6 / t7;
        let t10 = M_CBRT2;
        let t11 = t10 * t10;
        let t13 = pow_1_3(zeta_threshold);
        let t14 = t13 * zeta_threshold;
        let t16 = piecewise3(2.0 <= zeta_threshold, t14, 2.0 * t10);
        let t17 = t11 * t16;
        let t18 = pow_1_3(rho[ip]);
        let t22 = piecewise3(t5, 0.0, -3.0 / 16.0 * t9 * t17 * t18);
        let t23 = 0.0 <= dens_threshold;
        let t25 = piecewise3(0.0 <= zeta_threshold, t14, 0.0);
        let t26 = t11 * t25;
        let t30 = piecewise3(t23, 0.0, -3.0 / 16.0 * t9 * t26 * t18);
        let t32 = t3 * (t22 + t30);
        let t34 = param_c_x_1;
        let t35 = t34 * sigma[ip];
        let t36 = rho[ip] * rho[ip];
        let t37 = t18 * t18;
        let t39 = 1.0 / t37 / t36;
        let t40 = t11 * t39;
        let t41 = sigma[ip] * t11;
        let t42 = t41 * t39;
        let t44 = 1.0 + 0.004 * t42;
        let t45 = 1.0 / t44;
        let t46 = t40 * t45;
        let t49 = param_c_x_2;
        let t50 = sigma[ip] * sigma[ip];
        let t51 = t49 * t50;
        let t52 = t36 * t36;
        let t53 = t52 * rho[ip];
        let t55 = 1.0 / t18 / t53;
        let t56 = t10 * t55;
        let t57 = t44 * t44;
        let t58 = 1.0 / t57;
        let t59 = t56 * t58;
        let t62 = param_c_x_3;
        let t63 = M_CBRT6;
        let t64 = t63 * t63;
        let t65 = M_PI * M_PI;
        let t66 = pow_1_3(t65);
        let t67 = t66 * t66;
        let t68 = t64 * t67;
        let t69 = 3.0 / 10.0 * t68;
        let t70 = tau[ip] * t11;
        let t72 = 1.0 / t37 / rho[ip];
        let t73 = t70 * t72;
        let t74 = t69 - t73;
        let t75 = t62 * t74;
        let t76 = t69 + t73;
        let t77 = 1.0 / t76;
        let t79 = param_c_x_4;
        let t80 = t79 * t74;
        let t81 = t80 * t77;
        let t82 = t39 * t45;
        let t86 = param_c_x_0 + 0.004 * t35 * t46 + 3.2e-05 * t51 * t59 + t75 * t77 + 0.004 * t81 * t41 * t82;
        let t87 = t32 * t86;
        let t88 = t5 || t2;
        let t89 = 1.0 / M_PI;
        let t90 = pow_1_3(t89);
        let t91 = t6 * t90;
        let t92 = M_CBRT4;
        let t93 = t92 * t92;
        let t94 = t91 * t93;
        let t95 = 1.0 / t18;
        let t98 = piecewise3(t2, 1.0 / t13, 1.0);
        let t100 = t94 * t95 * t10 * t98;
        let t102 = 1.0 + 0.053425 * t100;
        let t103 = f64::sqrt(t100);
        let t106 = pow_3_2(t100);
        let t108 = t6 * t6;
        let t109 = t90 * t90;
        let t110 = t108 * t109;
        let t111 = t110 * t92;
        let t112 = 1.0 / t37;
        let t114 = t98 * t98;
        let t116 = t111 * t112 * t11 * t114;
        let t118 = 3.79785 * t103 + 0.8969 * t100 + 0.204775 * t106 + 0.123235 * t116;
        let t121 = 1.0 + 16.081979498692537 / t118;
        let t122 = f64::ln(t121);
        let t124 = 0.0621814 * t102 * t122;
        let t128 = 1.0 / (2.0 * t10 - 2.0);
        let t129 = (t16 + t25 - 2.0) * t128;
        let t131 = 1.0 + 0.05137 * t100;
        let t136 = 7.05945 * t103 + 1.549425 * t100 + 0.420775 * t106 + 0.1562925 * t116;
        let t139 = 1.0 + 32.16395899738507 / t136;
        let t140 = f64::ln(t139);
        let t144 = 1.0 + 0.0278125 * t100;
        let t149 = 5.1785 * t103 + 0.905775 * t100 + 0.1100325 * t106 + 0.1241775 * t116;
        let t152 = 1.0 + 29.608749977793437 / t149;
        let t153 = f64::ln(t152);
        let t154 = t144 * t153;
        let t163 = piecewise3(t88, 0.0, t3 * (-t124 + t129 * (-0.0310907 * t131 * t140 + t124 - 0.0197516734986138 * t154) + 0.0197516734986138 * t129 * t154) / 2.0);
        let t165 = param_c_ss_1;
        let t166 = t165 * t50;
        let t168 = 1.0 + 0.2 * t42;
        let t169 = t168 * t168;
        let t170 = 1.0 / t169;
        let t171 = t56 * t170;
        let t174 = param_c_ss_2;
        let t175 = t174 * t74;
        let t177 = param_c_ss_3;
        let t178 = t74 * t74;
        let t179 = t178 * t74;
        let t180 = t177 * t179;
        let t181 = t76 * t76;
        let t182 = t181 * t76;
        let t183 = 1.0 / t182;
        let t184 = t180 * t183;
        let t185 = t50 * t10;
        let t186 = t55 * t170;
        let t187 = t185 * t186;
        let t190 = param_c_ss_4;
        let t191 = t178 * t178;
        let t192 = t190 * t191;
        let t193 = t181 * t181;
        let t194 = 1.0 / t193;
        let t195 = t192 * t194;
        let t198 = param_c_ss_0 + 0.08 * t166 * t171 + t175 * t77 + 0.08 * t184 * t187 + 0.08 * t195 * t187;
        let t200 = 2.0 * t163 * t198;
        let t202 = t91 * t93 * t95;
        let t204 = 1.0 + 0.053425 * t202;
        let t205 = f64::sqrt(t202);
        let t208 = pow_3_2(t202);
        let t211 = t110 * t92 * t112;
        let t213 = 3.79785 * t205 + 0.8969 * t202 + 0.204775 * t208 + 0.123235 * t211;
        let t216 = 1.0 + 16.081979498692537 / t213;
        let t217 = f64::ln(t216);
        let t220 = piecewise3(t2, t14, 1.0);
        let t223 = (2.0 * t220 - 2.0) * t128;
        let t225 = 1.0 + 0.0278125 * t202;
        let t230 = 5.1785 * t205 + 0.905775 * t202 + 0.1100325 * t208 + 0.1241775 * t211;
        let t233 = 1.0 + 29.608749977793437 / t230;
        let t234 = f64::ln(t233);
        let t239 = -0.0621814 * t204 * t217 + 0.0197516734986138 * t223 * t225 * t234 - 2.0 * t163;
        let t241 = param_c_os_1;
        let t242 = t241 * sigma[ip];
        let t244 = 1.0 + 0.006 * t42;
        let t245 = 1.0 / t244;
        let t249 = param_c_os_2;
        let t250 = t50 * sigma[ip];
        let t251 = t249 * t250;
        let t252 = t52 * t52;
        let t253 = 1.0 / t252;
        let t254 = t244 * t244;
        let t255 = t254 * t244;
        let t256 = 1.0 / t255;
        let t257 = t253 * t256;
        let t260 = param_c_os_3;
        let t262 = 3.0 / 5.0 * t68 * t73;
        let t263 = tau[ip] * tau[ip];
        let t264 = t263 * t10;
        let t265 = t36 * rho[ip];
        let t267 = 1.0 / t18 / t265;
        let t269 = 4.0 * t264 * t267;
        let t270 = t262 - t269;
        let t271 = t260 * t270;
        let t272 = t262 + t269;
        let t273 = 1.0 / t272;
        let t275 = param_c_os_4;
        let t276 = t270 * t270;
        let t278 = t275 * t276 * t270;
        let t279 = t272 * t272;
        let t280 = t279 * t272;
        let t281 = 1.0 / t280;
        let t282 = t278 * t281;
        let t283 = 1.0 / t254;
        let t284 = t55 * t283;
        let t285 = t185 * t284;
        let t288 = param_c_os_0 + 0.006 * t242 * t40 * t245 + 8.64e-07 * t251 * t257 + t271 * t273 + 7.2e-05 * t282 * t285;
        let t289 = t239 * t288;
        let tzk0 = t87 + t200 + t289;
        zk[ip] += tzk0;
    }
}
