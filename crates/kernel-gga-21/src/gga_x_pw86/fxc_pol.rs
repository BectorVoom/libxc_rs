//! GGA_X_PW86 fxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 72 shared lines across all orders.
//! Delta: 108 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pw86_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_aa: f64,
    param_bb: f64,
    param_cc: f64,
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
        // --- shared preamble (72 lines) ---
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
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_aa * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t43 = t28 * t28;
        let t44 = param_bb * t43;
        let t46 = 1.0 / t31 / t30;
        let t47 = sigma0 * sigma0;
        let t48 = t46 * t47;
        let t49 = t35 * t35;
        let t50 = t49 * rho0;
        let t52 = 1.0 / t36 / t50;
        let t56 = t30 * t30;
        let t58 = param_cc / t56;
        let t59 = t47 * sigma0;
        let t60 = t49 * t49;
        let t61 = 1.0 / t60;
        let t65 = 1.0 + t29 * t34 * t39 / 24.0 + t44 * t48 * t52 / 576.0 + t58 * t59 * t61 / 2304.0;
        let t66 = f64::powf(t65, 1.0 / 15.0);
        let t70 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t66);
        let t71 = rho1 <= dens_threshold;
        let t72 = -t16;
        let t74 = piecewise5(t14, t11, t10, t15, t72 * t7);
        let t75 = 1.0 + t74;
        let t76 = t75 <= zeta_threshold;
        let t77 = pow_1_3(t75);
        let t79 = piecewise3(t76, t22, t77 * t75);
        let t80 = t79 * t26;
        let t81 = t33 * sigma2;
        let t82 = rho1 * rho1;
        let t83 = pow_1_3(rho1);
        let t84 = t83 * t83;
        let t86 = 1.0 / t84 / t82;
        let t90 = sigma2 * sigma2;
        let t91 = t46 * t90;
        let t92 = t82 * t82;
        let t93 = t92 * rho1;
        let t95 = 1.0 / t83 / t93;
        let t99 = t90 * sigma2;
        let t100 = t92 * t92;
        let t101 = 1.0 / t100;
        let t105 = 1.0 + t29 * t81 * t86 / 24.0 + t44 * t91 * t95 / 576.0 + t58 * t99 * t101 / 2304.0;
        let t106 = f64::powf(t105, 1.0 / 15.0);
        let t110 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t80 * t106);
        let tzk0 = t70 + t110;
        zk[ip] += tzk0;
        // --- vxc delta (63 lines) ---
        let t111 = t6 * t6;
        let t112 = 1.0 / t111;
        let t113 = t16 * t112;
        let t115 = piecewise5(t10, 0.0, t14, 0.0, t7 - t113);
        let t118 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t115);
        let t123 = t26 * t26;
        let t124 = 1.0 / t123;
        let t128 = t5 * t25 * t124 * t66 / 8.0;
        let t129 = t5 * t25;
        let t130 = t66 * t66;
        let t131 = t130 * t130;
        let t133 = t131 * t131;
        let t134 = t133 * t131 * t130;
        let t135 = 1.0 / t134;
        let t136 = t26 * t135;
        let t137 = t35 * rho0;
        let t139 = 1.0 / t37 / t137;
        let t143 = t49 * t35;
        let t145 = 1.0 / t36 / t143;
        let t149 = t60 * rho0;
        let t150 = 1.0 / t149;
        let t154 = -t29 * t34 * t139 / 9.0 - t44 * t48 * t145 / 108.0 - t58 * t59 * t150 / 288.0;
        let t155 = t136 * t154;
        let t159 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t118 * t26 * t66 - t128 - t129 * t155 / 40.0);
        let t160 = t72 * t112;
        let t162 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t160);
        let t165 = piecewise3(t76, 0.0, 4.0 / 3.0 * t77 * t162);
        let t173 = t5 * t79 * t124 * t106 / 8.0;
        let t175 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t165 * t26 * t106 - t173);
        let tvrho0 = t70 + t110 + t6 * (t159 + t175);
        vrho[ip * 2] += tvrho0;
        let t179 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t113);
        let t182 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t179);
        let t188 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t182 * t26 * t66 - t128);
        let t190 = piecewise5(t14, 0.0, t10, 0.0, t7 - t160);
        let t193 = piecewise3(t76, 0.0, 4.0 / 3.0 * t77 * t190);
        let t198 = t5 * t79;
        let t199 = t106 * t106;
        let t200 = t199 * t199;
        let t202 = t200 * t200;
        let t203 = t202 * t200 * t199;
        let t204 = 1.0 / t203;
        let t205 = t26 * t204;
        let t206 = t82 * rho1;
        let t208 = 1.0 / t84 / t206;
        let t212 = t92 * t82;
        let t214 = 1.0 / t83 / t212;
        let t218 = t100 * rho1;
        let t219 = 1.0 / t218;
        let t223 = -t29 * t81 * t208 / 9.0 - t44 * t91 * t214 / 108.0 - t58 * t99 * t219 / 288.0;
        let t224 = t205 * t223;
        let t228 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t193 * t26 * t106 - t173 - t198 * t224 / 40.0);
        let tvrho1 = t70 + t110 + t6 * (t188 + t228);
        vrho[ip * 2 + 1] += tvrho1;
        let t234 = t46 * sigma0;
        let t241 = t29 * t33 * t39 / 24.0 + t44 * t234 * t52 / 288.0 + t58 * t47 * t61 / 768.0;
        let t242 = t136 * t241;
        let t245 = piecewise3(t1, 0.0, -t129 * t242 / 40.0);
        let tvsigma0 = t6 * t245;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t249 = t46 * sigma2;
        let t256 = t29 * t33 * t86 / 24.0 + t44 * t249 * t95 / 288.0 + t58 * t90 * t101 / 768.0;
        let t257 = t205 * t256;
        let t260 = piecewise3(t71, 0.0, -t198 * t257 / 40.0);
        let tvsigma2 = t6 * t260;
        vsigma[ip * 3 + 2] += tvsigma2;
        // --- fxc delta (this level) (108 lines) ---
        let t263 = t23 * t23;
        let t264 = 1.0 / t263;
        let t265 = t115 * t115;
        let t268 = t111 * t6;
        let t269 = 1.0 / t268;
        let t270 = t16 * t269;
        let t273 = piecewise5(t10, 0.0, t14, 0.0, -2.0 * t112 + 2.0 * t270);
        let t277 = piecewise3(t20, 0.0, 4.0 / 9.0 * t264 * t265 + 4.0 / 3.0 * t23 * t273);
        let t284 = t5 * t118 * t124 * t66;
        let t286 = t5 * t118;
        let t290 = 1.0 / t123 / t6;
        let t294 = t5 * t25 * t290 * t66 / 12.0;
        let t295 = t124 * t135;
        let t296 = t295 * t154;
        let t297 = t129 * t296;
        let t300 = 1.0 / t134 / t65;
        let t301 = t26 * t300;
        let t302 = t154 * t154;
        let t303 = t301 * t302;
        let t307 = 1.0 / t37 / t49;
        let t313 = 1.0 / t36 / t49 / t137;
        let t318 = 1.0 / t60 / t35;
        let t322 = 11.0 / 27.0 * t29 * t34 * t307 + 19.0 / 324.0 * t44 * t48 * t313 + t58 * t59 * t318 / 32.0;
        let t323 = t136 * t322;
        let t327 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t277 * t26 * t66 - t284 / 4.0 - t286 * t155 / 20.0 + t294 - t297 / 60.0 + 7.0 / 300.0 * t129 * t303 - t129 * t323 / 40.0);
        let t328 = t77 * t77;
        let t329 = 1.0 / t328;
        let t330 = t162 * t162;
        let t333 = t72 * t269;
        let t336 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t112 + 2.0 * t333);
        let t340 = piecewise3(t76, 0.0, 4.0 / 9.0 * t329 * t330 + 4.0 / 3.0 * t77 * t336);
        let t347 = t5 * t165 * t124 * t106;
        let t352 = t5 * t79 * t290 * t106 / 12.0;
        let t354 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t340 * t26 * t106 - t347 / 4.0 + t352);
        let tv2rho20 = 2.0 * t159 + 2.0 * t175 + t6 * (t327 + t354);
        v2rho2[ip * 3] += tv2rho20;
        let t357 = t264 * t179;
        let t361 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t270);
        let t365 = piecewise3(t20, 0.0, 4.0 / 9.0 * t357 * t115 + 4.0 / 3.0 * t23 * t361);
        let t372 = t5 * t182 * t124 * t66;
        let t374 = t5 * t182;
        let t380 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t365 * t26 * t66 - t372 / 8.0 - t374 * t155 / 40.0 - t284 / 8.0 + t294 - t297 / 120.0);
        let t381 = t329 * t190;
        let t385 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t333);
        let t389 = piecewise3(t76, 0.0, 4.0 / 9.0 * t381 * t162 + 4.0 / 3.0 * t77 * t385);
        let t396 = t5 * t193 * t124 * t106;
        let t399 = t5 * t165;
        let t402 = t124 * t204;
        let t403 = t402 * t223;
        let t404 = t198 * t403;
        let t407 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t389 * t26 * t106 - t396 / 8.0 - t347 / 8.0 + t352 - t399 * t224 / 40.0 - t404 / 120.0);
        let tv2rho21 = t159 + t175 + t188 + t228 + t6 * (t380 + t407);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t412 = t179 * t179;
        let t417 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t112 + 2.0 * t270);
        let t421 = piecewise3(t20, 0.0, 4.0 / 9.0 * t264 * t412 + 4.0 / 3.0 * t23 * t417);
        let t428 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t421 * t26 * t66 - t372 / 4.0 + t294);
        let t429 = t190 * t190;
        let t434 = piecewise5(t14, 0.0, t10, 0.0, -2.0 * t112 + 2.0 * t333);
        let t438 = piecewise3(t76, 0.0, 4.0 / 9.0 * t329 * t429 + 4.0 / 3.0 * t77 * t434);
        let t444 = t5 * t193;
        let t449 = 1.0 / t203 / t105;
        let t450 = t26 * t449;
        let t451 = t223 * t223;
        let t452 = t450 * t451;
        let t456 = 1.0 / t84 / t92;
        let t462 = 1.0 / t83 / t92 / t206;
        let t467 = 1.0 / t100 / t82;
        let t471 = 11.0 / 27.0 * t29 * t81 * t456 + 19.0 / 324.0 * t44 * t91 * t462 + t58 * t99 * t467 / 32.0;
        let t472 = t205 * t471;
        let t476 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t438 * t26 * t106 - t396 / 4.0 - t444 * t224 / 20.0 + t352 - t404 / 60.0 + 7.0 / 300.0 * t198 * t452 - t198 * t472 / 40.0);
        let tv2rho22 = 2.0 * t188 + 2.0 * t228 + t6 * (t428 + t476);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t481 = t295 * t241;
        let t483 = t129 * t481 / 120.0;
        let t484 = t241 * t154;
        let t485 = t301 * t484;
        let t497 = -t29 * t33 * t139 / 9.0 - t44 * t234 * t145 / 54.0 - t58 * t47 * t150 / 96.0;
        let t498 = t136 * t497;
        let t502 = piecewise3(t1, 0.0, -t286 * t242 / 40.0 - t483 + 7.0 / 300.0 * t129 * t485 - t129 * t498 / 40.0);
        let tv2rhosigma0 = t6 * t502 + t245;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t506 = t402 * t256;
        let t508 = t198 * t506 / 120.0;
        let t510 = piecewise3(t71, 0.0, -t399 * t257 / 40.0 - t508);
        let tv2rhosigma2 = t6 * t510 + t260;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t515 = piecewise3(t1, 0.0, -t374 * t242 / 40.0 - t483);
        let tv2rhosigma3 = t6 * t515 + t245;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t519 = t256 * t223;
        let t520 = t450 * t519;
        let t532 = -t29 * t33 * t208 / 9.0 - t44 * t249 * t214 / 54.0 - t58 * t90 * t219 / 96.0;
        let t533 = t205 * t532;
        let t537 = piecewise3(t71, 0.0, -t444 * t257 / 40.0 - t508 + 7.0 / 300.0 * t198 * t520 - t198 * t533 / 40.0);
        let tv2rhosigma5 = t6 * t537 + t260;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t539 = t241 * t241;
        let t540 = t301 * t539;
        let t549 = t44 * t46 * t52 / 288.0 + t58 * sigma0 * t61 / 384.0;
        let t550 = t136 * t549;
        let t554 = piecewise3(t1, 0.0, 7.0 / 300.0 * t129 * t540 - t129 * t550 / 40.0);
        let tv2sigma20 = t6 * t554;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t555 = t256 * t256;
        let t556 = t450 * t555;
        let t565 = t44 * t46 * t95 / 288.0 + t58 * sigma2 * t101 / 384.0;
        let t566 = t205 * t565;
        let t570 = piecewise3(t71, 0.0, 7.0 / 300.0 * t198 * t556 - t198 * t566 / 40.0);
        let tv2sigma25 = t6 * t570;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
