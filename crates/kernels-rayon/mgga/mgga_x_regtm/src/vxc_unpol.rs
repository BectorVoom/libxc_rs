//! MGGA_X_REGTM vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_regtm.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_regtm_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t27 = t23 * t26;
        let t28 = sigma[ip] * t22;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t24 / t29;
        let t32 = t28 * t31;
        let t34 = t27 - t32 / 8.0;
        let t35 = M_CBRT6;
        let t36 = t34 * t35;
        let t37 = M_PI * M_PI;
        let t38 = pow_1_3(t37);
        let t39 = t38 * t38;
        let t40 = 1.0 / t39;
        let t41 = t35 * t40;
        let t42 = t41 * t32;
        let t44 = t36 * t40;
        let t46 = 1.0 - 5.0 / 9.0 * t44;
        let t47 = t46 * t46;
        let t48 = t47 * t46;
        let t49 = t34 * t34;
        let t50 = t35 * t35;
        let t51 = t49 * t50;
        let t53 = 1.0 / t38 / t37;
        let t56 = 1.0 + 0.6714891975308642 * t51 * t53;
        let t57 = f64::sqrt(t56);
        let t59 = 1.0 / t57 / t56;
        let t60 = t48 * t59;
        let t62 = f64::exp(-t42 / 8.0);
        let t64 = t42 / 24.0 + t60 * t62;
        let t65 = 1.0 / t64;
        let t66 = t40 * t65;
        let t69 = 1.0 + t36 * t66 / 3.0;
        let t70 = t69 * t69;
        let t72 = t70 * t69;
        let t73 = 1.0 / t72;
        let t75 = 1.0 / t70 + 3.0 * t73;
        let t76 = 1.0 + t73;
        let t77 = t76 * t76;
        let t78 = 1.0 / t77;
        let t79 = t75 * t78;
        let t81 = t50 * t53;
        let t82 = sigma[ip] * sigma[ip];
        let t83 = t82 * t21;
        let t84 = t29 * t29;
        let t85 = t84 * rho[ip];
        let t87 = 1.0 / t19 / t85;
        let t91 = 1.0 + 0.1504548888888889 * t42 + 0.00537989809245259 * t81 * t83 * t87;
        let t92 = f64::powf(t91, 1.0 / 5.0);
        let t103 = 1.0 + 0.06394332777777778 * t42 - 5.0 / 9.0 * (0.14554132 * t27 + 0.256337604 * t50 * t39 + 0.011867481666666667 * t32) * t35 * t40;
        let t104 = t92 * t92;
        let t105 = 1.0 / t104;
        let t108 = 1.0 / t92 + 7.0 / 9.0 * t103 * t105;
        let t110 = 1.0 - t79;
        let t113 = (10.0 / 81.0 + 25.0 / 8748.0 * t42) * t35;
        let t114 = t113 * t40;
        let t119 = t44 / 4.0 - 9.0 / 20.0 + t42 / 36.0;
        let t120 = t119 * t119;
        let t122 = 1.0 / rho[ip];
        let t123 = sigma[ip] * t122;
        let t124 = 1.0 / tau[ip];
        let t126 = t123 * t124 / 8.0;
        let t127 = t126 < 1.0;
        let t128 = piecewise3(t127, t126, 1.0);
        let t129 = t119 * t128;
        let t130 = 1.0 - t128;
        let t133 = 1.0 + 5.0 / 12.0 * t114 * t32 + 292.0 / 405.0 * t120 - 146.0 / 135.0 * t129 * t130;
        let t134 = f64::powf(t133, 1.0 / 10.0);
        let t136 = t108 * t79 + t110 * t134;
        let t140 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t136);
        let tzk0 = 2.0 * t140;
        zk[ip] += tzk0;
        let t142 = t18 / t24;
        let t146 = t23 * t31;
        let t148 = t29 * rho[ip];
        let t150 = 1.0 / t24 / t148;
        let t151 = t28 * t150;
        let t153 = -5.0 / 3.0 * t146 + t151 / 3.0;
        let t154 = t153 * t35;
        let t156 = t64 * t64;
        let t158 = t40 / t156;
        let t159 = t41 * t151;
        let t161 = t47 * t59;
        let t162 = t161 * t62;
        let t163 = t154 * t40;
        let t166 = t56 * t56;
        let t168 = 1.0 / t57 / t166;
        let t169 = t48 * t168;
        let t170 = t169 * t62;
        let t171 = t34 * t50;
        let t172 = t53 * t153;
        let t173 = t171 * t172;
        let t176 = t60 * t41;
        let t177 = t150 * t62;
        let t181 = -t159 / 9.0 - 5.0 / 3.0 * t162 * t163 - 2.0144675925925926 * t170 * t173 + t176 * t28 * t177 / 3.0;
        let t182 = t158 * t181;
        let t185 = t154 * t66 / 3.0 - t36 * t182 / 3.0;
        let t188 = t70 * t70;
        let t189 = 1.0 / t188;
        let t190 = t189 * t185;
        let t192 = -2.0 * t185 * t73 - 9.0 * t190;
        let t193 = t192 * t78;
        let t196 = 1.0 / t77 / t76;
        let t197 = t75 * t196;
        let t198 = t108 * t189;
        let t199 = t198 * t185;
        let t203 = 1.0 / t92 / t91;
        let t205 = t84 * t29;
        let t207 = 1.0 / t19 / t205;
        let t209 = t81 * t83 * t207;
        let t211 = -0.40121303703703703 * t159 - 0.028692789826413812 * t209;
        let t221 = -0.17051554074074074 * t159 - 5.0 / 9.0 * (-0.24256886666666666 * t146 - 0.031646617777777775 * t151) * t35 * t40;
        let t225 = 1.0 / t104 / t91;
        let t226 = t103 * t225;
        let t229 = -t203 * t211 / 5.0 + 7.0 / 9.0 * t221 * t105 - 14.0 / 45.0 * t226 * t211;
        let t233 = -6.0 * t190 * t197 - t193;
        let t235 = t134 * t134;
        let t236 = t235 * t235;
        let t237 = t236 * t236;
        let t238 = t237 * t134;
        let t239 = 1.0 / t238;
        let t240 = t110 * t239;
        let t246 = t163 / 4.0 - 2.0 / 27.0 * t159;
        let t249 = t246 * t128;
        let t252 = 1.0 / t29;
        let t253 = sigma[ip] * t252;
        let t256 = piecewise3(t127, -t253 * t124 / 8.0, 0.0);
        let t257 = t119 * t256;
        let t262 = -125.0 / 19683.0 * t209 - 10.0 / 9.0 * t114 * t151 + 584.0 / 405.0 * t119 * t246 - 146.0 / 135.0 * t249 * t130 - 146.0 / 135.0 * t257 * t130 + 146.0 / 135.0 * t129 * t256;
        let t265 = t193 * t108 + 6.0 * t197 * t199 + t79 * t229 + t233 * t134 + t240 * t262 / 10.0;
        let t270 = piecewise3(t3, 0.0, -t7 * t142 * t136 / 8.0 - 3.0 / 8.0 * t7 * t20 * t265);
        let tvrho0 = 2.0 * rho[ip] * t270 + 2.0 * t140;
        vrho[ip] += tvrho0;
        let t273 = t22 * t31;
        let t274 = t41 * t65;
        let t275 = t273 * t274;
        let t277 = t273 * t41;
        let t279 = t162 * t277;
        let t281 = t62 * t34;
        let t282 = t169 * t281;
        let t283 = t81 * t273;
        let t284 = t282 * t283;
        let t286 = t60 * t22;
        let t287 = t31 * t35;
        let t288 = t40 * t62;
        let t292 = t277 / 24.0 + 5.0 / 24.0 * t279 + 0.25180844907407407 * t284 - t286 * t287 * t288 / 8.0;
        let t293 = t158 * t292;
        let t296 = -t275 / 24.0 - t36 * t293 / 3.0;
        let t299 = t189 * t296;
        let t301 = -2.0 * t296 * t73 - 9.0 * t299;
        let t302 = t301 * t78;
        let t304 = t198 * t296;
        let t308 = sigma[ip] * t21;
        let t309 = t308 * t87;
        let t310 = t81 * t309;
        let t312 = 0.1504548888888889 * t277 + 0.01075979618490518 * t310;
        let t315 = t41 * t105;
        let t320 = -t203 * t312 / 5.0 + 0.04460577520576132 * t273 * t315 - 14.0 / 45.0 * t226 * t312;
        let t324 = -6.0 * t197 * t299 - t302;
        let t327 = t40 * t22;
        let t331 = t119 * t22;
        let t332 = t287 * t40;
        let t333 = t331 * t332;
        let t335 = t273 * t35;
        let t336 = t40 * t128;
        let t337 = t336 * t130;
        let t338 = t335 * t337;
        let t342 = piecewise3(t127, t122 * t124 / 8.0, 0.0);
        let t343 = t119 * t342;
        let t348 = 125.0 / 52488.0 * t310 + 5.0 / 12.0 * t113 * t327 * t31 - 73.0 / 14580.0 * t333 + 73.0 / 19440.0 * t338 - 146.0 / 135.0 * t343 * t130 + 146.0 / 135.0 * t129 * t342;
        let t351 = t302 * t108 + 6.0 * t197 * t304 + t79 * t320 + t324 * t134 + t240 * t348 / 10.0;
        let t355 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t351);
        let tvsigma0 = 2.0 * rho[ip] * t355;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t357 = t22 * t26;
        let t359 = t357 * t41;
        let t362 = t81 * t357;
        let t365 = -5.0 / 3.0 * t162 * t359 - 2.0144675925925926 * t282 * t362;
        let t366 = t158 * t365;
        let t369 = t357 * t274 / 3.0 - t36 * t366 / 3.0;
        let t372 = t189 * t369;
        let t374 = -2.0 * t369 * t73 - 9.0 * t372;
        let t375 = t374 * t78;
        let t377 = t198 * t369;
        let t380 = t79 * t22;
        let t381 = t26 * t35;
        let t382 = t40 * t105;
        let t383 = t381 * t382;
        let t388 = -6.0 * t197 * t372 - t375;
        let t390 = t381 * t40;
        let t393 = t357 * t35;
        let t396 = tau[ip] * tau[ip];
        let t397 = 1.0 / t396;
        let t400 = piecewise3(t127, -t123 * t397 / 8.0, 0.0);
        let t401 = t119 * t400;
        let t406 = 146.0 / 405.0 * t331 * t390 - 73.0 / 270.0 * t393 * t337 - 146.0 / 135.0 * t401 * t130 + 146.0 / 135.0 * t129 * t400;
        let t409 = t375 * t108 + 6.0 * t197 * t377 - 0.06288822469135802 * t380 * t383 + t388 * t134 + t240 * t406 / 10.0;
        let t413 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t409);
        let tvtau0 = 2.0 * rho[ip] * t413;
        vtau[ip] += tvtau0;
    }
}
