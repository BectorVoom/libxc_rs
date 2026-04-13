//! MGGA_X_JK lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_jk.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_jk_lxc_unpol(
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
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t6 = 1.0 / t5;
        let t7 = t4 * t6;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = t4 * t4;
        let t22 = param_beta * t21;
        let t24 = pow_1_3(1.0 / M_PI);
        let t25 = 1.0 / t24;
        let t26 = M_CBRT4;
        let t27 = t25 * t26;
        let t28 = t22 * t27;
        let t29 = M_CBRT2;
        let t30 = t29 * t29;
        let t31 = sigma[ip] * t30;
        let t32 = rho[ip] * rho[ip];
        let t33 = t19 * t19;
        let t34 = t33 * t32;
        let t35 = 1.0 / t34;
        let t36 = param_gamma * param_beta;
        let t37 = f64::sqrt(sigma[ip]);
        let t38 = t36 * t37;
        let t40 = 1.0 / t19 / rho[ip];
        let t41 = t29 * t40;
        let t44 = f64::ln(t37 * t29 * t40 + f64::sqrt(pow_2(t37 * t29 * t40) + 1.0));
        let t45 = t41 * t44;
        let t47 = t38 * t45 + 1.0;
        let t48 = 1.0 / t47;
        let t49 = t35 * t48;
        let t50 = t31 * t35;
        let t51 = lapl[ip] * t30;
        let t52 = t33 * rho[ip];
        let t53 = 1.0 / t52;
        let t55 = -t51 * t53 + t50;
        let t56 = 1.0 / sigma[ip];
        let t57 = t55 * t56;
        let t58 = t29 * t34;
        let t60 = t57 * t58 + 1.0;
        let t61 = 1.0 / t60;
        let t66 = 1.0 + 2.0 / 9.0 * t28 * t31 * t49 * t61;
        let t70 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t66);
        let tzk0 = 2.0 * t70;
        zk[ip] += tzk0;
        let t72 = t18 / t33;
        let t76 = t32 * rho[ip];
        let t78 = 1.0 / t33 / t76;
        let t79 = t78 * t48;
        let t85 = t22 * t27 * sigma[ip];
        let t86 = t30 * t35;
        let t87 = t47 * t47;
        let t88 = 1.0 / t87;
        let t89 = t88 * t61;
        let t91 = 1.0 / t19 / t32;
        let t93 = t29 * t91 * t44;
        let t95 = t36 * sigma[ip];
        let t96 = t30 * t78;
        let t97 = t50 + 1.0;
        let t98 = f64::sqrt(t97);
        let t99 = 1.0 / t98;
        let t100 = t96 * t99;
        let t103 = -4.0 / 3.0 * t95 * t100 - 4.0 / 3.0 * t38 * t93;
        let t104 = t89 * t103;
        let t105 = t86 * t104;
        let t108 = t60 * t60;
        let t109 = 1.0 / t108;
        let t110 = t48 * t109;
        let t115 = -8.0 / 3.0 * t31 * t78 + 5.0 / 3.0 * t51 * t35;
        let t116 = t115 * t56;
        let t118 = t29 * t52;
        let t121 = t116 * t58 + 8.0 / 3.0 * t57 * t118;
        let t122 = t110 * t121;
        let t123 = t86 * t122;
        let t126 = -16.0 / 27.0 * t28 * t31 * t79 * t61 - 2.0 / 9.0 * t85 * t105 - 2.0 / 9.0 * t85 * t123;
        let t131 = piecewise3(t3, 0.0, -t7 * t72 * t66 / 8.0 - 3.0 / 8.0 * t7 * t20 * t126);
        let tvrho0 = 2.0 * rho[ip] * t131 + 2.0 * t70;
        vrho[ip] += tvrho0;
        let t134 = t48 * t61;
        let t138 = t36 / t37;
        let t140 = t86 * t99;
        let t143 = t138 * t45 / 2.0 + t36 * t140 / 2.0;
        let t144 = t89 * t143;
        let t145 = t86 * t144;
        let t148 = sigma[ip] * sigma[ip];
        let t149 = 1.0 / t148;
        let t150 = t55 * t149;
        let t152 = -t150 * t58 + 2.0 * t56;
        let t153 = t110 * t152;
        let t154 = t86 * t153;
        let t157 = 2.0 / 9.0 * t28 * t86 * t134 - 2.0 / 9.0 * t85 * t145 - 2.0 / 9.0 * t85 * t154;
        let t161 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t157);
        let tvsigma0 = 2.0 * rho[ip] * t161;
        vsigma[ip] += tvsigma0;
        let t163 = t6 * t18;
        let t164 = t40 * param_beta;
        let t166 = t30 * t48;
        let t168 = t27 * t166 * t109;
        let t171 = piecewise3(t3, 0.0, -t163 * t164 * t168 / 2.0);
        let tvlapl0 = 2.0 * rho[ip] * t171;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t174 = t18 * t53;
        let t181 = t32 * t32;
        let t183 = 1.0 / t33 / t181;
        let t189 = t96 * t104;
        let t192 = t96 * t122;
        let t196 = 1.0 / t87 / t47;
        let t197 = t196 * t61;
        let t198 = t103 * t103;
        let t199 = t197 * t198;
        let t200 = t86 * t199;
        let t203 = t86 * t88;
        let t204 = t109 * t103;
        let t205 = t204 * t121;
        let t210 = 1.0 / t19 / t76;
        let t212 = t29 * t210 * t44;
        let t215 = t30 * t183;
        let t216 = t215 * t99;
        let t219 = t36 * t148;
        let t222 = 1.0 / t19 / t181 / t76;
        let t225 = 1.0 / t98 / t97;
        let t226 = t29 * t222 * t225;
        let t229 = 28.0 / 9.0 * t38 * t212 + 20.0 / 3.0 * t95 * t216 - 32.0 / 9.0 * t219 * t226;
        let t230 = t89 * t229;
        let t231 = t86 * t230;
        let t235 = 1.0 / t108 / t60;
        let t236 = t48 * t235;
        let t237 = t121 * t121;
        let t238 = t236 * t237;
        let t239 = t86 * t238;
        let t246 = 88.0 / 9.0 * t31 * t183 - 40.0 / 9.0 * t51 * t78;
        let t247 = t246 * t56;
        let t251 = t29 * t33;
        let t254 = t247 * t58 + 16.0 / 3.0 * t116 * t118 + 40.0 / 9.0 * t57 * t251;
        let t255 = t110 * t254;
        let t256 = t86 * t255;
        let t259 = 176.0 / 81.0 * t28 * t31 * t183 * t48 * t61 + 32.0 / 27.0 * t85 * t189 + 32.0 / 27.0 * t85 * t192 + 4.0 / 9.0 * t85 * t200 + 4.0 / 9.0 * t85 * t203 * t205 - 2.0 / 9.0 * t85 * t231 + 4.0 / 9.0 * t85 * t239 - 2.0 / 9.0 * t85 * t256;
        let t264 = piecewise3(t3, 0.0, t7 * t174 * t66 / 12.0 - t7 * t72 * t126 / 4.0 - 3.0 / 8.0 * t7 * t20 * t259);
        let tv2rho20 = 2.0 * rho[ip] * t264 + 4.0 * t131;
        v2rho2[ip] += tv2rho20;
        let t277 = t96 * t144;
        let t280 = t86 * t196;
        let t281 = t61 * t143;
        let t282 = t281 * t103;
        let t286 = t109 * t143;
        let t287 = t286 * t121;
        let t295 = t36 * t29;
        let t296 = t181 * t32;
        let t298 = 1.0 / t19 / t296;
        let t303 = -2.0 / 3.0 * t138 * t93 - 2.0 * t36 * t100 + 4.0 / 3.0 * t295 * t298 * t225 * sigma[ip];
        let t304 = t89 * t303;
        let t305 = t86 * t304;
        let t308 = t96 * t153;
        let t311 = t109 * t152;
        let t312 = t311 * t103;
        let t316 = t86 * t48;
        let t317 = t235 * t152;
        let t318 = t317 * t121;
        let t322 = t115 * t149;
        let t326 = -t322 * t58 - 8.0 / 3.0 * t150 * t118;
        let t327 = t110 * t326;
        let t328 = t86 * t327;
        let t331 = -16.0 / 27.0 * t28 * t96 * t134 - 2.0 / 9.0 * t28 * t105 - 2.0 / 9.0 * t28 * t123 + 16.0 / 27.0 * t85 * t277 + 4.0 / 9.0 * t85 * t280 * t282 + 2.0 / 9.0 * t85 * t203 * t287 - 2.0 / 9.0 * t85 * t305 + 16.0 / 27.0 * t85 * t308 + 2.0 / 9.0 * t85 * t203 * t312 + 4.0 / 9.0 * t85 * t316 * t318 - 2.0 / 9.0 * t85 * t328;
        let t336 = piecewise3(t3, 0.0, -t7 * t72 * t157 / 8.0 - 3.0 / 8.0 * t7 * t20 * t331);
        let tv2rhosigma0 = 2.0 * rho[ip] * t336 + 2.0 * t161;
        v2rhosigma[ip] += tv2rhosigma0;
        let t339 = t91 * param_beta;
        let t344 = t163 * t164 * t25;
        let t345 = t26 * t30;
        let t346 = t88 * t109;
        let t348 = t345 * t346 * t103;
        let t352 = t345 * t236 * t121;
        let t355 = piecewise3(t3, 0.0, 2.0 / 3.0 * t163 * t339 * t168 + t344 * t348 / 2.0 + t344 * t352);
        let tv2rholapl0 = 2.0 * rho[ip] * t355 + 2.0 * t171;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip] += tv2rhotau0;
        let t362 = t143 * t143;
        let t363 = t197 * t362;
        let t364 = t86 * t363;
        let t367 = t286 * t152;
        let t373 = t36 / t37 / sigma[ip];
        let t376 = t36 * t56;
        let t379 = t181 * rho[ip];
        let t382 = t29 / t19 / t379;
        let t383 = t382 * t225;
        let t386 = -t373 * t45 / 4.0 + t376 * t140 / 4.0 - t36 * t383 / 2.0;
        let t387 = t89 * t386;
        let t388 = t86 * t387;
        let t391 = t152 * t152;
        let t392 = t236 * t391;
        let t393 = t86 * t392;
        let t397 = t148 * sigma[ip];
        let t398 = 1.0 / t397;
        let t399 = t55 * t398;
        let t402 = 2.0 * t399 * t58 - 4.0 * t149;
        let t403 = t110 * t402;
        let t404 = t86 * t403;
        let t407 = -4.0 / 9.0 * t28 * t145 - 4.0 / 9.0 * t28 * t154 + 4.0 / 9.0 * t85 * t364 + 4.0 / 9.0 * t85 * t203 * t367 - 2.0 / 9.0 * t85 * t388 + 4.0 / 9.0 * t85 * t393 - 2.0 / 9.0 * t85 * t404;
        let t411 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t407);
        let tv2sigma20 = 2.0 * rho[ip] * t411;
        v2sigma2[ip] += tv2sigma20;
        let t414 = t345 * t346 * t143;
        let t418 = t345 * t236 * t152;
        let t421 = piecewise3(t3, 0.0, t344 * t414 / 2.0 + t344 * t418);
        let tv2sigmalapl0 = 2.0 * rho[ip] * t421;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t423 = 1.0 / t19;
        let t426 = t163 * t423 * param_beta * t25;
        let t428 = t345 * t236 * t56;
        let t431 = piecewise3(t3, 0.0, -2.0 * t426 * t428);
        let tv2lapl20 = 2.0 * rho[ip] * t431;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
        let t434 = t18 * t35;
        let t444 = t235 * t103;
        let t445 = t444 * t237;
        let t449 = t108 * t108;
        let t450 = 1.0 / t449;
        let t451 = t48 * t450;
        let t452 = t237 * t121;
        let t453 = t451 * t452;
        let t454 = t86 * t453;
        let t457 = t235 * t121;
        let t458 = t457 * t254;
        let t462 = t87 * t87;
        let t463 = 1.0 / t462;
        let t464 = t463 * t61;
        let t465 = t198 * t103;
        let t466 = t464 * t465;
        let t467 = t86 * t466;
        let t470 = t109 * t198;
        let t471 = t470 * t121;
        let t475 = t61 * t103;
        let t476 = t475 * t229;
        let t480 = t109 * t229;
        let t481 = t480 * t121;
        let t485 = t204 * t254;
        let t489 = t96 * t199;
        let t492 = t96 * t88;
        let t496 = t96 * t238;
        let t500 = 1.0 / t19 / t181;
        let t502 = t29 * t500 * t44;
        let t506 = 1.0 / t33 / t379;
        let t507 = t30 * t506;
        let t508 = t507 * t99;
        let t511 = t181 * t181;
        let t513 = 1.0 / t19 / t511;
        let t518 = t511 * t76;
        let t519 = 1.0 / t518;
        let t521 = t97 * t97;
        let t523 = 1.0 / t98 / t521;
        let t527 = -280.0 / 27.0 * t38 * t502 - 952.0 / 27.0 * t95 * t508 + 1184.0 / 27.0 * t219 * t29 * t513 * t225 - 256.0 / 9.0 * t36 * t397 * t519 * t523;
        let t528 = t89 * t527;
        let t529 = t86 * t528;
        let t536 = -1232.0 / 27.0 * t31 * t506 + 440.0 / 27.0 * t51 * t183;
        let t537 = t536 * t56;
        let t543 = t29 * t423;
        let t546 = t537 * t58 + 8.0 * t247 * t118 + 40.0 / 3.0 * t116 * t251 + 80.0 / 27.0 * t57 * t543;
        let t547 = t110 * t546;
        let t548 = t86 * t547;
        let t556 = t215 * t104;
        let t559 = t215 * t122;
        let t562 = t96 * t230;
        let t565 = t96 * t255;
        let t568 = -4.0 / 3.0 * t85 * t203 * t445 - 4.0 / 3.0 * t85 * t454 + 4.0 / 3.0 * t85 * t316 * t458 - 4.0 / 3.0 * t85 * t467 - 4.0 / 3.0 * t85 * t280 * t471 + 4.0 / 3.0 * t85 * t280 * t476 + 2.0 / 3.0 * t85 * t203 * t481 + 2.0 / 3.0 * t85 * t203 * t485 - 32.0 / 9.0 * t85 * t489 - 32.0 / 9.0 * t85 * t492 * t205 - 32.0 / 9.0 * t85 * t496 - 2.0 / 9.0 * t85 * t529 - 2.0 / 9.0 * t85 * t548 - 2464.0 / 243.0 * t28 * t31 * t506 * t48 * t61 - 176.0 / 27.0 * t85 * t556 - 176.0 / 27.0 * t85 * t559 + 16.0 / 9.0 * t85 * t562 + 16.0 / 9.0 * t85 * t565;
        let t573 = piecewise3(t3, 0.0, -5.0 / 36.0 * t7 * t434 * t66 + t7 * t174 * t126 / 4.0 - 3.0 / 8.0 * t7 * t72 * t259 - 3.0 / 8.0 * t7 * t20 * t568);
        let tv3rho30 = 2.0 * rho[ip] * t573 + 6.0 * t264;
        v3rho3[ip] += tv3rho30;
        let t589 = t61 * t303;
        let t590 = t589 * t103;
        let t594 = t281 * t229;
        let t598 = t109 * t303;
        let t599 = t598 * t121;
        let t603 = t286 * t254;
        let t607 = t450 * t152;
        let t608 = t607 * t237;
        let t612 = t235 * t143;
        let t613 = t612 * t237;
        let t617 = t311 * t198;
        let t621 = t86 * t463;
        let t622 = t281 * t198;
        let t629 = t96 * t48;
        let t633 = t109 * t326;
        let t634 = t633 * t103;
        let t638 = t311 * t229;
        let t642 = t235 * t326;
        let t643 = t642 * t121;
        let t647 = t317 * t254;
        let t651 = 176.0 / 81.0 * t28 * t215 * t134 - 32.0 / 27.0 * t85 * t492 * t287 + 8.0 / 9.0 * t85 * t280 * t590 + 4.0 / 9.0 * t85 * t280 * t594 + 4.0 / 9.0 * t85 * t203 * t599 + 2.0 / 9.0 * t85 * t203 * t603 - 4.0 / 3.0 * t85 * t316 * t608 - 4.0 / 9.0 * t85 * t203 * t613 - 4.0 / 9.0 * t85 * t280 * t617 - 4.0 / 3.0 * t85 * t621 * t622 - 32.0 / 27.0 * t85 * t492 * t312 - 64.0 / 27.0 * t85 * t629 * t318 + 4.0 / 9.0 * t85 * t203 * t634 + 2.0 / 9.0 * t85 * t203 * t638 + 8.0 / 9.0 * t85 * t316 * t643 + 4.0 / 9.0 * t85 * t316 * t647;
        let t652 = t96 * t196;
        let t671 = t22 * t25 * t26 * sigma[ip] * t30;
        let t672 = t35 * t88;
        let t673 = t672 * t235;
        let t674 = t152 * t103;
        let t675 = t674 * t121;
        let t676 = t673 * t675;
        let t679 = t35 * t196;
        let t680 = t679 * t109;
        let t681 = t143 * t103;
        let t682 = t681 * t121;
        let t683 = t680 * t682;
        let t686 = t246 * t149;
        let t692 = -t686 * t58 - 16.0 / 3.0 * t322 * t118 - 40.0 / 9.0 * t150 * t251;
        let t693 = t110 * t692;
        let t694 = t86 * t693;
        let t698 = t22 * t27 * t30;
        let t702 = t215 * t153;
        let t705 = t215 * t144;
        let t708 = t96 * t304;
        let t719 = t511 * t32;
        let t721 = 1.0 / t719 * t523;
        let t725 = 14.0 / 9.0 * t138 * t212 + 74.0 / 9.0 * t36 * t216 - 124.0 / 9.0 * t295 * t222 * t225 * sigma[ip] + 32.0 / 3.0 * t36 * t721 * t148;
        let t726 = t89 * t725;
        let t727 = t86 * t726;
        let t730 = t96 * t327;
        let t733 = -64.0 / 27.0 * t85 * t652 * t282 - 2.0 / 9.0 * t28 * t256 + 32.0 / 27.0 * t28 * t189 + 32.0 / 27.0 * t28 * t192 - 2.0 / 9.0 * t28 * t231 + 4.0 / 9.0 * t28 * t200 + 4.0 / 9.0 * t28 * t239 - 8.0 / 9.0 * t671 * t676 - 8.0 / 9.0 * t671 * t683 - 2.0 / 9.0 * t85 * t694 + 4.0 / 9.0 * t698 * t672 * t205 - 176.0 / 81.0 * t85 * t702 - 176.0 / 81.0 * t85 * t705 + 32.0 / 27.0 * t85 * t708 - 2.0 / 9.0 * t85 * t727 + 32.0 / 27.0 * t85 * t730;
        let t734 = t651 + t733;
        let t739 = piecewise3(t3, 0.0, t7 * t174 * t157 / 12.0 - t7 * t72 * t331 / 4.0 - 3.0 / 8.0 * t7 * t20 * t734);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t739 + 4.0 * t336;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t743 = t210 * param_beta;
        let t748 = t163 * t339 * t25;
        let t753 = t196 * t109;
        let t755 = t345 * t753 * t198;
        let t757 = t345 * t88;
        let t759 = t757 * t444 * t121;
        let t763 = t345 * t346 * t229;
        let t767 = t345 * t451 * t237;
        let t771 = t345 * t236 * t254;
        let t774 = piecewise3(t3, 0.0, -14.0 / 9.0 * t163 * t743 * t168 - 4.0 / 3.0 * t748 * t348 - 8.0 / 3.0 * t748 * t352 - t344 * t755 - 2.0 * t344 * t759 + t344 * t763 / 2.0 - 3.0 * t344 * t767 + t344 * t771);
        let tv3rho2lapl0 = 2.0 * rho[ip] * t774 + 4.0 * t355;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let tv3rho2tau0 = 0.0;
        v3rho2tau[ip] += tv3rho2tau0;
        let t783 = t61 * t386;
        let t784 = t783 * t103;
        let t788 = t109 * t386;
        let t789 = t788 * t121;
        let t793 = t235 * t391;
        let t794 = t793 * t103;
        let t798 = t450 * t391;
        let t799 = t798 * t121;
        let t803 = t317 * t326;
        let t807 = t109 * t402;
        let t808 = t807 * t103;
        let t812 = t235 * t402;
        let t813 = t812 * t121;
        let t817 = t61 * t362;
        let t818 = t817 * t103;
        let t822 = t109 * t362;
        let t823 = t822 * t121;
        let t827 = t281 * t303;
        let t831 = t598 * t152;
        let t835 = t286 * t326;
        let t841 = -32.0 / 27.0 * t85 * t492 * t367 + 4.0 / 9.0 * t85 * t280 * t784 + 2.0 / 9.0 * t85 * t203 * t789 - 4.0 / 9.0 * t85 * t203 * t794 - 4.0 / 3.0 * t85 * t316 * t799 + 8.0 / 9.0 * t85 * t316 * t803 + 2.0 / 9.0 * t85 * t203 * t808 + 4.0 / 9.0 * t85 * t316 * t813 - 4.0 / 3.0 * t85 * t621 * t818 - 4.0 / 9.0 * t85 * t280 * t823 + 8.0 / 9.0 * t85 * t280 * t827 + 4.0 / 9.0 * t85 * t203 * t831 + 4.0 / 9.0 * t85 * t203 * t835 + 32.0 / 27.0 * t28 * t308;
        let t848 = t143 * t152;
        let t849 = t848 * t103;
        let t850 = t680 * t849;
        let t853 = t848 * t121;
        let t854 = t673 * t853;
        let t871 = t29 * t298 * t225;
        let t874 = t511 * rho[ip];
        let t876 = 1.0 / t874 * t523;
        let t880 = t373 * t93 / 3.0 - t376 * t100 / 3.0 + 10.0 / 3.0 * t36 * t871 - 4.0 * t36 * t876 * sigma[ip];
        let t881 = t89 * t880;
        let t882 = t86 * t881;
        let t885 = t96 * t387;
        let t888 = t96 * t392;
        let t891 = t96 * t403;
        let t894 = t96 * t363;
        let t897 = t115 * t398;
        let t902 = 2.0 * t897 * t58 + 16.0 / 3.0 * t399 * t118;
        let t903 = t110 * t902;
        let t904 = t86 * t903;
        let t910 = -4.0 / 9.0 * t28 * t305 - 4.0 / 9.0 * t28 * t328 + 32.0 / 27.0 * t28 * t277 - 8.0 / 9.0 * t671 * t850 - 8.0 / 9.0 * t671 * t854 + 4.0 / 9.0 * t698 * t672 * t287 + 4.0 / 9.0 * t698 * t672 * t312 + 8.0 / 9.0 * t698 * t49 * t318 - 2.0 / 9.0 * t85 * t882 + 16.0 / 27.0 * t85 * t885 - 32.0 / 27.0 * t85 * t888 + 16.0 / 27.0 * t85 * t891 - 32.0 / 27.0 * t85 * t894 - 2.0 / 9.0 * t85 * t904 + 8.0 / 9.0 * t698 * t679 * t282;
        let t911 = t841 + t910;
        let t916 = piecewise3(t3, 0.0, -t7 * t72 * t407 / 8.0 - 3.0 / 8.0 * t7 * t20 * t911);
        let tv3rhosigma20 = 2.0 * rho[ip] * t916 + 2.0 * t411;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t921 = t345 * t196;
        let t923 = t921 * t286 * t103;
        let t926 = t757 * t612 * t121;
        let t929 = t345 * t346 * t303;
        let t935 = t757 * t317 * t103;
        let t937 = t345 * t48;
        let t939 = t937 * t607 * t121;
        let t943 = t345 * t236 * t326;
        let t946 = piecewise3(t3, 0.0, -2.0 / 3.0 * t748 * t414 - t344 * t923 - t344 * t926 + t344 * t929 / 2.0 - 4.0 / 3.0 * t748 * t418 - t344 * t935 - 3.0 * t344 * t939 + t344 * t943);
        let tv3rhosigmalapl0 = 2.0 * rho[ip] * t946 + 2.0 * t421;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let tv3rhosigmatau0 = 0.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let t951 = t235 * t56;
        let t953 = t757 * t951 * t103;
        let t956 = t450 * t56;
        let t958 = t937 * t956 * t121;
        let t962 = piecewise3(t3, 0.0, 2.0 / 3.0 * t344 * t428 + 2.0 * t426 * t953 + 6.0 * t426 * t958);
        let tv3rholapl20 = 2.0 * rho[ip] * t962 + 2.0 * t431;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let tv3rhotau20 = 0.0;
        v3rhotau2[ip] += tv3rhotau20;
        let t965 = t391 * t152;
        let t966 = t451 * t965;
        let t967 = t86 * t966;
        let t970 = t317 * t402;
        let t974 = t362 * t143;
        let t975 = t464 * t974;
        let t976 = t86 * t975;
        let t979 = t822 * t152;
        let t983 = t281 * t386;
        let t987 = t788 * t152;
        let t991 = t286 * t402;
        let t1000 = t148 * t148;
        let t1001 = 1.0 / t1000;
        let t1002 = t55 * t1001;
        let t1005 = -6.0 * t1002 * t58 + 12.0 * t398;
        let t1006 = t110 * t1005;
        let t1007 = t86 * t1006;
        let t1019 = t36 / t37 / t148;
        let t1022 = t36 * t149;
        let t1027 = 1.0 / t511;
        let t1031 = 3.0 / 8.0 * t1019 * t45 - 3.0 / 8.0 * t1022 * t140 - t376 * t383 / 4.0 + 3.0 / 2.0 * t36 * t1027 * t523;
        let t1032 = t89 * t1031;
        let t1033 = t86 * t1032;
        let t1036 = t612 * t391;
        let t1040 = -4.0 / 3.0 * t85 * t967 + 4.0 / 3.0 * t85 * t316 * t970 - 4.0 / 3.0 * t85 * t976 - 4.0 / 3.0 * t85 * t280 * t979 + 4.0 / 3.0 * t85 * t280 * t983 + 2.0 / 3.0 * t85 * t203 * t987 + 2.0 / 3.0 * t85 * t203 * t991 - 2.0 / 3.0 * t28 * t388 - 2.0 / 3.0 * t28 * t404 - 2.0 / 9.0 * t85 * t1007 + 4.0 / 3.0 * t28 * t364 + 4.0 / 3.0 * t698 * t672 * t367 + 4.0 / 3.0 * t28 * t393 - 2.0 / 9.0 * t85 * t1033 - 4.0 / 3.0 * t85 * t203 * t1036;
        let t1044 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t1040);
        let tv3sigma30 = 2.0 * rho[ip] * t1044;
        v3sigma3[ip] += tv3sigma30;
        let t1047 = t345 * t753 * t362;
        let t1050 = t757 * t612 * t152;
        let t1054 = t345 * t346 * t386;
        let t1058 = t345 * t451 * t391;
        let t1062 = t345 * t236 * t402;
        let t1065 = piecewise3(t3, 0.0, -t344 * t1047 - 2.0 * t344 * t1050 + t344 * t1054 / 2.0 - 3.0 * t344 * t1058 + t344 * t1062);
        let tv3sigma2lapl0 = 2.0 * rho[ip] * t1065;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let tv3sigma2tau0 = 0.0;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let t1068 = t757 * t951 * t143;
        let t1072 = t937 * t956 * t152;
        let t1076 = t345 * t236 * t149;
        let t1080 = piecewise3(t3, 0.0, 2.0 * t426 * t1068 + 6.0 * t426 * t1072 + 2.0 * t426 * t1076);
        let tv3sigmalapl20 = 2.0 * rho[ip] * t1080;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let tv3sigmatau20 = 0.0;
        v3sigmatau2[ip] += tv3sigmatau20;
        let t1084 = t163 * t33 * param_beta * t25;
        let t1086 = t345 * t451 * t149;
        let t1089 = piecewise3(t3, 0.0, -12.0 * t1084 * t1086);
        let tv3lapl30 = 2.0 * rho[ip] * t1089;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let tv3tau30 = 0.0;
        v3tau3[ip] += tv3tau30;
        let t1105 = t235 * t198;
        let t1110 = t450 * t103;
        let t1151 = t235 * t229;
        let t1156 = t215 * t88;
        let t1182 = 16.0 / 3.0 * t85 * t280 * t1105 * t237 + 16.0 / 3.0 * t85 * t203 * t1110 * t452 + 16.0 / 9.0 * t85 * t280 * t475 * t527 + 8.0 / 9.0 * t85 * t203 * t109 * t527 * t121 + 4.0 / 3.0 * t85 * t203 * t480 * t254 + 8.0 / 9.0 * t85 * t203 * t204 * t546 - 8.0 * t85 * t316 * t450 * t237 * t254 + 16.0 / 9.0 * t85 * t316 * t457 * t546 + 16.0 / 3.0 * t85 * t621 * t109 * t465 * t121 - 8.0 * t85 * t621 * t61 * t198 * t229 - 8.0 / 3.0 * t85 * t203 * t1151 * t237 + 704.0 / 27.0 * t85 * t1156 * t205 - 64.0 / 9.0 * t85 * t492 * t481 - 64.0 / 9.0 * t85 * t492 * t485 - 128.0 / 9.0 * t85 * t629 * t458 + 128.0 / 9.0 * t85 * t652 * t471 - 128.0 / 9.0 * t85 * t652 * t476 + 128.0 / 9.0 * t85 * t492 * t445 - 8.0 / 3.0 * t85 * t280 * t470 * t254;
        let t1184 = 1.0 / t33 / t296;
        let t1190 = t103 * t121;
        let t1200 = 1.0 / t449 / t60;
        let t1201 = t48 * t1200;
        let t1202 = t237 * t237;
        let t1207 = t254 * t254;
        let t1216 = 1.0 / t462 / t47;
        let t1217 = t1216 * t61;
        let t1218 = t198 * t198;
        let t1242 = t511 * t181;
        let t1254 = 1.0 / t98 / t521 / t97;
        let t1302 = t229 * t229;
        let t1310 = 41888.0 / 729.0 * t28 * t31 * t1184 * t48 * t61 - 16.0 / 3.0 * t671 * t673 * t1190 * t254 - 16.0 / 3.0 * t671 * t680 * t1190 * t229 + 16.0 / 3.0 * t85 * t86 * t1201 * t1202 + 4.0 / 3.0 * t85 * t86 * t236 * t1207 + 128.0 / 9.0 * t85 * t96 * t466 + 16.0 / 3.0 * t85 * t86 * t1217 * t1218 - 352.0 / 27.0 * t85 * t215 * t230 - 352.0 / 27.0 * t85 * t215 * t255 - 2.0 / 9.0 * t85 * t86 * t89 * (3640.0 / 81.0 * t38 * t382 * t44 + 5768.0 / 27.0 * t95 * t30 * t1184 * t99 - 37216.0 / 81.0 * t219 * t29 / t19 / t874 * t225 + 17920.0 / 27.0 * t36 * t397 / t1242 * t523 - 5120.0 / 27.0 * t36 * t1000 / t33 / t511 / t296 * t1254 * t30) - 2.0 / 9.0 * t85 * t86 * t110 * ((20944.0 / 81.0 * t31 * t1184 - 6160.0 / 81.0 * t51 * t506) * t56 * t58 + 32.0 / 3.0 * t537 * t118 + 80.0 / 3.0 * t247 * t251 + 320.0 / 27.0 * t116 * t543 - 80.0 / 81.0 * t57 * t41) + 9856.0 / 243.0 * t85 * t507 * t104 + 9856.0 / 243.0 * t85 * t507 * t122 + 64.0 / 27.0 * t85 * t96 * t547 + 704.0 / 27.0 * t85 * t215 * t199 + 704.0 / 27.0 * t85 * t215 * t238 + 64.0 / 27.0 * t85 * t96 * t528 + 4.0 / 3.0 * t85 * t86 * t197 * t1302 + 128.0 / 9.0 * t85 * t96 * t453;
        let t1316 = piecewise3(t3, 0.0, 10.0 / 27.0 * t7 * t18 * t78 * t66 - 5.0 / 9.0 * t7 * t434 * t126 + t7 * t174 * t259 / 2.0 - t7 * t72 * t568 / 2.0 - 3.0 / 8.0 * t7 * t20 * (t1182 + t1310));
        let tv4rho40 = 2.0 * rho[ip] * t1316 + 8.0 * t573;
        v4rho4[ip] += tv4rho40;
        let t1357 = t235 * t303;
        let t1377 = t450 * t326;
        let t1382 = t96 * t463;
        let t1386 = t86 * t1216;
        let t1391 = t450 * t143;
        let t1406 = -2464.0 / 243.0 * t28 * t507 * t134 - 32.0 / 9.0 * t85 * t629 * t647 + 2.0 / 3.0 * t85 * t203 * t109 * t692 * t103 + 2.0 / 3.0 * t85 * t203 * t633 * t229 + 2.0 / 9.0 * t85 * t203 * t311 * t527 + 4.0 / 3.0 * t85 * t316 * t235 * t692 * t121 + 4.0 / 3.0 * t85 * t316 * t642 * t254 - 4.0 / 3.0 * t85 * t203 * t1357 * t237 - 4.0 / 3.0 * t85 * t280 * t633 * t198 - 4.0 * t85 * t621 * t589 * t198 + 176.0 / 27.0 * t85 * t1156 * t312 + 352.0 / 27.0 * t85 * t215 * t48 * t318 - 4.0 * t85 * t316 * t1377 * t237 + 32.0 / 3.0 * t85 * t1382 * t622 + 16.0 / 3.0 * t85 * t1386 * t281 * t465 + 4.0 / 3.0 * t85 * t203 * t1391 * t452 + 32.0 / 9.0 * t85 * t652 * t617 + 4.0 / 3.0 * t85 * t621 * t311 * t465 + 32.0 / 3.0 * t85 * t629 * t608;
        let t1407 = t1200 * t152;
        let t1428 = t109 * t725;
        let t1473 = t35 * t463;
        let t1474 = t1473 * t61;
        let t1484 = 16.0 / 3.0 * t85 * t316 * t1407 * t452 + 32.0 / 9.0 * t85 * t492 * t613 + 4.0 / 3.0 * t85 * t280 * t61 * t725 * t103 + 4.0 / 3.0 * t85 * t280 * t589 * t229 + 4.0 / 9.0 * t85 * t280 * t281 * t527 + 2.0 / 3.0 * t85 * t203 * t1428 * t121 + 2.0 / 3.0 * t85 * t203 * t598 * t254 + 2.0 / 9.0 * t85 * t203 * t286 * t546 + 4.0 / 9.0 * t85 * t316 * t317 * t546 + 352.0 / 27.0 * t85 * t215 * t196 * t282 + 176.0 / 27.0 * t85 * t1156 * t287 - 64.0 / 9.0 * t85 * t652 * t590 - 32.0 / 9.0 * t85 * t652 * t594 - 32.0 / 9.0 * t85 * t492 * t599 - 16.0 / 9.0 * t85 * t492 * t603 - 32.0 / 9.0 * t85 * t492 * t634 - 16.0 / 9.0 * t85 * t492 * t638 - 64.0 / 9.0 * t85 * t629 * t643 - 4.0 * t671 * t1474 * t681 * t229 - 8.0 / 3.0 * t671 * t680 * t303 * t103 * t121;
        let t1495 = t679 * t235;
        let t1501 = t143 * t121;
        let t1529 = t672 * t450;
        let t1535 = t49 * t450;
        let t1536 = t152 * t121;
        let t1541 = t1473 * t109;
        let t1547 = t78 * t196;
        let t1548 = t1547 * t109;
        let t1552 = t78 * t88;
        let t1553 = t1552 * t235;
        let t1572 = -4.0 / 3.0 * t671 * t680 * t143 * t229 * t121 - 4.0 / 3.0 * t671 * t680 * t681 * t254 + 8.0 / 3.0 * t671 * t1495 * t143 * t237 * t103 - 4.0 / 3.0 * t671 * t673 * t1501 * t254 + 8.0 / 3.0 * t671 * t1495 * t152 * t198 * t121 - 4.0 / 3.0 * t671 * t680 * t674 * t229 - 8.0 / 3.0 * t671 * t673 * t326 * t103 * t121 - 4.0 / 3.0 * t671 * t673 * t152 * t229 * t121 - 4.0 / 3.0 * t671 * t673 * t674 * t254 + 4.0 * t671 * t1529 * t152 * t237 * t103 - 4.0 * t671 * t1535 * t1536 * t254 + 4.0 * t671 * t1541 * t143 * t198 * t121 + 64.0 / 9.0 * t671 * t1548 * t682 + 64.0 / 9.0 * t671 * t1553 * t675 + 2464.0 / 243.0 * t85 * t507 * t144 + 2464.0 / 243.0 * t85 * t507 * t153 - 4.0 / 3.0 * t698 * t672 * t445 - 4.0 / 3.0 * t698 * t679 * t471 - 176.0 / 27.0 * t85 * t215 * t304;
        let t1655 = 16.0 / 9.0 * t85 * t96 * t726 - 2.0 / 9.0 * t85 * t86 * t89 * (-140.0 / 27.0 * t138 * t502 - 364.0 / 9.0 * t36 * t508 + 3320.0 / 27.0 * t295 * t513 * t225 * sigma[ip] - 1952.0 / 9.0 * t36 * t519 * t523 * t148 + 640.0 / 9.0 * t36 / t33 / t511 / t379 * t1254 * t397 * t30) + 16.0 / 9.0 * t85 * t96 * t693 - 2.0 / 9.0 * t85 * t86 * t110 * (-t536 * t149 * t58 - 8.0 * t686 * t118 - 40.0 / 3.0 * t322 * t251 - 80.0 / 27.0 * t150 * t543) - 32.0 / 9.0 * t698 * t1552 * t205 + 4.0 / 3.0 * t698 * t679 * t476 + 2.0 / 3.0 * t698 * t672 * t481 - 176.0 / 27.0 * t85 * t215 * t327 + 2.0 / 3.0 * t698 * t672 * t485 + 4.0 / 3.0 * t698 * t49 * t458 - 4.0 / 3.0 * t28 * t467 - 4.0 / 3.0 * t28 * t454 - 32.0 / 9.0 * t28 * t489 - 32.0 / 9.0 * t28 * t496 + 16.0 / 9.0 * t28 * t565 - 2.0 / 9.0 * t28 * t548 - 176.0 / 27.0 * t28 * t556 - 176.0 / 27.0 * t28 * t559 + 16.0 / 9.0 * t28 * t562 - 2.0 / 9.0 * t28 * t529;
        let t1662 = piecewise3(t3, 0.0, -5.0 / 36.0 * t7 * t434 * t157 + t7 * t174 * t331 / 4.0 - 3.0 / 8.0 * t7 * t72 * t734 - 3.0 / 8.0 * t7 * t20 * (t1406 + t1484 + t1572 + t1655));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t1662 + 6.0 * t739;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t1689 = t463 * t109;
        let t1718 = t163 * t743 * t25;
        let t1727 = -3.0 * t344 * t757 * t1151 * t121 - 3.0 * t344 * t757 * t444 * t254 + 12.0 * t344 * t345 * t1201 * t452 - 9.0 * t344 * t937 * t450 * t121 * t254 + 4.0 * t748 * t755 + 8.0 * t748 * t759 + 12.0 * t748 * t767 + 3.0 * t344 * t345 * t1689 * t465 + 6.0 * t344 * t921 * t1105 * t121 - 3.0 * t344 * t921 * t204 * t229 - 2.0 * t748 * t763 - 4.0 * t748 * t771 + t344 * t345 * t346 * t527 / 2.0 + t344 * t345 * t236 * t546 + 140.0 / 27.0 * t163 * t500 * param_beta * t168 + 14.0 / 3.0 * t1718 * t348 + 28.0 / 3.0 * t1718 * t352 + 9.0 * t344 * t757 * t1110 * t237;
        let t1728 = piecewise3(t3, 0.0, t1727);
        let tv4rho3lapl0 = 2.0 * rho[ip] * t1728 + 6.0 * t774;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let tv4rho3tau0 = 0.0;
        v4rho3tau[ip] += tv4rho3tau0;
        let t1742 = t235 * t362;
        let t1751 = t450 * t402;
        let t1756 = t1200 * t391;
        let t1765 = t235 * t386;
        let t1784 = 16.0 / 3.0 * t85 * t1386 * t817 * t198 + 8.0 / 9.0 * t85 * t280 * t1742 * t237 - 4.0 / 9.0 * t85 * t280 * t807 * t198 - 4.0 / 3.0 * t85 * t316 * t1751 * t237 + 16.0 / 3.0 * t85 * t316 * t1756 * t237 - 4.0 / 3.0 * t85 * t621 * t783 * t198 - 4.0 / 9.0 * t85 * t203 * t1765 * t237 + 8.0 / 9.0 * t85 * t280 * t793 * t198 + 352.0 / 81.0 * t85 * t1156 * t367 + 8.0 / 9.0 * t85 * t316 * t317 * t692 - 64.0 / 27.0 * t85 * t492 * t831;
        let t1809 = t109 * t880;
        let t1826 = -64.0 / 27.0 * t85 * t492 * t835 - 64.0 / 27.0 * t85 * t652 * t784 - 32.0 / 27.0 * t85 * t492 * t789 + 4.0 / 9.0 * t85 * t280 * t783 * t229 + 2.0 / 9.0 * t85 * t203 * t788 * t254 - 16.0 / 9.0 * t698 * t676 + 8.0 / 9.0 * t85 * t280 * t61 * t880 * t103 + 4.0 / 9.0 * t85 * t203 * t1809 * t121 - 16.0 / 9.0 * t698 * t683 + 4.0 / 9.0 * t85 * t203 * t109 * t902 * t103 + 8.0 / 9.0 * t85 * t316 * t235 * t902 * t121;
        let t1868 = -128.0 / 27.0 * t85 * t652 * t827 - 4.0 / 3.0 * t85 * t621 * t817 * t229 - 4.0 / 9.0 * t85 * t280 * t822 * t254 + 8.0 / 9.0 * t85 * t280 * t281 * t725 + 4.0 / 9.0 * t85 * t203 * t1428 * t152 + 8.0 / 9.0 * t85 * t203 * t598 * t326 - 32.0 / 27.0 * t85 * t492 * t808 - 64.0 / 27.0 * t85 * t629 * t813 + 2.0 / 9.0 * t85 * t203 * t807 * t229 + 4.0 / 9.0 * t85 * t316 * t812 * t254 + 64.0 / 9.0 * t85 * t1382 * t818;
        let t1914 = 64.0 / 27.0 * t85 * t652 * t823 + 64.0 / 27.0 * t85 * t492 * t794 + 64.0 / 9.0 * t85 * t629 * t799 - 128.0 / 27.0 * t85 * t629 * t803 - 4.0 / 9.0 * t85 * t203 * t793 * t229 - 4.0 / 3.0 * t85 * t316 * t798 * t254 + 4.0 / 9.0 * t85 * t203 * t286 * t692 + 32.0 / 9.0 * t671 * t1495 * t848 * t1190 + 8.0 / 3.0 * t671 * t1541 * t848 * t198 + 8.0 / 3.0 * t671 * t1529 * t848 * t237 + 8.0 / 3.0 * t671 * t1541 * t362 * t103 * t121 - 16.0 / 3.0 * t671 * t1474 * t681 * t303;
        let t1978 = -16.0 / 9.0 * t671 * t680 * t1501 * t303 + 128.0 / 27.0 * t671 * t1553 * t853 - 2.0 / 9.0 * t85 * t86 * t89 * (-7.0 / 9.0 * t373 * t212 + 7.0 / 9.0 * t376 * t216 - 22.0 * t36 * t226 + 188.0 / 3.0 * t36 * t721 * sigma[ip] - 80.0 / 3.0 * t36 / t33 / t1242 * t1254 * t148 * t30) - 2.0 / 9.0 * t85 * t86 * t110 * (2.0 * t246 * t398 * t58 + 32.0 / 3.0 * t897 * t118 + 80.0 / 9.0 * t399 * t251) + 16.0 / 9.0 * t698 * t679 * t590 + 8.0 / 9.0 * t698 * t679 * t594 + 8.0 / 9.0 * t698 * t672 * t599 + 4.0 / 9.0 * t698 * t672 * t603 - 176.0 / 81.0 * t85 * t215 * t403 + 352.0 / 81.0 * t85 * t215 * t363 + 352.0 / 81.0 * t85 * t215 * t392;
        let t1979 = t326 * t326;
        let t2017 = 8.0 / 9.0 * t85 * t86 * t236 * t1979 - 176.0 / 81.0 * t85 * t215 * t387 - 8.0 / 3.0 * t698 * t49 * t608 + 32.0 / 27.0 * t85 * t96 * t881 - 128.0 / 27.0 * t698 * t1547 * t282 - 8.0 / 3.0 * t698 * t1473 * t622 - 64.0 / 27.0 * t698 * t1552 * t287 - 8.0 / 9.0 * t698 * t672 * t613 - 8.0 / 9.0 * t698 * t679 * t617 + 32.0 / 27.0 * t85 * t96 * t903 - 64.0 / 27.0 * t698 * t1552 * t312 - 128.0 / 27.0 * t698 * t79 * t318;
        let t2031 = t303 * t303;
        let t2048 = 8.0 / 9.0 * t698 * t672 * t634 + 4.0 / 9.0 * t698 * t672 * t638 + 16.0 / 9.0 * t698 * t49 * t643 + 8.0 / 9.0 * t698 * t49 * t647 + 8.0 / 9.0 * t85 * t86 * t197 * t2031 - 4.0 / 9.0 * t28 * t694 + 64.0 / 27.0 * t28 * t730 - 4.0 / 9.0 * t28 * t727 - 352.0 / 81.0 * t28 * t705 + 64.0 / 27.0 * t28 * t708 - 352.0 / 81.0 * t28 * t702;
        let t2072 = t303 * t152;
        let t2077 = t143 * t326;
        let t2101 = -8.0 / 9.0 * t671 * t673 * t402 * t103 * t121 - 16.0 / 3.0 * t671 * t1535 * t1536 * t326 - 8.0 / 9.0 * t671 * t680 * t386 * t103 * t121 + 8.0 / 3.0 * t671 * t1529 * t391 * t103 * t121 - 16.0 / 9.0 * t671 * t673 * t674 * t326 - 16.0 / 9.0 * t671 * t680 * t2072 * t103 - 16.0 / 9.0 * t671 * t680 * t2077 * t103 - 8.0 / 9.0 * t671 * t680 * t848 * t229 - 16.0 / 9.0 * t671 * t673 * t2072 * t121 - 16.0 / 9.0 * t671 * t673 * t2077 * t121 - 8.0 / 9.0 * t671 * t673 * t848 * t254 + 128.0 / 27.0 * t671 * t1548 * t849;
        let t2109 = piecewise3(t3, 0.0, t7 * t174 * t407 / 12.0 - t7 * t72 * t911 / 4.0 - 3.0 / 8.0 * t7 * t20 * (t1784 + t1826 + t1868 + t1914 + t1978 + t2017 + t2048 + t2101));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t2109 + 4.0 * t916;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t2130 = param_beta * t25 * t26;
        let t2131 = t163 * t40 * t2130;
        let t2133 = t30 * t196 * t235;
        let t2146 = t30 * t88 * t450;
        let t2150 = t345 * t463;
        let t2159 = -4.0 / 3.0 * t748 * t929 + t344 * t345 * t346 * t725 / 2.0 - 8.0 / 3.0 * t748 * t943 + t344 * t345 * t236 * t692 + 12.0 * t344 * t937 * t1407 * t237 + 4.0 * t2131 * t2133 * t682 + 3.0 * t344 * t757 * t1391 * t237 + 2.0 * t344 * t921 * t317 * t198 + 6.0 * t2131 * t2146 * t675 + 3.0 * t344 * t2150 * t286 * t198 + 28.0 / 9.0 * t1718 * t418 + 8.0 / 3.0 * t748 * t935;
        let t2197 = 8.0 * t748 * t939 - 2.0 * t344 * t757 * t642 * t103 - t344 * t757 * t317 * t229 - 6.0 * t344 * t937 * t1377 * t121 - 3.0 * t344 * t937 * t607 * t254 + 8.0 / 3.0 * t748 * t923 + 8.0 / 3.0 * t748 * t926 - 2.0 * t344 * t921 * t598 * t103 - t344 * t921 * t286 * t229 - 2.0 * t344 * t757 * t1357 * t121 - t344 * t757 * t612 * t254 + 14.0 / 9.0 * t1718 * t414;
        let t2199 = piecewise3(t3, 0.0, t2159 + t2197);
        let tv4rho2sigmalapl0 = 2.0 * rho[ip] * t2199 + 4.0 * t946;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let tv4rho2sigmatau0 = 0.0;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let t2214 = t163 * t423 * t2130;
        let t2224 = t1200 * t56;
        let t2234 = piecewise3(t3, 0.0, -8.0 / 9.0 * t748 * t428 - 4.0 / 3.0 * t344 * t953 - 4.0 * t344 * t958 - 4.0 * t426 * t921 * t951 * t198 - 12.0 * t2214 * t2146 * t56 * t103 * t121 + 2.0 * t426 * t757 * t951 * t229 - 24.0 * t426 * t937 * t2224 * t237 + 6.0 * t426 * t937 * t956 * t254);
        let tv4rho2lapl20 = 2.0 * rho[ip] * t2234 + 4.0 * t962;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let tv4rho2lapltau0 = 0.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let tv4rho2tau20 = 0.0;
        v4rho2tau2[ip] += tv4rho2tau20;
        let t2245 = t109 * t1031;
        let t2259 = t152 * t402;
        let t2264 = t143 * t391;
        let t2277 = t386 * t152;
        let t2282 = t143 * t402;
        let t2291 = t362 * t152;
        let t2304 = t143 * t386;
        let t2313 = 4.0 / 9.0 * t85 * t280 * t61 * t1031 * t103 + 2.0 / 9.0 * t85 * t203 * t2245 * t121 - 4.0 / 3.0 * t85 * t203 * t1357 * t391 + 2.0 / 9.0 * t85 * t203 * t109 * t1005 * t103 - 4.0 * t671 * t1535 * t2259 * t121 + 8.0 / 3.0 * t671 * t1495 * t2264 * t103 + 4.0 * t671 * t1529 * t2264 * t121 - 8.0 / 3.0 * t671 * t673 * t848 * t326 - 4.0 / 3.0 * t671 * t673 * t2277 * t121 - 4.0 / 3.0 * t671 * t680 * t2282 * t103 - 4.0 / 3.0 * t671 * t673 * t2282 * t121 + 4.0 * t671 * t1541 * t2291 * t103 + 8.0 / 3.0 * t671 * t1495 * t2291 * t121 - 8.0 / 3.0 * t671 * t680 * t848 * t303 - 4.0 * t671 * t1474 * t2304 * t103 - 4.0 / 3.0 * t671 * t680 * t2304 * t121;
        let t2353 = t109 * t974;
        let t2375 = -4.0 / 3.0 * t671 * t680 * t2277 * t103 - 4.0 / 3.0 * t671 * t673 * t2259 * t103 + 32.0 / 9.0 * t85 * t492 * t1036 - 16.0 / 9.0 * t85 * t492 * t991 + 32.0 / 9.0 * t85 * t652 * t979 - 32.0 / 9.0 * t85 * t652 * t983 - 16.0 / 9.0 * t85 * t492 * t987 - 32.0 / 9.0 * t85 * t629 * t970 + 4.0 / 3.0 * t85 * t316 * t642 * t402 + 4.0 / 3.0 * t85 * t316 * t317 * t902 + 16.0 / 3.0 * t85 * t1386 * t61 * t974 * t103 + 4.0 / 3.0 * t85 * t621 * t2353 * t121 - 4.0 * t85 * t621 * t817 * t303 - 4.0 / 3.0 * t85 * t280 * t822 * t326 + 4.0 / 3.0 * t698 * t679 * t784 + 2.0 / 3.0 * t698 * t672 * t789 + 2.0 / 3.0 * t698 * t672 * t808;
        let t2451 = 4.0 / 3.0 * t698 * t49 * t813 + 16.0 / 27.0 * t85 * t96 * t1032 + 16.0 / 27.0 * t85 * t96 * t1006 - 32.0 / 9.0 * t698 * t1552 * t367 + 32.0 / 9.0 * t85 * t96 * t966 + 32.0 / 9.0 * t85 * t96 * t975 - 2.0 / 9.0 * t85 * t86 * t89 * (-t1019 * t93 / 2.0 + t1022 * t100 / 2.0 + t376 * t871 / 3.0 - 14.0 * t36 * t876 + 10.0 * t36 / t33 / t518 * t1254 * sigma[ip] * t30) - 2.0 / 9.0 * t85 * t86 * t110 * (-6.0 * t115 * t1001 * t58 - 16.0 * t1002 * t118) - 4.0 * t698 * t1473 * t818 - 4.0 / 3.0 * t698 * t679 * t823 + 8.0 / 3.0 * t698 * t679 * t827 + 4.0 / 3.0 * t698 * t672 * t831 + 4.0 / 3.0 * t698 * t672 * t835 - 4.0 / 3.0 * t698 * t672 * t794 - 4.0 * t698 * t49 * t799 + 8.0 / 3.0 * t698 * t49 * t803 - 2.0 / 3.0 * t28 * t882;
        let t2495 = t450 * t965;
        let t2509 = -2.0 / 3.0 * t28 * t904 - 32.0 / 9.0 * t28 * t894 - 32.0 / 9.0 * t28 * t888 + 16.0 / 9.0 * t28 * t885 + 16.0 / 9.0 * t28 * t891 + 4.0 / 9.0 * t85 * t316 * t235 * t1005 * t121 - 8.0 / 3.0 * t698 * t850 - 8.0 / 3.0 * t698 * t854 + 4.0 / 3.0 * t85 * t280 * t589 * t386 + 4.0 / 3.0 * t85 * t280 * t281 * t880 + 2.0 / 3.0 * t85 * t203 * t1809 * t152 + 2.0 / 3.0 * t85 * t203 * t788 * t326 + 2.0 / 3.0 * t85 * t203 * t598 * t402 + 2.0 / 3.0 * t85 * t203 * t286 * t902 + 4.0 / 3.0 * t85 * t203 * t2495 * t103 + 16.0 / 3.0 * t85 * t316 * t1200 * t965 * t121 - 4.0 * t85 * t316 * t798 * t326;
        let t2516 = piecewise3(t3, 0.0, -t7 * t72 * t1040 / 8.0 - 3.0 / 8.0 * t7 * t20 * (t2313 + t2375 + t2451 + t2509));
        let tv4rhosigma30 = 2.0 * rho[ip] * t2516 + 2.0 * t1044;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t2588 = -t344 * t921 * t788 * t103 - t344 * t757 * t1765 * t121 + t344 * t345 * t346 * t880 / 2.0 + 4.0 * t748 * t1058 + 3.0 * t344 * t757 * t798 * t103 + 12.0 * t344 * t937 * t1756 * t121 - 6.0 * t344 * t937 * t607 * t326 - 4.0 / 3.0 * t748 * t1062 - t344 * t757 * t812 * t103 - 3.0 * t344 * t937 * t1751 * t121 + t344 * t345 * t236 * t902;
        let t2590 = piecewise3(t3, 0.0, 4.0 / 3.0 * t748 * t1047 + 3.0 * t344 * t2150 * t822 * t103 + 2.0 * t344 * t921 * t1742 * t121 - 2.0 * t344 * t921 * t286 * t303 + 8.0 / 3.0 * t748 * t1050 + 4.0 * t2131 * t2133 * t849 + 6.0 * t2131 * t2146 * t853 - 2.0 * t344 * t757 * t1357 * t152 - 2.0 * t344 * t757 * t612 * t326 - 2.0 / 3.0 * t748 * t1054 + t2588);
        let tv4rhosigma2lapl0 = 2.0 * rho[ip] * t2590 + 2.0 * t1065;
        v4rhosigma2lapl[ip] += tv4rhosigma2lapl0;
        let tv4rhosigma2tau0 = 0.0;
        v4rhosigma2tau[ip] += tv4rhosigma2tau0;
        let t2595 = t56 * t143;
        let t2610 = t56 * t152;
        let t2626 = t235 * t149;
        let t2631 = t450 * t149;
        let t2636 = -2.0 / 3.0 * t344 * t1068 - 4.0 * t2214 * t2133 * t2595 * t103 - 6.0 * t2214 * t2146 * t2595 * t121 + 2.0 * t426 * t757 * t951 * t303 - 2.0 * t344 * t1072 - 6.0 * t2214 * t2146 * t2610 * t103 - 24.0 * t2214 * t166 * t1200 * t2610 * t121 + 6.0 * t426 * t937 * t956 * t326 - 2.0 / 3.0 * t344 * t1076 - 2.0 * t426 * t757 * t2626 * t103 - 6.0 * t426 * t937 * t2631 * t121;
        let t2637 = piecewise3(t3, 0.0, t2636);
        let tv4rhosigmalapl20 = 2.0 * rho[ip] * t2637 + 2.0 * t1080;
        v4rhosigmalapl2[ip] += tv4rhosigmalapl20;
        let tv4rhosigmalapltau0 = 0.0;
        v4rhosigmalapltau[ip] += tv4rhosigmalapltau0;
        let tv4rhosigmatau20 = 0.0;
        v4rhosigmatau2[ip] += tv4rhosigmatau20;
        let t2646 = t1200 * t149;
        let t2652 = piecewise3(t3, 0.0, 12.0 * t1084 * t757 * t2631 * t103 + 48.0 * t1084 * t937 * t2646 * t121 - 8.0 * t426 * t1086);
        let tv4rholapl30 = 2.0 * rho[ip] * t2652 + 2.0 * t1089;
        v4rholapl3[ip] += tv4rholapl30;
        let tv4rholapl2tau0 = 0.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let tv4rholapltau20 = 0.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let tv4rhotau30 = 0.0;
        v4rhotau3[ip] += tv4rhotau30;
        let t2711 = t386 * t386;
        let t2716 = -16.0 / 3.0 * t671 * t680 * t848 * t386 - 16.0 / 3.0 * t671 * t673 * t2259 * t143 + 16.0 / 9.0 * t85 * t316 * t317 * t1005 - 8.0 / 3.0 * t85 * t203 * t1765 * t391 + 16.0 / 3.0 * t85 * t280 * t1742 * t391 + 8.0 / 9.0 * t85 * t203 * t286 * t1005 + 16.0 / 3.0 * t85 * t621 * t2353 * t152 - 8.0 * t85 * t621 * t817 * t386 - 8.0 / 3.0 * t85 * t280 * t822 * t402 + 16.0 / 9.0 * t85 * t280 * t281 * t1031 + 8.0 / 9.0 * t85 * t203 * t2245 * t152 + 4.0 / 3.0 * t85 * t203 * t788 * t402 + 16.0 / 3.0 * t85 * t203 * t2495 * t143 - 8.0 * t85 * t316 * t798 * t402 + 4.0 / 3.0 * t85 * t86 * t197 * t2711;
        let t2756 = t391 * t391;
        let t2764 = t402 * t402;
        let t2775 = t362 * t362;
        let t2794 = 8.0 / 3.0 * t698 * t672 * t987 - 2.0 / 9.0 * t85 * t86 * t89 * (-15.0 / 16.0 * t36 / t37 / t397 * t45 + 15.0 / 16.0 * t36 * t398 * t140 + 5.0 / 8.0 * t1022 * t383 + 3.0 / 4.0 * t36 * t56 * t1027 * t523 - 15.0 / 4.0 * t36 / t33 / t719 * t1254 * t30) - 2.0 / 9.0 * t85 * t86 * t110 * (-48.0 * t1001 + 24.0 * t55 / t1000 / sigma[ip] * t58) + 16.0 / 3.0 * t85 * t86 * t1201 * t2756 + 16.0 / 3.0 * t698 * t49 * t970 + 4.0 / 3.0 * t85 * t86 * t236 * t2764 - 16.0 / 3.0 * t698 * t672 * t1036 + 8.0 / 3.0 * t698 * t672 * t991 + 16.0 / 3.0 * t85 * t86 * t1217 * t2775 - 16.0 / 3.0 * t698 * t679 * t979 + 16.0 / 3.0 * t698 * t679 * t983 - 16.0 / 3.0 * t28 * t976 - 8.0 / 9.0 * t28 * t1033 - 8.0 / 9.0 * t28 * t1007 - 16.0 / 3.0 * t28 * t967;
        let t2799 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * (t2716 + t2794));
        let tv4sigma40 = 2.0 * rho[ip] * t2799;
        v4sigma4[ip] += tv4sigma40;
        let t2841 = piecewise3(t3, 0.0, 3.0 * t344 * t345 * t1689 * t974 + 6.0 * t344 * t921 * t1742 * t152 - 3.0 * t344 * t921 * t286 * t386 + 9.0 * t344 * t757 * t1391 * t391 - 3.0 * t344 * t757 * t1765 * t152 - 3.0 * t344 * t757 * t612 * t402 + t344 * t345 * t346 * t1031 / 2.0 + 12.0 * t344 * t345 * t1201 * t965 - 9.0 * t344 * t937 * t607 * t402 + t344 * t345 * t236 * t1005);
        let tv4sigma3lapl0 = 2.0 * rho[ip] * t2841;
        v4sigma3lapl[ip] += tv4sigma3lapl0;
        let tv4sigma3tau0 = 0.0;
        v4sigma3tau[ip] += tv4sigma3tau0;
        let t2876 = piecewise3(t3, 0.0, -4.0 * t426 * t757 * t2626 * t143 - 12.0 * t2214 * t2146 * t2595 * t152 - 12.0 * t426 * t937 * t2631 * t152 - 24.0 * t426 * t937 * t2224 * t391 - 4.0 * t426 * t345 * t236 * t398 - 4.0 * t426 * t921 * t951 * t362 + 2.0 * t426 * t757 * t951 * t386 + 6.0 * t426 * t937 * t956 * t402);
        let tv4sigma2lapl20 = 2.0 * rho[ip] * t2876;
        v4sigma2lapl2[ip] += tv4sigma2lapl20;
        let tv4sigma2lapltau0 = 0.0;
        v4sigma2lapltau[ip] += tv4sigma2lapltau0;
        let tv4sigma2tau20 = 0.0;
        v4sigma2tau2[ip] += tv4sigma2tau20;
        let t2891 = piecewise3(t3, 0.0, 12.0 * t1084 * t757 * t2631 * t143 + 48.0 * t1084 * t937 * t2646 * t152 + 24.0 * t1084 * t345 * t451 * t398);
        let tv4sigmalapl30 = 2.0 * rho[ip] * t2891;
        v4sigmalapl3[ip] += tv4sigmalapl30;
        let tv4sigmalapl2tau0 = 0.0;
        v4sigmalapl2tau[ip] += tv4sigmalapl2tau0;
        let tv4sigmalapltau20 = 0.0;
        v4sigmalapltau2[ip] += tv4sigmalapltau20;
        let tv4sigmatau30 = 0.0;
        v4sigmatau3[ip] += tv4sigmatau30;
        let t2900 = piecewise3(t3, 0.0, -96.0 * t163 * t52 * param_beta * t25 * t345 * t1201 * t398);
        let tv4lapl40 = 2.0 * rho[ip] * t2900;
        v4lapl4[ip] += tv4lapl40;
        let tv4lapl3tau0 = 0.0;
        v4lapl3tau[ip] += tv4lapl3tau0;
        let tv4lapl2tau20 = 0.0;
        v4lapl2tau2[ip] += tv4lapl2tau20;
        let tv4lapltau30 = 0.0;
        v4lapltau3[ip] += tv4lapltau30;
        let tv4tau40 = 0.0;
        v4tau4[ip] += tv4tau40;
    }
}
