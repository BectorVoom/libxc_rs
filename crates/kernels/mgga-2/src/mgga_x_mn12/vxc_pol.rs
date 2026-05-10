//! MGGA_X_MN12 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 270 shared lines across all orders.
//! Delta: 336 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_mn12_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_c_0: f64,
    param_c_1: f64,
    param_c_2: f64,
    param_c_3: f64,
    param_c_4: f64,
    param_c_5: f64,
    param_c_6: f64,
    param_c_7: f64,
    param_c_8: f64,
    param_c_9: f64,
    param_c_10: f64,
    param_c_11: f64,
    param_c_12: f64,
    param_c_13: f64,
    param_c_14: f64,
    param_c_15: f64,
    param_c_16: f64,
    param_c_17: f64,
    param_c_18: f64,
    param_c_19: f64,
    param_c_20: f64,
    param_c_21: f64,
    param_c_22: f64,
    param_c_23: f64,
    param_c_24: f64,
    param_c_25: f64,
    param_c_26: f64,
    param_c_27: f64,
    param_c_28: f64,
    param_c_29: f64,
    param_c_30: f64,
    param_c_31: f64,
    param_c_32: f64,
    param_c_33: f64,
    param_c_34: f64,
    param_c_35: f64,
    param_c_36: f64,
    param_c_37: f64,
    param_c_38: f64,
    param_c_39: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        // --- shared preamble (270 lines) ---
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t18 = t17 * t8;
        let t19 = piecewise5(t11, t12, t15, t16, t18);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = param_c_0;
        let t30 = param_c_1;
        let t31 = M_CBRT6;
        let t32 = t31 * t31;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t37 = 3.0 / 10.0 * t32 * t35;
        let t38 = pow_1_3(rho0);
        let t39 = t38 * t38;
        let t41 = 1.0 / t39 / rho0;
        let t42 = tau0 * t41;
        let t43 = t37 - t42;
        let t44 = t30 * t43;
        let t45 = t37 + t42;
        let t46 = 1.0 / t45;
        let t48 = param_c_2;
        let t49 = t43 * t43;
        let t50 = t48 * t49;
        let t51 = t45 * t45;
        let t52 = 1.0 / t51;
        let t54 = param_c_3;
        let t55 = t49 * t43;
        let t56 = t54 * t55;
        let t57 = t51 * t45;
        let t58 = 1.0 / t57;
        let t60 = param_c_4;
        let t61 = t49 * t49;
        let t62 = t60 * t61;
        let t63 = t51 * t51;
        let t64 = 1.0 / t63;
        let t66 = param_c_5;
        let t68 = t66 * t61 * t43;
        let t70 = 1.0 / t63 / t45;
        let t72 = param_c_6;
        let t73 = param_c_7;
        let t74 = t73 * t43;
        let t76 = param_c_8;
        let t77 = t76 * t49;
        let t79 = param_c_9;
        let t80 = t79 * t55;
        let t82 = param_c_10;
        let t83 = t82 * t61;
        let t85 = t74 * t46 + t77 * t52 + t80 * t58 + t83 * t64 + t72;
        let t86 = t85 * sigma0;
        let t87 = rho0 * rho0;
        let t89 = 1.0 / t39 / t87;
        let t92 = 1.0 + 0.4e-2 * sigma0 * t89;
        let t93 = 1.0 / t92;
        let t94 = t89 * t93;
        let t97 = param_c_11;
        let t98 = param_c_12;
        let t99 = t98 * t43;
        let t101 = param_c_13;
        let t102 = t101 * t49;
        let t104 = param_c_14;
        let t105 = t104 * t55;
        let t107 = t102 * t52 + t105 * t58 + t99 * t46 + t97;
        let t108 = sigma0 * sigma0;
        let t109 = t107 * t108;
        let t110 = t87 * t87;
        let t111 = t110 * rho0;
        let t113 = 1.0 / t38 / t111;
        let t114 = t92 * t92;
        let t115 = 1.0 / t114;
        let t116 = t113 * t115;
        let t119 = param_c_15;
        let t120 = param_c_16;
        let t121 = t120 * t43;
        let t123 = param_c_17;
        let t124 = t123 * t49;
        let t126 = t121 * t46 + t124 * t52 + t119;
        let t127 = t108 * sigma0;
        let t128 = t126 * t127;
        let t129 = t110 * t110;
        let t130 = 1.0 / t129;
        let t131 = t114 * t92;
        let t132 = 1.0 / t131;
        let t133 = t130 * t132;
        let t136 = param_c_18;
        let t137 = param_c_19;
        let t138 = t137 * t43;
        let t140 = param_c_20;
        let t141 = t140 * t49;
        let t143 = param_c_21;
        let t144 = t143 * t55;
        let t146 = param_c_22;
        let t147 = t146 * t61;
        let t149 = t138 * t46 + t141 * t52 + t144 * t58 + t147 * t64 + t136;
        let t151 = M_CBRT2;
        let t152 = 1.0 / t27 * t151;
        let t154 = 1.0 + t18 <= zeta_threshold;
        let t156 = 1.0 - t18 <= zeta_threshold;
        let t157 = piecewise5(t154, t12, t156, t16, t18);
        let t158 = 1.0 + t157;
        let t159 = 1.0 / t158;
        let t160 = pow_1_3(t159);
        let t163 = 1.0 + 0.39999999999999999998e0 * t152 * t160;
        let t164 = 1.0 / t163;
        let t166 = param_c_23;
        let t167 = param_c_24;
        let t168 = t167 * t43;
        let t170 = param_c_25;
        let t171 = t170 * t49;
        let t173 = param_c_26;
        let t174 = t173 * t55;
        let t176 = t168 * t46 + t171 * t52 + t174 * t58 + t166;
        let t177 = t176 * sigma0;
        let t178 = t94 * t164;
        let t181 = param_c_27;
        let t182 = param_c_28;
        let t183 = t182 * t43;
        let t185 = param_c_29;
        let t186 = t185 * t49;
        let t188 = t183 * t46 + t186 * t52 + t181;
        let t189 = t188 * t108;
        let t190 = t116 * t164;
        let t193 = param_c_30;
        let t194 = param_c_31;
        let t195 = t194 * t43;
        let t197 = param_c_32;
        let t198 = t197 * t49;
        let t200 = param_c_33;
        let t201 = t200 * t55;
        let t203 = t195 * t46 + t198 * t52 + t201 * t58 + t193;
        let t204 = t163 * t163;
        let t205 = 1.0 / t204;
        let t207 = param_c_34;
        let t208 = param_c_35;
        let t209 = t208 * t43;
        let t211 = param_c_36;
        let t212 = t211 * t49;
        let t214 = t209 * t46 + t212 * t52 + t207;
        let t215 = t214 * sigma0;
        let t216 = t94 * t205;
        let t219 = param_c_37;
        let t220 = param_c_38;
        let t221 = t220 * t43;
        let t223 = param_c_39;
        let t224 = t223 * t49;
        let t226 = t221 * t46 + t224 * t52 + t219;
        let t227 = t204 * t163;
        let t228 = 1.0 / t227;
        let t230 = t29 + t44 * t46 + t50 * t52 + t56 * t58 + t62 * t64 + t68 * t70 + 0.4e-2 * t86 * t94 + 0.16e-4 * t109 * t116 + 0.64e-7 * t128 * t133 + t149 * t164 + 0.4e-2 * t177 * t178 + 0.16e-4 * t189 * t190 + t203 * t205 + 0.4e-2 * t215 * t216 + t226 * t228;
        let t234 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t230);
        let t235 = rho1 <= dens_threshold;
        let t236 = -t17;
        let t238 = piecewise5(t15, t12, t11, t16, t236 * t8);
        let t239 = 1.0 + t238;
        let t240 = t239 <= zeta_threshold;
        let t241 = pow_1_3(t239);
        let t243 = piecewise3(t240, t23, t241 * t239);
        let t244 = t243 * t27;
        let t245 = pow_1_3(rho1);
        let t246 = t245 * t245;
        let t248 = 1.0 / t246 / rho1;
        let t249 = tau1 * t248;
        let t250 = t37 - t249;
        let t251 = t30 * t250;
        let t252 = t37 + t249;
        let t253 = 1.0 / t252;
        let t255 = t250 * t250;
        let t256 = t48 * t255;
        let t257 = t252 * t252;
        let t258 = 1.0 / t257;
        let t260 = t255 * t250;
        let t261 = t54 * t260;
        let t262 = t257 * t252;
        let t263 = 1.0 / t262;
        let t265 = t255 * t255;
        let t266 = t60 * t265;
        let t267 = t257 * t257;
        let t268 = 1.0 / t267;
        let t271 = t66 * t265 * t250;
        let t273 = 1.0 / t267 / t252;
        let t275 = t73 * t250;
        let t277 = t76 * t255;
        let t279 = t79 * t260;
        let t281 = t82 * t265;
        let t283 = t275 * t253 + t277 * t258 + t279 * t263 + t281 * t268 + t72;
        let t284 = t283 * sigma2;
        let t285 = rho1 * rho1;
        let t287 = 1.0 / t246 / t285;
        let t290 = 1.0 + 0.4e-2 * sigma2 * t287;
        let t291 = 1.0 / t290;
        let t292 = t287 * t291;
        let t295 = t98 * t250;
        let t297 = t101 * t255;
        let t299 = t104 * t260;
        let t301 = t295 * t253 + t297 * t258 + t299 * t263 + t97;
        let t302 = sigma2 * sigma2;
        let t303 = t301 * t302;
        let t304 = t285 * t285;
        let t305 = t304 * rho1;
        let t307 = 1.0 / t245 / t305;
        let t308 = t290 * t290;
        let t309 = 1.0 / t308;
        let t310 = t307 * t309;
        let t313 = t120 * t250;
        let t315 = t123 * t255;
        let t317 = t313 * t253 + t315 * t258 + t119;
        let t318 = t302 * sigma2;
        let t319 = t317 * t318;
        let t320 = t304 * t304;
        let t321 = 1.0 / t320;
        let t322 = t308 * t290;
        let t323 = 1.0 / t322;
        let t324 = t321 * t323;
        let t327 = t137 * t250;
        let t329 = t140 * t255;
        let t331 = t143 * t260;
        let t333 = t146 * t265;
        let t335 = t327 * t253 + t329 * t258 + t331 * t263 + t333 * t268 + t136;
        let t336 = piecewise5(t156, t12, t154, t16, -t18);
        let t337 = 1.0 + t336;
        let t338 = 1.0 / t337;
        let t339 = pow_1_3(t338);
        let t342 = 1.0 + 0.39999999999999999998e0 * t152 * t339;
        let t343 = 1.0 / t342;
        let t345 = t167 * t250;
        let t347 = t170 * t255;
        let t349 = t173 * t260;
        let t351 = t345 * t253 + t347 * t258 + t349 * t263 + t166;
        let t352 = t351 * sigma2;
        let t353 = t292 * t343;
        let t356 = t182 * t250;
        let t358 = t185 * t255;
        let t360 = t356 * t253 + t358 * t258 + t181;
        let t361 = t360 * t302;
        let t362 = t310 * t343;
        let t365 = t194 * t250;
        let t367 = t197 * t255;
        let t369 = t200 * t260;
        let t371 = t365 * t253 + t367 * t258 + t369 * t263 + t193;
        let t372 = t342 * t342;
        let t373 = 1.0 / t372;
        let t375 = t208 * t250;
        let t377 = t211 * t255;
        let t379 = t375 * t253 + t377 * t258 + t207;
        let t380 = t379 * sigma2;
        let t381 = t292 * t373;
        let t384 = t220 * t250;
        let t386 = t223 * t255;
        let t388 = t384 * t253 + t386 * t258 + t219;
        let t389 = t372 * t342;
        let t390 = 1.0 / t389;
        let t392 = t29 + t251 * t253 + t256 * t258 + t261 * t263 + t266 * t268 + t271 * t273 + 0.4e-2 * t284 * t292 + 0.16e-4 * t303 * t310 + 0.64e-7 * t319 * t324 + t335 * t343 + 0.4e-2 * t352 * t353 + 0.16e-4 * t361 * t362 + t371 * t373 + 0.4e-2 * t380 * t381 + t388 * t390;
        let t396 = piecewise3(t235, 0.0, -3.0 / 8.0 * t6 * t244 * t392);
        let tzk0 = t234 + t396;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (336 lines) ---
        let t397 = t7 * t7;
        let t398 = 1.0 / t397;
        let t399 = t17 * t398;
        let t400 = t8 - t399;
        let t401 = piecewise5(t11, 0.0, t15, 0.0, t400);
        let t404 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t401);
        let t405 = t404 * t27;
        let t409 = t27 * t27;
        let t410 = 1.0 / t409;
        let t411 = t26 * t410;
        let t414 = t6 * t411 * t230 / 8.0;
        let t415 = t189 * t113;
        let t416 = t115 * t205;
        let t419 = 1.0 / t27 / t7 * t151;
        let t421 = 0.13333333333333333333e0 * t419 * t160;
        let t422 = t160 * t160;
        let t423 = 1.0 / t422;
        let t424 = t158 * t158;
        let t425 = 1.0 / t424;
        let t426 = t423 * t425;
        let t427 = piecewise5(t154, 0.0, t156, 0.0, t400);
        let t428 = t426 * t427;
        let t431 = -t421 - 0.13333333333333333333e0 * t152 * t428;
        let t432 = t416 * t431;
        let t435 = t215 * t89;
        let t436 = t93 * t228;
        let t437 = t436 * t431;
        let t440 = t177 * t89;
        let t441 = t93 * t205;
        let t442 = t441 * t431;
        let t445 = t129 * rho0;
        let t446 = 1.0 / t445;
        let t447 = t446 * t132;
        let t450 = t87 * rho0;
        let t452 = 1.0 / t39 / t450;
        let t453 = t452 * t93;
        let t456 = t110 * t87;
        let t458 = 1.0 / t38 / t456;
        let t459 = t458 * t115;
        let t462 = t30 * tau0;
        let t463 = t89 * t46;
        let t466 = t73 * tau0;
        let t469 = t52 * tau0;
        let t470 = t469 * t89;
        let t473 = t76 * t43;
        let t476 = t58 * tau0;
        let t477 = t476 * t89;
        let t480 = t79 * t49;
        let t483 = t64 * tau0;
        let t484 = t483 * t89;
        let t487 = t82 * t55;
        let t490 = t70 * tau0;
        let t491 = t490 * t89;
        let t494 = 5.0 / 3.0 * t466 * t463 + 5.0 / 3.0 * t74 * t470 + 10.0 / 3.0 * t473 * t470 + 10.0 / 3.0 * t77 * t477 + 5.0 * t480 * t477 + 5.0 * t80 * t484 + 20.0 / 3.0 * t487 * t484 + 20.0 / 3.0 * t83 * t491;
        let t495 = t494 * sigma0;
        let t498 = t85 * t108;
        let t501 = t98 * tau0;
        let t506 = t101 * t43;
        let t511 = t104 * t49;
        let t516 = 5.0 / 3.0 * t501 * t463 + 5.0 / 3.0 * t99 * t470 + 10.0 / 3.0 * t506 * t470 + 10.0 / 3.0 * t102 * t477 + 5.0 * t511 * t477 + 5.0 * t105 * t484;
        let t517 = t516 * t108;
        let t520 = t107 * t127;
        let t523 = t120 * tau0;
        let t528 = t123 * t43;
        let t533 = 5.0 / 3.0 * t523 * t463 + 5.0 / 3.0 * t121 * t470 + 10.0 / 3.0 * t528 * t470 + 10.0 / 3.0 * t124 * t477;
        let t534 = t533 * t127;
        let t537 = t108 * t108;
        let t538 = t126 * t537;
        let t539 = t129 * t450;
        let t541 = 1.0 / t39 / t539;
        let t542 = t114 * t114;
        let t543 = 1.0 / t542;
        let t544 = t541 * t543;
        let t547 = t459 * t164;
        let t550 = t453 * t205;
        let t553 = t453 * t164;
        let t556 = t182 * tau0;
        let t561 = t185 * t43;
        let t566 = 5.0 / 3.0 * t556 * t463 + 5.0 / 3.0 * t183 * t470 + 10.0 / 3.0 * t561 * t470 + 10.0 / 3.0 * t186 * t477;
        let t567 = t566 * t108;
        let t570 = t188 * t127;
        let t571 = t447 * t164;
        let t574 = -0.16e-4 * t415 * t432 - 0.8e-2 * t435 * t437 - 0.4e-2 * t440 * t442 - 0.512e-6 * t128 * t447 - 0.10666666666666666667e-1 * t86 * t453 - 0.85333333333333333333e-4 * t109 * t459 + 5.0 / 3.0 * t462 * t463 + 0.4e-2 * t495 * t94 + 0.42666666666666666668e-4 * t498 * t459 + 0.16e-4 * t517 * t116 + 0.34133333333333333334e-6 * t520 * t447 + 0.64e-7 * t534 * t133 + 0.20480000000000000001e-8 * t538 * t544 - 0.85333333333333333333e-4 * t189 * t547 - 0.10666666666666666667e-1 * t215 * t550 - 0.10666666666666666667e-1 * t177 * t553 + 0.16e-4 * t567 * t190 + 0.34133333333333333334e-6 * t570 * t571;
        let t575 = t208 * tau0;
        let t580 = t211 * t43;
        let t585 = 5.0 / 3.0 * t575 * t463 + 5.0 / 3.0 * t209 * t470 + 10.0 / 3.0 * t580 * t470 + 10.0 / 3.0 * t212 * t477;
        let t586 = t585 * sigma0;
        let t589 = t214 * t108;
        let t590 = t459 * t205;
        let t593 = t167 * tau0;
        let t598 = t170 * t43;
        let t603 = t173 * t49;
        let t608 = 5.0 / 3.0 * t593 * t463 + 5.0 / 3.0 * t168 * t470 + 10.0 / 3.0 * t598 * t470 + 10.0 / 3.0 * t171 * t477 + 5.0 * t603 * t477 + 5.0 * t174 * t484;
        let t609 = t608 * sigma0;
        let t612 = t176 * t108;
        let t615 = t54 * t49;
        let t620 = t60 * t55;
        let t625 = t66 * t61;
        let t629 = 1.0 / t63 / t51;
        let t630 = t629 * tau0;
        let t636 = t48 * t43;
        let t641 = t194 * tau0;
        let t646 = t197 * t43;
        let t651 = t200 * t49;
        let t656 = 5.0 / 3.0 * t641 * t463 + 5.0 / 3.0 * t195 * t470 + 10.0 / 3.0 * t646 * t470 + 10.0 / 3.0 * t198 * t477 + 5.0 * t651 * t477 + 5.0 * t201 * t484;
        let t658 = t220 * tau0;
        let t663 = t223 * t43;
        let t668 = 5.0 / 3.0 * t658 * t463 + 5.0 / 3.0 * t221 * t470 + 10.0 / 3.0 * t663 * t470 + 10.0 / 3.0 * t224 * t477;
        let t670 = t137 * tau0;
        let t675 = t140 * t43;
        let t680 = t143 * t49;
        let t685 = t146 * t55;
        let t690 = 5.0 / 3.0 * t670 * t463 + 5.0 / 3.0 * t138 * t470 + 10.0 / 3.0 * t675 * t470 + 10.0 / 3.0 * t141 * t477 + 5.0 * t680 * t477 + 5.0 * t144 * t484 + 20.0 / 3.0 * t685 * t484 + 20.0 / 3.0 * t147 * t491;
        let t692 = t149 * t205;
        let t694 = t203 * t228;
        let t697 = t204 * t204;
        let t698 = 1.0 / t697;
        let t699 = t226 * t698;
        let t702 = 0.4e-2 * t586 * t216 + 0.42666666666666666668e-4 * t589 * t590 + 0.4e-2 * t609 * t178 + 0.42666666666666666668e-4 * t612 * t547 + 5.0 * t615 * t477 + 5.0 * t56 * t484 + 20.0 / 3.0 * t620 * t484 + 20.0 / 3.0 * t62 * t491 + 25.0 / 3.0 * t625 * t491 + 25.0 / 3.0 * t68 * t630 * t89 + 5.0 / 3.0 * t44 * t470 + 10.0 / 3.0 * t636 * t470 + 10.0 / 3.0 * t50 * t477 + t656 * t205 + t668 * t228 + t690 * t164 - t692 * t431 - 2.0 * t694 * t431 - 3.0 * t699 * t431;
        let t703 = t574 + t702;
        let t708 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t405 * t230 - t414 - 3.0 / 8.0 * t6 * t28 * t703);
        let t709 = t236 * t398;
        let t711 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t709);
        let t714 = piecewise3(t240, 0.0, 4.0 / 3.0 * t241 * t711);
        let t715 = t714 * t27;
        let t719 = t243 * t410;
        let t722 = t6 * t719 * t392 / 8.0;
        let t723 = t335 * t373;
        let t725 = 0.13333333333333333333e0 * t419 * t339;
        let t726 = t339 * t339;
        let t727 = 1.0 / t726;
        let t728 = t337 * t337;
        let t729 = 1.0 / t728;
        let t730 = t727 * t729;
        let t732 = piecewise5(t156, 0.0, t154, 0.0, -t400);
        let t733 = t730 * t732;
        let t736 = -t725 - 0.13333333333333333333e0 * t152 * t733;
        let t738 = t352 * t287;
        let t739 = t291 * t373;
        let t740 = t739 * t736;
        let t743 = t361 * t307;
        let t744 = t309 * t373;
        let t745 = t744 * t736;
        let t748 = t371 * t390;
        let t751 = t380 * t287;
        let t752 = t291 * t390;
        let t753 = t752 * t736;
        let t756 = t372 * t372;
        let t757 = 1.0 / t756;
        let t758 = t388 * t757;
        let t761 = -t723 * t736 - 0.4e-2 * t738 * t740 - 0.16e-4 * t743 * t745 - 2.0 * t748 * t736 - 0.8e-2 * t751 * t753 - 3.0 * t758 * t736;
        let t766 = piecewise3(t235, 0.0, -3.0 / 8.0 * t6 * t715 * t392 - t722 - 3.0 / 8.0 * t6 * t244 * t761);
        let tvrho0 = t234 + t396 + t7 * (t708 + t766);
        vrho[ip * 2] += tvrho0;
        let t769 = -t8 - t399;
        let t770 = piecewise5(t11, 0.0, t15, 0.0, t769);
        let t773 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t770);
        let t774 = t773 * t27;
        let t778 = piecewise5(t154, 0.0, t156, 0.0, t769);
        let t779 = t426 * t778;
        let t782 = -t421 - 0.13333333333333333333e0 * t152 * t779;
        let t784 = t441 * t782;
        let t787 = t416 * t782;
        let t792 = t436 * t782;
        let t797 = -t692 * t782 - 0.4e-2 * t440 * t784 - 0.16e-4 * t415 * t787 - 2.0 * t694 * t782 - 0.8e-2 * t435 * t792 - 3.0 * t699 * t782;
        let t802 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t774 * t230 - t414 - 3.0 / 8.0 * t6 * t28 * t797);
        let t804 = piecewise5(t15, 0.0, t11, 0.0, t8 - t709);
        let t807 = piecewise3(t240, 0.0, 4.0 / 3.0 * t241 * t804);
        let t808 = t807 * t27;
        let t812 = t167 * tau1;
        let t813 = t287 * t253;
        let t816 = t258 * tau1;
        let t817 = t816 * t287;
        let t820 = t170 * t250;
        let t823 = t263 * tau1;
        let t824 = t823 * t287;
        let t827 = t173 * t255;
        let t830 = t268 * tau1;
        let t831 = t830 * t287;
        let t834 = 5.0 / 3.0 * t812 * t813 + 5.0 / 3.0 * t345 * t817 + 10.0 / 3.0 * t820 * t817 + 10.0 / 3.0 * t347 * t824 + 5.0 * t827 * t824 + 5.0 * t349 * t831;
        let t835 = t834 * sigma2;
        let t838 = t182 * tau1;
        let t843 = t185 * t250;
        let t848 = 5.0 / 3.0 * t838 * t813 + 5.0 / 3.0 * t356 * t817 + 10.0 / 3.0 * t843 * t817 + 10.0 / 3.0 * t358 * t824;
        let t849 = t848 * t302;
        let t852 = t360 * t318;
        let t853 = t320 * rho1;
        let t854 = 1.0 / t853;
        let t855 = t854 * t323;
        let t856 = t855 * t343;
        let t859 = t351 * t302;
        let t860 = t304 * t285;
        let t862 = 1.0 / t245 / t860;
        let t863 = t862 * t309;
        let t864 = t863 * t343;
        let t869 = t48 * t250;
        let t874 = t54 * t255;
        let t879 = t60 * t260;
        let t882 = t273 * tau1;
        let t883 = t882 * t287;
        let t886 = t66 * t265;
        let t890 = 1.0 / t267 / t257;
        let t891 = t890 * tau1;
        let t895 = t285 * rho1;
        let t897 = 1.0 / t246 / t895;
        let t898 = t897 * t291;
        let t899 = t898 * t373;
        let t902 = t898 * t343;
        let t907 = t379 * t302;
        let t908 = t863 * t373;
        let t911 = t208 * tau1;
        let t916 = t211 * t250;
        let t921 = 5.0 / 3.0 * t911 * t813 + 5.0 / 3.0 * t375 * t817 + 10.0 / 3.0 * t916 * t817 + 10.0 / 3.0 * t377 * t824;
        let t922 = t921 * sigma2;
        let t925 = 0.4e-2 * t835 * t353 + 0.16e-4 * t849 * t362 + 0.34133333333333333334e-6 * t852 * t856 + 0.42666666666666666668e-4 * t859 * t864 + 5.0 / 3.0 * t251 * t817 + 10.0 / 3.0 * t869 * t817 + 10.0 / 3.0 * t256 * t824 + 5.0 * t874 * t824 + 5.0 * t261 * t831 + 20.0 / 3.0 * t879 * t831 + 20.0 / 3.0 * t266 * t883 + 25.0 / 3.0 * t886 * t883 + 25.0 / 3.0 * t271 * t891 * t287 - 0.10666666666666666667e-1 * t380 * t899 - 0.10666666666666666667e-1 * t352 * t902 - 0.85333333333333333333e-4 * t361 * t864 + 0.42666666666666666668e-4 * t907 * t908 + 0.4e-2 * t922 * t381;
        let t927 = piecewise5(t156, 0.0, t154, 0.0, -t769);
        let t928 = t730 * t927;
        let t931 = -t725 - 0.13333333333333333333e0 * t152 * t928;
        let t937 = t194 * tau1;
        let t942 = t197 * t250;
        let t947 = t200 * t255;
        let t952 = 5.0 / 3.0 * t937 * t813 + 5.0 / 3.0 * t365 * t817 + 10.0 / 3.0 * t942 * t817 + 10.0 / 3.0 * t367 * t824 + 5.0 * t947 * t824 + 5.0 * t369 * t831;
        let t954 = t220 * tau1;
        let t959 = t223 * t250;
        let t964 = 5.0 / 3.0 * t954 * t813 + 5.0 / 3.0 * t384 * t817 + 10.0 / 3.0 * t959 * t817 + 10.0 / 3.0 * t386 * t824;
        let t966 = t137 * tau1;
        let t971 = t140 * t250;
        let t976 = t143 * t255;
        let t981 = t146 * t260;
        let t986 = 5.0 / 3.0 * t966 * t813 + 5.0 / 3.0 * t327 * t817 + 10.0 / 3.0 * t971 * t817 + 10.0 / 3.0 * t329 * t824 + 5.0 * t976 * t824 + 5.0 * t331 * t831 + 20.0 / 3.0 * t981 * t831 + 20.0 / 3.0 * t333 * t883;
        let t988 = t752 * t931;
        let t991 = t739 * t931;
        let t994 = t744 * t931;
        let t997 = t30 * tau1;
        let t1000 = t73 * tau1;
        let t1005 = t76 * t250;
        let t1010 = t79 * t255;
        let t1015 = t82 * t260;
        let t1020 = 5.0 / 3.0 * t1000 * t813 + 5.0 / 3.0 * t275 * t817 + 10.0 / 3.0 * t1005 * t817 + 10.0 / 3.0 * t277 * t824 + 5.0 * t1010 * t824 + 5.0 * t279 * t831 + 20.0 / 3.0 * t1015 * t831 + 20.0 / 3.0 * t281 * t883;
        let t1021 = t1020 * sigma2;
        let t1024 = t283 * t302;
        let t1027 = t98 * tau1;
        let t1032 = t101 * t250;
        let t1037 = t104 * t255;
        let t1042 = 5.0 / 3.0 * t1027 * t813 + 5.0 / 3.0 * t295 * t817 + 10.0 / 3.0 * t1032 * t817 + 10.0 / 3.0 * t297 * t824 + 5.0 * t1037 * t824 + 5.0 * t299 * t831;
        let t1043 = t1042 * t302;
        let t1046 = t301 * t318;
        let t1049 = t120 * tau1;
        let t1054 = t123 * t250;
        let t1059 = 5.0 / 3.0 * t1049 * t813 + 5.0 / 3.0 * t313 * t817 + 10.0 / 3.0 * t1054 * t817 + 10.0 / 3.0 * t315 * t824;
        let t1060 = t1059 * t318;
        let t1063 = t302 * t302;
        let t1064 = t317 * t1063;
        let t1065 = t320 * t895;
        let t1067 = 1.0 / t246 / t1065;
        let t1068 = t308 * t308;
        let t1069 = 1.0 / t1068;
        let t1070 = t1067 * t1069;
        let t1079 = -2.0 * t748 * t931 - 3.0 * t758 * t931 - t723 * t931 + t952 * t373 + t964 * t390 + t986 * t343 - 0.8e-2 * t751 * t988 - 0.4e-2 * t738 * t991 - 0.16e-4 * t743 * t994 + 5.0 / 3.0 * t997 * t813 + 0.4e-2 * t1021 * t292 + 0.42666666666666666668e-4 * t1024 * t863 + 0.16e-4 * t1043 * t310 + 0.34133333333333333334e-6 * t1046 * t855 + 0.64e-7 * t1060 * t324 + 0.20480000000000000001e-8 * t1064 * t1070 - 0.10666666666666666667e-1 * t284 * t898 - 0.85333333333333333333e-4 * t303 * t863 - 0.512e-6 * t319 * t855;
        let t1080 = t925 + t1079;
        let t1085 = piecewise3(t235, 0.0, -3.0 / 8.0 * t6 * t808 * t392 - t722 - 3.0 / 8.0 * t6 * t244 * t1080);
        let tvrho1 = t234 + t396 + t7 * (t802 + t1085);
        vrho[ip * 2 + 1] += tvrho1;
        let t1093 = t107 * sigma0;
        let t1098 = t126 * t108;
        let t1101 = t129 * t87;
        let t1103 = 1.0 / t39 / t1101;
        let t1104 = t1103 * t543;
        let t1107 = t176 * t89;
        let t1108 = t93 * t164;
        let t1113 = t188 * sigma0;
        let t1116 = t133 * t164;
        let t1119 = t214 * t89;
        let t1122 = t116 * t205;
        let t1125 = 0.4e-2 * t85 * t89 * t93 - 0.16e-4 * t86 * t116 + 0.32e-4 * t1093 * t116 - 0.128e-6 * t109 * t133 + 0.192e-6 * t1098 * t133 - 0.768e-9 * t128 * t1104 + 0.4e-2 * t1107 * t1108 - 0.16e-4 * t177 * t190 + 0.32e-4 * t1113 * t190 - 0.128e-6 * t189 * t1116 + 0.4e-2 * t1119 * t441 - 0.16e-4 * t215 * t1122;
        let t1129 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t1125);
        let tvsigma0 = t7 * t1129;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t1135 = t301 * sigma2;
        let t1140 = t317 * t302;
        let t1143 = t320 * t285;
        let t1145 = 1.0 / t246 / t1143;
        let t1146 = t1145 * t1069;
        let t1149 = t351 * t287;
        let t1150 = t291 * t343;
        let t1155 = t360 * sigma2;
        let t1158 = t324 * t343;
        let t1161 = t379 * t287;
        let t1164 = t310 * t373;
        let t1167 = 0.4e-2 * t283 * t287 * t291 - 0.16e-4 * t284 * t310 + 0.32e-4 * t1135 * t310 - 0.128e-6 * t303 * t324 + 0.192e-6 * t1140 * t324 - 0.768e-9 * t319 * t1146 + 0.4e-2 * t1149 * t1150 - 0.16e-4 * t352 * t362 + 0.32e-4 * t1155 * t362 - 0.128e-6 * t361 * t1158 + 0.4e-2 * t1161 * t739 - 0.16e-4 * t380 * t1164;
        let t1171 = piecewise3(t235, 0.0, -3.0 / 8.0 * t6 * t244 * t1167);
        let tvsigma2 = t7 * t1171;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t1174 = t52 * t41;
        let t1178 = t58 * t41;
        let t1183 = t64 * t41;
        let t1188 = t70 * t41;
        let t1211 = -t73 * t41 * t46 - 2.0 * t473 * t1174 - t74 * t1174 - 3.0 * t480 * t1178 - 2.0 * t77 * t1178 - 4.0 * t487 * t1183 - 3.0 * t80 * t1183 - 4.0 * t83 * t1188;
        let t1212 = t1211 * sigma0;
        let t1226 = -t98 * t41 * t46 - 2.0 * t102 * t1178 - 3.0 * t105 * t1183 - 2.0 * t506 * t1174 - t99 * t1174 - 3.0 * t511 * t1178;
        let t1227 = t1226 * t108;
        let t1237 = -t120 * t41 * t46 - t121 * t1174 - 2.0 * t528 * t1174 - 2.0 * t124 * t1178;
        let t1238 = t1237 * t127;
        let t1256 = -t137 * t41 * t46 - t138 * t1174 - 2.0 * t675 * t1174 - 2.0 * t141 * t1178 - 3.0 * t680 * t1178 - 3.0 * t144 * t1183 - 4.0 * t685 * t1183 - 4.0 * t147 * t1188;
        let t1269 = -t167 * t41 * t46 - t168 * t1174 - 2.0 * t598 * t1174 - 2.0 * t171 * t1178 - 3.0 * t603 * t1178 - 3.0 * t174 * t1183;
        let t1270 = t1269 * sigma0;
        let t1280 = -t182 * t41 * t46 - t183 * t1174 - 2.0 * t561 * t1174 - 2.0 * t186 * t1178;
        let t1281 = t1280 * t108;
        let t1295 = -t194 * t41 * t46 - t195 * t1174 - 2.0 * t646 * t1174 - 2.0 * t198 * t1178 - 3.0 * t651 * t1178 - 3.0 * t201 * t1183;
        let t1304 = -t208 * t41 * t46 - t209 * t1174 - 2.0 * t580 * t1174 - 2.0 * t212 * t1178;
        let t1305 = t1304 * sigma0;
        let t1315 = -t220 * t41 * t46 - t221 * t1174 - 2.0 * t663 * t1174 - 2.0 * t224 * t1178;
        let t1317 = -t30 * t41 * t46 - t44 * t1174 - 2.0 * t636 * t1174 - 2.0 * t50 * t1178 - 3.0 * t615 * t1178 - 3.0 * t56 * t1183 - 4.0 * t620 * t1183 - 4.0 * t62 * t1188 - 5.0 * t625 * t1188 - 5.0 * t68 * t629 * t41 + 0.4e-2 * t1212 * t94 + 0.16e-4 * t1227 * t116 + 0.64e-7 * t1238 * t133 + t1256 * t164 + 0.4e-2 * t1270 * t178 + 0.16e-4 * t1281 * t190 + t1295 * t205 + 0.4e-2 * t1305 * t216 + t1315 * t228;
        let t1321 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t1317);
        let tvtau0 = t7 * t1321;
        vtau[ip * 2] += tvtau0;
        let t1324 = t258 * t248;
        let t1328 = t263 * t248;
        let t1333 = t268 * t248;
        let t1338 = t273 * t248;
        let t1361 = -t73 * t248 * t253 - 2.0 * t1005 * t1324 - 3.0 * t1010 * t1328 - 4.0 * t1015 * t1333 - t275 * t1324 - 2.0 * t277 * t1328 - 3.0 * t279 * t1333 - 4.0 * t281 * t1338;
        let t1362 = t1361 * sigma2;
        let t1376 = -t98 * t248 * t253 - 2.0 * t1032 * t1324 - 3.0 * t1037 * t1328 - t295 * t1324 - 2.0 * t297 * t1328 - 3.0 * t299 * t1333;
        let t1377 = t1376 * t302;
        let t1387 = -t120 * t248 * t253 - 2.0 * t1054 * t1324 - t313 * t1324 - 2.0 * t315 * t1328;
        let t1388 = t1387 * t318;
        let t1406 = -t137 * t248 * t253 - t327 * t1324 - 2.0 * t971 * t1324 - 2.0 * t329 * t1328 - 3.0 * t976 * t1328 - 3.0 * t331 * t1333 - 4.0 * t981 * t1333 - 4.0 * t333 * t1338;
        let t1419 = -t167 * t248 * t253 - t345 * t1324 - 2.0 * t820 * t1324 - 2.0 * t347 * t1328 - 3.0 * t827 * t1328 - 3.0 * t349 * t1333;
        let t1420 = t1419 * sigma2;
        let t1430 = -t182 * t248 * t253 - t356 * t1324 - 2.0 * t843 * t1324 - 2.0 * t358 * t1328;
        let t1431 = t1430 * t302;
        let t1445 = -t194 * t248 * t253 - t365 * t1324 - 2.0 * t942 * t1324 - 2.0 * t367 * t1328 - 3.0 * t947 * t1328 - 3.0 * t369 * t1333;
        let t1454 = -t208 * t248 * t253 - t375 * t1324 - 2.0 * t916 * t1324 - 2.0 * t377 * t1328;
        let t1455 = t1454 * sigma2;
        let t1465 = -t220 * t248 * t253 - t384 * t1324 - 2.0 * t959 * t1324 - 2.0 * t386 * t1328;
        let t1467 = -t30 * t248 * t253 - t251 * t1324 - 2.0 * t869 * t1324 - 2.0 * t256 * t1328 - 3.0 * t874 * t1328 - 3.0 * t261 * t1333 - 4.0 * t879 * t1333 - 4.0 * t266 * t1338 - 5.0 * t886 * t1338 - 5.0 * t271 * t890 * t248 + 0.4e-2 * t1362 * t292 + 0.16e-4 * t1377 * t310 + 0.64e-7 * t1388 * t324 + t1406 * t343 + 0.4e-2 * t1420 * t353 + 0.16e-4 * t1431 * t362 + t1445 * t373 + 0.4e-2 * t1455 * t381 + t1465 * t390;
        let t1471 = piecewise3(t235, 0.0, -3.0 / 8.0 * t6 * t244 * t1467);
        let tvtau1 = t7 * t1471;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
