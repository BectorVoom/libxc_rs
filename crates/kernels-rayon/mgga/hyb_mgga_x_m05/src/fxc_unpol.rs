//! HYB_MGGA_X_M05 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_m05.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_mgga_x_m05_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    param_csi_HF: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_a_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t4 / t5 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t21 = t20 * param_csi_HF;
        let t22 = M_CBRT6;
        let t23 = M_PI * M_PI;
        let t24 = pow_1_3(t23);
        let t25 = t24 * t24;
        let t26 = 1.0 / t25;
        let t27 = t22 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = sigma[ip] * t29;
        let t31 = rho[ip] * rho[ip];
        let t32 = t20 * t20;
        let t34 = 1.0 / t32 / t31;
        let t38 = 0.804 + 0.009146457198521547 * t27 * t30 * t34;
        let t41 = 1.804 - 0.646416 / t38;
        let t43 = param_a_1;
        let t44 = t22 * t22;
        let t46 = 3.0 / 10.0 * t44 * t25;
        let t47 = tau[ip] * t29;
        let t49 = 1.0 / t32 / rho[ip];
        let t50 = t47 * t49;
        let t51 = t46 - t50;
        let t52 = t43 * t51;
        let t53 = t46 + t50;
        let t54 = 1.0 / t53;
        let t56 = param_a_2;
        let t57 = t51 * t51;
        let t58 = t56 * t57;
        let t59 = t53 * t53;
        let t60 = 1.0 / t59;
        let t62 = param_a_3;
        let t63 = t57 * t51;
        let t64 = t62 * t63;
        let t65 = t59 * t53;
        let t66 = 1.0 / t65;
        let t68 = param_a_4;
        let t69 = t57 * t57;
        let t70 = t68 * t69;
        let t71 = t59 * t59;
        let t72 = 1.0 / t71;
        let t74 = param_a_5;
        let t75 = t69 * t51;
        let t76 = t74 * t75;
        let t77 = t71 * t53;
        let t78 = 1.0 / t77;
        let t80 = param_a_6;
        let t81 = t69 * t57;
        let t82 = t80 * t81;
        let t83 = t71 * t59;
        let t84 = 1.0 / t83;
        let t86 = param_a_7;
        let t87 = t69 * t63;
        let t88 = t86 * t87;
        let t89 = t71 * t65;
        let t90 = 1.0 / t89;
        let t92 = param_a_8;
        let t93 = t69 * t69;
        let t94 = t92 * t93;
        let t95 = t71 * t71;
        let t96 = 1.0 / t95;
        let t98 = param_a_9;
        let t99 = t93 * t51;
        let t100 = t98 * t99;
        let t102 = 1.0 / t95 / t53;
        let t104 = param_a_10;
        let t105 = t93 * t57;
        let t106 = t104 * t105;
        let t108 = 1.0 / t95 / t59;
        let t110 = param_a_11;
        let t112 = t110 * t93 * t63;
        let t114 = 1.0 / t95 / t65;
        let t116 = t100 * t102 + t106 * t108 + t112 * t114 + t52 * t54 + t58 * t60 + t64 * t66 + t70 * t72 + t76 * t78 + t82 * t84 + t88 * t90 + t94 * t96 + param_a_0;
        let t117 = t41 * t116;
        let t121 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t21 * t117);
        let tzk0 = 2.0 * t121;
        zk[ip] += tzk0;
        let t123 = 1.0 / t32 * param_csi_HF;
        let t127 = t4 * t18;
        let t128 = t31 * rho[ip];
        let t130 = 1.0 / t20 / t128;
        let t131 = t130 * param_csi_HF;
        let t132 = t38 * t38;
        let t133 = 1.0 / t132;
        let t135 = t127 * t131 * t133;
        let t137 = t27 * t30 * t116;
        let t140 = t43 * tau[ip];
        let t145 = t52 * t60;
        let t146 = t47 * t34;
        let t149 = t56 * t51;
        let t150 = t149 * t60;
        let t153 = t58 * t66;
        let t156 = t62 * t57;
        let t157 = t156 * t66;
        let t160 = t64 * t72;
        let t163 = t68 * t63;
        let t164 = t163 * t72;
        let t167 = t70 * t78;
        let t170 = t74 * t69;
        let t171 = t170 * t78;
        let t174 = t76 * t84;
        let t177 = t80 * t75;
        let t178 = t177 * t84;
        let t181 = 5.0 / 3.0 * t140 * t29 * t34 * t54 + 5.0 / 3.0 * t145 * t146 + 10.0 / 3.0 * t150 * t146 + 10.0 / 3.0 * t153 * t146 + 5.0 * t157 * t146 + 5.0 * t160 * t146 + 20.0 / 3.0 * t164 * t146 + 20.0 / 3.0 * t167 * t146 + 25.0 / 3.0 * t171 * t146 + 25.0 / 3.0 * t174 * t146 + 10.0 * t178 * t146;
        let t182 = t82 * t90;
        let t185 = t86 * t81;
        let t186 = t185 * t90;
        let t189 = t88 * t96;
        let t192 = t92 * t87;
        let t193 = t192 * t96;
        let t196 = t94 * t102;
        let t199 = t98 * t93;
        let t200 = t199 * t102;
        let t203 = t100 * t108;
        let t206 = t104 * t99;
        let t207 = t206 * t108;
        let t210 = t106 * t114;
        let t213 = t110 * t105;
        let t214 = t213 * t114;
        let t218 = 1.0 / t95 / t71;
        let t219 = t112 * t218;
        let t222 = 10.0 * t182 * t146 + 35.0 / 3.0 * t186 * t146 + 35.0 / 3.0 * t189 * t146 + 40.0 / 3.0 * t193 * t146 + 40.0 / 3.0 * t196 * t146 + 15.0 * t200 * t146 + 15.0 * t203 * t146 + 50.0 / 3.0 * t207 * t146 + 50.0 / 3.0 * t210 * t146 + 55.0 / 3.0 * t214 * t146 + 55.0 / 3.0 * t219 * t146;
        let t223 = t181 + t222;
        let t224 = t41 * t223;
        let t229 = piecewise3(t3, 0.0, -t19 * t123 * t117 / 8.0 + 0.0040369036088841095 * t135 * t137 - 3.0 / 8.0 * t19 * t21 * t224);
        let tvrho0 = 2.0 * rho[ip] * t229 + 2.0 * t121;
        vrho[ip] += tvrho0;
        let t235 = t127 / t20 / t31 * param_csi_HF;
        let t236 = t133 * t22;
        let t237 = t26 * t29;
        let t239 = t236 * t237 * t116;
        let t242 = piecewise3(t3, 0.0, -0.0015138388533315413 * t235 * t239);
        let tvsigma0 = 2.0 * rho[ip] * t242;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t244 = t43 * t29;
        let t247 = t60 * t29;
        let t248 = t247 * t49;
        let t252 = t66 * t29;
        let t253 = t252 * t49;
        let t258 = t72 * t29;
        let t259 = t258 * t49;
        let t264 = t78 * t29;
        let t265 = t264 * t49;
        let t270 = t84 * t29;
        let t271 = t270 * t49;
        let t276 = -t244 * t49 * t54 - 2.0 * t149 * t248 - 3.0 * t156 * t253 - 4.0 * t163 * t259 - 5.0 * t170 * t265 - 6.0 * t177 * t271 - t52 * t248 - 2.0 * t58 * t253 - 3.0 * t64 * t259 - 4.0 * t70 * t265 - 5.0 * t76 * t271;
        let t277 = t90 * t29;
        let t278 = t277 * t49;
        let t283 = t96 * t29;
        let t284 = t283 * t49;
        let t289 = t102 * t29;
        let t290 = t289 * t49;
        let t295 = t108 * t29;
        let t296 = t295 * t49;
        let t301 = t114 * t29;
        let t302 = t301 * t49;
        let t307 = t218 * t29;
        let t311 = -11.0 * t112 * t307 * t49 - 9.0 * t100 * t296 - 10.0 * t106 * t302 - 7.0 * t185 * t278 - 8.0 * t192 * t284 - 9.0 * t199 * t290 - 10.0 * t206 * t296 - 11.0 * t213 * t302 - 6.0 * t82 * t278 - 7.0 * t88 * t284 - 8.0 * t94 * t290;
        let t312 = t276 + t311;
        let t313 = t41 * t312;
        let t317 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t21 * t313);
        let tvtau0 = 2.0 * rho[ip] * t317;
        vtau[ip] += tvtau0;
        let t320 = t49 * param_csi_HF;
        let t324 = t31 * t31;
        let t326 = 1.0 / t20 / t324;
        let t327 = t326 * param_csi_HF;
        let t329 = t127 * t327 * t133;
        let t335 = t324 * t128;
        let t336 = 1.0 / t335;
        let t337 = t336 * param_csi_HF;
        let t339 = 1.0 / t132 / t38;
        let t341 = t127 * t337 * t339;
        let t343 = 1.0 / t24 / t23;
        let t344 = t44 * t343;
        let t345 = sigma[ip] * sigma[ip];
        let t346 = t345 * t28;
        let t348 = t344 * t346 * t116;
        let t352 = t27 * t30 * t223;
        let t356 = 1.0 / t32 / t128;
        let t361 = tau[ip] * tau[ip];
        let t362 = t43 * t361;
        let t363 = t324 * rho[ip];
        let t365 = 1.0 / t20 / t363;
        let t366 = t28 * t365;
        let t367 = t366 * t60;
        let t370 = t56 * t361;
        let t373 = t82 * t96;
        let t374 = t361 * t28;
        let t375 = t374 * t365;
        let t378 = t86 * t75;
        let t379 = t378 * t90;
        let t382 = t185 * t96;
        let t385 = t88 * t102;
        let t388 = t92 * t81;
        let t389 = t388 * t96;
        let t392 = t192 * t102;
        let t395 = t94 * t108;
        let t398 = t98 * t87;
        let t399 = t398 * t102;
        let t402 = t199 * t108;
        let t405 = t149 * t66;
        let t408 = -40.0 / 9.0 * t140 * t29 * t356 * t54 + 100.0 / 9.0 * t362 * t367 + 100.0 / 9.0 * t370 * t367 + 700.0 / 3.0 * t373 * t375 + 700.0 / 3.0 * t379 * t375 + 4900.0 / 9.0 * t382 * t375 + 2800.0 / 9.0 * t385 * t375 + 2800.0 / 9.0 * t389 * t375 + 6400.0 / 9.0 * t392 * t375 + 400.0 * t395 * t375 + 400.0 * t399 * t375 + 900.0 * t402 * t375 + 400.0 / 9.0 * t405 * t375;
        let t409 = t58 * t72;
        let t412 = t62 * t51;
        let t413 = t412 * t66;
        let t416 = t156 * t72;
        let t419 = t64 * t78;
        let t422 = t68 * t57;
        let t423 = t422 * t72;
        let t426 = t163 * t78;
        let t429 = t70 * t84;
        let t432 = t74 * t63;
        let t433 = t432 * t78;
        let t436 = t170 * t84;
        let t439 = t76 * t90;
        let t442 = t52 * t66;
        let t445 = t47 * t356;
        let t452 = 100.0 / 3.0 * t409 * t375 + 100.0 / 3.0 * t413 * t375 + 100.0 * t416 * t375 + 200.0 / 3.0 * t419 * t375 + 200.0 / 3.0 * t423 * t375 + 1600.0 / 9.0 * t426 * t375 + 1000.0 / 9.0 * t429 * t375 + 1000.0 / 9.0 * t433 * t375 + 2500.0 / 9.0 * t436 * t375 + 500.0 / 3.0 * t439 * t375 + 100.0 / 9.0 * t442 * t375 - 400.0 / 9.0 * t210 * t445 - 440.0 / 9.0 * t214 * t445 - 440.0 / 9.0 * t219 * t445;
        let t480 = -80.0 / 3.0 * t182 * t445 - 280.0 / 9.0 * t186 * t445 - 280.0 / 9.0 * t189 * t445 - 320.0 / 9.0 * t193 * t445 - 320.0 / 9.0 * t196 * t445 - 40.0 * t200 * t445 - 40.0 * t203 * t445 - 400.0 / 9.0 * t207 * t445 - 80.0 / 9.0 * t153 * t445 - 40.0 / 3.0 * t157 * t445 - 40.0 / 3.0 * t160 * t445 - 160.0 / 9.0 * t164 * t445 - 160.0 / 9.0 * t167 * t445;
        let t491 = t100 * t114;
        let t494 = t104 * t93;
        let t495 = t494 * t108;
        let t498 = t206 * t114;
        let t501 = t106 * t218;
        let t504 = t110 * t99;
        let t505 = t504 * t114;
        let t508 = t213 * t218;
        let t512 = 1.0 / t95 / t77;
        let t513 = t112 * t512;
        let t516 = t80 * t69;
        let t517 = t516 * t84;
        let t520 = t177 * t90;
        let t523 = -200.0 / 9.0 * t171 * t445 - 200.0 / 9.0 * t174 * t445 - 80.0 / 3.0 * t178 * t445 - 40.0 / 9.0 * t145 * t445 - 80.0 / 9.0 * t150 * t445 + 500.0 * t491 * t375 + 500.0 * t495 * t375 + 10000.0 / 9.0 * t498 * t375 + 5500.0 / 9.0 * t501 * t375 + 5500.0 / 9.0 * t505 * t375 + 12100.0 / 9.0 * t508 * t375 + 2200.0 / 3.0 * t513 * t375 + 500.0 / 3.0 * t517 * t375 + 400.0 * t520 * t375;
        let t525 = t408 + t452 + t480 + t523;
        let t526 = t41 * t525;
        let t531 = piecewise3(t3, 0.0, t19 * t320 * t117 / 12.0 - 0.01211071082665233 * t329 * t137 - t19 * t123 * t224 / 4.0 + 0.0003938492381143005 * t341 * t348 + 0.008073807217768219 * t135 * t352 - 3.0 / 8.0 * t19 * t21 * t526);
        let tv2rho20 = 2.0 * rho[ip] * t531 + 4.0 * t229;
        v2rho2[ip] += tv2rho20;
        let t534 = t127 * t131;
        let t537 = t324 * t31;
        let t538 = 1.0 / t537;
        let t539 = t538 * param_csi_HF;
        let t541 = t127 * t539 * t339;
        let t544 = t344 * t28 * t116 * sigma[ip];
        let t548 = t236 * t237 * t223;
        let t552 = piecewise3(t3, 0.0, 0.003532290657773596 * t534 * t239 - 0.00014769346429286268 * t541 * t544 - 0.0015138388533315413 * t235 * t548);
        let tv2rhosigma0 = 2.0 * rho[ip] * t552 + 2.0 * t242;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t559 = t27 * t30 * t312;
        let t565 = t56 * tau[ip];
        let t566 = t28 * t326;
        let t570 = t301 * t34;
        let t576 = t283 * t34;
        let t581 = t289 * t34;
        let t586 = t295 * t34;
        let t593 = t264 * t34;
        let t598 = 5.0 / 3.0 * t244 * t34 * t54 - 20.0 / 3.0 * t565 * t566 * t60 + 55.0 / 3.0 * t213 * t570 + 55.0 / 3.0 * t112 * t307 * t34 + 35.0 / 3.0 * t88 * t576 + 40.0 / 3.0 * t192 * t576 + 40.0 / 3.0 * t94 * t581 + 15.0 * t199 * t581 + 15.0 * t100 * t586 + 50.0 / 3.0 * t206 * t586 + 50.0 / 3.0 * t106 * t570 + 20.0 / 3.0 * t70 * t593 + 25.0 / 3.0 * t170 * t593;
        let t599 = t270 * t34;
        let t604 = t277 * t34;
        let t609 = t247 * t34;
        let t614 = t252 * t34;
        let t619 = t258 * t34;
        let t624 = t43 * t28;
        let t625 = t326 * t60;
        let t629 = t566 * tau[ip];
        let t636 = 25.0 / 3.0 * t76 * t599 + 10.0 * t177 * t599 + 10.0 * t82 * t604 + 35.0 / 3.0 * t185 * t604 + 5.0 / 3.0 * t52 * t609 + 10.0 / 3.0 * t149 * t609 + 10.0 / 3.0 * t58 * t614 + 5.0 * t156 * t614 + 5.0 * t64 * t619 + 20.0 / 3.0 * t163 * t619 - 20.0 / 3.0 * t624 * t625 * tau[ip] - 60.0 * t416 * t629 - 40.0 * t419 * t629 - 100.0 * t439 * t629;
        let t664 = -100.0 * t517 * t629 - 240.0 * t520 * t629 - 140.0 * t373 * t629 - 140.0 * t379 * t629 - 980.0 / 3.0 * t382 * t629 - 20.0 / 3.0 * t442 * t629 - 80.0 / 3.0 * t405 * t629 - 20.0 * t409 * t629 - 20.0 * t413 * t629 - 440.0 * t513 * t629 - 560.0 / 3.0 * t385 * t629 - 560.0 / 3.0 * t389 * t629 - 1280.0 / 3.0 * t392 * t629;
        let t693 = -240.0 * t395 * t629 - 240.0 * t399 * t629 - 540.0 * t402 * t629 - 300.0 * t491 * t629 - 300.0 * t495 * t629 - 2000.0 / 3.0 * t498 * t629 - 40.0 * t423 * t629 - 320.0 / 3.0 * t426 * t629 - 200.0 / 3.0 * t429 * t629 - 200.0 / 3.0 * t433 * t629 - 500.0 / 3.0 * t436 * t629 - 1100.0 / 3.0 * t501 * t629 - 1100.0 / 3.0 * t505 * t629 - 2420.0 / 3.0 * t508 * t629;
        let t695 = t598 + t636 + t664 + t693;
        let t696 = t41 * t695;
        let t701 = piecewise3(t3, 0.0, -t19 * t123 * t313 / 8.0 + 0.0040369036088841095 * t135 * t559 - 3.0 / 8.0 * t19 * t21 * t696);
        let tv2rhotau0 = 2.0 * rho[ip] * t701 + 2.0 * t317;
        v2rhotau[ip] += tv2rhotau0;
        let t704 = 1.0 / t363;
        let t706 = t127 * t704 * param_csi_HF;
        let t707 = t339 * t44;
        let t708 = t343 * t28;
        let t710 = t707 * t708 * t116;
        let t713 = piecewise3(t3, 0.0, 5.538504910982351e-05 * t706 * t710);
        let tv2sigma20 = 2.0 * rho[ip] * t713;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t716 = t236 * t237 * t312;
        let t719 = piecewise3(t3, 0.0, -0.0015138388533315413 * t235 * t716);
        let tv2sigmatau0 = 2.0 * rho[ip] * t719;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t721 = t56 * t28;
        let t722 = t130 * t60;
        let t727 = t108 * t28;
        let t728 = t727 * t130;
        let t731 = t114 * t28;
        let t732 = t731 * t130;
        let t739 = t218 * t28;
        let t740 = t739 * t130;
        let t747 = t512 * t28;
        let t751 = t78 * t28;
        let t752 = t751 * t130;
        let t755 = t72 * t28;
        let t756 = t755 * t130;
        let t761 = t84 * t28;
        let t762 = t761 * t130;
        let t769 = 264.0 * t112 * t747 * t130 + 180.0 * t100 * t732 + 220.0 * t106 * t740 + 64.0 * t163 * t752 + 100.0 * t170 * t762 + 324.0 * t199 * t728 + 400.0 * t206 * t732 + 484.0 * t213 * t740 + 24.0 * t422 * t756 + 40.0 * t432 * t752 + 180.0 * t494 * t728 + 220.0 * t504 * t732 + 4.0 * t624 * t722 + 24.0 * t64 * t752 + 40.0 * t70 * t762 + 4.0 * t721 * t722;
        let t770 = t90 * t28;
        let t771 = t770 * t130;
        let t778 = t96 * t28;
        let t779 = t778 * t130;
        let t786 = t102 * t28;
        let t787 = t786 * t130;
        let t798 = t66 * t28;
        let t799 = t798 * t130;
        let t810 = 16.0 * t149 * t799 + 36.0 * t156 * t756 + 144.0 * t177 * t771 + 196.0 * t185 * t779 + 256.0 * t192 * t787 + 84.0 * t378 * t771 + 112.0 * t388 * t779 + 144.0 * t398 * t787 + 12.0 * t412 * t799 + 60.0 * t516 * t762 + 4.0 * t52 * t799 + 12.0 * t58 * t756 + 144.0 * t94 * t728 + 60.0 * t76 * t771 + 84.0 * t82 * t779 + 112.0 * t88 * t787;
        let t811 = t769 + t810;
        let t812 = t41 * t811;
        let t816 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t21 * t812);
        let tv2tau20 = 2.0 * rho[ip] * t816;
        v2tau2[ip] += tv2tau20;
    }
}
