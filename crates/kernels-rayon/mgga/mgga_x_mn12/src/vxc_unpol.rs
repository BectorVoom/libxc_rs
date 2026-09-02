//! MGGA_X_MN12 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mn12.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mn12_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c_1: f64,
    param_c_2: f64,
    param_c_3: f64,
    param_c_4: f64,
    param_c_5: f64,
    param_c_7: f64,
    param_c_8: f64,
    param_c_9: f64,
    param_c_10: f64,
    param_c_6: f64,
    param_c_12: f64,
    param_c_13: f64,
    param_c_14: f64,
    param_c_11: f64,
    param_c_16: f64,
    param_c_17: f64,
    param_c_15: f64,
    param_c_19: f64,
    param_c_20: f64,
    param_c_21: f64,
    param_c_22: f64,
    param_c_18: f64,
    param_c_24: f64,
    param_c_25: f64,
    param_c_26: f64,
    param_c_23: f64,
    param_c_28: f64,
    param_c_29: f64,
    param_c_27: f64,
    param_c_31: f64,
    param_c_32: f64,
    param_c_33: f64,
    param_c_30: f64,
    param_c_35: f64,
    param_c_36: f64,
    param_c_34: f64,
    param_c_38: f64,
    param_c_39: f64,
    param_c_37: f64,
    param_c_0: f64,
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
        let t22 = param_c_1;
        let t23 = M_CBRT6;
        let t24 = t23 * t23;
        let t25 = M_PI * M_PI;
        let t26 = pow_1_3(t25);
        let t27 = t26 * t26;
        let t29 = 3.0 / 10.0 * t24 * t27;
        let t30 = M_CBRT2;
        let t31 = t30 * t30;
        let t32 = tau[ip] * t31;
        let t33 = t19 * t19;
        let t35 = 1.0 / t33 / rho[ip];
        let t36 = t32 * t35;
        let t37 = t29 - t36;
        let t38 = t22 * t37;
        let t39 = t29 + t36;
        let t40 = 1.0 / t39;
        let t42 = param_c_2;
        let t43 = t37 * t37;
        let t44 = t42 * t43;
        let t45 = t39 * t39;
        let t46 = 1.0 / t45;
        let t48 = param_c_3;
        let t49 = t43 * t37;
        let t50 = t48 * t49;
        let t51 = t45 * t39;
        let t52 = 1.0 / t51;
        let t54 = param_c_4;
        let t55 = t43 * t43;
        let t56 = t54 * t55;
        let t57 = t45 * t45;
        let t58 = 1.0 / t57;
        let t60 = param_c_5;
        let t62 = t60 * t55 * t37;
        let t64 = 1.0 / t57 / t39;
        let t67 = param_c_7;
        let t68 = t67 * t37;
        let t70 = param_c_8;
        let t71 = t70 * t43;
        let t73 = param_c_9;
        let t74 = t73 * t49;
        let t76 = param_c_10;
        let t77 = t76 * t55;
        let t79 = t68 * t40 + t71 * t46 + t74 * t52 + t77 * t58 + param_c_6;
        let t80 = t79 * sigma[ip];
        let t81 = rho[ip] * rho[ip];
        let t83 = 1.0 / t33 / t81;
        let t84 = t31 * t83;
        let t85 = sigma[ip] * t31;
        let t88 = 1.0 + 0.004 * t85 * t83;
        let t89 = 1.0 / t88;
        let t90 = t84 * t89;
        let t94 = param_c_12;
        let t95 = t94 * t37;
        let t97 = param_c_13;
        let t98 = t97 * t43;
        let t100 = param_c_14;
        let t101 = t100 * t49;
        let t103 = t101 * t52 + t95 * t40 + t98 * t46 + param_c_11;
        let t104 = sigma[ip] * sigma[ip];
        let t105 = t103 * t104;
        let t106 = t81 * t81;
        let t107 = t106 * rho[ip];
        let t109 = 1.0 / t19 / t107;
        let t110 = t30 * t109;
        let t111 = t88 * t88;
        let t112 = 1.0 / t111;
        let t113 = t110 * t112;
        let t117 = param_c_16;
        let t118 = t117 * t37;
        let t120 = param_c_17;
        let t121 = t120 * t43;
        let t123 = t118 * t40 + t121 * t46 + param_c_15;
        let t124 = t104 * sigma[ip];
        let t125 = t123 * t124;
        let t126 = t106 * t106;
        let t127 = 1.0 / t126;
        let t128 = t111 * t88;
        let t129 = 1.0 / t128;
        let t130 = t127 * t129;
        let t134 = param_c_19;
        let t135 = t134 * t37;
        let t137 = param_c_20;
        let t138 = t137 * t43;
        let t140 = param_c_21;
        let t141 = t140 * t49;
        let t143 = param_c_22;
        let t144 = t143 * t55;
        let t146 = t135 * t40 + t138 * t46 + t141 * t52 + t144 * t58 + param_c_18;
        let t149 = 1.0 / t12;
        let t150 = pow_1_3(t149);
        let t153 = 1.0 + 0.4 / t19 * t30 * t150;
        let t154 = 1.0 / t153;
        let t157 = param_c_24;
        let t158 = t157 * t37;
        let t160 = param_c_25;
        let t161 = t160 * t43;
        let t163 = param_c_26;
        let t164 = t163 * t49;
        let t166 = t158 * t40 + t161 * t46 + t164 * t52 + param_c_23;
        let t167 = t166 * sigma[ip];
        let t168 = t167 * t31;
        let t169 = t83 * t89;
        let t170 = t169 * t154;
        let t174 = param_c_28;
        let t175 = t174 * t37;
        let t177 = param_c_29;
        let t178 = t177 * t43;
        let t180 = t175 * t40 + t178 * t46 + param_c_27;
        let t181 = t180 * t104;
        let t182 = t181 * t30;
        let t183 = t109 * t112;
        let t184 = t183 * t154;
        let t188 = param_c_31;
        let t189 = t188 * t37;
        let t191 = param_c_32;
        let t192 = t191 * t43;
        let t194 = param_c_33;
        let t195 = t194 * t49;
        let t197 = t189 * t40 + t192 * t46 + t195 * t52 + param_c_30;
        let t198 = t153 * t153;
        let t199 = 1.0 / t198;
        let t202 = param_c_35;
        let t203 = t202 * t37;
        let t205 = param_c_36;
        let t206 = t205 * t43;
        let t208 = t203 * t40 + t206 * t46 + param_c_34;
        let t209 = t208 * sigma[ip];
        let t210 = t209 * t31;
        let t211 = t169 * t199;
        let t215 = param_c_38;
        let t216 = t215 * t37;
        let t218 = param_c_39;
        let t219 = t218 * t43;
        let t221 = t216 * t40 + t219 * t46 + param_c_37;
        let t222 = t198 * t153;
        let t223 = 1.0 / t222;
        let t225 = param_c_0 + t38 * t40 + t44 * t46 + t50 * t52 + t56 * t58 + t62 * t64 + 0.004 * t80 * t90 + 3.2e-05 * t105 * t113 + 2.56e-07 * t125 * t130 + t146 * t154 + 0.004 * t168 * t170 + 3.2e-05 * t182 * t184 + t197 * t199 + 0.004 * t210 * t211 + t221 * t223;
        let t229 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t225);
        let tzk0 = 2.0 * t229;
        zk[ip] += tzk0;
        let t231 = t18 / t33;
        let t235 = t67 * tau[ip];
        let t236 = t84 * t40;
        let t239 = t68 * t46;
        let t240 = t32 * t83;
        let t243 = t70 * t37;
        let t244 = t243 * t46;
        let t247 = t71 * t52;
        let t250 = t73 * t43;
        let t251 = t250 * t52;
        let t254 = t74 * t58;
        let t257 = t76 * t49;
        let t258 = t257 * t58;
        let t261 = t77 * t64;
        let t264 = 5.0 / 3.0 * t235 * t236 + 5.0 / 3.0 * t239 * t240 + 10.0 / 3.0 * t244 * t240 + 10.0 / 3.0 * t247 * t240 + 5.0 * t251 * t240 + 5.0 * t254 * t240 + 20.0 / 3.0 * t258 * t240 + 20.0 / 3.0 * t261 * t240;
        let t265 = t264 * sigma[ip];
        let t268 = t79 * t104;
        let t269 = t106 * t81;
        let t271 = 1.0 / t19 / t269;
        let t272 = t30 * t271;
        let t273 = t272 * t112;
        let t276 = t94 * tau[ip];
        let t279 = t95 * t46;
        let t282 = t97 * t37;
        let t283 = t282 * t46;
        let t286 = t98 * t52;
        let t289 = t100 * t43;
        let t290 = t289 * t52;
        let t293 = t101 * t58;
        let t296 = 5.0 / 3.0 * t276 * t236 + 5.0 / 3.0 * t279 * t240 + 10.0 / 3.0 * t283 * t240 + 10.0 / 3.0 * t286 * t240 + 5.0 * t290 * t240 + 5.0 * t293 * t240;
        let t297 = t296 * t104;
        let t300 = t81 * rho[ip];
        let t302 = 1.0 / t33 / t300;
        let t303 = t31 * t302;
        let t304 = t303 * t89;
        let t309 = t104 * t104;
        let t310 = t123 * t309;
        let t311 = t126 * t300;
        let t313 = 1.0 / t33 / t311;
        let t314 = t111 * t111;
        let t315 = 1.0 / t314;
        let t317 = t313 * t315 * t31;
        let t320 = t146 * t199;
        let t324 = 1.0 / t19 / rho[ip] * t30 * t150;
        let t327 = t180 * t124;
        let t328 = t126 * rho[ip];
        let t329 = 1.0 / t328;
        let t330 = t329 * t129;
        let t331 = t330 * t154;
        let t334 = t197 * t223;
        let t337 = t198 * t198;
        let t338 = 1.0 / t337;
        let t339 = t221 * t338;
        let t342 = t22 * tau[ip];
        let t345 = t117 * tau[ip];
        let t348 = t118 * t46;
        let t351 = t120 * t37;
        let t352 = t351 * t46;
        let t355 = t121 * t52;
        let t358 = 5.0 / 3.0 * t345 * t236 + 5.0 / 3.0 * t348 * t240 + 10.0 / 3.0 * t352 * t240 + 10.0 / 3.0 * t355 * t240;
        let t359 = t358 * t124;
        let t364 = t103 * t124;
        let t367 = t48 * t43;
        let t368 = t367 * t52;
        let t371 = t50 * t58;
        let t374 = t54 * t49;
        let t375 = t374 * t58;
        let t378 = t56 * t64;
        let t381 = 0.004 * t265 * t90 + 8.533333333333334e-05 * t268 * t273 + 3.2e-05 * t297 * t113 - 0.010666666666666666 * t80 * t304 - 0.00017066666666666668 * t105 * t273 + 8.192e-09 * t310 * t317 + 0.13333333333333333 * t320 * t324 + 1.3653333333333333e-06 * t327 * t331 + 0.26666666666666666 * t334 * t324 + 0.4 * t339 * t324 + 5.0 / 3.0 * t342 * t236 + 2.56e-07 * t359 * t130 - 2.048e-06 * t125 * t330 + 1.3653333333333333e-06 * t364 * t330 + 5.0 * t368 * t240 + 5.0 * t371 * t240 + 20.0 / 3.0 * t375 * t240 + 20.0 / 3.0 * t378 * t240;
        let t382 = t60 * t55;
        let t383 = t382 * t64;
        let t387 = 1.0 / t57 / t45;
        let t388 = t62 * t387;
        let t391 = t302 * t89;
        let t392 = t391 * t154;
        let t395 = t271 * t112;
        let t396 = t395 * t154;
        let t399 = t391 * t199;
        let t402 = t166 * t104;
        let t403 = t402 * t30;
        let t406 = 1.0 / t106;
        let t409 = t89 * t199 * t150;
        let t412 = t174 * tau[ip];
        let t415 = t175 * t46;
        let t418 = t177 * t37;
        let t419 = t418 * t46;
        let t422 = t178 * t52;
        let t425 = 5.0 / 3.0 * t412 * t236 + 5.0 / 3.0 * t415 * t240 + 10.0 / 3.0 * t419 * t240 + 10.0 / 3.0 * t422 * t240;
        let t426 = t425 * t104;
        let t427 = t426 * t30;
        let t430 = t202 * tau[ip];
        let t433 = t203 * t46;
        let t436 = t205 * t37;
        let t437 = t436 * t46;
        let t440 = t206 * t52;
        let t443 = 5.0 / 3.0 * t430 * t236 + 5.0 / 3.0 * t433 * t240 + 10.0 / 3.0 * t437 * t240 + 10.0 / 3.0 * t440 * t240;
        let t444 = t443 * sigma[ip];
        let t445 = t444 * t31;
        let t448 = t208 * t104;
        let t449 = t448 * t30;
        let t450 = t395 * t199;
        let t454 = t89 * t223;
        let t455 = t454 * t150;
        let t458 = t157 * tau[ip];
        let t461 = t158 * t46;
        let t464 = t160 * t37;
        let t465 = t464 * t46;
        let t468 = t161 * t52;
        let t471 = t163 * t43;
        let t472 = t471 * t52;
        let t475 = t164 * t58;
        let t478 = 5.0 / 3.0 * t458 * t236 + 5.0 / 3.0 * t461 * t240 + 10.0 / 3.0 * t465 * t240 + 10.0 / 3.0 * t468 * t240 + 5.0 * t472 * t240 + 5.0 * t475 * t240;
        let t479 = t478 * sigma[ip];
        let t480 = t479 * t31;
        let t483 = t38 * t46;
        let t486 = t42 * t37;
        let t487 = t486 * t46;
        let t490 = t44 * t52;
        let t493 = t188 * tau[ip];
        let t496 = t189 * t46;
        let t499 = t191 * t37;
        let t500 = t499 * t46;
        let t503 = t192 * t52;
        let t506 = t194 * t43;
        let t507 = t506 * t52;
        let t510 = t195 * t58;
        let t513 = 5.0 / 3.0 * t493 * t236 + 5.0 / 3.0 * t496 * t240 + 10.0 / 3.0 * t500 * t240 + 10.0 / 3.0 * t503 * t240 + 5.0 * t507 * t240 + 5.0 * t510 * t240;
        let t515 = t215 * tau[ip];
        let t518 = t216 * t46;
        let t521 = t218 * t37;
        let t522 = t521 * t46;
        let t525 = t219 * t52;
        let t528 = 5.0 / 3.0 * t515 * t236 + 5.0 / 3.0 * t518 * t240 + 10.0 / 3.0 * t522 * t240 + 10.0 / 3.0 * t525 * t240;
        let t530 = t134 * tau[ip];
        let t533 = t135 * t46;
        let t536 = t137 * t37;
        let t537 = t536 * t46;
        let t540 = t138 * t52;
        let t543 = t140 * t43;
        let t544 = t543 * t52;
        let t547 = t141 * t58;
        let t550 = t143 * t49;
        let t551 = t550 * t58;
        let t554 = t144 * t64;
        let t557 = 5.0 / 3.0 * t530 * t236 + 5.0 / 3.0 * t533 * t240 + 10.0 / 3.0 * t537 * t240 + 10.0 / 3.0 * t540 * t240 + 5.0 * t544 * t240 + 5.0 * t547 * t240 + 20.0 / 3.0 * t551 * t240 + 20.0 / 3.0 * t554 * t240;
        let t559 = t181 * t31;
        let t561 = 1.0 / t33 / t269;
        let t562 = t561 * t112;
        let t563 = t199 * t150;
        let t564 = t562 * t563;
        let t567 = 25.0 / 3.0 * t383 * t240 + 25.0 / 3.0 * t388 * t240 - 0.010666666666666666 * t168 * t392 - 0.00017066666666666668 * t182 * t396 - 0.010666666666666666 * t210 * t399 + 8.533333333333334e-05 * t403 * t396 + 0.0010666666666666667 * t167 * t406 * t409 + 3.2e-05 * t427 * t184 + 0.004 * t445 * t211 + 8.533333333333334e-05 * t449 * t450 + 0.0021333333333333334 * t209 * t406 * t455 + 0.004 * t480 * t170 + 5.0 / 3.0 * t483 * t240 + 10.0 / 3.0 * t487 * t240 + 10.0 / 3.0 * t490 * t240 + t513 * t199 + t528 * t223 + t557 * t154 + 4.266666666666667e-06 * t559 * t564;
        let t568 = t381 + t567;
        let t573 = piecewise3(t3, 0.0, -t7 * t231 * t225 / 8.0 - 3.0 / 8.0 * t7 * t20 * t568);
        let tvrho0 = 2.0 * rho[ip] * t573 + 2.0 * t229;
        vrho[ip] += tvrho0;
        let t576 = t79 * t31;
        let t581 = t103 * sigma[ip];
        let t586 = t123 * t104;
        let t589 = t126 * t81;
        let t591 = 1.0 / t33 / t589;
        let t593 = t591 * t315 * t31;
        let t596 = t166 * t31;
        let t602 = t180 * sigma[ip];
        let t603 = t602 * t30;
        let t606 = t130 * t154;
        let t609 = t208 * t31;
        let t613 = t183 * t199;
        let t616 = 0.004 * t576 * t169 - 3.2e-05 * t80 * t113 + 6.4e-05 * t581 * t113 - 5.12e-07 * t105 * t130 + 7.68e-07 * t586 * t130 - 3.072e-09 * t125 * t593 + 0.004 * t596 * t170 - 3.2e-05 * t167 * t30 * t184 + 6.4e-05 * t603 * t184 - 5.12e-07 * t181 * t606 + 0.004 * t609 * t211 - 3.2e-05 * t209 * t30 * t613;
        let t620 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t616);
        let tvsigma0 = 2.0 * rho[ip] * t620;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t622 = t22 * t31;
        let t623 = t35 * t40;
        let t625 = t46 * t31;
        let t626 = t625 * t35;
        let t630 = t52 * t31;
        let t631 = t630 * t35;
        let t636 = t58 * t31;
        let t637 = t636 * t35;
        let t642 = t64 * t31;
        let t643 = t642 * t35;
        let t648 = t387 * t31;
        let t652 = t67 * t31;
        let t667 = -2.0 * t243 * t626 - 3.0 * t250 * t631 - 4.0 * t257 * t637 - t652 * t623 - t68 * t626 - 2.0 * t71 * t631 - 3.0 * t74 * t637 - 4.0 * t77 * t643;
        let t668 = t667 * sigma[ip];
        let t671 = t94 * t31;
        let t682 = -3.0 * t101 * t637 - 2.0 * t282 * t626 - 3.0 * t289 * t631 - t671 * t623 - t95 * t626 - 2.0 * t98 * t631;
        let t683 = t682 * t104;
        let t686 = t117 * t31;
        let t693 = -t118 * t626 - 2.0 * t121 * t631 - 2.0 * t351 * t626 - t686 * t623;
        let t694 = t693 * t124;
        let t697 = t134 * t31;
        let t712 = -t135 * t626 - 2.0 * t138 * t631 - 3.0 * t141 * t637 - 4.0 * t144 * t643 - 2.0 * t536 * t626 - 3.0 * t543 * t631 - 4.0 * t550 * t637 - t697 * t623;
        let t714 = t157 * t31;
        let t725 = -t158 * t626 - 2.0 * t161 * t631 - 3.0 * t164 * t637 - 2.0 * t464 * t626 - 3.0 * t471 * t631 - t714 * t623;
        let t726 = t725 * sigma[ip];
        let t727 = t726 * t31;
        let t730 = t174 * t31;
        let t737 = -t175 * t626 - 2.0 * t178 * t631 - 2.0 * t418 * t626 - t730 * t623;
        let t738 = t737 * t104;
        let t739 = t738 * t30;
        let t742 = t188 * t31;
        let t753 = -t189 * t626 - 2.0 * t192 * t631 - 3.0 * t195 * t637 - 2.0 * t499 * t626 - 3.0 * t506 * t631 - t742 * t623;
        let t755 = t202 * t31;
        let t762 = -t203 * t626 - 2.0 * t206 * t631 - 2.0 * t436 * t626 - t755 * t623;
        let t763 = t762 * sigma[ip];
        let t764 = t763 * t31;
        let t767 = t215 * t31;
        let t774 = -t216 * t626 - 2.0 * t219 * t631 - 2.0 * t521 * t626 - t767 * t623;
        let t776 = -t622 * t623 - t38 * t626 - 2.0 * t486 * t626 - 2.0 * t44 * t631 - 3.0 * t367 * t631 - 3.0 * t50 * t637 - 4.0 * t374 * t637 - 4.0 * t56 * t643 - 5.0 * t382 * t643 - 5.0 * t62 * t648 * t35 + 0.004 * t668 * t90 + 3.2e-05 * t683 * t113 + 2.56e-07 * t694 * t130 + t712 * t154 + 0.004 * t727 * t170 + 3.2e-05 * t739 * t184 + t753 * t199 + 0.004 * t764 * t211 + t774 * t223;
        let t780 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t776);
        let tvtau0 = 2.0 * rho[ip] * t780;
        vtau[ip] += tvtau0;
    }
}
