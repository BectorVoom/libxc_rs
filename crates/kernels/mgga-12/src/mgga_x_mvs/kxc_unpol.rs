//! MGGA_X_MVS kxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 51 shared lines across all orders.
//! Delta: 227 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_mvs_kxc_unpol(
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
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rho2lapl: &mut Array<f64>,
    v3rho2tau: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3rhosigmalapl: &mut Array<f64>,
    v3rhosigmatau: &mut Array<f64>,
    v3rholapl2: &mut Array<f64>,
    v3rholapltau: &mut Array<f64>,
    v3rhotau2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v3sigma2lapl: &mut Array<f64>,
    v3sigma2tau: &mut Array<f64>,
    v3sigmalapl2: &mut Array<f64>,
    v3sigmalapltau: &mut Array<f64>,
    v3sigmatau2: &mut Array<f64>,
    v3lapl3: &mut Array<f64>,
    v3lapl2tau: &mut Array<f64>,
    v3lapltau2: &mut Array<f64>,
    v3tau3: &mut Array<f64>,
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
        // --- fxc delta (121 lines) ---
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
        // --- kxc delta (this level) (227 lines) ---
        let t523 = 1.0 / t269;
        let t524 = t18 * t523;
        let t526 = t7 * t524 * t67;
        let t534 = t7 * t210 * t121;
        let t540 = t18 / t20 / t269 / t74;
        let t542 = t7 * t540 * t67;
        let t548 = 1.0 / t24 / t74;
        let t551 = -440.0 / 27.0 * t23 * t218 + 154.0 / 27.0 * t28 * t548;
        let t552 = param_k0 * t551;
        let t555 = t222 * t35;
        let t558 = t40 * t232;
        let t559 = t558 * t233;
        let t562 = t226 * t256;
        let t567 = 1.0 / t64 / t230 / t63;
        let t568 = t233 * t117;
        let t569 = t567 * t568;
        let t572 = t232 * t117;
        let t576 = t237 * t34;
        let t577 = t239 * t99;
        let t578 = t60 * t577;
        let t581 = t310 * t221;
        let t586 = t50 * t551;
        let t589 = param_c1 * t34;
        let t597 = 10000.0 / 729.0 * t576 * t578 + 10000.0 / 729.0 * t309 * t581 + 100.0 / 27.0 * t314 * t247 + 100.0 / 81.0 * t108 * t586 + 10000.0 / 729.0 * t589 * t578 + 5000.0 / 243.0 * t321 * t581 + 5000.0 / 2187.0 * t113 * t60 * t551;
        let t598 = t106 * t597;
        let t601 = -5.0 / 9.0 * t552 * t102 + 5.0 / 12.0 * t555 * t227 - 25.0 / 48.0 * t225 * t559 + 5.0 / 12.0 * t225 * t562 - 45.0 / 64.0 * t44 * t569 + 15.0 / 16.0 * t44 * t572 * t256 - t44 * t598 / 4.0;
        let t607 = t7 * t128 * t260;
        let t611 = t7 * t273 * t121;
        let t614 = t57 * t57;
        let t617 = t4 / t5 / t614;
        let t618 = t269 * t269;
        let t623 = t617 * t18 / t24 / t618 / t29;
        let t626 = 1.0 / t81 / t276 / t80;
        let t627 = t67 * t626;
        let t628 = t279 * param_b;
        let t629 = t282 * t71;
        let t630 = t628 * t629;
        let t631 = t627 * t630;
        let t635 = piecewise3(t3, 0.0, -5.0 / 36.0 * t19 * t31 * t67 * t82 + t19 * t26 * t121 * t82 / 4.0 - 355.0 / 10368.0 * t526 * t137 - 3.0 / 8.0 * t19 * t87 * t260 * t82 + 17.0 / 1152.0 * t534 * t137 + t542 * t285 / 512.0 - 3.0 / 8.0 * t19 * t20 * t601 * t82 - t607 * t137 / 384.0 - t611 * t285 / 3072.0 - 17.0 / 331776.0 * t623 * t631);
        let tv3rho30 = 2.0 * rho[ip] * t635 + 6.0 * t289;
        v3rho3[ip] += tv3rho30;
        let t648 = t7 * t210 * t161;
        let t651 = t144 * t218;
        let t652 = t651 * t102;
        let t654 = t296 * t299;
        let t656 = t101 * t234;
        let t657 = t145 * t656;
        let t659 = t101 * t257;
        let t660 = t145 * t659;
        let t664 = t157 * t117;
        let t665 = t558 * t664;
        let t668 = t226 * t327;
        let t675 = t232 * t327;
        let t682 = t576 * t35;
        let t683 = t59 * t239;
        let t684 = t683 * t148;
        let t685 = t682 * t684;
        let t687 = t59 * t221;
        let t688 = t687 * t148;
        let t689 = t309 * t688;
        let t691 = t310 * t317;
        let t692 = t309 * t691;
        let t694 = t107 * t221;
        let t695 = t694 * t149;
        let t697 = t314 * t318;
        let t700 = t50 * t22 * t218;
        let t701 = t108 * t700;
        let t703 = t589 * t35;
        let t704 = t703 * t684;
        let t706 = t321 * t691;
        let t708 = t321 * t688;
        let t711 = t152 * t153 * t218;
        let t713 = -1250.0 / 729.0 * t685 - 1250.0 / 2187.0 * t689 + 20000.0 / 6561.0 * t692 - 25.0 / 162.0 * t695 + 200.0 / 243.0 * t697 - 1100.0 / 729.0 * t701 - 1250.0 / 729.0 * t704 + 10000.0 / 2187.0 * t706 - 625.0 / 729.0 * t708 - 55000.0 / 19683.0 * t711;
        let t714 = t106 * t713;
        let t717 = 55.0 / 81.0 * t652 + 5.0 / 54.0 * t654 + 25.0 / 1152.0 * t657 - 5.0 / 288.0 * t660 + 5.0 / 36.0 * t555 * t302 - 25.0 / 72.0 * t225 * t665 + 5.0 / 18.0 * t225 * t668 - 45.0 / 64.0 * t44 * t567 * t157 * t233 + 5.0 / 8.0 * t44 * t675 * t117 + 5.0 / 16.0 * t44 * t305 * t256 - t44 * t714 / 4.0;
        let t723 = t7 * t128 * t331;
        let t727 = t7 * t273 * t161;
        let t737 = t7 * t167 * t260;
        let t741 = t7 * t349 * t121;
        let t748 = t617 * t18 / t24 / t618 / rho[ip];
        let t749 = t282 * sigma[ip];
        let t750 = t628 * t749;
        let t751 = t627 * t750;
        let t754 = t19 * t26 * t161 * t82 / 12.0 - t19 * t87 * t331 * t82 / 4.0 + 17.0 / 3456.0 * t648 * t137 - 3.0 / 8.0 * t19 * t20 * t717 * t82 - t723 * t137 / 576.0 - t727 * t285 / 9216.0 + 5.0 / 512.0 * t212 * t172 - 5.0 / 1536.0 * t266 * t172 - 49.0 / 73728.0 * t275 * t355 + t737 * t172 / 3072.0 + t741 * t355 / 12288.0 + 17.0 / 884736.0 * t748 * t751;
        let t755 = piecewise3(t3, 0.0, t754);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t755 + 4.0 * t359;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rho2lapl0 = 0.0;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t768 = t7 * t210 * t192;
        let t779 = t188 * t117;
        let t780 = t558 * t779;
        let t783 = t226 * t385;
        let t786 = t567 * t188;
        let t790 = t232 * t385;
        let t797 = t683 * t181;
        let t800 = t687 * t181;
        let t814 = 10000.0 / 729.0 * t682 * t797 + 10000.0 / 2187.0 * t309 * t800 - 100000.0 / 6561.0 * t312 + 100.0 / 81.0 * t694 * t182 - 1000.0 / 243.0 * t315 + 4000.0 / 729.0 * t319 + 10000.0 / 729.0 * t703 * t797 - 50000.0 / 2187.0 * t322 + 5000.0 / 729.0 * t321 * t800 + 200000.0 / 19683.0 * t325;
        let t815 = t106 * t814;
        let t818 = -200.0 / 81.0 * t297 - 25.0 / 54.0 * t300 - 25.0 / 144.0 * t178 * t656 + 5.0 / 36.0 * t178 * t659 + 5.0 / 36.0 * t555 * t369 - 25.0 / 72.0 * t225 * t780 + 5.0 / 18.0 * t225 * t783 - 45.0 / 64.0 * t44 * t786 * t233 + 5.0 / 8.0 * t44 * t790 * t117 + 5.0 / 16.0 * t44 * t372 * t256 - t44 * t815 / 4.0;
        let t824 = t7 * t128 * t389;
        let t828 = t7 * t273 * t192;
        let t832 = piecewise3(t3, 0.0, t19 * t26 * t192 * t82 / 12.0 - t19 * t87 * t389 * t82 / 4.0 + 17.0 / 3456.0 * t768 * t137 - 3.0 / 8.0 * t19 * t20 * t818 * t82 - t824 * t137 / 576.0 - t828 * t285 / 9216.0);
        let tv3rho2tau0 = 2.0 * rho[ip] * t832 + 4.0 * t399;
        v3rho2tau[ip] += tv3rho2tau0;
        let t839 = t296 * t402;
        let t842 = t144 * t31 * t35;
        let t843 = t842 * t665;
        let t845 = t101 * t328;
        let t846 = t145 * t845;
        let t848 = t558 * t405;
        let t851 = t567 * t405;
        let t858 = t226 * t420;
        let t861 = t232 * t420;
        let t866 = t409 * t76 * t99;
        let t867 = t682 * t866;
        let t870 = 1.0 / t20 / t126;
        let t871 = t409 * t870;
        let t872 = t309 * t871;
        let t876 = t413 * t870 * t47 * t49;
        let t878 = t703 * t866;
        let t880 = t321 * t871;
        let t882 = 625.0 / 1458.0 * t867 - 5000.0 / 6561.0 * t872 - 50.0 / 243.0 * t876 + 625.0 / 1458.0 * t878 - 2500.0 / 2187.0 * t880;
        let t883 = t106 * t882;
        let t886 = 5.0 / 54.0 * t839 + 25.0 / 576.0 * t843 - 5.0 / 144.0 * t846 - 25.0 / 144.0 * t225 * t848 - 45.0 / 64.0 * t44 * t851 * t117 + 5.0 / 8.0 * t44 * t305 * t327 + 5.0 / 36.0 * t225 * t858 + 5.0 / 16.0 * t44 * t861 * t117 - t44 * t883 / 4.0;
        let t892 = t7 * t128 * t424;
        let t898 = t7 * t167 * t331;
        let t902 = t7 * t349 * t161;
        let t908 = t7 * t436 * t121;
        let t914 = t617 * t18 / t24 / t618;
        let t915 = t628 * t282;
        let t916 = t627 * t915;
        let t923 = -t19 * t87 * t424 * t82 / 8.0 - 3.0 / 8.0 * t19 * t20 * t886 * t82 - t892 * t137 / 1152.0 - 5.0 / 1536.0 * t337 * t172 + t898 * t172 / 1536.0 + t902 * t355 / 12288.0 + 13.0 / 65536.0 * t351 * t441 - t908 * t441 / 65536.0 - 17.0 / 2359296.0 * t914 * t916 - 5.0 / 3072.0 * t130 * t445 + t343 * t445 / 3072.0;
        let t924 = piecewise3(t3, 0.0, t923);
        let tv3rhosigma20 = 2.0 * rho[ip] * t924 + 2.0 * t449;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let t933 = t144 * t26 * t35;
        let t938 = t296 * t453;
        let t940 = t842 * t780;
        let t942 = t101 * t386;
        let t943 = t145 * t942;
        let t945 = t188 * t157;
        let t946 = t558 * t945;
        let t949 = t44 * t567;
        let t950 = t945 * t117;
        let t959 = t226 * t470;
        let t962 = t232 * t470;
        let t967 = t409 * t460 * t99;
        let t968 = t682 * t967;
        let t972 = t703 * t967;
        let t975 = -2500.0 / 729.0 * t968 + 32500.0 / 6561.0 * t411 + 325.0 / 243.0 * t416 - 2500.0 / 729.0 * t972 + 16250.0 / 2187.0 * t418;
        let t976 = t106 * t975;
        let t979 = -25.0 / 108.0 * t403 - 25.0 / 144.0 * t933 * t665 + 5.0 / 36.0 * t178 * t845 + 5.0 / 108.0 * t938 + 25.0 / 1152.0 * t940 - 5.0 / 288.0 * t943 - 25.0 / 144.0 * t225 * t946 - 45.0 / 64.0 * t949 * t950 + 5.0 / 16.0 * t44 * t790 * t157 + 5.0 / 16.0 * t44 * t372 * t327 + 5.0 / 36.0 * t225 * t959 + 5.0 / 16.0 * t44 * t962 * t117 - t44 * t976 / 4.0;
        let t985 = t7 * t128 * t474;
        let t991 = t7 * t167 * t389;
        let t995 = t7 * t349 * t192;
        let t999 = piecewise3(t3, 0.0, -t19 * t87 * t474 * t82 / 8.0 - 3.0 / 8.0 * t19 * t20 * t979 * t82 - t985 * t137 / 1152.0 - 5.0 / 3072.0 * t395 * t172 + t991 * t172 / 3072.0 + t995 * t355 / 24576.0);
        let tv3rhosigmatau0 = 2.0 * rho[ip] * t999 + 2.0 * t484;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let tv3rholapl20 = 0.0;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let t1011 = t558 * t488;
        let t1014 = t567 * t488;
        let t1021 = t226 * t503;
        let t1024 = t232 * t503;
        let t1029 = t409 * t493 * t99;
        let t1037 = 20000.0 / 729.0 * t682 * t1029 - 200000.0 / 6561.0 * t462 - 2000.0 / 243.0 * t466 + 20000.0 / 729.0 * t703 * t1029 - 100000.0 / 2187.0 * t468;
        let t1038 = t106 * t1037;
        let t1041 = -25.0 / 54.0 * t454 - 25.0 / 72.0 * t933 * t780 + 5.0 / 18.0 * t178 * t942 - 25.0 / 144.0 * t225 * t1011 - 45.0 / 64.0 * t44 * t1014 * t117 + 5.0 / 8.0 * t44 * t372 * t385 + 5.0 / 36.0 * t225 * t1021 + 5.0 / 16.0 * t44 * t1024 * t117 - t44 * t1038 / 4.0;
        let t1047 = t7 * t128 * t507;
        let t1051 = piecewise3(t3, 0.0, -t19 * t87 * t507 * t82 / 8.0 - 3.0 / 8.0 * t19 * t20 * t1041 * t82 - t1047 * t137 / 1152.0);
        let tv3rhotau20 = 2.0 * rho[ip] * t1051 + 2.0 * t512;
        v3rhotau2[ip] += tv3rhotau20;
        let t1054 = t101 * t406;
        let t1055 = t145 * t1054;
        let t1057 = t101 * t421;
        let t1058 = t145 * t1057;
        let t1060 = t405 * t157;
        let t1061 = t567 * t1060;
        let t1067 = t60 * t523;
        let t1068 = t576 * t1067;
        let t1069 = t589 * t1067;
        let t1071 = -625.0 / 5832.0 * t1068 - 625.0 / 5832.0 * t1069;
        let t1072 = t106 * t1071;
        let t1075 = 25.0 / 384.0 * t1055 - 5.0 / 96.0 * t1058 - 45.0 / 64.0 * t44 * t1061 + 15.0 / 16.0 * t44 * t305 * t420 - t44 * t1072 / 4.0;
        let t1081 = t7 * t167 * t424;
        let t1085 = t7 * t436 * t161;
        let t1094 = t617 * t18 / t24 / t269 / t208;
        let t1095 = t628 * t352;
        let t1096 = t627 * t1095;
        let t1101 = t281 * t59 * sigma[ip] * t22;
        let t1105 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t1075 * t82 + t1081 * t172 / 1024.0 - 3.0 / 65536.0 * t1085 * t441 + t430 * t445 / 1024.0 + 17.0 / 6291456.0 * t1094 * t1096 - 3.0 / 65536.0 * t438 * t1101);
        let tv3sigma30 = 2.0 * rho[ip] * t1105;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let t1111 = t842 * t946;
        let t1113 = t101 * t471;
        let t1114 = t145 * t1113;
        let t1125 = t60 * t209;
        let t1126 = t576 * t1125;
        let t1127 = t589 * t1125;
        let t1129 = 625.0 / 729.0 * t1126 + 625.0 / 729.0 * t1127;
        let t1130 = t106 * t1129;
        let t1133 = -25.0 / 144.0 * t178 * t1054 + 5.0 / 36.0 * t178 * t1057 + 25.0 / 576.0 * t1111 - 5.0 / 144.0 * t1114 - 45.0 / 64.0 * t44 * t786 * t405 + 5.0 / 8.0 * t44 * t962 * t157 + 5.0 / 16.0 * t44 * t372 * t420 - t44 * t1130 / 4.0;
        let t1139 = t7 * t167 * t474;
        let t1143 = t7 * t436 * t192;
        let t1149 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t1133 * t82 + t1139 * t172 / 1536.0 - t1143 * t441 / 65536.0 + t480 * t445 / 3072.0);
        let tv3sigma2tau0 = 2.0 * rho[ip] * t1149;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let tv3sigmalapl20 = 0.0;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let t1155 = t101 * t489;
        let t1156 = t145 * t1155;
        let t1164 = t101 * t504;
        let t1165 = t145 * t1164;
        let t1170 = t60 * t127;
        let t1171 = t576 * t1170;
        let t1172 = t589 * t1170;
        let t1174 = -5000.0 / 729.0 * t1171 - 5000.0 / 729.0 * t1172;
        let t1175 = t106 * t1174;
        let t1178 = -25.0 / 72.0 * t933 * t946 + 5.0 / 18.0 * t178 * t1113 + 25.0 / 1152.0 * t1156 - 45.0 / 64.0 * t44 * t1014 * t157 + 5.0 / 8.0 * t44 * t372 * t470 - 5.0 / 288.0 * t1165 + 5.0 / 16.0 * t44 * t1024 * t157 - t44 * t1175 / 4.0;
        let t1184 = t7 * t167 * t507;
        let t1188 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t1178 * t82 + t1184 * t172 / 3072.0);
        let tv3sigmatau20 = 2.0 * rho[ip] * t1188;
        v3sigmatau2[ip] += tv3sigmatau20;
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let t1194 = t488 * t188;
        let t1195 = t567 * t1194;
        let t1201 = t60 * t166;
        let t1205 = 40000.0 / 729.0 * t576 * t1201 + 40000.0 / 729.0 * t589 * t1201;
        let t1206 = t106 * t1205;
        let t1209 = -25.0 / 48.0 * t178 * t1155 + 5.0 / 12.0 * t178 * t1164 - 45.0 / 64.0 * t44 * t1195 + 15.0 / 16.0 * t44 * t372 * t503 - t44 * t1206 / 4.0;
        let t1214 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t1209 * t82);
        let tv3tau30 = 2.0 * rho[ip] * t1214;
        v3tau3[ip] += tv3tau30;
    }
}
