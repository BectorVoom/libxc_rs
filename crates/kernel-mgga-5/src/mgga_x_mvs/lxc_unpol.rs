//! MGGA_X_MVS lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mvs.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_mvs_lxc_unpol(
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
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho3lapl: &mut Array<f64>,
    v4rho3tau: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rho2sigmalapl: &mut Array<f64>,
    v4rho2sigmatau: &mut Array<f64>,
    v4rho2lapl2: &mut Array<f64>,
    v4rho2lapltau: &mut Array<f64>,
    v4rho2tau2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4rhosigma2lapl: &mut Array<f64>,
    v4rhosigma2tau: &mut Array<f64>,
    v4rhosigmalapl2: &mut Array<f64>,
    v4rhosigmalapltau: &mut Array<f64>,
    v4rhosigmatau2: &mut Array<f64>,
    v4rholapl3: &mut Array<f64>,
    v4rholapl2tau: &mut Array<f64>,
    v4rholapltau2: &mut Array<f64>,
    v4rhotau3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    v4sigma3lapl: &mut Array<f64>,
    v4sigma3tau: &mut Array<f64>,
    v4sigma2lapl2: &mut Array<f64>,
    v4sigma2lapltau: &mut Array<f64>,
    v4sigma2tau2: &mut Array<f64>,
    v4sigmalapl3: &mut Array<f64>,
    v4sigmalapl2tau: &mut Array<f64>,
    v4sigmalapltau2: &mut Array<f64>,
    v4sigmatau3: &mut Array<f64>,
    v4lapl4: &mut Array<f64>,
    v4lapl3tau: &mut Array<f64>,
    v4lapl2tau2: &mut Array<f64>,
    v4lapltau3: &mut Array<f64>,
    v4tau4: &mut Array<f64>,
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
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
        let t1232 = 1.0 / t24 / t126;
        let t1235 = 6160.0 / 81.0 * t23 * t548 - 2618.0 / 81.0 * t28 * t1232;
        let t1239 = t552 * t35;
        let t1246 = t40 * t567;
        let t1251 = t558 * t117 * t256;
        let t1257 = t230 * t230;
        let t1259 = 1.0 / t64 / t1257;
        let t1260 = t233 * t233;
        let t1268 = t256 * t256;
        let t1275 = t239 * t239;
        let t1279 = t683 * t221;
        let t1282 = t221 * t221;
        let t1283 = t60 * t1282;
        let t1286 = t310 * t551;
        let t1310 = 10000.0 / 729.0 * t237 * t1275 * t60 + 20000.0 / 243.0 * t682 * t1279 + 10000.0 / 729.0 * t238 * t1283 + 40000.0 / 2187.0 * t309 * t1286 + 100.0 / 27.0 * t107 * t1282 * t47 * t49 + 400.0 / 81.0 * t314 * t586 + 100.0 / 81.0 * t108 * t50 * t1235 + 10000.0 / 729.0 * param_c1 * t1275 * t60 + 20000.0 / 243.0 * t703 * t1279 + 5000.0 / 243.0 * t250 * t1283 + 20000.0 / 729.0 * t321 * t1286 + 5000.0 / 2187.0 * t113 * t60 * t1235;
        let t1314 = -5.0 / 9.0 * param_k0 * t1235 * t102 + 5.0 / 9.0 * t1239 * t227 - 25.0 / 24.0 * t555 * t559 + 5.0 / 6.0 * t555 * t562 + 25.0 / 16.0 * t225 * t1246 * t568 - 25.0 / 12.0 * t225 * t1251 + 5.0 / 9.0 * t225 * t226 * t597 + 585.0 / 256.0 * t44 * t1259 * t1260 - 135.0 / 32.0 * t44 * t567 * t233 * t256 + 15.0 / 16.0 * t44 * t232 * t1268 + 5.0 / 4.0 * t44 * t572 * t597 - t44 * t106 * t1310 / 4.0;
        let t1323 = t269 * rho[ip];
        let t1329 = t276 * t276;
        let t1332 = t279 * t279;
        let t1333 = 1.0 / t81 / t1329 * t1332;
        let t1334 = t282 * t282;
        let t1339 = 1.0 / t1323;
        let t1380 = t121 * t626;
        let t1384 = -5.0 / 9.0 * t19 * t31 * t121 * t82 + t19 * t26 * t260 * t82 / 2.0 - t19 * t87 * t601 * t82 / 2.0 - 3.0 / 8.0 * t19 * t20 * t1314 * t82 - t7 * t273 * t260 * t285 / 1536.0 - 425.0 / 0.143327232e9 * t617 * t18 / t618 / t1323 * t67 * t1333 * t1334 * t444 + 4255.0 / 15552.0 * t7 * t18 * t1339 * t67 * t137 - 355.0 / 2592.0 * t7 * t524 * t121 * t137 - 2515.0 / 82944.0 * t7 * t18 / t20 / t269 / t126 * t67 * t285 + 17.0 / 576.0 * t7 * t210 * t260 * t137 + t7 * t540 * t121 * t285 / 128.0 - t7 * t128 * t601 * t137 / 288.0 + 10.0 / 27.0 * t19 * t96 * t67 * t82 + 935.0 / 497664.0 * t617 * t18 / t24 / t618 / t94 * t631 - 17.0 / 82944.0 * t623 * t1380 * t630;
        let t1385 = piecewise3(t3, 0.0, t1384);
        let tv4rho40 = 2.0 * rho[ip] * t1385 + 8.0 * t635;
        v4rho4[ip] += tv4rho40;
        let t1425 = t107 * t551;
        let t1429 = param_c1 * t577 * t35;
        let t1433 = t153 * t96 * t221;
        let t1437 = t153 * t31 * t551;
        let t1444 = t237 * t577 * t35;
        let t1450 = t153 * t96 * t239;
        let t1453 = t589 * t60;
        let t1454 = t99 * t221;
        let t1455 = t148 * t1454;
        let t1459 = t153 * t218 * t99;
        let t1474 = t576 * t60;
        let t1479 = -25.0 / 162.0 * t1425 * t149 - 1250.0 / 729.0 * t1429 * t154 + 5000.0 / 729.0 * t321 * t1433 - 625.0 / 729.0 * t321 * t1437 + 770000.0 / 59049.0 * t152 * t153 * t548 - 1250.0 / 729.0 * t1444 * t154 - 1250.0 / 2187.0 * t309 * t1437 + 10000.0 / 729.0 * t703 * t1450 - 1250.0 / 243.0 * t1453 * t1455 - 55000.0 / 2187.0 * t321 * t1459 + 10000.0 / 2187.0 * t309 * t1433 - 110000.0 / 6561.0 * t309 * t1459 + 100.0 / 81.0 * t694 * t318 - 1100.0 / 243.0 * t314 * t700 + 15400.0 / 2187.0 * t108 * t50 * t22 * t548 - 1250.0 / 243.0 * t1474 * t1455 + 10000.0 / 729.0 * t682 * t1450;
        let t1491 = t101 * t598;
        let t1497 = t558 * t327 * t117;
        let t1501 = t558 * t157 * t256;
        let t1504 = t101 * t569;
        let t1510 = t1246 * t157 * t233;
        let t1515 = -770.0 / 243.0 * t144 * t548 * t102 - 55.0 / 108.0 * t651 * t299 + 5.0 / 36.0 * t296 * t659 - 5.0 / 288.0 * t145 * t1491 - 25.0 / 48.0 * t555 * t665 - 25.0 / 24.0 * t225 * t1497 - 25.0 / 48.0 * t225 * t1501 - 25.0 / 512.0 * t145 * t1504 + 25.0 / 384.0 * t842 * t1251 + 75.0 / 64.0 * t225 * t1510 - 25.0 / 144.0 * t296 * t656;
        let t1571 = t161 * t626;
        let t1582 = t19 * t26 * t331 * t82 / 4.0 - 3.0 / 8.0 * t19 * t87 * t717 * t82 - 3.0 / 8.0 * t19 * t20 * (5.0 / 36.0 * t1239 * t302 + 5.0 / 12.0 * t555 * t668 + 5.0 / 12.0 * t225 * t226 * t713 + 585.0 / 256.0 * t44 * t1259 * t157 * t568 - 135.0 / 64.0 * t949 * t664 * t256 - 135.0 / 64.0 * t44 * t567 * t327 * t233 + 15.0 / 16.0 * t44 * t232 * t713 * t117 + 15.0 / 16.0 * t44 * t675 * t256 + 5.0 / 16.0 * t44 * t305 * t597 - t44 * t106 * t1479 / 4.0 + t1515) * t82 - 49.0 / 24576.0 * t611 * t355 + t7 * t167 * t601 * t172 / 3072.0 + t7 * t349 * t260 * t355 / 8192.0 + 425.0 / 0.382205952e9 * t617 * t18 / t618 / t269 * t67 * t1333 * t282 * t352 * t444 + t7 * t540 * t161 * t285 / 512.0 - t7 * t128 * t717 * t137 / 384.0 - t7 * t273 * t331 * t285 / 3072.0 + 15.0 / 512.0 * t534 * t172 + 2083.0 / 221184.0 * t542 * t355 - 5.0 / 1024.0 * t607 * t172 - 355.0 / 10368.0 * t7 * t524 * t161 * t137 + 17.0 / 1152.0 * t7 * t210 * t331 * t137 - 5.0 / 36.0 * t19 * t31 * t161 * t82 - 17.0 / 331776.0 * t623 * t1571 * t630 - 289.0 / 442368.0 * t623 * t751 + 17.0 / 294912.0 * t748 * t1380 * t750 - 35.0 / 512.0 * t526 * t172;
        let t1583 = piecewise3(t3, 0.0, t1582);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t1583 + 6.0 * t755;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho3lapl0 = 0.0;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let t1618 = t1259 * t188;
        let t1625 = t181 * t1454;
        let t1633 = t59 * t551 * t181;
        let t1652 = 10000.0 / 243.0 * t1453 * t1625 + 10000.0 / 243.0 * t1474 * t1625 + 10000.0 / 729.0 * t1444 * t185 + 10000.0 / 2187.0 * t309 * t1633 + 100.0 / 81.0 * t1425 * t182 + 10000.0 / 729.0 * t1429 * t185 + 5000.0 / 729.0 * t321 * t1633 - 44000.0 / 2187.0 * t701 - 50000.0 / 729.0 * t704 + 200000.0 / 2187.0 * t706 - 50000.0 / 729.0 * t685 + 400000.0 / 6561.0 * t692 + 4000.0 / 243.0 * t697 - 500.0 / 81.0 * t695 - 25000.0 / 729.0 * t708 - 2200000.0 / 59049.0 * t711 - 50000.0 / 2187.0 * t689;
        let t1657 = t1246 * t188 * t233;
        let t1664 = t232 * t814;
        let t1675 = t567 * t385;
        let t1684 = t558 * t385 * t117;
        let t1688 = t558 * t188 * t256;
        let t1695 = 15.0 / 16.0 * t44 * t790 * t256 + 5.0 / 16.0 * t44 * t372 * t597 - 135.0 / 64.0 * t44 * t1675 * t233 + 5.0 / 36.0 * t178 * t1491 - 25.0 / 48.0 * t555 * t780 - 25.0 / 24.0 * t225 * t1684 - 25.0 / 48.0 * t225 * t1688 + 125.0 / 144.0 * t657 + 2200.0 / 243.0 * t652 + 50.0 / 27.0 * t654 - 25.0 / 36.0 * t660;
        let t1709 = t192 * t626;
        let t1714 = piecewise3(t3, 0.0, -5.0 / 36.0 * t19 * t31 * t192 * t82 + t19 * t26 * t389 * t82 / 4.0 - 355.0 / 10368.0 * t7 * t524 * t192 * t137 - 3.0 / 8.0 * t19 * t87 * t818 * t82 + 17.0 / 1152.0 * t7 * t210 * t389 * t137 + t7 * t540 * t192 * t285 / 512.0 - 3.0 / 8.0 * t19 * t20 * (5.0 / 36.0 * t1239 * t369 + 5.0 / 12.0 * t555 * t783 + 5.0 / 12.0 * t225 * t226 * t814 + 585.0 / 256.0 * t44 * t1618 * t568 - 135.0 / 64.0 * t949 * t779 * t256 - t44 * t106 * t1652 / 4.0 + 75.0 / 64.0 * t225 * t1657 + 25.0 / 64.0 * t178 * t1504 - 25.0 / 48.0 * t933 * t1251 + 15.0 / 16.0 * t44 * t1664 * t117 + t1695) * t82 - t7 * t128 * t818 * t137 / 384.0 - t7 * t273 * t389 * t285 / 3072.0 - 17.0 / 331776.0 * t623 * t1709 * t630);
        let tv4rho3tau0 = 2.0 * rho[ip] * t1714 + 6.0 * t832;
        v4rho3tau[ip] += tv4rho3tau0;
        let t1723 = t558 * t420 * t117;
        let t1745 = t237 * t239 * t35;
        let t1749 = t409 * t870 * t99;
        let t1753 = t409 * t76 * t221;
        let t1757 = 1.0 / t20 / t208;
        let t1758 = t409 * t1757;
        let t1766 = param_c1 * t239 * t35;
        let t1784 = t144 * t96 * t35;
        let t1787 = -25.0 / 72.0 * t225 * t1723 - 25.0 / 144.0 * t555 * t848 + 585.0 / 256.0 * t44 * t1259 * t405 * t233 - 45.0 / 16.0 * t949 * t664 * t327 + 5.0 / 36.0 * t555 * t858 + 5.0 / 18.0 * t225 * t226 * t882 - 45.0 / 64.0 * t44 * t567 * t420 * t233 - t44 * t106 * (625.0 / 1458.0 * t1745 * t410 - 10000.0 / 2187.0 * t682 * t1749 + 625.0 / 1458.0 * t682 * t1753 + 95000.0 / 19683.0 * t309 * t1758 + 950.0 / 729.0 * t413 * t1757 * t47 * t49 + 625.0 / 1458.0 * t1766 * t410 - 10000.0 / 2187.0 * t703 * t1749 + 625.0 / 1458.0 * t703 * t1753 + 47500.0 / 6561.0 * t321 * t1758) / 4.0 + 25.0 / 576.0 * t842 * t1501 - 55.0 / 162.0 * t651 * t402 - 25.0 / 108.0 * t1784 * t665;
        let t1802 = t327 * t327;
        let t1811 = t101 * t714;
        let t1815 = t1246 * t405 * t117;
        let t1819 = t558 * t157 * t327;
        let t1822 = 25.0 / 288.0 * t842 * t1497 - 25.0 / 256.0 * t842 * t1510 + 5.0 / 8.0 * t44 * t232 * t882 * t117 + 5.0 / 16.0 * t44 * t861 * t256 - 45.0 / 64.0 * t44 * t851 * t256 + 5.0 / 8.0 * t44 * t232 * t1802 + 5.0 / 8.0 * t44 * t305 * t713 + 5.0 / 27.0 * t296 * t845 - 5.0 / 144.0 * t145 * t1811 + 25.0 / 32.0 * t225 * t1815 - 25.0 / 36.0 * t225 * t1819;
        let t1890 = 5.0 / 512.0 * t212 * t445 + 17.0 / 3456.0 * t7 * t210 * t424 * t137 - t7 * t128 * t886 * t137 / 576.0 - t7 * t273 * t424 * t285 / 9216.0 - 5.0 / 768.0 * t723 * t172 - 49.0 / 36864.0 * t727 * t355 + t7 * t167 * t717 * t172 / 1536.0 + t7 * t349 * t331 * t355 / 6144.0 + t19 * t26 * t424 * t82 / 12.0 - 241.0 / 98304.0 * t275 * t441 + 5.0 / 256.0 * t648 * t172;
        let t1892 = piecewise3(t3, 0.0, -t19 * t87 * t886 * t82 / 4.0 - 3.0 / 8.0 * t19 * t20 * (t1787 + t1822) * t82 - 5.0 / 1536.0 * t266 * t445 + t737 * t445 / 3072.0 + 17.0 / 442368.0 * t748 * t1571 * t750 + 1513.0 / 7077888.0 * t748 * t916 - 17.0 / 1179648.0 * t914 * t1380 * t915 + 13.0 / 32768.0 * t741 * t441 - t7 * t436 * t260 * t441 / 65536.0 - 425.0 / 0.1019215872e10 * t617 * t18 / t618 / t208 * t67 * t1333 * t629 * t444 + t1890);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t1892 + 4.0 * t924;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rho2sigmalapl0 = 0.0;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let t1909 = t44 * t1259;
        let t1916 = t385 * t157;
        let t1920 = t188 * t327;
        let t1933 = t409 * t460 * t221;
        let t1950 = t558 * t1920;
        let t1954 = t558 * t470 * t117;
        let t1959 = t101 * t815;
        let t1964 = t558 * t1916;
        let t1969 = 125.0 / 216.0 * t843 + 585.0 / 256.0 * t1909 * t945 * t233 + 5.0 / 18.0 * t225 * t226 * t975 - 45.0 / 32.0 * t949 * t1916 * t117 - 45.0 / 32.0 * t949 * t1920 * t117 - 45.0 / 64.0 * t949 * t945 * t256 + 5.0 / 36.0 * t555 * t959 - t44 * t106 * (-2500.0 / 729.0 * t1745 * t461 + 65000.0 / 2187.0 * t867 - 2500.0 / 729.0 * t682 * t1933 - 520000.0 / 19683.0 * t872 - 5200.0 / 729.0 * t876 - 2500.0 / 729.0 * t1766 * t461 + 65000.0 / 2187.0 * t878 - 2500.0 / 729.0 * t703 * t1933 - 260000.0 / 6561.0 * t880) / 4.0 - 55.0 / 324.0 * t651 * t453 - 25.0 / 72.0 * t225 * t1950 - 25.0 / 72.0 * t225 * t1954 + 5.0 / 54.0 * t296 * t942 - 5.0 / 288.0 * t145 * t1959 - 25.0 / 144.0 * t555 * t946 - 25.0 / 72.0 * t225 * t1964 + 5.0 / 36.0 * t178 * t1811;
        let t1970 = t232 * t975;
        let t1977 = t567 * t470;
        let t2010 = 5.0 / 8.0 * t44 * t1970 * t117 + 5.0 / 16.0 * t44 * t962 * t256 - 45.0 / 64.0 * t44 * t1977 * t233 + 5.0 / 16.0 * t44 * t1664 * t157 + 5.0 / 8.0 * t44 * t790 * t327 + 5.0 / 16.0 * t44 * t372 * t713 + 25.0 / 32.0 * t100 * t101 * t786 * t664 - 25.0 / 72.0 * t933 * t1497 - 25.0 / 144.0 * t933 * t1501 - 25.0 / 216.0 * t1784 * t780 + 25.0 / 576.0 * t842 * t1684 + 25.0 / 1152.0 * t842 * t1688 + 25.0 / 64.0 * t933 * t1510 - 25.0 / 512.0 * t842 * t1657 - 25.0 / 54.0 * t846 + 50.0 / 81.0 * t839;
        let t2041 = t19 * t26 * t474 * t82 / 12.0 - t19 * t87 * t979 * t82 / 4.0 + 17.0 / 3456.0 * t7 * t210 * t474 * t137 - 3.0 / 8.0 * t19 * t20 * (t1969 + t2010) * t82 - t7 * t128 * t979 * t137 / 576.0 - t7 * t273 * t474 * t285 / 9216.0 + 5.0 / 512.0 * t768 * t172 - 5.0 / 1536.0 * t824 * t172 - 49.0 / 73728.0 * t828 * t355 + t7 * t167 * t818 * t172 / 3072.0 + t7 * t349 * t389 * t355 / 12288.0 + 17.0 / 884736.0 * t748 * t1709 * t750;
        let t2042 = piecewise3(t3, 0.0, t2041);
        let tv4rho2sigmatau0 = 2.0 * rho[ip] * t2042 + 4.0 * t999;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let tv4rho2lapl20 = 0.0;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let tv4rho2lapltau0 = 0.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let t2062 = t1259 * t488;
        let t2074 = t567 * t503;
        let t2086 = t409 * t493 * t221;
        let t2104 = 25.0 / 32.0 * t933 * t1657 - 25.0 / 144.0 * t555 * t1011 + 585.0 / 256.0 * t44 * t2062 * t233 - 45.0 / 16.0 * t949 * t779 * t385 + 5.0 / 36.0 * t555 * t1021 + 5.0 / 18.0 * t225 * t226 * t1037 - 45.0 / 64.0 * t44 * t2074 * t233 - 25.0 / 36.0 * t933 * t1684 - 25.0 / 72.0 * t933 * t1688 - t44 * t106 * (20000.0 / 729.0 * t1745 * t494 - 400000.0 / 2187.0 * t968 + 20000.0 / 729.0 * t682 * t2086 + 2600000.0 / 19683.0 * t411 + 26000.0 / 729.0 * t416 + 20000.0 / 729.0 * t1766 * t494 - 400000.0 / 2187.0 * t972 + 20000.0 / 729.0 * t703 * t2086 + 1300000.0 / 6561.0 * t418) / 4.0 - 45.0 / 64.0 * t44 * t1014 * t256;
        let t2105 = t385 * t385;
        let t2112 = t232 * t1037;
        let t2122 = t1246 * t488 * t117;
        let t2126 = t558 * t188 * t385;
        let t2130 = t558 * t503 * t117;
        let t2136 = 5.0 / 8.0 * t44 * t232 * t2105 + 5.0 / 8.0 * t44 * t372 * t814 + 5.0 / 8.0 * t44 * t2112 * t117 + 5.0 / 16.0 * t44 * t1024 * t256 + 5.0 / 18.0 * t178 * t1959 + 25.0 / 32.0 * t225 * t2122 - 25.0 / 36.0 * t225 * t2126 - 25.0 / 72.0 * t225 * t2130 - 25.0 / 27.0 * t943 + 100.0 / 81.0 * t938 + 125.0 / 108.0 * t940;
        let t2151 = piecewise3(t3, 0.0, t19 * t26 * t507 * t82 / 12.0 - t19 * t87 * t1041 * t82 / 4.0 + 17.0 / 3456.0 * t7 * t210 * t507 * t137 - 3.0 / 8.0 * t19 * t20 * (t2104 + t2136) * t82 - t7 * t128 * t1041 * t137 / 576.0 - t7 * t273 * t507 * t285 / 9216.0);
        let tv4rho2tau20 = 2.0 * rho[ip] * t2151 + 4.0 * t1051;
        v4rho2tau2[ip] += tv4rho2tau20;
        let t2168 = t101 * t883;
        let t2181 = t157 * t420;
        let t2182 = t558 * t2181;
        let t2201 = t237 * t99;
        let t2204 = t60 * t1339;
        let t2207 = param_c1 * t99;
        let t2216 = -25.0 / 144.0 * t296 * t1054 - 75.0 / 512.0 * t842 * t1815 + 25.0 / 192.0 * t842 * t1819 + 5.0 / 36.0 * t296 * t1057 + 25.0 / 384.0 * t842 * t1723 - 5.0 / 96.0 * t145 * t2168 + 25.0 / 64.0 * t225 * t1246 * t1060 + 585.0 / 256.0 * t44 * t1259 * t1060 * t117 - 135.0 / 64.0 * t44 * t851 * t327 - 25.0 / 48.0 * t225 * t2182 - 135.0 / 64.0 * t949 * t2181 * t117 + 15.0 / 16.0 * t44 * t675 * t420 + 15.0 / 16.0 * t44 * t305 * t882 + 5.0 / 36.0 * t225 * t226 * t1071 + 5.0 / 16.0 * t44 * t232 * t1071 * t117 - t44 * t106 * (-625.0 / 5832.0 * t2201 * t1067 + 625.0 / 729.0 * t576 * t2204 - 625.0 / 5832.0 * t2207 * t1067 + 625.0 / 729.0 * t589 * t2204) / 4.0;
        let t2266 = -t19 * t87 * t1075 * t82 / 8.0 - 3.0 / 8.0 * t19 * t20 * t2216 * t82 - t7 * t128 * t1075 * t137 / 1152.0 - 5.0 / 1024.0 * t892 * t172 + t7 * t167 * t886 * t172 / 1024.0 + t7 * t349 * t424 * t355 / 8192.0 + 39.0 / 65536.0 * t902 * t441 - 3.0 / 65536.0 * t7 * t436 * t331 * t441 - 17.0 / 786432.0 * t914 * t1571 * t915 - 5.0 / 1024.0 * t337 * t445 + t898 * t445 / 1024.0 - 1207.0 / 0.18874368e8 * t914 * t1096 + 17.0 / 6291456.0 * t1094 * t1380 * t1095 + 425.0 / 0.2717908992e10 * t617 * t18 / t618 / t126 * t67 * t1333 * t749 * t444 + 31.0 / 65536.0 * t351 * t1101 - 3.0 / 65536.0 * t908 * t1101;
        let t2267 = piecewise3(t3, 0.0, t2266);
        let tv4rhosigma30 = 2.0 * rho[ip] * t2267 + 2.0 * t1105;
        v4rhosigma3[ip] += tv4rhosigma30;
        let tv4rhosigma2lapl0 = 0.0;
        v4rhosigma2lapl[ip] += tv4rhosigma2lapl0;
        let t2274 = t1246 * t950;
        let t2305 = t188 * t405;
        let t2306 = t1246 * t2305;
        let t2309 = t470 * t157;
        let t2310 = t558 * t2309;
        let t2313 = -25.0 / 256.0 * t842 * t2274 - t44 * t106 * (625.0 / 729.0 * t2201 * t1125 - 4375.0 / 729.0 * t1068 + 625.0 / 729.0 * t2207 * t1125 - 4375.0 / 729.0 * t1069) / 4.0 + 125.0 / 432.0 * t1055 - 25.0 / 108.0 * t1058 + 5.0 / 54.0 * t296 * t1113 - 25.0 / 144.0 * t933 * t1723 + 25.0 / 576.0 * t842 * t1964 + 25.0 / 576.0 * t842 * t1950 + 25.0 / 576.0 * t842 * t1954 - 25.0 / 216.0 * t1784 * t946 + 25.0 / 64.0 * t933 * t1815 - 25.0 / 72.0 * t933 * t1819 + 25.0 / 64.0 * t225 * t2306 - 25.0 / 72.0 * t225 * t2310;
        let t2314 = t188 * t420;
        let t2315 = t558 * t2314;
        let t2320 = t101 * t976;
        let t2341 = t232 * t1129;
        let t2357 = -25.0 / 144.0 * t225 * t2315 + 5.0 / 36.0 * t178 * t2168 - 5.0 / 144.0 * t145 * t2320 - 45.0 / 32.0 * t949 * t2309 * t117 - 45.0 / 64.0 * t949 * t2314 * t117 + 5.0 / 36.0 * t225 * t226 * t1129 + 585.0 / 256.0 * t1909 * t2305 * t117 - 45.0 / 32.0 * t949 * t945 * t327 + 5.0 / 16.0 * t44 * t372 * t882 + 5.0 / 16.0 * t44 * t2341 * t117 - 45.0 / 64.0 * t44 * t1675 * t405 + 5.0 / 8.0 * t44 * t1970 * t157 + 5.0 / 8.0 * t44 * t962 * t327 + 5.0 / 16.0 * t44 * t790 * t420;
        let t2390 = -t19 * t87 * t1133 * t82 / 8.0 - 3.0 / 8.0 * t19 * t20 * (t2313 + t2357) * t82 - t7 * t128 * t1133 * t137 / 1152.0 - 5.0 / 1536.0 * t985 * t172 + t7 * t167 * t979 * t172 / 1536.0 + t7 * t349 * t474 * t355 / 12288.0 + 13.0 / 65536.0 * t995 * t441 - t7 * t436 * t389 * t441 / 65536.0 - 17.0 / 2359296.0 * t914 * t1709 * t915 - 5.0 / 3072.0 * t395 * t445 + t991 * t445 / 3072.0;
        let t2391 = piecewise3(t3, 0.0, t2390);
        let tv4rhosigma2tau0 = 2.0 * rho[ip] * t2391 + 2.0 * t1149;
        v4rhosigma2tau[ip] += tv4rhosigma2tau0;
        let tv4rhosigmalapl20 = 0.0;
        v4rhosigmalapl2[ip] += tv4rhosigmalapl20;
        let tv4rhosigmalapltau0 = 0.0;
        v4rhosigmalapltau[ip] += tv4rhosigmalapltau0;
        let t2401 = t101 * t1038;
        let t2404 = t503 * t157;
        let t2405 = t558 * t2404;
        let t2410 = t488 * t157;
        let t2411 = t1246 * t2410;
        let t2414 = t188 * t470;
        let t2415 = t558 * t2414;
        let t2431 = 125.0 / 216.0 * t1111 + 25.0 / 32.0 * t933 * t2274 - 5.0 / 288.0 * t145 * t2401 - 25.0 / 144.0 * t225 * t2405 + 5.0 / 18.0 * t178 * t2320 + 25.0 / 64.0 * t225 * t2411 - 25.0 / 72.0 * t225 * t2415 + 5.0 / 108.0 * t296 * t1164 - 25.0 / 432.0 * t296 * t1155 - 25.0 / 54.0 * t1114 - 25.0 / 512.0 * t842 * t2122 + 25.0 / 576.0 * t842 * t2126 + 25.0 / 1152.0 * t842 * t2130 - 25.0 / 72.0 * t933 * t1964;
        let t2467 = t232 * t1174;
        let t2480 = -25.0 / 72.0 * t933 * t1950 - 25.0 / 72.0 * t933 * t1954 + 5.0 / 16.0 * t44 * t2112 * t157 + 5.0 / 16.0 * t44 * t1024 * t327 - 45.0 / 32.0 * t949 * t945 * t385 - 45.0 / 32.0 * t949 * t2414 * t117 - 45.0 / 64.0 * t949 * t2404 * t117 + 5.0 / 36.0 * t225 * t226 * t1174 + 585.0 / 256.0 * t1909 * t2410 * t117 - t44 * t106 * (-5000.0 / 729.0 * t2201 * t1170 + 10000.0 / 243.0 * t1126 - 5000.0 / 729.0 * t2207 * t1170 + 10000.0 / 243.0 * t1127) / 4.0 + 5.0 / 16.0 * t44 * t2467 * t117 - 45.0 / 64.0 * t44 * t1014 * t327 + 5.0 / 8.0 * t44 * t790 * t470 + 5.0 / 8.0 * t44 * t372 * t975;
        let t2501 = piecewise3(t3, 0.0, -t19 * t87 * t1178 * t82 / 8.0 - 3.0 / 8.0 * t19 * t20 * (t2431 + t2480) * t82 - t7 * t128 * t1178 * t137 / 1152.0 - 5.0 / 3072.0 * t1047 * t172 + t7 * t167 * t1041 * t172 / 3072.0 + t7 * t349 * t507 * t355 / 24576.0);
        let tv4rhosigmatau20 = 2.0 * rho[ip] * t2501 + 2.0 * t1188;
        v4rhosigmatau2[ip] += tv4rhosigmatau20;
        let tv4rholapl30 = 0.0;
        v4rholapl3[ip] += tv4rholapl30;
        let tv4rholapl2tau0 = 0.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let tv4rholapltau20 = 0.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let t2521 = t1259 * t1194;
        let t2528 = t188 * t503;
        let t2529 = t558 * t2528;
        let t2544 = t232 * t1205;
        let t2558 = 125.0 / 144.0 * t1156 + 75.0 / 64.0 * t933 * t2122 - 25.0 / 24.0 * t933 * t2126 - 25.0 / 36.0 * t1165 - 25.0 / 48.0 * t933 * t2130 + 5.0 / 12.0 * t178 * t2401 + 25.0 / 64.0 * t225 * t1246 * t1194 + 585.0 / 256.0 * t44 * t2521 * t117 - 135.0 / 64.0 * t44 * t1014 * t385 - 25.0 / 48.0 * t225 * t2529 - 135.0 / 64.0 * t949 * t2528 * t117 + 15.0 / 16.0 * t44 * t790 * t503 + 15.0 / 16.0 * t44 * t372 * t1037 + 5.0 / 36.0 * t225 * t226 * t1205 + 5.0 / 16.0 * t44 * t2544 * t117 - t44 * t106 * (40000.0 / 729.0 * t2201 * t1201 - 200000.0 / 729.0 * t1171 + 40000.0 / 729.0 * t2207 * t1201 - 200000.0 / 729.0 * t1172) / 4.0;
        let t2568 = piecewise3(t3, 0.0, -t19 * t87 * t1209 * t82 / 8.0 - 3.0 / 8.0 * t19 * t20 * t2558 * t82 - t7 * t128 * t1209 * t137 / 1152.0);
        let tv4rhotau30 = 2.0 * rho[ip] * t2568 + 2.0 * t1214;
        v4rhotau3[ip] += tv4rhotau30;
        let t2571 = t101 * t1061;
        let t2576 = t101 * t1072;
        let t2579 = t405 * t405;
        let t2586 = t420 * t420;
        let t2593 = t237 * t22;
        let t2597 = 1.0 / t24 / t433 * t35 * t59;
        let t2599 = param_c1 * t22;
        let t2644 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * (-25.0 / 128.0 * t145 * t2571 + 25.0 / 96.0 * t842 * t2182 - 5.0 / 72.0 * t145 * t2576 + 585.0 / 256.0 * t44 * t1259 * t2579 - 135.0 / 32.0 * t44 * t851 * t420 + 15.0 / 16.0 * t44 * t232 * t2586 + 5.0 / 4.0 * t44 * t305 * t1071 - t44 * t106 * (625.0 / 46656.0 * t2593 * t2597 + 625.0 / 46656.0 * t2599 * t2597) / 4.0) * t82 + t7 * t167 * t1075 * t172 / 768.0 - 3.0 / 32768.0 * t7 * t436 * t424 * t441 + t1081 * t445 / 512.0 + 17.0 / 1572864.0 * t1094 * t1571 * t1095 - 3.0 / 16384.0 * t1085 * t1101 - 425.0 / 0.7247757312e10 * t617 * t18 / t618 / t74 * t67 * t1333 * t282 * t444 + 17.0 / 1048576.0 * t1094 * t627 * t628 * t71 - 3.0 / 65536.0 * t438 * t280 * t60 * t22);
        let tv4sigma40 = 2.0 * rho[ip] * t2644;
        v4sigma4[ip] += tv4sigma40;
        let tv4sigma3lapl0 = 0.0;
        v4sigma3lapl[ip] += tv4sigma3lapl0;
        let t2658 = t101 * t1130;
        let t2682 = 1.0 / t24 / t1323 * t35 * t59;
        let t2690 = 25.0 / 64.0 * t178 * t2571 - 25.0 / 48.0 * t933 * t2182 + 5.0 / 36.0 * t178 * t2576 - 75.0 / 512.0 * t842 * t2306 + 25.0 / 192.0 * t842 * t2310 + 25.0 / 384.0 * t842 * t2315 - 5.0 / 96.0 * t145 * t2658 + 585.0 / 256.0 * t44 * t1618 * t1060 - 135.0 / 64.0 * t44 * t1977 * t405 - 135.0 / 64.0 * t949 * t945 * t420 + 15.0 / 16.0 * t44 * t2341 * t157 + 15.0 / 16.0 * t44 * t962 * t420 + 5.0 / 16.0 * t44 * t372 * t1071 - t44 * t106 * (-625.0 / 5832.0 * t2593 * t2682 - 625.0 / 5832.0 * t2599 * t2682) / 4.0;
        let t2711 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t2690 * t82 + t7 * t167 * t1133 * t172 / 1024.0 - 3.0 / 65536.0 * t7 * t436 * t474 * t441 + t1139 * t445 / 1024.0 + 17.0 / 6291456.0 * t1094 * t1709 * t1095 - 3.0 / 65536.0 * t1143 * t1101);
        let tv4sigma3tau0 = 2.0 * rho[ip] * t2711;
        v4sigma3tau[ip] += tv4sigma3tau0;
        let tv4sigma2lapl20 = 0.0;
        v4sigma2lapl2[ip] += tv4sigma2lapl20;
        let tv4sigma2lapltau0 = 0.0;
        v4sigma2lapltau[ip] += tv4sigma2lapltau0;
        let t2734 = t470 * t470;
        let t2743 = t101 * t1175;
        let t2758 = 1.0 / t24 / t269 * t35 * t59;
        let t2766 = 25.0 / 32.0 * t933 * t2306 - 25.0 / 36.0 * t933 * t2310 - 25.0 / 72.0 * t933 * t2315 + 5.0 / 18.0 * t178 * t2658 - 25.0 / 256.0 * t842 * t2411 + 25.0 / 288.0 * t842 * t2415 + 585.0 / 256.0 * t44 * t2062 * t405 - 45.0 / 16.0 * t949 * t945 * t470 - 45.0 / 64.0 * t44 * t1014 * t420 + 5.0 / 8.0 * t44 * t232 * t2734 + 5.0 / 8.0 * t44 * t372 * t1129 + 25.0 / 576.0 * t842 * t2405 - 5.0 / 144.0 * t145 * t2743 - 45.0 / 64.0 * t44 * t2074 * t405 + 5.0 / 8.0 * t44 * t2467 * t157 + 5.0 / 16.0 * t44 * t1024 * t420 - t44 * t106 * (625.0 / 729.0 * t2593 * t2758 + 625.0 / 729.0 * t2599 * t2758) / 4.0;
        let t2782 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t2766 * t82 + t7 * t167 * t1178 * t172 / 1536.0 - t7 * t436 * t507 * t441 / 65536.0 + t1184 * t445 / 3072.0);
        let tv4sigma2tau20 = 2.0 * rho[ip] * t2782;
        v4sigma2tau2[ip] += tv4sigma2tau20;
        let tv4sigmalapl30 = 0.0;
        v4sigmalapl3[ip] += tv4sigmalapl30;
        let tv4sigmalapl2tau0 = 0.0;
        v4sigmalapl2tau[ip] += tv4sigmalapl2tau0;
        let tv4sigmalapltau20 = 0.0;
        v4sigmalapltau2[ip] += tv4sigmalapltau20;
        let t2792 = t101 * t1195;
        let t2812 = t101 * t1206;
        let t2821 = 1.0 / t24 / t208 * t35 * t59;
        let t2829 = 75.0 / 64.0 * t933 * t2411 - 25.0 / 24.0 * t933 * t2415 - 25.0 / 48.0 * t933 * t2405 + 5.0 / 12.0 * t178 * t2743 - 25.0 / 512.0 * t145 * t2792 + 585.0 / 256.0 * t44 * t2521 * t157 - 135.0 / 64.0 * t44 * t1014 * t470 + 25.0 / 384.0 * t842 * t2529 - 135.0 / 64.0 * t949 * t2528 * t157 + 15.0 / 16.0 * t44 * t962 * t503 + 15.0 / 16.0 * t44 * t372 * t1174 - 5.0 / 288.0 * t145 * t2812 + 5.0 / 16.0 * t44 * t2544 * t157 - t44 * t106 * (-5000.0 / 729.0 * t2593 * t2821 - 5000.0 / 729.0 * t2599 * t2821) / 4.0;
        let t2839 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t2829 * t82 + t7 * t167 * t1209 * t172 / 3072.0);
        let tv4sigmatau30 = 2.0 * rho[ip] * t2839;
        v4sigmatau3[ip] += tv4sigmatau30;
        let tv4lapl40 = 0.0;
        v4lapl4[ip] += tv4lapl40;
        let tv4lapl3tau0 = 0.0;
        v4lapl3tau[ip] += tv4lapl3tau0;
        let tv4lapl2tau20 = 0.0;
        v4lapl2tau2[ip] += tv4lapl2tau20;
        let tv4lapltau30 = 0.0;
        v4lapltau3[ip] += tv4lapltau30;
        let t2847 = t488 * t488;
        let t2854 = t503 * t503;
        let t2862 = t1232 * t35 * t59;
        let t2875 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * (25.0 / 16.0 * t178 * t2792 - 25.0 / 12.0 * t933 * t2529 + 5.0 / 9.0 * t178 * t2812 + 585.0 / 256.0 * t44 * t1259 * t2847 - 135.0 / 32.0 * t44 * t1014 * t503 + 15.0 / 16.0 * t44 * t232 * t2854 + 5.0 / 4.0 * t44 * t372 * t1205 - t44 * t106 * (40000.0 / 729.0 * t2593 * t2862 + 40000.0 / 729.0 * t2599 * t2862) / 4.0) * t82);
        let tv4tau40 = 2.0 * rho[ip] * t2875;
        v4tau4[ip] += tv4tau40;
    }
}
