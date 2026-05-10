//! MGGA_X_EDMGGA fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 58 shared lines across all orders.
//! Delta: 196 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_edmgga_fxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (58 lines) ---
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
        let t21 = M_CBRT4;
        let t22 = t4 * t4;
        let t24 = M_PI * M_PI;
        let t25 = pow_1_3(t24);
        let t27 = t21 * t22 * t25 / 9.0;
        let t28 = 1.0 - t27;
        let t29 = M_CBRT2;
        let t30 = t29 * t29;
        let t31 = tau[ip] * t30;
        let t32 = t19 * t19;
        let t34 = 1.0 / t32 / rho[ip];
        let t36 = sigma[ip] * t30;
        let t37 = rho[ip] * rho[ip];
        let t39 = 1.0 / t32 / t37;
        let t42 = lapl[ip] * t30;
        let t46 = M_CBRT6;
        let t48 = t25 * t25;
        let t49 = 1.0 / t48;
        let t50 = (t31 * t34 - t36 * t39 / 8.0 - t42 * t34 / 4.0) * t46 * t49;
        let t51 = 5.0 / 9.0 * t50;
        let t52 = -t51 < -0.14205545454545454545e5;
        let t53 = 0.39111111111111111111e0 * t50;
        let t55 = 0.0 < 0.70414204545454545455e0 - t53;
        let t57 = piecewise3(t55, -0.14204545454545454545e-3, 0.704e0 - t53);
        let t60 = t57 * t57;
        let t61 = t60 * t57;
        let t62 = 1.0 / t61;
        let t65 = 1.0 - t51;
        let t66 = t65 * t65;
        let t68 = 1.0 + 0.495616e0 * t66;
        let t69 = f64::sqrt(t68);
        let t71 = piecewise3(t52, -1.0 / t57 / 2.0 + t62 / 8.0, 0.704e0 - t53 + t69);
        let t72 = t28 * t71;
        let t73 = f64::sqrt(30.0);
        let t74 = t28 * t73;
        let t75 = f64::sqrt(t71);
        let t76 = t28 * t28;
        let t81 = 0.60184783083548636238e0 * t76 - 0.206514e-1;
        let t82 = t71 - 1.0;
        let t86 = f64::ln(0.39102932048925120047e0 / t76 / t28 * t73 * t81 * t82 + f64::sqrt(pow_2(0.39102932048925120047e0 / t76 / t28 * t73 * t81 * t82) + 1.0));
        let t90 = 1.0 + 0.14163895778062926267e0 * t74 * t75 * t86;
        let t91 = 1.0 / t90;
        let t93 = t72 * t91 + t27;
        let t97 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t93);
        let tzk0 = 2.0 * t97;
        zk[ip] += tzk0;
        // --- vxc delta (73 lines) ---
        let t99 = t18 / t32;
        let t103 = 1.0 / t60;
        let t106 = t37 * rho[ip];
        let t108 = 1.0 / t32 / t106;
        let t113 = -5.0 / 3.0 * t31 * t39 + t36 * t108 / 3.0 + 5.0 / 12.0 * t42 * t39;
        let t115 = t113 * t46 * t49;
        let t116 = 0.39111111111111111111e0 * t115;
        let t117 = piecewise3(t55, 0.0, -t116);
        let t120 = t60 * t60;
        let t121 = 1.0 / t120;
        let t125 = 1.0 / t69;
        let t126 = t125 * t65;
        let t130 = piecewise3(t52, t103 * t117 / 2.0 - 3.0 / 8.0 * t121 * t117, -t116 - 0.27534222222222222222e0 * t126 * t115);
        let t131 = t28 * t130;
        let t133 = t90 * t90;
        let t134 = 1.0 / t133;
        let t135 = 1.0 / t75;
        let t136 = t135 * t86;
        let t140 = 1.0 / t76;
        let t141 = t140 * t75;
        let t142 = t81 * t130;
        let t143 = t76 * t76;
        let t144 = t143 * t76;
        let t146 = t81 * t81;
        let t148 = t82 * t82;
        let t151 = 0.4587117884468565861e1 / t144 * t146 * t148 + 1.0;
        let t152 = f64::sqrt(t151);
        let t153 = 1.0 / t152;
        let t157 = 0.70819478890314631335e-1 * t74 * t136 * t130 + 0.16615495624729559964e1 * t141 * t142 * t153;
        let t158 = t134 * t157;
        let t160 = t131 * t91 - t72 * t158;
        let t165 = piecewise3(t3, 0.0, -t7 * t99 * t93 / 8.0 - 3.0 / 8.0 * t7 * t20 * t160);
        let tvrho0 = 2.0 * rho[ip] * t165 + 2.0 * t97;
        vrho[ip] += tvrho0;
        let t168 = t30 * t39;
        let t169 = t46 * t49;
        let t170 = t168 * t169;
        let t171 = 0.48888888888888888889e-1 * t170;
        let t172 = piecewise3(t55, 0.0, t171);
        let t175 = t121 * t172;
        let t178 = t126 * t30;
        let t180 = t39 * t46 * t49;
        let t181 = t178 * t180;
        let t184 = piecewise3(t52, t103 * t172 / 2.0 - 3.0 / 8.0 * t175, t171 + 0.34417777777777777778e-1 * t181);
        let t185 = t28 * t184;
        let t194 = 0.70819478890314631335e-1 * t74 * t136 * t184 + 0.16615495624729559964e1 * t141 * t81 * t184 * t153;
        let t195 = t134 * t194;
        let t197 = t185 * t91 - t72 * t195;
        let t201 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t197);
        let tvsigma0 = 2.0 * rho[ip] * t201;
        vsigma[ip] += tvsigma0;
        let t203 = t30 * t34;
        let t204 = t203 * t169;
        let t205 = 0.97777777777777777778e-1 * t204;
        let t206 = piecewise3(t55, 0.0, t205);
        let t209 = t121 * t206;
        let t213 = t34 * t46 * t49;
        let t214 = t178 * t213;
        let t217 = piecewise3(t52, t103 * t206 / 2.0 - 3.0 / 8.0 * t209, t205 + 0.68835555555555555555e-1 * t214);
        let t218 = t28 * t217;
        let t227 = 0.70819478890314631335e-1 * t74 * t136 * t217 + 0.16615495624729559964e1 * t141 * t81 * t217 * t153;
        let t228 = t134 * t227;
        let t230 = t218 * t91 - t72 * t228;
        let t234 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t230);
        let tvlapl0 = 2.0 * rho[ip] * t234;
        vlapl[ip] += tvlapl0;
        let t236 = 0.39111111111111111111e0 * t204;
        let t237 = piecewise3(t55, 0.0, -t236);
        let t240 = t121 * t237;
        let t245 = piecewise3(t52, t103 * t237 / 2.0 - 3.0 / 8.0 * t240, -t236 - 0.27534222222222222222e0 * t214);
        let t246 = t28 * t245;
        let t255 = 0.70819478890314631335e-1 * t74 * t136 * t245 + 0.16615495624729559964e1 * t141 * t81 * t245 * t153;
        let t256 = t134 * t255;
        let t258 = t246 * t91 - t72 * t256;
        let t262 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t258);
        let tvtau0 = 2.0 * rho[ip] * t262;
        vtau[ip] += tvtau0;
        // --- fxc delta (this level) (196 lines) ---
        let t265 = t18 * t34;
        let t272 = t117 * t117;
        let t276 = t37 * t37;
        let t278 = 1.0 / t32 / t276;
        let t283 = 40.0 / 9.0 * t31 * t108 - 11.0 / 9.0 * t36 * t278 - 10.0 / 9.0 * t42 * t108;
        let t285 = t283 * t46 * t49;
        let t286 = 0.39111111111111111111e0 * t285;
        let t287 = piecewise3(t55, 0.0, -t286);
        let t291 = 1.0 / t120 / t57;
        let t298 = 1.0 / t69 / t68;
        let t299 = t298 * t66;
        let t300 = t113 * t113;
        let t301 = t46 * t46;
        let t304 = 1.0 / t25 / t24;
        let t309 = t301 * t304;
        let t315 = piecewise3(t52, -t62 * t272 + t103 * t287 / 2.0 + 3.0 / 2.0 * t291 * t272 - 3.0 / 8.0 * t121 * t287, -t286 - 0.75813339338271604937e-1 * t299 * t300 * t301 * t304 + 0.15296790123456790123e0 * t125 * t300 * t309 - 0.27534222222222222222e0 * t126 * t285);
        let t316 = t28 * t315;
        let t321 = 1.0 / t133 / t90;
        let t322 = t157 * t157;
        let t323 = t321 * t322;
        let t327 = 1.0 / t75 / t71;
        let t328 = t327 * t86;
        let t329 = t130 * t130;
        let t333 = t140 * t135;
        let t345 = t143 * t143;
        let t346 = 1.0 / t345;
        let t347 = t346 * t75;
        let t348 = t146 * t81;
        let t349 = t347 * t348;
        let t351 = 1.0 / t152 / t151;
        let t352 = t329 * t351;
        let t356 = -0.35409739445157315668e-1 * t74 * t328 * t329 + 0.16615495624729559964e1 * t333 * t81 * t329 * t153 + 0.70819478890314631335e-1 * t74 * t136 * t315 + 0.16615495624729559964e1 * t141 * t81 * t315 * t153 - 0.76217237139506171188e1 * t349 * t352 * t82;
        let t357 = t134 * t356;
        let t359 = -2.0 * t131 * t158 + t316 * t91 + 2.0 * t72 * t323 - t72 * t357;
        let t364 = piecewise3(t3, 0.0, t7 * t265 * t93 / 12.0 - t7 * t99 * t160 / 4.0 - 3.0 / 8.0 * t7 * t20 * t359);
        let tv2rho20 = 2.0 * rho[ip] * t364 + 4.0 * t165;
        v2rho2[ip] += tv2rho20;
        let t370 = t62 * t172;
        let t372 = t30 * t108;
        let t373 = t372 * t169;
        let t374 = 0.13037037037037037037e0 * t373;
        let t375 = piecewise3(t55, 0.0, -t374);
        let t378 = t291 * t172;
        let t381 = t121 * t375;
        let t384 = t299 * t30;
        let t385 = t39 * t301;
        let t386 = t304 * t113;
        let t388 = t384 * t385 * t386;
        let t390 = t125 * t113;
        let t391 = t390 * t301;
        let t392 = t304 * t30;
        let t393 = t392 * t39;
        let t394 = t391 * t393;
        let t398 = t178 * t108 * t46 * t49;
        let t401 = piecewise3(t52, -t370 * t117 + t103 * t375 / 2.0 + 3.0 / 2.0 * t378 * t117 - 3.0 / 8.0 * t381, -t374 + 0.94766674172839506173e-2 * t388 - 0.19120987654320987654e-1 * t394 - 0.91780740740740740741e-1 * t398);
        let t402 = t28 * t401;
        let t406 = t321 * t194;
        let t407 = t406 * t157;
        let t410 = t74 * t327;
        let t411 = t86 * t184;
        let t415 = t333 * t81;
        let t416 = t130 * t153;
        let t427 = t184 * t351;
        let t428 = t82 * t130;
        let t429 = t427 * t428;
        let t432 = -0.35409739445157315668e-1 * t410 * t411 * t130 + 0.16615495624729559964e1 * t415 * t416 * t184 + 0.70819478890314631335e-1 * t74 * t136 * t401 + 0.16615495624729559964e1 * t141 * t81 * t401 * t153 - 0.76217237139506171188e1 * t349 * t429;
        let t433 = t134 * t432;
        let t435 = -t131 * t195 - t185 * t158 + t402 * t91 + 2.0 * t72 * t407 - t72 * t433;
        let t440 = piecewise3(t3, 0.0, -t7 * t99 * t197 / 8.0 - 3.0 / 8.0 * t7 * t20 * t435);
        let tv2rhosigma0 = 2.0 * rho[ip] * t440 + 2.0 * t201;
        v2rhosigma[ip] += tv2rhosigma0;
        let t446 = t62 * t206;
        let t448 = 0.16296296296296296296e0 * t170;
        let t449 = piecewise3(t55, 0.0, -t448);
        let t452 = t291 * t206;
        let t455 = t121 * t449;
        let t458 = t34 * t301;
        let t460 = t384 * t458 * t386;
        let t462 = t392 * t34;
        let t463 = t391 * t462;
        let t467 = piecewise3(t52, -t446 * t117 + t103 * t449 / 2.0 + 3.0 / 2.0 * t452 * t117 - 3.0 / 8.0 * t455, -t448 + 0.18953334834567901234e-1 * t460 - 0.38241975308641975308e-1 * t463 - 0.11472592592592592593e0 * t181);
        let t468 = t28 * t467;
        let t472 = t321 * t227;
        let t473 = t472 * t157;
        let t476 = t86 * t217;
        let t490 = t217 * t351;
        let t491 = t490 * t428;
        let t494 = -0.35409739445157315668e-1 * t410 * t476 * t130 + 0.16615495624729559964e1 * t415 * t416 * t217 + 0.70819478890314631335e-1 * t74 * t136 * t467 + 0.16615495624729559964e1 * t141 * t81 * t467 * t153 - 0.76217237139506171188e1 * t349 * t491;
        let t495 = t134 * t494;
        let t497 = -t131 * t228 - t218 * t158 + t468 * t91 + 2.0 * t72 * t473 - t72 * t495;
        let t502 = piecewise3(t3, 0.0, -t7 * t99 * t230 / 8.0 - 3.0 / 8.0 * t7 * t20 * t497);
        let tv2rholapl0 = 2.0 * rho[ip] * t502 + 2.0 * t234;
        v2rholapl[ip] += tv2rholapl0;
        let t508 = t62 * t237;
        let t510 = 0.65185185185185185185e0 * t170;
        let t511 = piecewise3(t55, 0.0, t510);
        let t514 = t291 * t237;
        let t517 = t121 * t511;
        let t524 = piecewise3(t52, -t508 * t117 + t103 * t511 / 2.0 + 3.0 / 2.0 * t514 * t117 - 3.0 / 8.0 * t517, t510 - 0.75813339338271604937e-1 * t460 + 0.15296790123456790123e0 * t463 + 0.4589037037037037037e0 * t181);
        let t525 = t28 * t524;
        let t529 = t321 * t255;
        let t530 = t529 * t157;
        let t533 = t86 * t245;
        let t547 = t245 * t351;
        let t548 = t547 * t428;
        let t551 = -0.35409739445157315668e-1 * t410 * t533 * t130 + 0.16615495624729559964e1 * t415 * t416 * t245 + 0.70819478890314631335e-1 * t74 * t136 * t524 + 0.16615495624729559964e1 * t141 * t81 * t524 * t153 - 0.76217237139506171188e1 * t349 * t548;
        let t552 = t134 * t551;
        let t554 = -t131 * t256 - t246 * t158 + t525 * t91 + 2.0 * t72 * t530 - t72 * t552;
        let t559 = piecewise3(t3, 0.0, -t7 * t99 * t258 / 8.0 - 3.0 / 8.0 * t7 * t20 * t554);
        let tv2rhotau0 = 2.0 * rho[ip] * t559 + 2.0 * t262;
        v2rhotau[ip] += tv2rhotau0;
        let t562 = t172 * t172;
        let t564 = piecewise3(t55, 0.0, 0.0);
        let t566 = t103 * t564 / 2.0;
        let t567 = t291 * t562;
        let t569 = t121 * t564;
        let t570 = 3.0 / 8.0 * t569;
        let t572 = t299 * t29;
        let t573 = t276 * rho[ip];
        let t575 = 1.0 / t19 / t573;
        let t577 = t575 * t301 * t304;
        let t578 = t572 * t577;
        let t580 = t125 * t29;
        let t581 = t580 * t577;
        let t584 = piecewise3(t52, -t62 * t562 + t566 + 3.0 / 2.0 * t567 - t570, -0.23691668543209876543e-2 * t578 + 0.47802469135802469136e-2 * t581);
        let t585 = t28 * t584;
        let t589 = t194 * t194;
        let t590 = t321 * t589;
        let t593 = t184 * t184;
        let t608 = t593 * t351;
        let t612 = -0.35409739445157315668e-1 * t74 * t328 * t593 + 0.16615495624729559964e1 * t333 * t81 * t593 * t153 + 0.70819478890314631335e-1 * t74 * t136 * t584 + 0.16615495624729559964e1 * t141 * t81 * t584 * t153 - 0.76217237139506171188e1 * t349 * t608 * t82;
        let t613 = t134 * t612;
        let t615 = -2.0 * t185 * t195 + t585 * t91 + 2.0 * t72 * t590 - t72 * t613;
        let t619 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t615);
        let tv2sigma20 = 2.0 * rho[ip] * t619;
        v2sigma2[ip] += tv2sigma20;
        let t626 = 1.0 / t19 / t276;
        let t628 = t626 * t301 * t304;
        let t629 = t572 * t628;
        let t631 = t580 * t628;
        let t634 = piecewise3(t52, -t446 * t172 + t566 + 3.0 / 2.0 * t452 * t172 - t570, -0.47383337086419753086e-2 * t629 + 0.95604938271604938271e-2 * t631);
        let t635 = t28 * t634;
        let t639 = t472 * t194;
        let t645 = t184 * t153;
        let t656 = t82 * t184;
        let t657 = t490 * t656;
        let t660 = -0.35409739445157315668e-1 * t410 * t476 * t184 + 0.16615495624729559964e1 * t415 * t645 * t217 + 0.70819478890314631335e-1 * t74 * t136 * t634 + 0.16615495624729559964e1 * t141 * t81 * t634 * t153 - 0.76217237139506171188e1 * t349 * t657;
        let t661 = t134 * t660;
        let t663 = -t185 * t228 - t218 * t195 + t635 * t91 + 2.0 * t72 * t639 - t72 * t661;
        let t667 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t663);
        let tv2sigmalapl0 = 2.0 * rho[ip] * t667;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t676 = piecewise3(t52, -t508 * t172 + t566 + 3.0 / 2.0 * t514 * t172 - t570, 0.18953334834567901234e-1 * t629 - 0.38241975308641975308e-1 * t631);
        let t677 = t28 * t676;
        let t681 = t529 * t194;
        let t697 = t547 * t656;
        let t700 = -0.35409739445157315668e-1 * t410 * t533 * t184 + 0.16615495624729559964e1 * t415 * t645 * t245 + 0.70819478890314631335e-1 * t74 * t136 * t676 + 0.16615495624729559964e1 * t141 * t81 * t676 * t153 - 0.76217237139506171188e1 * t349 * t697;
        let t701 = t134 * t700;
        let t703 = -t185 * t256 - t246 * t195 + t677 * t91 + 2.0 * t72 * t681 - t72 * t701;
        let t707 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t703);
        let tv2sigmatau0 = 2.0 * rho[ip] * t707;
        v2sigmatau[ip] += tv2sigmatau0;
        let t709 = t206 * t206;
        let t711 = t291 * t709;
        let t715 = 1.0 / t19 / t106;
        let t717 = t715 * t301 * t304;
        let t718 = t572 * t717;
        let t720 = t580 * t717;
        let t723 = piecewise3(t52, -t62 * t709 + t566 + 3.0 / 2.0 * t711 - t570, -0.94766674172839506171e-2 * t718 + 0.19120987654320987654e-1 * t720);
        let t724 = t28 * t723;
        let t728 = t227 * t227;
        let t729 = t321 * t728;
        let t732 = t217 * t217;
        let t747 = t732 * t351;
        let t751 = -0.35409739445157315668e-1 * t74 * t328 * t732 + 0.16615495624729559964e1 * t333 * t81 * t732 * t153 + 0.70819478890314631335e-1 * t74 * t136 * t723 + 0.16615495624729559964e1 * t141 * t81 * t723 * t153 - 0.76217237139506171188e1 * t349 * t747 * t82;
        let t752 = t134 * t751;
        let t754 = -2.0 * t218 * t228 + 2.0 * t72 * t729 - t72 * t752 + t724 * t91;
        let t758 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t754);
        let tv2lapl20 = 2.0 * rho[ip] * t758;
        v2lapl2[ip] += tv2lapl20;
        let t767 = piecewise3(t52, -t508 * t206 + t566 + 3.0 / 2.0 * t514 * t206 - t570, 0.37906669669135802469e-1 * t718 - 0.76483950617283950617e-1 * t720);
        let t768 = t28 * t767;
        let t772 = t529 * t227;
        let t778 = t217 * t153;
        let t789 = t82 * t217;
        let t790 = t547 * t789;
        let t793 = -0.35409739445157315668e-1 * t410 * t533 * t217 + 0.16615495624729559964e1 * t415 * t778 * t245 + 0.70819478890314631335e-1 * t74 * t136 * t767 + 0.16615495624729559964e1 * t141 * t81 * t767 * t153 - 0.76217237139506171188e1 * t349 * t790;
        let t794 = t134 * t793;
        let t796 = -t218 * t256 - t246 * t228 + 2.0 * t72 * t772 - t72 * t794 + t768 * t91;
        let t800 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t796);
        let tv2lapltau0 = 2.0 * rho[ip] * t800;
        v2lapltau[ip] += tv2lapltau0;
        let t802 = t237 * t237;
        let t804 = t291 * t802;
        let t810 = piecewise3(t52, -t62 * t802 + t566 + 3.0 / 2.0 * t804 - t570, -0.15162667867654320988e0 * t718 + 0.30593580246913580247e0 * t720);
        let t811 = t28 * t810;
        let t815 = t255 * t255;
        let t816 = t321 * t815;
        let t819 = t245 * t245;
        let t834 = t819 * t351;
        let t838 = -0.35409739445157315668e-1 * t74 * t328 * t819 + 0.16615495624729559964e1 * t333 * t81 * t819 * t153 + 0.70819478890314631335e-1 * t74 * t136 * t810 + 0.16615495624729559964e1 * t141 * t81 * t810 * t153 - 0.76217237139506171188e1 * t349 * t834 * t82;
        let t839 = t134 * t838;
        let t841 = -2.0 * t246 * t256 + 2.0 * t72 * t816 - t72 * t839 + t811 * t91;
        let t845 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t841);
        let tv2tau20 = 2.0 * rho[ip] * t845;
        v2tau2[ip] += tv2tau20;
    }
}
