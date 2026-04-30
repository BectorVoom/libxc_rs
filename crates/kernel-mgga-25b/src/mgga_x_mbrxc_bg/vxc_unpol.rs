//! MGGA_X_MBRXC_BG vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbrxc_bg.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::mbrxc::xc_mgga_x_mbrxc_get_x;

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_mbrxc_bg_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = tau[ip] * t22;
        let t24 = t19 * t19;
        let t26 = 1.0 / t24 / rho[ip];
        let t29 = M_CBRT6;
        let t30 = t29 * t29;
        let t31 = M_PI * M_PI;
        let t32 = pow_1_3(t31);
        let t33 = t32 * t32;
        let t36 = sigma[ip] * t22;
        let t37 = rho[ip] * rho[ip];
        let t39 = 1.0 / t24 / t37;
        let t42 = sigma[ip] * sigma[ip];
        let t43 = t42 * t21;
        let t44 = t37 * t37;
        let t45 = t44 * rho[ip];
        let t47 = 1.0 / t19 / t45;
        let t50 = 0.149492e0 * t23 * t26 - 3.0 / 10.0 * t30 * t33 + 0.147e0 * t36 * t39 + 0.64e-2 * t43 * t47;
        let t51 = xc_mgga_x_mbrxc_get_x(t50);
        let t52 = pow_1_4(f64::EPSILON);
        let t53 = t51 < t52;
        let t54 = pow_1_3(32.0);
        let t55 = t54 * t5;
        let t56 = t4 * t4;
        let t58 = pow_1_3(1.0 / M_PI);
        let t59 = 1.0 / t58;
        let t60 = t56 * t59;
        let t61 = M_CBRT4;
        let t62 = t60 * t61;
        let t63 = t55 * t62;
        let t65 = t55 * t56;
        let t66 = t59 * t61;
        let t67 = t51 * t51;
        let t68 = t66 * t67;
        let t71 = t67 * t51;
        let t72 = t66 * t71;
        let t75 = t67 * t67;
        let t76 = t66 * t75;
        let t79 = t75 * t51;
        let t80 = t66 * t79;
        let t83 = t75 * t67;
        let t84 = t66 * t83;
        let t92 = t55 * t60;
        let t93 = t52 < t51;
        let t94 = piecewise3(t93, t51, t52);
        let t96 = f64::exp(t94 / 3.0);
        let t97 = t61 * t96;
        let t98 = f64::exp(-t94);
        let t99 = t94 * t94;
        let t101 = t99 + 5.0 * t94 + 8.0;
        let t102 = t98 * t101;
        let t103 = 8.0 - t102;
        let t104 = 1.0 / t94;
        let t105 = t103 * t104;
        let t106 = 1.0 + t94;
        let t107 = pow_1_3(t106);
        let t108 = 1.0 / t107;
        let t109 = t105 * t108;
        let t113 = piecewise3(t53, -t63 / 12.0 - t65 * t68 / 108.0 + t65 * t72 / 108.0 - 13.0 / 1620.0 * t65 * t76 + 67.0 / 9720.0 * t65 * t80 - 52.0 / 8505.0 * t65 * t84 + 1811.0 / 326592.0 * t65 * t66 * t75 * t71, -t92 * t97 * t109 / 36.0);
        let t117 = piecewise3(t3, 0.0, 3.0 / 16.0 * t7 * t20 * t113);
        let tzk0 = 2.0 * t117;
        zk[ip] += tzk0;
        let t119 = t18 / t24;
        let t123 = M_PI * t56;
        let t124 = t66 * t51;
        let t125 = t123 * t124;
        let t128 = t37 * rho[ip];
        let t130 = 1.0 / t24 / t128;
        let t133 = t44 * t37;
        let t135 = 1.0 / t19 / t133;
        let t138 = -0.24915333333333333333e0 * t23 * t39 - 0.392e0 * t36 * t130 - 0.34133333333333333333e-1 * t43 * t135;
        let t139 = t50 * t50;
        let t140 = 1.0 / t139;
        let t141 = t138 * t140;
        let t142 = 1.0 + t51;
        let t143 = pow_1_3(t142);
        let t144 = t143 * t143;
        let t145 = 1.0 / t144;
        let t146 = t141 * t145;
        let t148 = f64::exp(-2.0 / 3.0 * t51);
        let t149 = 1.0 / t148;
        let t151 = t67 - 3.0 * t51 + 6.0;
        let t152 = 1.0 / t151;
        let t153 = t149 * t152;
        let t154 = t51 - 3.0;
        let t155 = t154 * t154;
        let t156 = t153 * t155;
        let t157 = t146 * t156;
        let t160 = t123 * t68;
        let t163 = t123 * t72;
        let t166 = t123 * t76;
        let t169 = t123 * t80;
        let t172 = t123 * t84;
        let t176 = t54 * t54;
        let t177 = t5 * t5;
        let t178 = t176 * t177;
        let t179 = t178 * t141;
        let t180 = t145 * t149;
        let t181 = t152 * t155;
        let t182 = t180 * t181;
        let t183 = t179 * t182;
        let t185 = piecewise3(t93, t183 / 4.0, 0.0);
        let t186 = t185 * t96;
        let t190 = t185 * t98;
        let t191 = t190 * t101;
        let t195 = 2.0 * t94 * t185 + 5.0 * t185;
        let t196 = t98 * t195;
        let t197 = t191 - t196;
        let t198 = t197 * t104;
        let t199 = t198 * t108;
        let t203 = t96 * t103;
        let t204 = 1.0 / t99;
        let t205 = t204 * t108;
        let t206 = t205 * t185;
        let t211 = 1.0 / t107 / t106;
        let t212 = t104 * t211;
        let t213 = t212 * t185;
        let t218 = piecewise3(t53, -4.0 / 27.0 * t125 * t157 + 2.0 / 9.0 * t160 * t157 - 104.0 / 405.0 * t163 * t157 + 67.0 / 243.0 * t166 * t157 - 832.0 / 2835.0 * t169 * t157 + 1811.0 / 5832.0 * t172 * t157, -t63 * t186 * t109 / 108.0 - t92 * t97 * t199 / 36.0 + t63 * t203 * t206 / 36.0 + t63 * t203 * t213 / 108.0);
        let t223 = piecewise3(t3, 0.0, t7 * t119 * t113 / 16.0 + 3.0 / 16.0 * t7 * t20 * t218);
        let tvrho0 = 2.0 * rho[ip] * t223 + 2.0 * t117;
        vrho[ip] += tvrho0;
        let t226 = t22 * t39;
        let t228 = sigma[ip] * t21;
        let t231 = 0.147e0 * t226 + 0.128e-1 * t228 * t47;
        let t232 = t231 * t140;
        let t233 = t232 * t145;
        let t234 = t233 * t156;
        let t248 = t178 * t232;
        let t249 = t248 * t182;
        let t251 = piecewise3(t93, t249 / 4.0, 0.0);
        let t252 = t251 * t96;
        let t256 = t251 * t98;
        let t257 = t256 * t101;
        let t261 = 2.0 * t94 * t251 + 5.0 * t251;
        let t262 = t98 * t261;
        let t263 = t257 - t262;
        let t264 = t263 * t104;
        let t265 = t264 * t108;
        let t269 = t205 * t251;
        let t273 = t212 * t251;
        let t278 = piecewise3(t53, -4.0 / 27.0 * t125 * t234 + 2.0 / 9.0 * t160 * t234 - 104.0 / 405.0 * t163 * t234 + 67.0 / 243.0 * t166 * t234 - 832.0 / 2835.0 * t169 * t234 + 1811.0 / 5832.0 * t172 * t234, -t63 * t252 * t109 / 108.0 - t92 * t97 * t265 / 36.0 + t63 * t203 * t269 / 36.0 + t63 * t203 * t273 / 108.0);
        let t282 = piecewise3(t3, 0.0, 3.0 / 16.0 * t7 * t20 * t278);
        let tvsigma0 = 2.0 * rho[ip] * t282;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t284 = t61 * t51;
        let t286 = t60 * t284 * t22;
        let t287 = t26 * t140;
        let t288 = t287 * t145;
        let t289 = t288 * t156;
        let t292 = t61 * t67;
        let t294 = t60 * t292 * t22;
        let t297 = t61 * t71;
        let t299 = t60 * t297 * t22;
        let t302 = t61 * t75;
        let t304 = t60 * t302 * t22;
        let t307 = t61 * t79;
        let t309 = t60 * t307 * t22;
        let t312 = t61 * t83;
        let t314 = t60 * t312 * t22;
        let t318 = t176 * t22;
        let t319 = t318 * t287;
        let t320 = t319 * t182;
        let t322 = piecewise3(t93, 0.80166183658230359753e-1 * t320, 0.0);
        let t323 = t322 * t96;
        let t327 = t322 * t98;
        let t332 = 2.0 * t94 * t322 + 5.0 * t322;
        let t333 = t98 * t332;
        let t334 = t327 * t101 - t333;
        let t335 = t334 * t104;
        let t336 = t335 * t108;
        let t340 = t205 * t322;
        let t344 = t212 * t322;
        let t349 = piecewise3(t53, -0.69576736143769684563e-1 * t286 * t289 + 0.10436510421565452684e0 * t294 * t289 - 0.12059967598253411991e0 * t299 * t289 + 0.12949003671201580183e0 * t304 * t289 - 0.13782820112289613704e0 * t309 * t289 + 0.14583734856060983651e0 * t314 * t289, -t63 * t323 * t109 / 108.0 - t92 * t97 * t336 / 36.0 + t63 * t203 * t340 / 36.0 + t63 * t203 * t344 / 108.0);
        let t353 = piecewise3(t3, 0.0, 3.0 / 16.0 * t7 * t20 * t349);
        let tvtau0 = 2.0 * rho[ip] * t353;
        vtau[ip] += tvtau0;
    }
}
