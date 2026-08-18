//! LDA_C_PW fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pw.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_pw_fxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_a_0: f64,
    param_alpha1_0: f64,
    param_beta1_0: f64,
    param_beta2_0: f64,
    param_beta3_0: f64,
    param_pp_0: f64,
    param_beta4_0: f64,
    param_a_2: f64,
    param_alpha1_2: f64,
    param_beta1_2: f64,
    param_beta2_2: f64,
    param_beta3_2: f64,
    param_pp_2: f64,
    param_beta4_2: f64,
    param_fz20: f64,
    param_a_1: f64,
    param_alpha1_1: f64,
    param_beta1_1: f64,
    param_beta2_1: f64,
    param_beta3_1: f64,
    param_pp_1: f64,
    param_beta4_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = param_a_0;
        let t2 = param_alpha1_0;
        let t3 = M_CBRT3;
        let t4 = t2 * t3;
        let t5 = 1.0 / M_PI;
        let t6 = pow_1_3(t5);
        let t7 = M_CBRT4;
        let t8 = t7 * t7;
        let t9 = t6 * t8;
        let t10 = rho0 + rho1;
        let t11 = pow_1_3(t10);
        let t12 = 1.0 / t11;
        let t13 = t9 * t12;
        let t16 = 1.0 + t4 * t13 / 4.0;
        let t18 = 1.0 / t1;
        let t19 = param_beta1_0;
        let t20 = t3 * t6;
        let t22 = t20 * t8 * t12;
        let t23 = f64::sqrt(t22);
        let t27 = param_beta2_0 * t3;
        let t30 = param_beta3_0;
        let t31 = pow_3_2(t22);
        let t35 = t22 / 4.0;
        let t37 = param_pp_0 + 1.0;
        let t38 = f64::powf(t35, t37);
        let t39 = param_beta4_0 * t38;
        let t40 = t19 * t23 / 2.0 + t27 * t13 / 4.0 + 0.125 * t30 * t31 + t39;
        let t44 = 1.0 + t18 / t40 / 2.0;
        let t45 = f64::ln(t44);
        let t46 = t1 * t16 * t45;
        let t47 = 2.0 * t46;
        let t48 = rho0 - rho1;
        let t49 = t48 * t48;
        let t50 = t49 * t49;
        let t51 = t10 * t10;
        let t52 = t51 * t51;
        let t53 = 1.0 / t52;
        let t54 = t50 * t53;
        let t55 = 1.0 / t10;
        let t56 = t48 * t55;
        let t57 = 1.0 + t56;
        let t58 = t57 <= zeta_threshold;
        let t59 = pow_1_3(zeta_threshold);
        let t60 = t59 * zeta_threshold;
        let t61 = pow_1_3(t57);
        let t63 = piecewise3(t58, t60, t61 * t57);
        let t64 = 1.0 - t56;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3(t64);
        let t68 = piecewise3(t65, t60, t66 * t64);
        let t69 = t63 + t68 - 2.0;
        let t70 = M_CBRT2;
        let t73 = 1.0 / (2.0 * t70 - 2.0);
        let t74 = t69 * t73;
        let t75 = param_a_1;
        let t76 = param_alpha1_1;
        let t77 = t76 * t3;
        let t80 = 1.0 + t77 * t13 / 4.0;
        let t82 = 1.0 / t75;
        let t83 = param_beta1_1;
        let t87 = param_beta2_1 * t3;
        let t90 = param_beta3_1;
        let t95 = param_pp_1 + 1.0;
        let t96 = f64::powf(t35, t95);
        let t97 = param_beta4_1 * t96;
        let t98 = t83 * t23 / 2.0 + t87 * t13 / 4.0 + 0.125 * t90 * t31 + t97;
        let t102 = 1.0 + t82 / t98 / 2.0;
        let t103 = f64::ln(t102);
        let t105 = param_a_2;
        let t106 = param_alpha1_2;
        let t107 = t106 * t3;
        let t110 = 1.0 + t107 * t13 / 4.0;
        let t112 = 1.0 / t105;
        let t113 = param_beta1_2;
        let t117 = param_beta2_2 * t3;
        let t120 = param_beta3_2;
        let t125 = param_pp_2 + 1.0;
        let t126 = f64::powf(t35, t125);
        let t127 = param_beta4_2 * t126;
        let t128 = t113 * t23 / 2.0 + t117 * t13 / 4.0 + 0.125 * t120 * t31 + t127;
        let t132 = 1.0 + t112 / t128 / 2.0;
        let t133 = f64::ln(t132);
        let t134 = 1.0 / param_fz20;
        let t135 = t133 * t134;
        let t138 = -2.0 * t75 * t80 * t103 - 2.0 * t105 * t110 * t135 + 2.0 * t46;
        let t139 = t74 * t138;
        let t140 = t54 * t139;
        let t143 = t110 * t133 * t134;
        let t145 = 2.0 * t74 * t105 * t143;
        let tzk0 = -t47 + t140 + t145;
        zk[ip] += tzk0;
        let t147 = t1 * t2 * t3;
        let t149 = 1.0 / t11 / t10;
        let t152 = t147 * t9 * t149 * t45;
        let t153 = t152 / 6.0;
        let t154 = t40 * t40;
        let t155 = 1.0 / t154;
        let t156 = t16 * t155;
        let t157 = 1.0 / t23;
        let t159 = t19 * t157 * t3;
        let t160 = t9 * t149;
        let t165 = f64::sqrt(t22);
        let t167 = t30 * t165 * t3;
        let t173 = -t159 * t160 / 12.0 - t27 * t160 / 12.0 - 0.0625 * t167 * t160 - t39 * t37 * t55 / 3.0;
        let t174 = 1.0 / t44;
        let t175 = t173 * t174;
        let t176 = t156 * t175;
        let t177 = t49 * t48;
        let t178 = t177 * t53;
        let t179 = t178 * t139;
        let t180 = 4.0 * t179;
        let t181 = t52 * t10;
        let t182 = 1.0 / t181;
        let t183 = t50 * t182;
        let t184 = t183 * t139;
        let t185 = 4.0 * t184;
        let t186 = 1.0 / t51;
        let t187 = t48 * t186;
        let t188 = t55 - t187;
        let t191 = piecewise3(t58, 0.0, 4.0 / 3.0 * t61 * t188);
        let t192 = -t188;
        let t195 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t192);
        let t197 = (t191 + t195) * t73;
        let t198 = t197 * t138;
        let t199 = t54 * t198;
        let t201 = t75 * t76 * t3;
        let t206 = t98 * t98;
        let t207 = 1.0 / t206;
        let t208 = t80 * t207;
        let t210 = t83 * t157 * t3;
        let t216 = t90 * t165 * t3;
        let t222 = -t210 * t160 / 12.0 - t87 * t160 / 12.0 - 0.0625 * t216 * t160 - t97 * t95 * t55 / 3.0;
        let t223 = 1.0 / t102;
        let t224 = t222 * t223;
        let t226 = t105 * t106;
        let t227 = t226 * t20;
        let t228 = t8 * t149;
        let t232 = t128 * t128;
        let t233 = 1.0 / t232;
        let t234 = t110 * t233;
        let t236 = t113 * t157 * t3;
        let t242 = t120 * t165 * t3;
        let t248 = -t236 * t160 / 12.0 - t117 * t160 / 12.0 - 0.0625 * t242 * t160 - t127 * t125 * t55 / 3.0;
        let t249 = 1.0 / t132;
        let t251 = t248 * t249 * t134;
        let t253 = t201 * t9 * t149 * t103 / 6.0 + t208 * t224 - t153 - t176 + t227 * t228 * t135 / 6.0 + t234 * t251;
        let t254 = t74 * t253;
        let t255 = t54 * t254;
        let t257 = t197 * t105 * t143;
        let t258 = 2.0 * t257;
        let t259 = t226 * t3;
        let t260 = t74 * t259;
        let t263 = t9 * t149 * t133 * t134;
        let t264 = t260 * t263;
        let t265 = t264 / 6.0;
        let t266 = t74 * t110;
        let t268 = t249 * t134;
        let t269 = t233 * t248 * t268;
        let t270 = t266 * t269;
        let tvrho0 = -t47 + t140 + t145 + t10 * (t153 + t176 + t180 - t185 + t199 + t255 + t258 - t265 - t270);
        vrho[ip * 2] += tvrho0;
        let t273 = -t55 - t187;
        let t276 = piecewise3(t58, 0.0, 4.0 / 3.0 * t61 * t273);
        let t277 = -t273;
        let t280 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t277);
        let t282 = (t276 + t280) * t73;
        let t283 = t282 * t138;
        let t284 = t54 * t283;
        let t286 = t282 * t105 * t143;
        let t287 = 2.0 * t286;
        let tvrho1 = -t47 + t140 + t145 + t10 * (t153 + t176 - t180 - t185 + t284 + t255 + t287 - t265 - t270);
        vrho[ip * 2 + 1] += tvrho1;
        let t290 = t152 / 3.0;
        let t291 = 2.0 * t176;
        let t292 = 8.0 * t179;
        let t293 = 8.0 * t184;
        let t295 = 2.0 * t255;
        let t297 = t264 / 3.0;
        let t298 = 2.0 * t270;
        let t300 = 1.0 / t23 / t22;
        let t302 = t3 * t3;
        let t303 = t19 * t300 * t302;
        let t304 = t6 * t6;
        let t305 = t304 * t7;
        let t306 = t11 * t11;
        let t309 = t305 / t306 / t51;
        let t313 = 1.0 / t11 / t51;
        let t314 = t9 * t313;
        let t319 = 1.0/f64::sqrt(t22);
        let t321 = t30 * t319 * t302;
        let t326 = t37 * t37;
        let t333 = -t303 * t309 / 18.0 + t159 * t314 / 9.0 + t27 * t314 / 9.0 + 0.041666666666666664 * t321 * t309 + 0.08333333333333333 * t167 * t314 + t39 * t326 * t186 / 9.0 + t39 * t37 * t186 / 3.0;
        let t334 = t333 * t174;
        let t335 = t156 * t334;
        let t336 = t49 * t53;
        let t337 = t336 * t139;
        let t338 = 12.0 * t337;
        let t339 = t177 * t182;
        let t340 = t339 * t139;
        let t341 = 32.0 * t340;
        let t343 = 1.0 / t52 / t51;
        let t344 = t50 * t343;
        let t345 = t344 * t139;
        let t346 = 20.0 * t345;
        let t347 = t154 * t40;
        let t348 = 1.0 / t347;
        let t349 = t16 * t348;
        let t350 = t173 * t173;
        let t351 = t350 * t174;
        let t352 = t349 * t351;
        let t353 = 2.0 * t352;
        let t354 = t154 * t154;
        let t355 = 1.0 / t354;
        let t356 = t16 * t355;
        let t357 = t44 * t44;
        let t358 = 1.0 / t357;
        let t360 = t350 * t358 * t18;
        let t361 = t356 * t360;
        let t362 = t361 / 2.0;
        let t363 = t178 * t198;
        let t364 = 8.0 * t363;
        let t365 = t178 * t254;
        let t366 = 8.0 * t365;
        let t367 = t183 * t198;
        let t368 = 8.0 * t367;
        let t369 = t183 * t254;
        let t370 = 8.0 * t369;
        let t371 = t61 * t61;
        let t372 = 1.0 / t371;
        let t373 = t188 * t188;
        let t376 = t51 * t10;
        let t377 = 1.0 / t376;
        let t378 = t48 * t377;
        let t380 = -2.0 * t186 + 2.0 * t378;
        let t384 = piecewise3(t58, 0.0, 4.0 / 9.0 * t372 * t373 + 4.0 / 3.0 * t61 * t380);
        let t385 = t66 * t66;
        let t386 = 1.0 / t385;
        let t387 = t192 * t192;
        let t390 = -t380;
        let t394 = piecewise3(t65, 0.0, 4.0 / 9.0 * t386 * t387 + 4.0 / 3.0 * t66 * t390);
        let t396 = (t384 + t394) * t73;
        let t397 = t396 * t138;
        let t398 = t54 * t397;
        let t399 = t335 + t338 - t341 + t346 - t353 + t362 + t364 + t366 - t368 - t370 + t398;
        let t400 = t197 * t253;
        let t401 = t54 * t400;
        let t402 = 2.0 * t401;
        let t407 = t77 * t9;
        let t408 = t149 * t207;
        let t412 = t206 * t98;
        let t413 = 1.0 / t412;
        let t414 = t80 * t413;
        let t415 = t222 * t222;
        let t416 = t415 * t223;
        let t420 = t83 * t300 * t302;
        let t428 = t90 * t319 * t302;
        let t433 = t95 * t95;
        let t440 = -t420 * t309 / 18.0 + t210 * t314 / 9.0 + t87 * t314 / 9.0 + 0.041666666666666664 * t428 * t309 + 0.08333333333333333 * t216 * t314 + t97 * t433 * t186 / 9.0 + t97 * t95 * t186 / 3.0;
        let t441 = t440 * t223;
        let t443 = t206 * t206;
        let t444 = 1.0 / t443;
        let t445 = t80 * t444;
        let t446 = t102 * t102;
        let t447 = 1.0 / t446;
        let t449 = t415 * t447 * t82;
        let t454 = t147 * t9 * t313 * t45;
        let t455 = 2.0 / 9.0 * t454;
        let t456 = t4 * t9;
        let t457 = t149 * t155;
        let t459 = t456 * t457 * t175;
        let t460 = t459 / 6.0;
        let t461 = t8 * t313;
        let t465 = t107 * t9;
        let t466 = t149 * t233;
        let t470 = t232 * t128;
        let t471 = 1.0 / t470;
        let t472 = t110 * t471;
        let t473 = t248 * t248;
        let t474 = t473 * t249;
        let t475 = t474 * t134;
        let t479 = t113 * t300 * t302;
        let t487 = t120 * t319 * t302;
        let t492 = t125 * t125;
        let t499 = -t479 * t309 / 18.0 + t236 * t314 / 9.0 + t117 * t314 / 9.0 + 0.041666666666666664 * t487 * t309 + 0.08333333333333333 * t242 * t314 + t127 * t492 * t186 / 9.0 + t127 * t125 * t186 / 3.0;
        let t500 = t499 * t249;
        let t501 = t500 * t134;
        let t503 = t232 * t232;
        let t504 = 1.0 / t503;
        let t505 = t110 * t504;
        let t506 = t505 * t473;
        let t507 = t132 * t132;
        let t508 = 1.0 / t507;
        let t509 = t508 * t134;
        let t510 = t509 * t112;
        let t513 = -2.0 / 9.0 * t201 * t9 * t313 * t103 - t407 * t408 * t224 / 6.0 - 2.0 * t414 * t416 + t208 * t441 + t445 * t449 / 2.0 + t455 + t460 + t353 - t335 - t362 - 2.0 / 9.0 * t227 * t461 * t135 - t465 * t466 * t251 / 6.0 - 2.0 * t472 * t475 + t234 * t501 + t506 * t510 / 2.0;
        let t514 = t74 * t513;
        let t515 = t54 * t514;
        let t516 = t197 * t110;
        let t517 = t516 * t269;
        let t518 = 2.0 * t517;
        let t520 = t233 * t499 * t268;
        let t521 = t266 * t520;
        let t522 = t197 * t259;
        let t523 = t522 * t263;
        let t524 = t523 / 3.0;
        let t526 = t471 * t473 * t268;
        let t527 = t266 * t526;
        let t528 = 2.0 * t527;
        let t529 = t74 * t505;
        let t530 = t473 * t508;
        let t531 = t134 * t112;
        let t532 = t530 * t531;
        let t533 = t529 * t532;
        let t534 = t533 / 2.0;
        let t536 = t396 * t105 * t143;
        let t537 = 2.0 * t536;
        let t540 = t9 * t313 * t133 * t134;
        let t541 = t260 * t540;
        let t542 = 2.0 / 9.0 * t541;
        let t543 = t107 * t6;
        let t544 = t74 * t543;
        let t545 = t228 * t233;
        let t546 = t545 * t251;
        let t547 = t544 * t546;
        let t548 = t547 / 6.0;
        let t549 = t402 + t515 - t518 - t521 - t524 - t455 - t460 + t528 - t534 + t537 + t542 + t548;
        let tv2rho20 = t290 + t291 + t292 - t293 + 2.0 * t199 + t295 + 4.0 * t257 - t297 - t298 + t10 * (t399 + t549);
        v2rho2[ip * 3] += tv2rho20;
        let t552 = t372 * t273;
        let t555 = t61 * t48;
        let t559 = piecewise3(t58, 0.0, 4.0 / 9.0 * t552 * t188 + 8.0 / 3.0 * t555 * t377);
        let t560 = t386 * t277;
        let t563 = t66 * t48;
        let t567 = piecewise3(t65, 0.0, 4.0 / 9.0 * t560 * t192 - 8.0 / 3.0 * t563 * t377);
        let t569 = (t559 + t567) * t73;
        let t571 = t569 * t105 * t143;
        let t573 = t282 * t259;
        let t574 = t573 * t263;
        let t577 = t569 * t138;
        let t578 = t54 * t577;
        let t579 = t282 * t253;
        let t580 = t54 * t579;
        let t581 = t178 * t283;
        let t583 = t548 + 2.0 * t571 - t353 + t335 - t460 - t534 - t574 / 6.0 - t523 / 6.0 + t542 - t455 + t578 + t580 + 4.0 * t581;
        let t584 = t183 * t283;
        let t588 = t282 * t110;
        let t589 = t588 * t269;
        let t590 = -4.0 * t584 - t338 + t346 + t362 - 4.0 * t363 - 4.0 * t367 - t517 - t521 + t528 - t589 - t370 + t401 + t515;
        let tv2rho21 = t290 + t291 - t293 + t199 + t295 + t258 - t297 - t298 + t284 + t287 + t10 * (t583 + t590);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t595 = t273 * t273;
        let t599 = 2.0 * t186 + 2.0 * t378;
        let t603 = piecewise3(t58, 0.0, 4.0 / 9.0 * t372 * t595 + 4.0 / 3.0 * t61 * t599);
        let t604 = t277 * t277;
        let t607 = -t599;
        let t611 = piecewise3(t65, 0.0, 4.0 / 9.0 * t386 * t604 + 4.0 / 3.0 * t66 * t607);
        let t613 = (t603 + t611) * t73;
        let t614 = t613 * t138;
        let t615 = t54 * t614;
        let t617 = t613 * t105 * t143;
        let t618 = 2.0 * t617;
        let t619 = t615 + t618 + t335 + t338 + t341 + t346 - t353 + t362 - t366 - t370 + t515;
        let t620 = 2.0 * t589;
        let t621 = t574 / 3.0;
        let t622 = 2.0 * t580;
        let t623 = 8.0 * t581;
        let t624 = 8.0 * t584;
        let t625 = -t620 - t521 - t621 - t455 - t460 + t528 - t534 + t622 - t623 - t624 + t542 + t548;
        let tv2rho22 = t290 + t291 - t292 - t293 + 2.0 * t284 + t295 + 4.0 * t286 - t297 - t298 + t10 * (t619 + t625);
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
