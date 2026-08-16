//! GGA_X_HJS vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hjs.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_hjs_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let t52 = f64::sqrt(sigma[ip]);
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
        let t176 = 0.1e-9 < t175;
        let t177 = piecewise3(t176, t175, 0.1e-9);
        let t178 = param_hyb_omega_0 * param_hyb_omega_0;
        let t179 = t178 * t3;
        let t180 = t26 * t26;
        let t182 = t32 / t180;
        let t183 = 1.0 / t38;
        let t185 = t179 * t182 * t183;
        let t187 = 0.60965e0 + t177 + t185 / 3.0;
        let t188 = f64::sqrt(t187);
        let t189 = 1.0 / t188;
        let t191 = t25 * t29 * t189;
        let t193 = 1.0 - t191 / 3.0;
        let t194 = 0.60965e0 + t177;
        let t195 = 1.0 / t194;
        let t198 = t33 * t46;
        let t200 = 1.0 + t198 / 96.0;
        let t201 = 1.0 / t200;
        let t202 = t41 * t201;
        let t206 = 1.0 + 0.13006513974354692214e-1 * t34 * t202 + 0.42141105276909202774e1 * t177;
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
        let t229 = f64::sqrt(t194);
        let t230 = t229 * t227;
        let t231 = f64::sqrt(M_PI);
        let t233 = f64::sqrt(t177);
        let t236 = 0.0 < 0.7572109999e0 + t177;
        let t238 = piecewise3(t236, 0.757211e0 + t177, 0.1e-9);
        let t239 = f64::sqrt(t238);
        let t241 = 4.0 / 5.0 * t231 + 12.0 / 5.0 * t233 - 12.0 / 5.0 * t239;
        let t243 = 0.474596e-1 * t206 * t194 + 0.28363733333333333333e-1 * t220 - 0.9086532e0 * t227 - t230 * t241;
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
        let t270 = f64::sqrt(t269);
        let t272 = 9.0 * t238 + t268;
        let t273 = f64::sqrt(t272);
        let t275 = t270 / 3.0 - t273 / 3.0;
        let t279 = t24 * t27;
        let t281 = t21 * t279 * t28;
        let t283 = t281 / 3.0 + t270 / 3.0;
        let t285 = t281 / 3.0 + t188;
        let t286 = 1.0 / t285;
        let t288 = f64::ln(t283 * t286);
        let t292 = t281 / 3.0 + t273 / 3.0;
        let t294 = f64::ln(t292 * t286);
        let t297 = 0.757211e0 + 0.47272888888888888889e-1 * t193 * t195 + 0.26366444444444444444e-1 * t219 * t221 - t263 * t264 / 9.0 + 2.0 / 3.0 * t25 * t29 * t275 + 2.0 * t177 * t288 - 2.0 * t238 * t294;
        let t301 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t297);
        let tzk0 = 2.0 * t301;
        zk[ip] += tzk0;
        let t302 = t17 * t183;
        let t306 = t27 * t115;
        let t308 = t25 * t306 * t189;
        let t310 = t37 * rho[ip];
        let t312 = 1.0 / t38 / t310;
        let t313 = t36 * t312;
        let t317 = t45 * t312;
        let t320 = 1.0 / t67;
        let t321 = t53 * t320;
        let t325 = 1.0 / t18 / t80;
        let t326 = t66 * t325;
        let t329 = t54 * t310;
        let t331 = 1.0 / t38 / t329;
        let t332 = t79 * t331;
        let t335 = 1.0 / t103;
        let t336 = t90 * t335;
        let t340 = 1.0 / t18 / t154;
        let t341 = t102 * t340;
        let t344 = -t44 * t317 / 9.0 - t51 * t321 / 6.0 - t64 * t326 / 54.0 - 5.0 / 432.0 * t77 * t332 - t89 * t336 / 72.0 - 7.0 / 5184.0 * t100 * t341;
        let t345 = t344 * t171;
        let t349 = t33 * t45;
        let t350 = t40 * t109;
        let t351 = t170 * t170;
        let t352 = 1.0 / t351;
        let t354 = 1.0 / t18 / t37;
        let t370 = t91 * t310;
        let t372 = 1.0 / t38 / t370;
        let t376 = t91 * t67;
        let t377 = 1.0 / t376;
        let t381 = -t112 * t113 * t354 / 9.0 - t121 * t317 / 9.0 - t125 * t321 / 6.0 - t130 * t326 / 54.0 - 5.0 / 432.0 * t135 * t332 - t139 * t336 / 72.0 - 7.0 / 5184.0 * t144 * t341 - t151 * t153 * t372 / 1296.0 - t163 * t164 * t377 / 1152.0;
        let t382 = t352 * t381;
        let t383 = t350 * t382;
        let t387 = piecewise3(t176, -t34 * t313 * t172 / 9.0 + t34 * t41 * t345 / 24.0 - t349 * t383 / 24.0, 0.0);
        let t389 = t179 * t182 * t254;
        let t391 = t387 - 2.0 / 9.0 * t389;
        let t392 = t214 * t391;
        let t394 = t25 * t29 * t392;
        let t396 = t308 / 9.0 + t394 / 6.0;
        let t399 = t193 * t221;
        let t402 = t313 * t201;
        let t405 = t60 * t63;
        let t406 = t405 * t65;
        let t408 = t200 * t200;
        let t409 = 1.0 / t408;
        let t410 = t35 * t325 * t409;
        let t414 = -0.34684037264945845904e-1 * t34 * t402 + 0.72258410968637178967e-3 * t406 * t410 + 0.42141105276909202774e1 * t387;
        let t415 = t414 * t218;
        let t420 = 1.0 / t37;
        let t423 = t208 * t210 * t420 * t214;
        let t425 = t208 * t210;
        let t426 = t211 * t258;
        let t428 = t425 * t426 * t391;
        let t430 = t308 / 3.0 + t394 / 2.0 - t423 / 3.0 - t428 / 2.0;
        let t431 = t206 * t430;
        let t434 = t264 * t387;
        let t441 = t194 * t387;
        let t445 = t229 * t220;
        let t446 = t445 * t241;
        let t449 = 1.0 / t233;
        let t451 = 1.0 / t239;
        let t452 = piecewise3(t236, t387, 0.0);
        let t455 = 6.0 / 5.0 * t449 * t387 - 6.0 / 5.0 * t451 * t452;
        let t457 = 0.474596e-1 * t414 * t194 + 0.474596e-1 * t206 * t387 + 0.56727466666666666666e-1 * t441 - 0.27259596e1 * t220 * t387 - 7.0 / 2.0 * t446 * t387 - t230 * t455;
        let t458 = t457 * t262;
        let t465 = t252 * t40;
        let t469 = t256 * t187;
        let t471 = 1.0 / t188 / t469;
        let t472 = t471 * t391;
        let t476 = 5.0 / 3.0 * t308 + 5.0 / 2.0 * t394 - 10.0 / 3.0 * t423 - 5.0 * t428 + 5.0 / 9.0 * t249 * t465 * t258 + 5.0 / 6.0 * t249 * t255 * t472;
        let t477 = t243 * t476;
        let t480 = t220 * t220;
        let t481 = 1.0 / t480;
        let t482 = t481 * t387;
        let t488 = 1.0 / t270;
        let t490 = 2.0 * t389;
        let t491 = 9.0 * t387 - t490;
        let t492 = t488 * t491;
        let t493 = 1.0 / t273;
        let t495 = 9.0 * t452 - t490;
        let t496 = t493 * t495;
        let t498 = t492 / 6.0 - t496 / 6.0;
        let t505 = t21 * t279 * t115;
        let t506 = t505 / 9.0;
        let t508 = -t506 + t492 / 6.0;
        let t510 = t285 * t285;
        let t511 = 1.0 / t510;
        let t512 = t283 * t511;
        let t515 = -t506 + t189 * t391 / 2.0;
        let t517 = t508 * t286 - t512 * t515;
        let t518 = t177 * t517;
        let t519 = 1.0 / t283;
        let t520 = t519 * t285;
        let t526 = -t506 + t496 / 6.0;
        let t528 = t292 * t511;
        let t530 = t526 * t286 - t528 * t515;
        let t531 = t238 * t530;
        let t532 = 1.0 / t292;
        let t533 = t532 * t285;
        let t536 = 0.47272888888888888889e-1 * t396 * t195 - 0.47272888888888888889e-1 * t399 * t387 + 0.26366444444444444444e-1 * t415 * t221 + 0.26366444444444444444e-1 * t431 * t221 - 0.52732888888888888888e-1 * t219 * t434 - t458 * t264 / 9.0 - t477 * t264 / 9.0 + t263 * t482 / 3.0 - 2.0 / 9.0 * t25 * t306 * t275 + 2.0 / 3.0 * t25 * t29 * t498 + 2.0 * t387 * t288 + 2.0 * t518 * t520 - 2.0 * t452 * t294 - 2.0 * t531 * t533;
        let t541 = piecewise3(t2, 0.0, -t6 * t302 * t297 / 8.0 - 3.0 / 8.0 * t6 * t19 * t536);
        let tvrho0 = 2.0 * rho[ip] * t541 + 2.0 * t301;
        vrho[ip] += tvrho0;
        let t544 = t21 * t279;
        let t545 = t28 * t214;
        let t546 = t33 * t36;
        let t549 = t32 * t36;
        let t550 = t549 * t40;
        let t553 = t52 * t55;
        let t556 = sigma[ip] * t35;
        let t557 = t556 * t69;
        let t560 = t53 * t36;
        let t561 = t560 * t82;
        let t564 = t65 * t92;
        let t567 = t78 * t35;
        let t568 = t567 * t105;
        let t571 = t43 * t550 / 24.0 + t51 * t553 / 16.0 + t64 * t557 / 144.0 + 5.0 / 1152.0 * t77 * t561 + t89 * t564 / 192.0 + 7.0 / 13824.0 * t100 * t568;
        let t572 = t571 * t171;
        let t575 = 1.0 / t52;
        let t576 = t575 * t35;
        let t592 = t90 * t36;
        let t599 = t112 * t576 * t115 / 24.0 + t120 * t550 / 24.0 + t125 * t553 / 16.0 + t130 * t557 / 144.0 + 5.0 / 1152.0 * t135 * t561 + t139 * t564 / 192.0 + 7.0 / 13824.0 * t144 * t568 + t151 * t592 * t156 / 3456.0 + t163 * t101 * t166 / 3072.0;
        let t600 = t352 * t599;
        let t601 = t350 * t600;
        let t605 = piecewise3(t176, t546 * t350 * t171 / 24.0 + t34 * t41 * t572 / 24.0 - t349 * t601 / 24.0, 0.0);
        let t606 = t605 * t195;
        let t616 = t35 * t69 * t409;
        let t620 = 0.13006513974354692214e-1 * t33 * t202 - 0.27096904113238942112e-3 * t405 * sigma[ip] * t616 + 0.42141105276909202774e1 * t605;
        let t621 = t620 * t218;
        let t624 = t214 * t605;
        let t626 = t25 * t29 * t624;
        let t628 = t425 * t426 * t605;
        let t630 = t626 / 2.0 - t628 / 2.0;
        let t631 = t206 * t630;
        let t634 = t264 * t605;
        let t641 = t194 * t605;
        let t648 = piecewise3(t236, t605, 0.0);
        let t651 = 6.0 / 5.0 * t449 * t605 - 6.0 / 5.0 * t451 * t648;
        let t653 = 0.474596e-1 * t620 * t194 + 0.474596e-1 * t206 * t605 + 0.56727466666666666666e-1 * t641 - 0.27259596e1 * t220 * t605 - 7.0 / 2.0 * t446 * t605 - t230 * t651;
        let t654 = t653 * t262;
        let t659 = t471 * t605;
        let t663 = 5.0 / 2.0 * t626 - 5.0 * t628 + 5.0 / 6.0 * t249 * t255 * t659;
        let t664 = t243 * t663;
        let t667 = t481 * t605;
        let t670 = t488 * t605;
        let t671 = t493 * t648;
        let t673 = 3.0 / 2.0 * t670 - 3.0 / 2.0 * t671;
        let t681 = t189 * t605;
        let t684 = 3.0 / 2.0 * t670 * t286 - t512 * t681 / 2.0;
        let t685 = t177 * t684;
        let t694 = 3.0 / 2.0 * t671 * t286 - t528 * t681 / 2.0;
        let t695 = t238 * t694;
        let t698 = 0.78788148148148148148e-2 * t544 * t545 * t606 - 0.47272888888888888889e-1 * t399 * t605 + 0.26366444444444444444e-1 * t621 * t221 + 0.26366444444444444444e-1 * t631 * t221 - 0.52732888888888888888e-1 * t219 * t634 - t654 * t264 / 9.0 - t664 * t264 / 9.0 + t263 * t667 / 3.0 + 2.0 / 3.0 * t25 * t29 * t673 + 2.0 * t605 * t288 + 2.0 * t685 * t520 - 2.0 * t648 * t294 - 2.0 * t695 * t533;
        let t702 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t698);
        let tvsigma0 = 2.0 * rho[ip] * t702;
        vsigma[ip] += tvsigma0;
    }
}
