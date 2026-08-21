//! GGA_X_HJS exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hjs.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_hjs_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t12 = t11 <= zeta_threshold;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t12, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = t3 * t3;
        let t21 = param_hyb_omega_0 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = t21 * t24;
        let t26 = piecewise3(t12, t13, t15);
        let t27 = 1.0 / t26;
        let t28 = 1.0 / t18;
        let t29 = t27 * t28;
        let t30 = M_CBRT6;
        let t31 = t23 * t23;
        let t32 = 1.0 / t31;
        let t33 = t30 * t32;
        let t34 = t33 * sigma[ip];
        let t35 = M_CBRT2;
        let t36 = t35 * t35;
        let t37 = rho[ip] * rho[ip];
        let t38 = t18 * t18;
        let t40 = 1.0 / t38 / t37;
        let t41 = t36 * t40;
        let t43 = param_a_0 * t30;
        let t44 = t43 * t32;
        let t45 = sigma[ip] * t36;
        let t46 = t45 * t40;
        let t50 = 1.0 / t22;
        let t51 = param_a_1 * t50;
        let t52 = rmath::sqrt(sigma[ip]);
        let t53 = t52 * sigma[ip];
        let t54 = t37 * t37;
        let t55 = 1.0 / t54;
        let t56 = t53 * t55;
        let t60 = t30 * t30;
        let t61 = param_a_2 * t60;
        let t63 = 1.0 / t23 / t22;
        let t64 = t61 * t63;
        let t65 = sigma[ip] * sigma[ip];
        let t66 = t65 * t35;
        let t67 = t54 * rho[ip];
        let t69 = 1.0 / t18 / t67;
        let t70 = t66 * t69;
        let t76 = 1.0 / t31 / t22;
        let t77 = param_a_3 * t30 * t76;
        let t78 = t52 * t65;
        let t79 = t78 * t36;
        let t80 = t54 * t37;
        let t82 = 1.0 / t38 / t80;
        let t83 = t79 * t82;
        let t87 = t22 * t22;
        let t88 = 1.0 / t87;
        let t89 = param_a_4 * t88;
        let t90 = t65 * sigma[ip];
        let t91 = t54 * t54;
        let t92 = 1.0 / t91;
        let t93 = t90 * t92;
        let t99 = 1.0 / t23 / t87;
        let t100 = param_a_5 * t60 * t99;
        let t101 = t52 * t90;
        let t102 = t101 * t35;
        let t103 = t91 * rho[ip];
        let t105 = 1.0 / t18 / t103;
        let t106 = t102 * t105;
        let t109 = t44 * t46 / 24.0 + t51 * t56 / 24.0 + t64 * t70 / 288.0 + t77 * t83 / 576.0 + t89 * t93 / 576.0 + t100 * t106 / 6912.0;
        let t112 = param_b_0 * t60 * t24;
        let t113 = t52 * t35;
        let t115 = 1.0 / t18 / rho[ip];
        let t120 = param_b_1 * t30;
        let t121 = t120 * t32;
        let t125 = param_b_2 * t50;
        let t129 = param_b_3 * t60;
        let t130 = t129 * t63;
        let t135 = param_b_4 * t30 * t76;
        let t139 = param_b_5 * t88;
        let t144 = param_b_6 * t60 * t99;
        let t148 = param_b_7 * t30;
        let t150 = 1.0 / t31 / t87;
        let t151 = t148 * t150;
        let t152 = t65 * t65;
        let t153 = t152 * t36;
        let t154 = t91 * t37;
        let t156 = 1.0 / t38 / t154;
        let t163 = param_b_8 / t87 / t22;
        let t164 = t52 * t152;
        let t165 = t91 * t54;
        let t166 = 1.0 / t165;
        let t170 = 1.0 + t112 * t113 * t115 / 12.0 + t121 * t46 / 24.0 + t125 * t56 / 24.0 + t130 * t70 / 288.0 + t135 * t83 / 576.0 + t139 * t93 / 576.0 + t144 * t106 / 6912.0 + t151 * t153 * t156 / 13824.0 + t163 * t164 * t166 / 13824.0;
        let t171 = 1.0 / t170;
        let t172 = t109 * t171;
        let t175 = t34 * t41 * t172 / 24.0;
        let t176 = 1e-10 < t175;
        let t177 = piecewise3(t176, t175, 1e-10);
        let t178 = param_hyb_omega_0 * param_hyb_omega_0;
        let t179 = t178 * t3;
        let t180 = t26 * t26;
        let t182 = t32 / t180;
        let t183 = 1.0 / t38;
        let t185 = t179 * t182 * t183;
        let t187 = 0.60965 + t177 + t185 / 3.0;
        let t188 = rmath::sqrt(t187);
        let t189 = 1.0 / t188;
        let t191 = t25 * t29 * t189;
        let t193 = 1.0 - t191 / 3.0;
        let t194 = 0.60965 + t177;
        let t195 = 1.0 / t194;
        let t198 = t33 * t46;
        let t200 = 1.0 + t198 / 96.0;
        let t201 = 1.0 / t200;
        let t202 = t41 * t201;
        let t206 = 1.0 + 0.013006513974354691 * t34 * t202 + 4.21411052769092 * t177;
        let t208 = t178 * param_hyb_omega_0 * t50;
        let t210 = 1.0 / t180 / t26;
        let t211 = 1.0 / rho[ip];
        let t212 = t210 * t211;
        let t214 = 1.0 / t188 / t187;
        let t216 = t208 * t212 * t214;
        let t218 = 2.0 - t191 + t216 / 3.0;
        let t219 = t206 * t218;
        let t220 = t194 * t194;
        let t221 = 1.0 / t220;
        let t227 = t220 * t194;
        let t229 = rmath::sqrt(t194);
        let t230 = t229 * t227;
        let t231 = rmath::sqrt(M_PI);
        let t233 = rmath::sqrt(t177);
        let t236 = 0.0 < 0.7572109999 + t177;
        let t238 = piecewise3(t236, 0.757211 + t177, 1e-10);
        let t239 = rmath::sqrt(t238);
        let t241 = 4.0 / 5.0 * t231 + 12.0 / 5.0 * t233 - 12.0 / 5.0 * t239;
        let t243 = 0.0474596 * t206 * t194 + 0.028363733333333332 * t220 - 0.9086532 * t227 - t230 * t241;
        let t246 = t178 * t178;
        let t248 = t246 * param_hyb_omega_0 * t3;
        let t249 = t248 * t76;
        let t250 = t180 * t180;
        let t252 = 1.0 / t250 / t26;
        let t254 = 1.0 / t38 / rho[ip];
        let t255 = t252 * t254;
        let t256 = t187 * t187;
        let t258 = 1.0 / t188 / t256;
        let t262 = 8.0 - 5.0 * t191 + 10.0 / 3.0 * t216 - t249 * t255 * t258 / 3.0;
        let t263 = t243 * t262;
        let t264 = 1.0 / t227;
        let t268 = 3.0 * t185;
        let t269 = 9.0 * t177 + t268;
        let t270 = rmath::sqrt(t269);
        let t272 = 9.0 * t238 + t268;
        let t273 = rmath::sqrt(t272);
        let t275 = t270 / 3.0 - t273 / 3.0;
        let t279 = t24 * t27;
        let t281 = t21 * t279 * t28;
        let t283 = t281 / 3.0 + t270 / 3.0;
        let t285 = t281 / 3.0 + t188;
        let t286 = 1.0 / t285;
        let t288 = rmath::ln(t283 * t286);
        let t292 = t281 / 3.0 + t273 / 3.0;
        let t294 = rmath::ln(t292 * t286);
        let t297 = 0.757211 + 0.04727288888888889 * t193 * t195 + 0.026366444444444446 * t219 * t221 - t263 * t264 / 9.0 + 2.0 / 3.0 * t25 * t29 * t275 + 2.0 * t177 * t288 - 2.0 * t238 * t294;
        let t301 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t297);
        let tzk0 = 2.0 * t301;
        zk[ip] += tzk0;
    }
}
