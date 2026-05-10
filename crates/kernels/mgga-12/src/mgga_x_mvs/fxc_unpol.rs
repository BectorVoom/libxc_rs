//! MGGA_X_MVS fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 51 shared lines across all orders.
//! Delta: 121 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_mvs_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (51 lines) ---
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
        let t19 = t7 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = tau[ip] * t22;
        let t24 = t20 * t20;
        let t26 = 1.0 / t24 / rho[ip];
        let t28 = sigma[ip] * t22;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t24 / t29;
        let t34 = t23 * t26 - t28 * t31 / 8.0;
        let t35 = M_CBRT6;
        let t37 = M_PI * M_PI;
        let t38 = pow_1_3(t37);
        let t39 = t38 * t38;
        let t40 = 1.0 / t39;
        let t44 = param_k0 * (1.0 - 5.0 / 9.0 * t34 * t35 * t40);
        let t45 = t34 * t34;
        let t47 = t35 * t35;
        let t49 = 1.0 / t38 / t37;
        let t50 = t47 * t49;
        let t53 = 1.0 + 25.0 / 81.0 * param_e1 * t45 * t50;
        let t54 = t53 * t53;
        let t55 = t45 * t45;
        let t57 = t37 * t37;
        let t59 = 1.0 / t39 / t57;
        let t60 = t35 * t59;
        let t63 = t54 + 1250.0 / 2187.0 * param_c1 * t55 * t60;
        let t64 = pow_1_4(t63);
        let t65 = 1.0 / t64;
        let t67 = t44 * t65 + 1.0;
        let t71 = sigma[ip] * sigma[ip];
        let t73 = t29 * t29;
        let t74 = t73 * rho[ip];
        let t76 = 1.0 / t20 / t74;
        let t80 = 1.0 + param_b * t47 * t49 * t71 * t21 * t76 / 288.0;
        let t81 = f64::powf(t80, 1.0 / 8.0);
        let t82 = 1.0 / t81;
        let t86 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t67 * t82);
        let tzk0 = 2.0 * t86;
        zk[ip] += tzk0;
        // --- vxc delta (52 lines) ---
        let t87 = 1.0 / t24;
        let t94 = t29 * rho[ip];
        let t96 = 1.0 / t24 / t94;
        let t99 = -5.0 / 3.0 * t23 * t31 + t28 * t96 / 3.0;
        let t100 = param_k0 * t99;
        let t101 = t35 * t40;
        let t102 = t101 * t65;
        let t106 = 1.0 / t64 / t63;
        let t107 = t53 * param_e1;
        let t108 = t107 * t34;
        let t113 = param_c1 * t45 * t34;
        let t117 = 100.0 / 81.0 * t108 * t50 * t99 + 5000.0 / 2187.0 * t113 * t60 * t99;
        let t118 = t106 * t117;
        let t121 = -5.0 / 9.0 * t100 * t102 - t44 * t118 / 4.0;
        let t126 = t73 * t29;
        let t127 = 1.0 / t126;
        let t128 = t18 * t127;
        let t130 = t7 * t128 * t67;
        let t133 = 1.0 / t81 / t80 * param_b;
        let t134 = t133 * t47;
        let t137 = t134 * t49 * t71 * t21;
        let t141 = piecewise3(t3, 0.0, -t19 * t87 * t67 * t82 / 8.0 - 3.0 / 8.0 * t19 * t20 * t121 * t82 - t130 * t137 / 1152.0);
        let tvrho0 = 2.0 * rho[ip] * t141 + 2.0 * t86;
        vrho[ip] += tvrho0;
        let t144 = param_k0 * t22;
        let t145 = t144 * t31;
        let t146 = t145 * t102;
        let t148 = t22 * t31;
        let t149 = t50 * t148;
        let t150 = t108 * t149;
        let t152 = t113 * t35;
        let t153 = t59 * t22;
        let t154 = t153 * t31;
        let t155 = t152 * t154;
        let t157 = -25.0 / 162.0 * t150 - 625.0 / 2187.0 * t155;
        let t158 = t106 * t157;
        let t161 = 5.0 / 72.0 * t146 - t44 * t158 / 4.0;
        let t166 = 1.0 / t74;
        let t167 = t18 * t166;
        let t169 = t7 * t167 * t67;
        let t172 = t134 * t49 * sigma[ip] * t21;
        let t176 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t161 * t82 + t169 * t172 / 3072.0);
        let tvsigma0 = 2.0 * rho[ip] * t176;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t178 = t144 * t26;
        let t181 = t22 * t26;
        let t182 = t50 * t181;
        let t185 = t153 * t26;
        let t188 = 100.0 / 81.0 * t108 * t182 + 5000.0 / 2187.0 * t152 * t185;
        let t189 = t106 * t188;
        let t192 = -5.0 / 9.0 * t178 * t102 - t44 * t189 / 4.0;
        let t197 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t192 * t82);
        let tvtau0 = 2.0 * rho[ip] * t197;
        vtau[ip] += tvtau0;
        // --- fxc delta (this level) (121 lines) ---
        let t208 = t73 * t94;
        let t209 = 1.0 / t208;
        let t210 = t18 * t209;
        let t212 = t7 * t210 * t67;
        let t218 = 1.0 / t24 / t73;
        let t221 = 40.0 / 9.0 * t23 * t96 - 11.0 / 9.0 * t28 * t218;
        let t222 = param_k0 * t221;
        let t225 = t100 * t35;
        let t226 = t40 * t106;
        let t227 = t226 * t117;
        let t230 = t63 * t63;
        let t232 = 1.0 / t64 / t230;
        let t233 = t117 * t117;
        let t234 = t232 * t233;
        let t237 = param_e1 * param_e1;
        let t238 = t237 * t45;
        let t239 = t99 * t99;
        let t240 = t60 * t239;
        let t247 = t50 * t221;
        let t250 = param_c1 * t45;
        let t256 = 10000.0 / 2187.0 * t238 * t240 + 100.0 / 81.0 * t107 * t239 * t47 * t49 + 100.0 / 81.0 * t108 * t247 + 5000.0 / 729.0 * t250 * t240 + 5000.0 / 2187.0 * t113 * t60 * t221;
        let t257 = t106 * t256;
        let t260 = -5.0 / 9.0 * t222 * t102 + 5.0 / 18.0 * t225 * t227 + 5.0 / 16.0 * t44 * t234 - t44 * t257 / 4.0;
        let t266 = t7 * t128 * t121;
        let t269 = t73 * t73;
        let t273 = t18 / t20 / t269 / t73;
        let t275 = t7 * t273 * t67;
        let t276 = t80 * t80;
        let t279 = param_b * param_b;
        let t280 = 1.0 / t81 / t276 * t279;
        let t281 = t280 * t35;
        let t282 = t71 * t71;
        let t285 = t281 * t59 * t282 * t22;
        let t289 = piecewise3(t3, 0.0, t19 * t26 * t67 * t82 / 12.0 - t19 * t87 * t121 * t82 / 4.0 + 17.0 / 3456.0 * t212 * t137 - 3.0 / 8.0 * t19 * t20 * t260 * t82 - t266 * t137 / 576.0 - t275 * t285 / 9216.0);
        let tv2rho20 = 2.0 * rho[ip] * t289 + 4.0 * t141;
        v2rho2[ip] += tv2rho20;
        let t296 = t144 * t96;
        let t297 = t296 * t102;
        let t299 = t101 * t118;
        let t300 = t145 * t299;
        let t302 = t226 * t157;
        let t305 = t232 * t157;
        let t309 = t238 * t35;
        let t310 = t59 * t99;
        let t311 = t310 * t148;
        let t312 = t309 * t311;
        let t314 = t107 * t99;
        let t315 = t314 * t149;
        let t317 = t22 * t96;
        let t318 = t50 * t317;
        let t319 = t108 * t318;
        let t321 = t250 * t35;
        let t322 = t321 * t311;
        let t325 = t152 * t153 * t96;
        let t327 = -1250.0 / 2187.0 * t312 - 25.0 / 162.0 * t315 + 100.0 / 243.0 * t319 - 625.0 / 729.0 * t322 + 5000.0 / 6561.0 * t325;
        let t328 = t106 * t327;
        let t331 = -5.0 / 27.0 * t297 - 5.0 / 288.0 * t300 + 5.0 / 36.0 * t225 * t302 + 5.0 / 16.0 * t44 * t305 * t117 - t44 * t328 / 4.0;
        let t337 = t7 * t128 * t161;
        let t343 = t7 * t167 * t121;
        let t349 = t18 / t20 / t269 / t94;
        let t351 = t7 * t349 * t67;
        let t352 = t71 * sigma[ip];
        let t355 = t281 * t59 * t352 * t22;
        let t359 = piecewise3(t3, 0.0, -t19 * t87 * t161 * t82 / 8.0 - 3.0 / 8.0 * t19 * t20 * t331 * t82 - t337 * t137 / 1152.0 - 5.0 / 3072.0 * t130 * t172 + t343 * t172 / 3072.0 + t351 * t355 / 24576.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t359 + 2.0 * t176;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t369 = t226 * t188;
        let t372 = t232 * t188;
        let t376 = t310 * t181;
        let t385 = 10000.0 / 2187.0 * t309 * t376 + 100.0 / 81.0 * t314 * t182 - 500.0 / 243.0 * t150 + 5000.0 / 729.0 * t321 * t376 - 25000.0 / 6561.0 * t155;
        let t386 = t106 * t385;
        let t389 = 25.0 / 27.0 * t146 + 5.0 / 36.0 * t178 * t299 + 5.0 / 36.0 * t225 * t369 + 5.0 / 16.0 * t44 * t372 * t117 - t44 * t386 / 4.0;
        let t395 = t7 * t128 * t192;
        let t399 = piecewise3(t3, 0.0, -t19 * t87 * t192 * t82 / 8.0 - 3.0 / 8.0 * t19 * t20 * t389 * t82 - t395 * t137 / 1152.0);
        let tv2rhotau0 = 2.0 * rho[ip] * t399 + 2.0 * t197;
        v2rhotau[ip] += tv2rhotau0;
        let t402 = t101 * t158;
        let t403 = t145 * t402;
        let t405 = t157 * t157;
        let t406 = t232 * t405;
        let t409 = t59 * t21;
        let t410 = t409 * t76;
        let t411 = t309 * t410;
        let t413 = t107 * t21;
        let t416 = t413 * t76 * t47 * t49;
        let t418 = t321 * t410;
        let t420 = 625.0 / 4374.0 * t411 + 25.0 / 648.0 * t416 + 625.0 / 2916.0 * t418;
        let t421 = t106 * t420;
        let t424 = -5.0 / 144.0 * t403 + 5.0 / 16.0 * t44 * t406 - t44 * t421 / 4.0;
        let t430 = t7 * t167 * t161;
        let t433 = t269 * t29;
        let t436 = t18 / t20 / t433;
        let t438 = t7 * t436 * t67;
        let t441 = t281 * t59 * t71 * t22;
        let t444 = t50 * t21;
        let t445 = t133 * t444;
        let t449 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t424 * t82 + t430 * t172 / 1536.0 - t438 * t441 / 65536.0 + t169 * t445 / 3072.0);
        let tv2sigma20 = 2.0 * rho[ip] * t449;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t453 = t101 * t189;
        let t454 = t145 * t453;
        let t460 = 1.0 / t20 / t73;
        let t461 = t409 * t460;
        let t462 = t309 * t461;
        let t466 = t413 * t460 * t47 * t49;
        let t468 = t321 * t461;
        let t470 = -2500.0 / 2187.0 * t462 - 25.0 / 81.0 * t466 - 1250.0 / 729.0 * t468;
        let t471 = t106 * t470;
        let t474 = 5.0 / 36.0 * t178 * t402 - 5.0 / 288.0 * t454 + 5.0 / 16.0 * t44 * t372 * t157 - t44 * t471 / 4.0;
        let t480 = t7 * t167 * t192;
        let t484 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t474 * t82 + t480 * t172 / 3072.0);
        let tv2sigmatau0 = 2.0 * rho[ip] * t484;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t488 = t188 * t188;
        let t489 = t232 * t488;
        let t493 = 1.0 / t20 / t94;
        let t494 = t409 * t493;
        let t503 = 20000.0 / 2187.0 * t309 * t494 + 200.0 / 81.0 * t413 * t493 * t47 * t49 + 10000.0 / 729.0 * t321 * t494;
        let t504 = t106 * t503;
        let t507 = 5.0 / 18.0 * t178 * t453 + 5.0 / 16.0 * t44 * t489 - t44 * t504 / 4.0;
        let t512 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t507 * t82);
        let tv2tau20 = 2.0 * rho[ip] * t512;
        v2tau2[ip] += tv2tau20;
    }
}
