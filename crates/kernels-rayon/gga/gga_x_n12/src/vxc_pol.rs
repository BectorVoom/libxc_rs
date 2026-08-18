//! GGA_X_N12 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_n12.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_n12_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_CC_0_1: f64,
    param_CC_0_2: f64,
    param_CC_0_3: f64,
    param_CC_1_1: f64,
    param_CC_1_2: f64,
    param_CC_1_3: f64,
    param_CC_1_0: f64,
    param_CC_2_1: f64,
    param_CC_2_2: f64,
    param_CC_2_3: f64,
    param_CC_2_0: f64,
    param_CC_3_1: f64,
    param_CC_3_2: f64,
    param_CC_3_3: f64,
    param_CC_3_0: f64,
    param_CC_0_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t17 = t16 * t7;
        let t18 = piecewise5(t10, t11, t14, t15, t17);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = param_CC_0_0;
        let t29 = param_CC_0_1;
        let t30 = t29 * sigma0;
        let t31 = rho0 * rho0;
        let t32 = pow_1_3(rho0);
        let t33 = t32 * t32;
        let t35 = 1.0 / t33 / t31;
        let t38 = 1.0 + 0.004 * sigma0 * t35;
        let t39 = 1.0 / t38;
        let t40 = t35 * t39;
        let t43 = param_CC_0_2;
        let t44 = sigma0 * sigma0;
        let t45 = t43 * t44;
        let t46 = t31 * t31;
        let t47 = t46 * rho0;
        let t49 = 1.0 / t32 / t47;
        let t50 = t38 * t38;
        let t51 = 1.0 / t50;
        let t52 = t49 * t51;
        let t55 = param_CC_0_3;
        let t56 = t44 * sigma0;
        let t57 = t55 * t56;
        let t58 = t46 * t46;
        let t59 = 1.0 / t58;
        let t60 = t50 * t38;
        let t61 = 1.0 / t60;
        let t62 = t59 * t61;
        let t65 = param_CC_1_0;
        let t66 = param_CC_1_1;
        let t67 = t66 * sigma0;
        let t70 = param_CC_1_2;
        let t71 = t70 * t44;
        let t74 = param_CC_1_3;
        let t75 = t74 * t56;
        let t78 = t65 + 0.004 * t67 * t40 + 1.6e-05 * t71 * t52 + 6.4e-08 * t75 * t62;
        let t80 = M_CBRT2;
        let t81 = 1.0 / t26 * t80;
        let t83 = 1.0 + t17 <= zeta_threshold;
        let t85 = 1.0 - t17 <= zeta_threshold;
        let t86 = piecewise5(t83, t11, t85, t15, t17);
        let t87 = 1.0 + t86;
        let t88 = t87 <= zeta_threshold;
        let t89 = 1.0 / t21;
        let t90 = pow_1_3(t87);
        let t92 = piecewise3(t88, t89, 1.0 / t90);
        let t95 = 1.0 + 0.4 * t81 * t92;
        let t96 = 1.0 / t95;
        let t98 = param_CC_2_0;
        let t99 = param_CC_2_1;
        let t100 = t99 * sigma0;
        let t103 = param_CC_2_2;
        let t104 = t103 * t44;
        let t107 = param_CC_2_3;
        let t108 = t107 * t56;
        let t111 = t98 + 0.004 * t100 * t40 + 1.6e-05 * t104 * t52 + 6.4e-08 * t108 * t62;
        let t112 = t95 * t95;
        let t113 = 1.0 / t112;
        let t115 = param_CC_3_0;
        let t116 = param_CC_3_1;
        let t117 = t116 * sigma0;
        let t120 = param_CC_3_2;
        let t121 = t120 * t44;
        let t124 = param_CC_3_3;
        let t125 = t124 * t56;
        let t128 = t115 + 0.004 * t117 * t40 + 1.6e-05 * t121 * t52 + 6.4e-08 * t125 * t62;
        let t129 = t112 * t95;
        let t130 = 1.0 / t129;
        let t132 = t28 + 0.004 * t30 * t40 + 1.6e-05 * t45 * t52 + 6.4e-08 * t57 * t62 + t78 * t96 + t111 * t113 + t128 * t130;
        let t136 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t132);
        let t137 = rho1 <= dens_threshold;
        let t138 = -t16;
        let t140 = piecewise5(t14, t11, t10, t15, t138 * t7);
        let t141 = 1.0 + t140;
        let t142 = t141 <= zeta_threshold;
        let t143 = pow_1_3(t141);
        let t145 = piecewise3(t142, t22, t143 * t141);
        let t146 = t145 * t26;
        let t147 = t29 * sigma2;
        let t148 = rho1 * rho1;
        let t149 = pow_1_3(rho1);
        let t150 = t149 * t149;
        let t152 = 1.0 / t150 / t148;
        let t155 = 1.0 + 0.004 * sigma2 * t152;
        let t156 = 1.0 / t155;
        let t157 = t152 * t156;
        let t160 = sigma2 * sigma2;
        let t161 = t43 * t160;
        let t162 = t148 * t148;
        let t163 = t162 * rho1;
        let t165 = 1.0 / t149 / t163;
        let t166 = t155 * t155;
        let t167 = 1.0 / t166;
        let t168 = t165 * t167;
        let t171 = t160 * sigma2;
        let t172 = t55 * t171;
        let t173 = t162 * t162;
        let t174 = 1.0 / t173;
        let t175 = t166 * t155;
        let t176 = 1.0 / t175;
        let t177 = t174 * t176;
        let t180 = t66 * sigma2;
        let t183 = t70 * t160;
        let t186 = t74 * t171;
        let t189 = t65 + 0.004 * t180 * t157 + 1.6e-05 * t183 * t168 + 6.4e-08 * t186 * t177;
        let t190 = piecewise5(t85, t11, t83, t15, -t17);
        let t191 = 1.0 + t190;
        let t192 = t191 <= zeta_threshold;
        let t193 = pow_1_3(t191);
        let t195 = piecewise3(t192, t89, 1.0 / t193);
        let t198 = 1.0 + 0.4 * t81 * t195;
        let t199 = 1.0 / t198;
        let t201 = t99 * sigma2;
        let t204 = t103 * t160;
        let t207 = t107 * t171;
        let t210 = t98 + 0.004 * t201 * t157 + 1.6e-05 * t204 * t168 + 6.4e-08 * t207 * t177;
        let t211 = t198 * t198;
        let t212 = 1.0 / t211;
        let t214 = t116 * sigma2;
        let t217 = t120 * t160;
        let t220 = t124 * t171;
        let t223 = t115 + 0.004 * t214 * t157 + 1.6e-05 * t217 * t168 + 6.4e-08 * t220 * t177;
        let t224 = t211 * t198;
        let t225 = 1.0 / t224;
        let t227 = t28 + 0.004 * t147 * t157 + 1.6e-05 * t161 * t168 + 6.4e-08 * t172 * t177 + t189 * t199 + t210 * t212 + t223 * t225;
        let t231 = piecewise3(t137, 0.0, -3.0 / 8.0 * t5 * t146 * t227);
        let tzk0 = t136 + t231;
        zk[ip] += tzk0;
        let t232 = t6 * t6;
        let t233 = 1.0 / t232;
        let t234 = t16 * t233;
        let t235 = t7 - t234;
        let t236 = piecewise5(t10, 0.0, t14, 0.0, t235);
        let t239 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t236);
        let t240 = t239 * t26;
        let t244 = t26 * t26;
        let t245 = 1.0 / t244;
        let t246 = t25 * t245;
        let t249 = t5 * t246 * t132 / 8.0;
        let t250 = t31 * rho0;
        let t252 = 1.0 / t33 / t250;
        let t253 = t252 * t39;
        let t256 = t29 * t44;
        let t257 = t46 * t31;
        let t259 = 1.0 / t32 / t257;
        let t260 = t259 * t51;
        let t265 = t43 * t56;
        let t266 = t58 * rho0;
        let t267 = 1.0 / t266;
        let t268 = t267 * t61;
        let t273 = t44 * t44;
        let t274 = t55 * t273;
        let t275 = t58 * t250;
        let t277 = 1.0 / t33 / t275;
        let t278 = t50 * t50;
        let t279 = 1.0 / t278;
        let t280 = t277 * t279;
        let t285 = t66 * t44;
        let t290 = t70 * t56;
        let t295 = t74 * t273;
        let t298 = -0.010666666666666666 * t67 * t253 + 4.266666666666667e-05 * t285 * t260 - 8.533333333333334e-05 * t71 * t260 + 3.413333333333333e-07 * t290 * t268 - 5.12e-07 * t75 * t268 + 2.048e-09 * t295 * t280;
        let t300 = t78 * t113;
        let t303 = 1.0 / t26 / t6 * t80;
        let t305 = 0.13333333333333333 * t303 * t92;
        let t307 = 1.0 / t90 / t87;
        let t308 = piecewise5(t83, 0.0, t85, 0.0, t235);
        let t311 = piecewise3(t88, 0.0, -t307 * t308 / 3.0);
        let t314 = -t305 + 0.4 * t81 * t311;
        let t318 = t99 * t44;
        let t323 = t103 * t56;
        let t328 = t107 * t273;
        let t331 = -0.010666666666666666 * t100 * t253 + 4.266666666666667e-05 * t318 * t260 - 8.533333333333334e-05 * t104 * t260 + 3.413333333333333e-07 * t323 * t268 - 5.12e-07 * t108 * t268 + 2.048e-09 * t328 * t280;
        let t333 = t111 * t130;
        let t338 = t116 * t44;
        let t343 = t120 * t56;
        let t348 = t124 * t273;
        let t351 = -0.010666666666666666 * t117 * t253 + 4.266666666666667e-05 * t338 * t260 - 8.533333333333334e-05 * t121 * t260 + 3.413333333333333e-07 * t343 * t268 - 5.12e-07 * t125 * t268 + 2.048e-09 * t348 * t280;
        let t353 = t112 * t112;
        let t354 = 1.0 / t353;
        let t355 = t128 * t354;
        let t358 = -0.010666666666666666 * t30 * t253 + 4.266666666666667e-05 * t256 * t260 - 8.533333333333334e-05 * t45 * t260 + 3.413333333333333e-07 * t265 * t268 - 5.12e-07 * t57 * t268 + 2.048e-09 * t274 * t280 + t298 * t96 - t300 * t314 + t331 * t113 - 2.0 * t333 * t314 + t351 * t130 - 3.0 * t355 * t314;
        let t363 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t240 * t132 - t249 - 3.0 / 8.0 * t5 * t27 * t358);
        let t364 = t138 * t233;
        let t366 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t364);
        let t369 = piecewise3(t142, 0.0, 4.0 / 3.0 * t143 * t366);
        let t370 = t369 * t26;
        let t374 = t145 * t245;
        let t377 = t5 * t374 * t227 / 8.0;
        let t378 = t189 * t212;
        let t380 = 0.13333333333333333 * t303 * t195;
        let t382 = 1.0 / t193 / t191;
        let t384 = piecewise5(t85, 0.0, t83, 0.0, -t235);
        let t387 = piecewise3(t192, 0.0, -t382 * t384 / 3.0);
        let t390 = -t380 + 0.4 * t81 * t387;
        let t392 = t210 * t225;
        let t395 = t211 * t211;
        let t396 = 1.0 / t395;
        let t397 = t223 * t396;
        let t400 = -t378 * t390 - 2.0 * t392 * t390 - 3.0 * t397 * t390;
        let t405 = piecewise3(t137, 0.0, -3.0 / 8.0 * t5 * t370 * t227 - t377 - 3.0 / 8.0 * t5 * t146 * t400);
        let tvrho0 = t136 + t231 + t6 * (t363 + t405);
        vrho[ip * 2] += tvrho0;
        let t408 = -t7 - t234;
        let t409 = piecewise5(t10, 0.0, t14, 0.0, t408);
        let t412 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t409);
        let t413 = t412 * t26;
        let t417 = piecewise5(t83, 0.0, t85, 0.0, t408);
        let t420 = piecewise3(t88, 0.0, -t307 * t417 / 3.0);
        let t423 = -t305 + 0.4 * t81 * t420;
        let t429 = -t300 * t423 - 2.0 * t333 * t423 - 3.0 * t355 * t423;
        let t434 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t413 * t132 - t249 - 3.0 / 8.0 * t5 * t27 * t429);
        let t436 = piecewise5(t14, 0.0, t10, 0.0, t7 - t364);
        let t439 = piecewise3(t142, 0.0, 4.0 / 3.0 * t143 * t436);
        let t440 = t439 * t26;
        let t444 = t148 * rho1;
        let t446 = 1.0 / t150 / t444;
        let t447 = t446 * t156;
        let t450 = t29 * t160;
        let t451 = t162 * t148;
        let t453 = 1.0 / t149 / t451;
        let t454 = t453 * t167;
        let t459 = t43 * t171;
        let t460 = t173 * rho1;
        let t461 = 1.0 / t460;
        let t462 = t461 * t176;
        let t467 = t160 * t160;
        let t468 = t55 * t467;
        let t469 = t173 * t444;
        let t471 = 1.0 / t150 / t469;
        let t472 = t166 * t166;
        let t473 = 1.0 / t472;
        let t474 = t471 * t473;
        let t479 = t66 * t160;
        let t484 = t70 * t171;
        let t489 = t74 * t467;
        let t492 = -0.010666666666666666 * t180 * t447 + 4.266666666666667e-05 * t479 * t454 - 8.533333333333334e-05 * t183 * t454 + 3.413333333333333e-07 * t484 * t462 - 5.12e-07 * t186 * t462 + 2.048e-09 * t489 * t474;
        let t495 = piecewise5(t85, 0.0, t83, 0.0, -t408);
        let t498 = piecewise3(t192, 0.0, -t382 * t495 / 3.0);
        let t501 = -t380 + 0.4 * t81 * t498;
        let t505 = t99 * t160;
        let t510 = t103 * t171;
        let t515 = t107 * t467;
        let t518 = -0.010666666666666666 * t201 * t447 + 4.266666666666667e-05 * t505 * t454 - 8.533333333333334e-05 * t204 * t454 + 3.413333333333333e-07 * t510 * t462 - 5.12e-07 * t207 * t462 + 2.048e-09 * t515 * t474;
        let t524 = t116 * t160;
        let t529 = t120 * t171;
        let t534 = t124 * t467;
        let t537 = -0.010666666666666666 * t214 * t447 + 4.266666666666667e-05 * t524 * t454 - 8.533333333333334e-05 * t217 * t454 + 3.413333333333333e-07 * t529 * t462 - 5.12e-07 * t220 * t462 + 2.048e-09 * t534 * t474;
        let t541 = -0.010666666666666666 * t147 * t447 + 4.266666666666667e-05 * t450 * t454 - 8.533333333333334e-05 * t161 * t454 + 3.413333333333333e-07 * t459 * t462 - 5.12e-07 * t172 * t462 + 2.048e-09 * t468 * t474 + t492 * t199 - t378 * t501 + t518 * t212 - 2.0 * t392 * t501 + t537 * t225 - 3.0 * t397 * t501;
        let t546 = piecewise3(t137, 0.0, -3.0 / 8.0 * t5 * t440 * t227 - t377 - 3.0 / 8.0 * t5 * t146 * t541);
        let tvrho1 = t136 + t231 + t6 * (t434 + t546);
        vrho[ip * 2 + 1] += tvrho1;
        let t554 = t43 * sigma0;
        let t559 = t55 * t44;
        let t562 = t58 * t31;
        let t564 = 1.0 / t33 / t562;
        let t565 = t564 * t279;
        let t573 = t70 * sigma0;
        let t578 = t74 * t44;
        let t583 = 0.004 * t66 * t35 * t39 - 1.6e-05 * t67 * t52 + 3.2e-05 * t573 * t52 - 1.28e-07 * t71 * t62 + 1.92e-07 * t578 * t62 - 7.68e-10 * t75 * t565;
        let t590 = t103 * sigma0;
        let t595 = t107 * t44;
        let t600 = 0.004 * t99 * t35 * t39 - 1.6e-05 * t100 * t52 + 3.2e-05 * t590 * t52 - 1.28e-07 * t104 * t62 + 1.92e-07 * t595 * t62 - 7.68e-10 * t108 * t565;
        let t607 = t120 * sigma0;
        let t612 = t124 * t44;
        let t617 = 0.004 * t116 * t35 * t39 - 1.6e-05 * t117 * t52 + 3.2e-05 * t607 * t52 - 1.28e-07 * t121 * t62 + 1.92e-07 * t612 * t62 - 7.68e-10 * t125 * t565;
        let t619 = 0.004 * t29 * t35 * t39 - 1.6e-05 * t30 * t52 + 3.2e-05 * t554 * t52 - 1.28e-07 * t45 * t62 + 1.92e-07 * t559 * t62 - 7.68e-10 * t57 * t565 + t583 * t96 + t600 * t113 + t617 * t130;
        let t623 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t619);
        let tvsigma0 = t6 * t623;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t629 = t43 * sigma2;
        let t634 = t55 * t160;
        let t637 = t173 * t148;
        let t639 = 1.0 / t150 / t637;
        let t640 = t639 * t473;
        let t648 = t70 * sigma2;
        let t653 = t74 * t160;
        let t658 = 0.004 * t66 * t152 * t156 - 1.6e-05 * t180 * t168 + 3.2e-05 * t648 * t168 - 1.28e-07 * t183 * t177 + 1.92e-07 * t653 * t177 - 7.68e-10 * t186 * t640;
        let t665 = t103 * sigma2;
        let t670 = t107 * t160;
        let t675 = 0.004 * t99 * t152 * t156 - 1.6e-05 * t201 * t168 + 3.2e-05 * t665 * t168 - 1.28e-07 * t204 * t177 + 1.92e-07 * t670 * t177 - 7.68e-10 * t207 * t640;
        let t682 = t120 * sigma2;
        let t687 = t124 * t160;
        let t692 = 0.004 * t116 * t152 * t156 - 1.6e-05 * t214 * t168 + 3.2e-05 * t682 * t168 - 1.28e-07 * t217 * t177 + 1.92e-07 * t687 * t177 - 7.68e-10 * t220 * t640;
        let t694 = 0.004 * t29 * t152 * t156 - 1.6e-05 * t147 * t168 + 3.2e-05 * t629 * t168 - 1.28e-07 * t161 * t177 + 1.92e-07 * t634 * t177 - 7.68e-10 * t172 * t640 + t658 * t199 + t675 * t212 + t692 * t225;
        let t698 = piecewise3(t137, 0.0, -3.0 / 8.0 * t5 * t146 * t694);
        let tvsigma2 = t6 * t698;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
