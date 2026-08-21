//! GGA_X_PW91 lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pw91.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pw91_lxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_a: f64,
    param_alpha: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_expo: f64,
    param_f: f64,
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
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t37 = rmath::exp(-param_alpha * t20 * t25 * t34 / 24.0);
        let t40 = (param_d * t37 + param_c) * t20;
        let t41 = t40 * t25;
        let t44 = t20 * t20;
        let t45 = 1.0 / t23;
        let t46 = t44 * t45;
        let t47 = rmath::sqrt(sigma[ip]);
        let t50 = 1.0 / t18 / rho[ip];
        let t51 = t47 * t27 * t50;
        let t54 = rmath::pow(t46 * t51 / 12.0, param_expo);
        let t55 = param_f * t54;
        let t56 = t41 * t34 / 24.0 - t55;
        let t57 = t46 * t47;
        let t63 = rmath::ln(param_b * t44 * t45 * t51 / 12.0 + rmath::sqrt(pow_2(param_b * t44 * t45 * t51 / 12.0) + 1.0));
        let t64 = param_a * t63;
        let t65 = t27 * t50 * t64;
        let t68 = 1.0 + t57 * t65 / 12.0 + t55;
        let t69 = 1.0 / t68;
        let t71 = t56 * t69 + 1.0;
        let t75 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t71);
        let tzk0 = 2.0 * t75;
        zk[ip] += tzk0;
        let t77 = t17 / t31;
        let t81 = param_d * param_alpha;
        let t83 = 1.0 / t23 / t22;
        let t84 = t44 * t83;
        let t85 = t81 * t84;
        let t86 = sigma[ip] * sigma[ip];
        let t87 = t86 * t27;
        let t88 = t30 * t30;
        let t89 = t88 * t30;
        let t91 = 1.0 / t18 / t89;
        let t92 = t91 * t37;
        let t96 = t30 * rho[ip];
        let t98 = 1.0 / t31 / t96;
        let t102 = 1.0 / rho[ip];
        let t105 = 4.0 / 3.0 * t55 * param_expo * t102;
        let t106 = t85 * t87 * t92 / 108.0 - t41 * t29 * t98 / 9.0 + t105;
        let t108 = t68 * t68;
        let t109 = 1.0 / t108;
        let t110 = t56 * t109;
        let t114 = t27 / t18 / t30 * t64;
        let t117 = t20 * t25;
        let t118 = t117 * t29;
        let t120 = param_b * param_b;
        let t125 = 6.0 * t120 * t20 * t25 * t34 + 144.0;
        let t126 = rmath::sqrt(t125);
        let t128 = param_b / t126;
        let t129 = t98 * param_a * t128;
        let t132 = -t57 * t114 / 9.0 - 2.0 / 3.0 * t118 * t129 - t105;
        let t134 = t106 * t69 - t110 * t132;
        let t139 = piecewise3(t2, 0.0, -t6 * t77 * t71 / 8.0 - 3.0 / 8.0 * t6 * t19 * t134);
        let tvrho0 = 2.0 * rho[ip] * t139 + 2.0 * t75;
        vrho[ip] += tvrho0;
        let t142 = t88 * rho[ip];
        let t144 = 1.0 / t18 / t142;
        let t145 = t27 * t144;
        let t146 = t37 * sigma[ip];
        let t150 = t25 * t28;
        let t154 = 1.0 / sigma[ip];
        let t157 = t55 * param_expo * t154 / 2.0;
        let t158 = -t85 * t145 * t146 / 288.0 + t40 * t150 * t33 / 24.0 - t157;
        let t161 = t46 / t47;
        let t164 = t117 * t28;
        let t166 = t33 * param_a * t128;
        let t169 = t161 * t65 / 24.0 + t164 * t166 / 4.0 + t157;
        let t171 = -t110 * t169 + t158 * t69;
        let t175 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t171);
        let tvsigma0 = 2.0 * rho[ip] * t175;
        vsigma[ip] += tvsigma0;
        let t180 = t17 / t31 / rho[ip];
        let t187 = t88 * t96;
        let t189 = 1.0 / t18 / t187;
        let t190 = t189 * t37;
        let t194 = param_alpha * param_alpha;
        let t195 = param_d * t194;
        let t196 = t22 * t22;
        let t197 = 1.0 / t196;
        let t198 = t195 * t197;
        let t199 = t86 * sigma[ip];
        let t200 = t88 * t88;
        let t201 = t200 * t30;
        let t202 = 1.0 / t201;
        let t208 = 1.0 / t31 / t88;
        let t212 = param_expo * param_expo;
        let t213 = 1.0 / t30;
        let t214 = t212 * t213;
        let t216 = 16.0 / 9.0 * t55 * t214;
        let t219 = 4.0 / 3.0 * t55 * param_expo * t213;
        let t220 = -t85 * t87 * t190 / 12.0 + t198 * t199 * t202 * t37 / 81.0 + 11.0 / 27.0 * t41 * t29 * t208 - t216 - t219;
        let t222 = t106 * t109;
        let t226 = 1.0 / t108 / t68;
        let t227 = t56 * t226;
        let t228 = t132 * t132;
        let t234 = t27 / t18 / t96 * t64;
        let t238 = t208 * param_a * t128;
        let t241 = t84 * t87;
        let t243 = t120 * param_b;
        let t245 = 1.0 / t126 / t125;
        let t246 = t243 * t245;
        let t247 = t189 * param_a * t246;
        let t250 = 7.0 / 27.0 * t57 * t234 + 10.0 / 3.0 * t118 * t238 - 32.0 / 3.0 * t241 * t247 + t216 + t219;
        let t252 = -t110 * t250 - 2.0 * t222 * t132 + t220 * t69 + 2.0 * t227 * t228;
        let t257 = piecewise3(t2, 0.0, t6 * t180 * t71 / 12.0 - t6 * t77 * t134 / 4.0 - 3.0 / 8.0 * t6 * t19 * t252);
        let tv2rho20 = 2.0 * rho[ip] * t257 + 4.0 * t139;
        v2rho2[ip] += tv2rho20;
        let t263 = t27 * t91;
        let t267 = t200 * rho[ip];
        let t268 = 1.0 / t267;
        let t276 = t212 * t102;
        let t279 = 2.0 / 3.0 * t55 * t276 * t154;
        let t280 = t85 * t263 * t146 / 36.0 - t198 * t268 * t86 * t37 / 216.0 - t40 * t150 * t98 / 9.0 + t279;
        let t282 = t158 * t109;
        let t285 = t169 * t132;
        let t294 = param_a * t243 * t245 * sigma[ip];
        let t297 = -t161 * t114 / 18.0 - t164 * t129 + 4.0 * t84 * t263 * t294 - t279;
        let t299 = -t110 * t297 - t282 * t132 - t222 * t169 + 2.0 * t227 * t285 + t280 * t69;
        let t304 = piecewise3(t2, 0.0, -t6 * t77 * t171 / 8.0 - 3.0 / 8.0 * t6 * t19 * t299);
        let tv2rhosigma0 = 2.0 * rho[ip] * t304 + 2.0 * t175;
        v2rhosigma[ip] += tv2rhosigma0;
        let t307 = 1.0 / t200;
        let t312 = t81 * t44;
        let t313 = t83 * t27;
        let t318 = 1.0 / t86;
        let t321 = t55 * t212 * t318 / 4.0;
        let t324 = t55 * param_expo * t318 / 2.0;
        let t325 = t198 * t307 * t37 * sigma[ip] / 576.0 - t312 * t313 * t144 * t37 / 144.0 - t321 + t324;
        let t329 = t169 * t169;
        let t334 = t46 / t47 / sigma[ip];
        let t338 = t117 * t154 * t28;
        let t341 = t84 * t27;
        let t343 = t144 * param_a * t246;
        let t346 = -t334 * t65 / 48.0 + t338 * t166 / 8.0 - 3.0 / 2.0 * t341 * t343 + t321 - t324;
        let t348 = -t110 * t346 - 2.0 * t282 * t169 + 2.0 * t227 * t329 + t325 * t69;
        let t352 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t348);
        let tv2sigma20 = 2.0 * rho[ip] * t352;
        v2sigma2[ip] += tv2sigma20;
        let t355 = t17 * t33;
        let t366 = 1.0 / t18 / t200;
        let t371 = t200 * t96;
        let t372 = 1.0 / t371;
        let t378 = param_d * t194 * param_alpha;
        let t379 = t86 * t86;
        let t380 = t197 * t379;
        let t381 = t378 * t380;
        let t382 = t200 * t142;
        let t384 = 1.0 / t31 / t382;
        let t386 = t150 * t37;
        let t391 = 1.0 / t31 / t142;
        let t395 = t212 * param_expo;
        let t396 = 1.0 / t96;
        let t397 = t395 * t396;
        let t399 = 64.0 / 27.0 * t55 * t397;
        let t400 = t212 * t396;
        let t402 = 16.0 / 3.0 * t55 * t400;
        let t405 = 8.0 / 3.0 * t55 * param_expo * t396;
        let t406 = 341.0 / 486.0 * t85 * t87 * t366 * t37 - 19.0 / 81.0 * t198 * t199 * t372 * t37 + t381 * t384 * t20 * t386 / 729.0 - 154.0 / 81.0 * t41 * t29 * t391 + t399 + t402 + t405;
        let t408 = t220 * t109;
        let t411 = t106 * t226;
        let t416 = t108 * t108;
        let t417 = 1.0 / t416;
        let t418 = t56 * t417;
        let t419 = t228 * t132;
        let t422 = t132 * t250;
        let t428 = t27 / t18 / t88 * t64;
        let t432 = t391 * param_a * t128;
        let t439 = t197 * t199;
        let t441 = t120 * t120;
        let t442 = t441 * param_b;
        let t444 = t125 * t125;
        let t446 = 1.0 / t126 / t444;
        let t447 = param_a * t442 * t446;
        let t450 = -70.0 / 81.0 * t57 * t428 - 476.0 / 27.0 * t118 * t432 + 1184.0 / 9.0 * t241 * t366 * param_a * t246 - 3072.0 * t439 * t372 * t447 - t399 - t402 - t405;
        let t452 = -t110 * t450 - 3.0 * t408 * t132 - 3.0 * t222 * t250 + 6.0 * t227 * t422 + 6.0 * t411 * t228 + t406 * t69 - 6.0 * t418 * t419;
        let t457 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t355 * t71 + t6 * t180 * t134 / 4.0 - 3.0 / 8.0 * t6 * t77 * t252 - 3.0 / 8.0 * t6 * t19 * t452);
        let tv3rho30 = 2.0 * rho[ip] * t457 + 6.0 * t257;
        v3rho3[ip] += tv3rho30;
        let t467 = t27 * t189;
        let t475 = t200 * t88;
        let t478 = t197 / t31 / t475;
        let t479 = t378 * t478;
        let t481 = t199 * t20 * t386;
        let t487 = t395 * t213;
        let t490 = 8.0 / 9.0 * t55 * t487 * t154;
        let t493 = 2.0 / 3.0 * t55 * t214 * t154;
        let t494 = -65.0 / 324.0 * t85 * t467 * t146 + 17.0 / 216.0 * t198 * t202 * t86 * t37 - t479 * t481 / 1944.0 + 11.0 / 27.0 * t40 * t150 * t208 - t490 - t493;
        let t496 = t280 * t109;
        let t499 = t158 * t226;
        let t508 = t169 * t228;
        let t511 = t297 * t132;
        let t514 = t169 * t250;
        let t525 = t197 * t202 * param_a;
        let t526 = t442 * t446;
        let t527 = t526 * t86;
        let t530 = 7.0 / 54.0 * t161 * t234 + 37.0 / 9.0 * t164 * t238 - 124.0 / 3.0 * t84 * t467 * t294 + 1152.0 * t525 * t527 + t490 + t493;
        let t532 = -t110 * t530 - 2.0 * t496 * t132 - t408 * t169 - 2.0 * t222 * t297 + 4.0 * t227 * t511 + 2.0 * t227 * t514 + 2.0 * t499 * t228 - t282 * t250 + 4.0 * t411 * t285 - 6.0 * t418 * t508 + t494 * t69;
        let t537 = piecewise3(t2, 0.0, t6 * t180 * t171 / 12.0 - t6 * t77 * t299 / 4.0 - 3.0 / 8.0 * t6 * t19 * t532);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t537 + 4.0 * t304;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t549 = t197 / t31 / t371;
        let t550 = t378 * t549;
        let t553 = t117 * t86 * t28 * t37;
        let t559 = t395 * t102;
        let t562 = t55 * t559 * t318 / 3.0;
        let t565 = 2.0 / 3.0 * t55 * t276 * t318;
        let t566 = -5.0 / 216.0 * t198 * t268 * t37 * sigma[ip] + t550 * t553 / 5184.0 + t312 * t313 * t92 / 27.0 + t562 - t565;
        let t568 = t325 * t109;
        let t578 = t329 * t132;
        let t581 = t169 * t297;
        let t585 = t346 * t132;
        let t593 = t91 * param_a * t246;
        let t596 = t197 * t268;
        let t598 = t526 * sigma[ip];
        let t601 = t334 * t114 / 36.0 - t338 * t129 / 6.0 + 10.0 * t341 * t593 - 432.0 * t596 * param_a * t598 - t562 + t565;
        let t603 = -t110 * t601 - t568 * t132 - 2.0 * t496 * t169 - t222 * t346 + 4.0 * t227 * t581 + 2.0 * t227 * t585 - 2.0 * t282 * t297 + 4.0 * t499 * t285 + 2.0 * t411 * t329 - 6.0 * t418 * t578 + t566 * t69;
        let t608 = piecewise3(t2, 0.0, -t6 * t77 * t348 / 8.0 - 3.0 / 8.0 * t6 * t19 * t603);
        let tv3rhosigma20 = 2.0 * rho[ip] * t608 + 2.0 * t352;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t613 = t197 / t31 / t201;
        let t614 = t378 * t613;
        let t615 = t28 * t37;
        let t617 = t117 * t615 * sigma[ip];
        let t620 = t197 * t307;
        let t624 = 1.0 / t199;
        let t627 = t55 * t395 * t624 / 8.0;
        let t630 = 3.0 / 4.0 * t55 * t212 * t624;
        let t632 = t55 * param_expo * t624;
        let t633 = -t614 * t617 / 13824.0 + t195 * t620 * t37 / 192.0 - t627 + t630 - t632;
        let t641 = t329 * t169;
        let t644 = t169 * t346;
        let t649 = t46 / t47 / t86;
        let t653 = t117 * t318 * t28;
        let t657 = t84 * t154 * t27;
        let t662 = t649 * t65 / 32.0 - 3.0 / 16.0 * t653 * t166 - 3.0 / 4.0 * t657 * t343 + 162.0 * t620 * t447 + t627 - t630 + t632;
        let t664 = -t110 * t662 - 3.0 * t568 * t169 + 6.0 * t227 * t644 - 3.0 * t282 * t346 + 6.0 * t499 * t329 - 6.0 * t418 * t641 + t633 * t69;
        let t668 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t664);
        let tv3sigma30 = 2.0 * rho[ip] * t668;
        v3sigma3[ip] += tv3sigma30;
        let t685 = 1.0 / t18 / t267;
        let t690 = 1.0 / t475;
        let t695 = t200 * t89;
        let t697 = 1.0 / t31 / t695;
        let t702 = t194 * t194;
        let t703 = param_d * t702;
        let t707 = t200 * t200;
        let t712 = t313 * t37;
        let t717 = 1.0 / t31 / t89;
        let t721 = t212 * t212;
        let t722 = 1.0 / t88;
        let t725 = 256.0 / 81.0 * t55 * t721 * t722;
        let t728 = 128.0 / 9.0 * t55 * t395 * t722;
        let t731 = 176.0 / 9.0 * t55 * t212 * t722;
        let t734 = 8.0 * t55 * param_expo * t722;
        let t737 = t406 * t109;
        let t740 = t220 * t226;
        let t745 = t106 * t417;
        let t754 = t56 / t416 / t68;
        let t755 = t228 * t228;
        let t761 = t250 * t250;
        let t783 = t441 * t243;
        let t786 = 1.0 / t126 / t444 / t125;
        let t793 = (-3047.0 / 486.0 * t85 * t87 * t685 * t37 + 2563.0 / 729.0 * t198 * t199 * t690 * t37 - 98.0 / 2187.0 * t381 * t697 * t20 * t386 + 2.0 / 6561.0 * t703 * t197 * t379 * sigma[ip] / t18 / t707 / rho[ip] * t44 * t712 + 2618.0 / 243.0 * t41 * t29 * t717 - t725 - t728 - t731 - t734) * t69 - 4.0 * t737 * t132 + 12.0 * t740 * t228 - 6.0 * t408 * t250 - 24.0 * t745 * t419 + 24.0 * t411 * t422 - 4.0 * t222 * t450 + 24.0 * t754 * t755 - 36.0 * t418 * t228 * t250 + 6.0 * t227 * t761 + 8.0 * t227 * t132 * t450 - t110 * (910.0 / 243.0 * t57 * t145 * t64 + 2884.0 / 27.0 * t118 * t717 * param_a * t128 - 37216.0 / 27.0 * t241 * t685 * param_a * t246 + 71680.0 * t439 * t690 * t447 - 122880.0 * t380 * t697 * param_a * t783 * t786 * t164 + t725 + t728 + t731 + t734);
        let t798 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 * t98 * t71 - 5.0 / 9.0 * t6 * t355 * t134 + t6 * t180 * t252 / 2.0 - t6 * t77 * t452 / 2.0 - 3.0 / 8.0 * t6 * t19 * t793);
        let tv4rho40 = 2.0 * rho[ip] * t798 + 8.0 * t457;
        v4rho4[ip] += tv4rho40;
        let t811 = t280 * t226;
        let t814 = t158 * t417;
        let t838 = t27 * t366;
        let t846 = t197 * t384;
        let t864 = 32.0 / 27.0 * t55 * t721 * t396 * t154;
        let t867 = 8.0 / 3.0 * t55 * t397 * t154;
        let t870 = 4.0 / 3.0 * t55 * t400 * t154;
        let t881 = t494 * t109;
        let t903 = param_a * t783;
        let t911 = (253.0 / 162.0 * t85 * t838 * t146 - 1025.0 / 972.0 * t198 * t372 * t86 * t37 + 89.0 / 5832.0 * t378 * t846 * t481 - t703 * t197 / t18 / t707 * t379 * t44 * t712 / 8748.0 - 154.0 / 81.0 * t40 * t150 * t391 + t864 + t867 + t870) * t69 + 24.0 * t754 * t169 * t419 - 18.0 * t418 * t285 * t250 - 18.0 * t745 * t508 - 3.0 * t881 * t132 - 3.0 * t496 * t250 - t282 * t450 - t737 * t169 - 3.0 * t408 * t297 - 3.0 * t222 * t530 - t110 * (-35.0 / 81.0 * t161 * t428 - 182.0 / 9.0 * t164 * t432 + 3320.0 / 9.0 * t84 * t838 * t294 - 23424.0 * t197 * t372 * param_a * t527 + 46080.0 * t846 * t903 * t786 * t199 * t164 - t864 - t867 - t870);
        let t917 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t355 * t171 + t6 * t180 * t299 / 4.0 - 3.0 / 8.0 * t6 * t77 * t532 - 3.0 / 8.0 * t6 * t19 * (6.0 * t227 * t530 * t132 + 2.0 * t227 * t169 * t450 + 6.0 * t227 * t297 * t250 - 18.0 * t418 * t297 * t228 + 6.0 * t811 * t228 + 6.0 * t740 * t285 + 12.0 * t411 * t511 + 6.0 * t411 * t514 - 6.0 * t814 * t419 + 6.0 * t499 * t422 + t911));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t917 + 6.0 * t537;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t949 = 4.0 / 9.0 * t55 * t721 * t213 * t318;
        let t952 = 5.0 / 9.0 * t55 * t487 * t318;
        let t955 = 2.0 / 3.0 * t55 * t214 * t318;
        let t988 = (167.0 / 648.0 * t198 * t202 * t37 * sigma[ip] - 25.0 / 5184.0 * t479 * t553 + t703 * t197 / t18 / t200 / t187 * t84 * t199 * t27 * t37 / 23328.0 - 19.0 / 81.0 * t312 * t313 * t190 - t949 + t952 + t955) * t69 - 24.0 * t418 * t285 * t297 + 24.0 * t754 * t329 * t228 - 6.0 * t418 * t346 * t228 + 8.0 * t811 * t285 + 8.0 * t499 * t511 + 4.0 * t499 * t514 - 12.0 * t745 * t578 + 8.0 * t411 * t581 - 6.0 * t418 * t329 * t250 + 4.0 * t227 * t169 * t530 + 4.0 * t411 * t585 + 4.0 * t227 * t601 * t132;
        let t994 = t297 * t297;
        let t997 = t566 * t109;
        let t1027 = t325 * t226;
        let t1030 = 2.0 * t227 * t346 * t250 - 12.0 * t814 * t508 + 4.0 * t227 * t994 - 2.0 * t997 * t132 - t568 * t250 - 2.0 * t881 * t169 - 4.0 * t496 * t297 - 2.0 * t282 * t530 + 2.0 * t740 * t329 - t408 * t346 - 2.0 * t222 * t601 - t110 * (-7.0 / 108.0 * t334 * t234 + 7.0 / 18.0 * t338 * t238 - 66.0 * t341 * t247 + 6768.0 * t525 * t598 - 17280.0 * t478 * t903 * t786 * t86 * t164 + t949 - t952 - t955) + 2.0 * t1027 * t228;
        let t1036 = piecewise3(t2, 0.0, t6 * t180 * t348 / 12.0 - t6 * t77 * t603 / 4.0 - 3.0 / 8.0 * t6 * t19 * (t988 + t1030));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t1036 + 4.0 * t608;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t1058 = t55 * t721 * t102 * t624 / 6.0;
        let t1060 = t55 * t559 * t624;
        let t1063 = 4.0 / 3.0 * t55 * t276 * t624;
        let t1066 = t633 * t109;
        let t1118 = t786 * t20;
        let t1126 = -3.0 * t282 * t601 - 6.0 * t745 * t641 + 24.0 * t754 * t641 * t132 - 18.0 * t418 * t329 * t297 + 6.0 * t411 * t644 - 18.0 * t418 * t644 * t132 + 6.0 * t227 * t297 * t346 + 6.0 * t227 * t169 * t601 - t222 * t662 + 2.0 * t227 * t662 * t132 - t110 * (-t649 * t114 / 24.0 + t653 * t129 / 4.0 + t657 * t593 - 1512.0 * t596 * t447 + 6480.0 * t549 * t903 * t1118 * t25 * sigma[ip] * t28 - t1058 + t1060 - t1063);
        let t1132 = piecewise3(t2, 0.0, -t6 * t77 * t664 / 8.0 - 3.0 / 8.0 * t6 * t19 * ((7.0 / 5184.0 * t550 * t617 - t703 * t197 / t18 / t695 * t84 * t87 * t37 / 62208.0 - t195 * t596 * t37 / 24.0 + t1058 - t1060 + t1063) * t69 - t1066 * t132 - 3.0 * t997 * t169 + 6.0 * t1027 * t285 - 3.0 * t568 * t297 + 6.0 * t811 * t329 - 18.0 * t814 * t578 + 12.0 * t499 * t581 - 3.0 * t496 * t346 + 6.0 * t499 * t585 + t1126));
        let tv4rhosigma30 = 2.0 * rho[ip] * t1132 + 2.0 * t668;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t1147 = 1.0 / t379;
        let t1150 = t55 * t721 * t1147 / 16.0;
        let t1153 = 3.0 / 4.0 * t55 * t395 * t1147;
        let t1156 = 11.0 / 4.0 * t55 * t212 * t1147;
        let t1159 = 3.0 * t55 * param_expo * t1147;
        let t1174 = t329 * t329;
        let t1180 = t346 * t346;
        let t1209 = (t703 * t197 / t18 / t382 * t84 * t27 * t37 * sigma[ip] / 165888.0 - t614 * t117 * t615 / 3456.0 - t1150 + t1153 - t1156 + t1159) * t69 - 4.0 * t1066 * t169 + 12.0 * t1027 * t329 - 6.0 * t568 * t346 - 24.0 * t814 * t641 + 24.0 * t499 * t644 - 4.0 * t282 * t662 + 24.0 * t754 * t1174 - 36.0 * t418 * t329 * t346 + 6.0 * t227 * t1180 + 8.0 * t227 * t169 * t662 - t110 * (-5.0 / 64.0 * t46 / t47 / t199 * t65 + 15.0 / 32.0 * t117 * t624 * t28 * t166 + 15.0 / 8.0 * t84 * t318 * t27 * t343 + 81.0 * t197 * t154 * t307 * t447 - 2430.0 * t613 * t903 * t1118 * t150 + t1150 - t1153 + t1156 - t1159);
        let t1213 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t1209);
        let tv4sigma40 = 2.0 * rho[ip] * t1213;
        v4sigma4[ip] += tv4sigma40;
    }
}
