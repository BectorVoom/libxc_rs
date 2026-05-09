//! GGA_C_WI lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 26 shared lines across all orders.
//! Delta: 22 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_wi_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_k: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (26 lines) ---
        let t1 = param_b * sigma[ip];
        let t2 = rho[ip] * rho[ip];
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / t2;
        let t7 = param_k * sigma[ip];
        let t9 = f64::exp(-t7 * t6);
        let t12 = t1 * t6 * t9 + param_a;
        let t13 = M_CBRT3;
        let t15 = pow_1_3(1.0 / M_PI);
        let t16 = t13 * t15;
        let t17 = M_CBRT4;
        let t18 = t17 * t17;
        let t22 = t13 * t13;
        let t23 = M_CBRTPI;
        let t25 = f64::sqrt(sigma[ip]);
        let t26 = t25 * sigma[ip];
        let t27 = t2 * t2;
        let t28 = 1.0 / t27;
        let t31 = 1.0 / t3 / rho[ip];
        let t32 = t25 * t31;
        let t33 = f64::sqrt(t32);
        let t38 = 1.0 + param_d * t17 * t22 * t23 * t33 * t26 * t28 / 3.0;
        let t42 = param_c + t16 * t18 / t3 * t38 / 4.0;
        let t43 = 1.0 / t42;
        let tzk0 = t12 * t43;
        zk[ip] += tzk0;
        // --- vxc delta (30 lines) ---
        let t44 = t2 * rho[ip];
        let t46 = 1.0 / t4 / t44;
        let t49 = sigma[ip] * sigma[ip];
        let t50 = param_b * t49;
        let t51 = t27 * t2;
        let t53 = 1.0 / t3 / t51;
        let t58 = 8.0 / 3.0 * t50 * t53 * param_k * t9 - 8.0 / 3.0 * t1 * t46 * t9;
        let t59 = rho[ip] * t58;
        let t61 = rho[ip] * t12;
        let t62 = t42 * t42;
        let t63 = 1.0 / t62;
        let t71 = t33 * sigma[ip] * t6;
        let t72 = t23 * t71;
        let t73 = t72 * t25;
        let t76 = -t16 * t18 * t31 * t38 / 12.0 - 14.0 / 3.0 * t15 * t6 * param_d * t73;
        let t77 = t63 * t76;
        let tvrho0 = t59 * t43 - t61 * t77 + tzk0;
        vrho[ip] += tvrho0;
        let t81 = t27 * rho[ip];
        let t83 = 1.0 / t3 / t81;
        let t87 = -t1 * t83 * param_k * t9 + param_b * t6 * t9;
        let t88 = rho[ip] * t87;
        let t90 = 1.0 / t4;
        let t91 = t90 * t12;
        let t92 = t63 * t15;
        let t93 = t91 * t92;
        let t94 = param_d * t23;
        let t95 = 1.0 / t25;
        let t96 = t71 * t95;
        let t97 = t94 * t96;
        let tvsigma0 = t88 * t43 - 7.0 / 4.0 * t93 * t97;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (64 lines) ---
        let t102 = t12 * t63;
        let t106 = 1.0 / t4 / t27;
        let t110 = t27 * t44;
        let t112 = 1.0 / t3 / t110;
        let t117 = t49 * sigma[ip];
        let t118 = param_b * t117;
        let t119 = t27 * t27;
        let t120 = t119 * t2;
        let t121 = 1.0 / t120;
        let t122 = param_k * param_k;
        let t127 = 88.0 / 9.0 * t1 * t106 * t9 - 24.0 * t50 * t112 * param_k * t9 + 64.0 / 9.0 * t118 * t121 * t122 * t9;
        let t128 = rho[ip] * t127;
        let t133 = 1.0 / t62 / t42;
        let t134 = t76 * t76;
        let t135 = t133 * t134;
        let t148 = 1.0 / t81;
        let t151 = t33 * t32;
        let t152 = t23 * t151;
        let t153 = t152 * sigma[ip];
        let t156 = t16 * t18 / t3 / t2 * t38 / 9.0 + 14.0 * t15 * t46 * param_d * t73 + 140.0 / 9.0 * t15 * t148 * param_d * t153;
        let t157 = t63 * t156;
        let tv2rho20 = -2.0 * t102 * t76 + t128 * t43 + 2.0 * t61 * t135 - t61 * t157 + 2.0 * t58 * t43 - 2.0 * t59 * t77;
        v2rho2[ip] += tv2rho20;
        let t163 = param_b * t53;
        let t164 = t7 * t9;
        let t167 = t119 * rho[ip];
        let t168 = 1.0 / t167;
        let t173 = -8.0 / 3.0 * param_b * t46 * t9 + 8.0 * t163 * t164 - 8.0 / 3.0 * t50 * t168 * t122 * t9;
        let t174 = rho[ip] * t173;
        let t178 = 1.0 / t4 / rho[ip];
        let t179 = t178 * t12;
        let t180 = t179 * t92;
        let t183 = t90 * t58;
        let t184 = t183 * t92;
        let t187 = t133 * t15;
        let t188 = t91 * t187;
        let t190 = t94 * t96 * t76;
        let t193 = 1.0 / t44;
        let t194 = t193 * t12;
        let t196 = t15 * param_d;
        let t197 = t196 * t152;
        let tv2rhosigma0 = t87 * t43 + t174 * t43 - t88 * t77 + 7.0 / 6.0 * t180 * t97 - 7.0 / 4.0 * t184 * t97 + 7.0 / 2.0 * t188 * t190 + 35.0 / 6.0 * t194 * t63 * t197;
        v2rhosigma[ip] += tv2rhosigma0;
        let t201 = param_k * t9;
        let t204 = 1.0 / t119;
        let t208 = t1 * t204 * t122 * t9 - 2.0 * param_b * t83 * t201;
        let t209 = rho[ip] * t208;
        let t211 = t90 * t87;
        let t212 = t211 * t92;
        let t215 = t168 * t12;
        let t216 = t215 * t133;
        let t217 = t15 * t15;
        let t218 = param_d * param_d;
        let t219 = t217 * t218;
        let t220 = t23 * t23;
        let t222 = t219 * t220 * t26;
        let t225 = 1.0 / t2;
        let t226 = t225 * t12;
        let t227 = t226 * t92;
        let t228 = 1.0 / sigma[ip];
        let t229 = t151 * t228;
        let t230 = t94 * t229;
        let t233 = 1.0 / t26;
        let t234 = t71 * t233;
        let t235 = t94 * t234;
        let tv2sigma20 = t209 * t43 - 7.0 / 2.0 * t212 * t97 + 49.0 / 8.0 * t216 * t222 - 35.0 / 16.0 * t227 * t230 + 7.0 / 8.0 * t93 * t235;
        v2sigma2[ip] += tv2sigma20;
        // --- kxc delta (109 lines) ---
        let t240 = t58 * t63;
        let t243 = t12 * t133;
        let t249 = 1.0 / t4 / t81;
        let t254 = 1.0 / t3 / t119;
        let t259 = t119 * t44;
        let t260 = 1.0 / t259;
        let t265 = t49 * t49;
        let t266 = param_b * t265;
        let t267 = t119 * t81;
        let t269 = 1.0 / t4 / t267;
        let t270 = t122 * param_k;
        let t275 = -1232.0 / 27.0 * t1 * t249 * t9 + 5456.0 / 27.0 * t50 * t254 * param_k * t9 - 1216.0 / 9.0 * t118 * t260 * t122 * t9 + 512.0 / 27.0 * t266 * t269 * t270 * t9;
        let t276 = rho[ip] * t275;
        let t284 = t62 * t62;
        let t285 = 1.0 / t284;
        let t286 = t134 * t76;
        let t287 = t285 * t286;
        let t290 = t133 * t76;
        let t291 = t290 * t156;
        let t295 = 1.0 / t3 / t44;
        let t312 = t23 * t33 * t26;
        let t315 = -7.0 / 27.0 * t16 * t18 * t295 * t38 - 1442.0 / 27.0 * t15 * t106 * param_d * t73 - 1120.0 / 9.0 * t15 / t51 * param_d * t153 - 280.0 / 9.0 * t15 * t112 * param_d * t312;
        let t316 = t63 * t315;
        let tv3rho30 = -3.0 * t102 * t156 + 3.0 * t127 * t43 - 3.0 * t128 * t77 + 6.0 * t243 * t134 + 6.0 * t59 * t135 - 3.0 * t59 * t157 - 6.0 * t240 * t76 + t276 * t43 - 6.0 * t61 * t287 + 6.0 * t61 * t291 - t61 * t316;
        v3rho3[ip] += tv3rho30;
        let t318 = t285 * t15;
        let t319 = t91 * t318;
        let t321 = t94 * t96 * t134;
        let t327 = t87 * t63;
        let t333 = param_b * t112;
        let t336 = param_b * t121;
        let t338 = t122 * t49 * t9;
        let t341 = t119 * t27;
        let t343 = 1.0 / t4 / t341;
        let t348 = 88.0 / 9.0 * param_b * t106 * t9 - 520.0 / 9.0 * t333 * t164 + 136.0 / 3.0 * t336 * t338 - 64.0 / 9.0 * t118 * t343 * t270 * t9;
        let t349 = rho[ip] * t348;
        let t355 = t28 * t12;
        let t359 = t193 * t58;
        let t363 = t6 * t12;
        let t364 = t363 * t92;
        let t367 = t179 * t187;
        let t370 = t183 * t187;
        let t374 = t94 * t96 * t156;
        let t377 = t178 * t58;
        let t378 = t377 * t92;
        let t381 = t90 * t127;
        let t382 = t381 * t92;
        let t385 = t194 * t187;
        let t387 = t94 * t151 * t76;
        let t390 = t83 * t12;
        let t391 = t390 * t92;
        let t393 = t94 * t33 * t25;
        let tv3rho2sigma0 = -21.0 / 2.0 * t319 * t321 - 2.0 * t174 * t77 - t88 * t157 - 2.0 * t327 * t76 + t349 * t43 + 2.0 * t173 * t43 + 2.0 * t88 * t135 - 385.0 / 18.0 * t355 * t63 * t197 + 35.0 / 3.0 * t359 * t63 * t197 - 35.0 / 18.0 * t364 * t97 - 14.0 / 3.0 * t367 * t190 + 7.0 * t370 * t190 + 7.0 / 2.0 * t188 * t374 + 7.0 / 3.0 * t378 * t97 - 7.0 / 4.0 * t382 * t97 - 70.0 / 3.0 * t385 * t387 - 35.0 / 3.0 * t391 * t393;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t399 = param_b * t168;
        let t401 = t122 * sigma[ip] * t9;
        let t405 = 1.0 / t4 / t259;
        let t410 = 32.0 / 3.0 * t163 * t201 - 40.0 / 3.0 * t399 * t401 + 8.0 / 3.0 * t50 * t405 * t270 * t9;
        let t411 = rho[ip] * t410;
        let t414 = t178 * t87;
        let t415 = t414 * t92;
        let t418 = t90 * t173;
        let t419 = t418 * t92;
        let t422 = t211 * t187;
        let t425 = t193 * t87;
        let t429 = t121 * t12;
        let t430 = t429 * t133;
        let t433 = t168 * t58;
        let t434 = t433 * t133;
        let t437 = t285 * t217;
        let t438 = t215 * t437;
        let t439 = t218 * t220;
        let t441 = t439 * t26 * t76;
        let t444 = t194 * t92;
        let t447 = t225 * t58;
        let t448 = t447 * t92;
        let t451 = t226 * t187;
        let t453 = t94 * t229 * t76;
        let t457 = 1.0 / t3 / t27;
        let t458 = t457 * t12;
        let t459 = t458 * t92;
        let t460 = t33 * t95;
        let t461 = t94 * t460;
        let t469 = t94 * t234 * t76;
        let tv3rhosigma20 = t208 * t43 + t411 * t43 - t209 * t77 + 7.0 / 3.0 * t415 * t97 - 7.0 / 2.0 * t419 * t97 + 7.0 * t422 * t190 + 35.0 / 3.0 * t425 * t63 * t197 - 441.0 / 8.0 * t430 * t222 + 49.0 / 8.0 * t434 * t222 - 147.0 / 8.0 * t438 * t441 + 35.0 / 24.0 * t444 * t230 - 35.0 / 16.0 * t448 * t230 + 35.0 / 8.0 * t451 * t453 + 35.0 / 8.0 * t459 * t461 - 7.0 / 12.0 * t180 * t235 + 7.0 / 8.0 * t184 * t235 - 7.0 / 4.0 * t188 * t469;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t473 = t122 * t9;
        let t477 = 1.0 / t4 / t120;
        let t481 = -t1 * t477 * t270 * t9 + 3.0 * param_b * t204 * t473;
        let t482 = rho[ip] * t481;
        let t484 = t90 * t208;
        let t485 = t484 * t92;
        let t488 = t168 * t87;
        let t489 = t488 * t133;
        let t492 = t225 * t87;
        let t493 = t492 * t92;
        let t498 = t477 * t12;
        let t500 = t218 * param_d;
        let t501 = t500 * sigma[ip];
        let t502 = t501 * t71;
        let t506 = t219 * t220 * t25;
        let t509 = t295 * t12;
        let t510 = t509 * t92;
        let t511 = t33 * t233;
        let t512 = t94 * t511;
        let t515 = 1.0 / t49;
        let t516 = t151 * t515;
        let t517 = t94 * t516;
        let t521 = 1.0 / t25 / t49;
        let t522 = t71 * t521;
        let t523 = t94 * t522;
        let tv3sigma30 = t482 * t43 - 21.0 / 4.0 * t485 * t97 + 147.0 / 8.0 * t489 * t222 - 105.0 / 16.0 * t493 * t230 + 21.0 / 8.0 * t212 * t235 - 1029.0 / 32.0 * t498 * t285 * t502 + 441.0 / 32.0 * t216 * t506 - 105.0 / 64.0 * t510 * t512 + 105.0 / 32.0 * t227 * t517 - 21.0 / 16.0 * t93 * t523;
        v3sigma3[ip] += tv3sigma30;
        // --- lxc delta (this level) (22 lines) ---
        let t562 = 1.0 / t33;
        let t563 = t23 * t562;
        let t578 = 1.0 / t4 / t51;
        let t588 = 1.0 / t341;
        let t593 = t119 * t51;
        let t602 = t119 * t119;
        let t606 = t122 * t122;
        let t621 = 1.0 / t284 / t42;
        let t622 = t134 * t134;
        let t630 = t156 * t156;
        let tv4rho40 = 24.0 * t58 * t133 * t134 - 24.0 * t12 * t285 * t286 + 24.0 * t243 * t76 * t156 - 4.0 * t276 * t77 - 6.0 * t128 * t157 - 4.0 * t59 * t316 - t61 * t63 * (70.0 / 81.0 * t16 * t18 * t457 * t38 + 6860.0 / 27.0 * t15 * t249 * param_d * t73 + 74900.0 / 81.0 * t15 / t110 * param_d * t153 + 12880.0 / 27.0 * t15 * t254 * param_d * t312 + 560.0 / 27.0 * t15 / t4 / t167 * param_d * t563 * t49) - 12.0 * t127 * t63 * t76 - 12.0 * t240 * t156 - 4.0 * t102 * t315 + rho[ip] * (20944.0 / 81.0 * t1 * t578 * t9 - 48752.0 / 27.0 * t50 / t3 / t167 * param_k * t9 + 164032.0 / 81.0 * t118 * t588 * t122 * t9 - 50176.0 / 81.0 * t266 / t4 / t593 * t270 * t9 + 4096.0 / 81.0 * param_b * t265 * sigma[ip] / t3 / t602 / rho[ip] * t606 * t9) * t43 + 12.0 * t128 * t135 - 24.0 * t59 * t287 + 24.0 * t59 * t291 + 24.0 * t61 * t621 * t622 - 36.0 * t61 * t285 * t134 * t156 + 6.0 * t61 * t133 * t630 + 8.0 * t61 * t290 * t315 + 4.0 * t275 * t43;
        v4rho4[ip] += tv4rho40;
        let t725 = -6.0 * t173 * t63 * t76 - 3.0 * t327 * t156 + 6.0 * t87 * t133 * t134 + rho[ip] * (-1232.0 / 27.0 * param_b * t249 * t9 + 4048.0 / 9.0 * param_b * t254 * t164 - 16400.0 / 27.0 * param_b * t260 * t338 + 5696.0 / 27.0 * param_b * t269 * t270 * t117 * t9 - 512.0 / 27.0 * t266 / t3 / t602 * t606 * t9) * t43 + 3.0 * t348 * t43 + 6.0 * t88 * t291 + 140.0 / 27.0 * t46 * t12 * t92 * t97 + 105.0 * t194 * t318 * t94 * t151 * t134 + 385.0 / 3.0 * t355 * t187 * t387 + 105.0 * t53 * t12 * t92 * t393 - 70.0 * t359 * t187 * t387 - 35.0 * t83 * t58 * t92 * t393 - 35.0 / 6.0 * t6 * t58 * t92 * t97 - 35.0 * t385 * t94 * t151 * t156 + 7.0 / 2.0 * t178 * t127 * t92 * t97 - 7.0 / 4.0 * t90 * t275 * t92 * t97 + 70.0 / 9.0 / t4 / t110 * t12 * t92 * t94 * t562 * sigma[ip] - 385.0 / 6.0 * t28 * t58 * t63 * t197;
        let t785 = 35.0 / 2.0 * t193 * t127 * t63 * t197 + 2485.0 / 27.0 * t148 * t12 * t63 * t197 - 63.0 / 2.0 * t183 * t318 * t321 + 35.0 / 3.0 * t363 * t187 * t190 - 14.0 * t377 * t187 * t190 - 7.0 * t367 * t374 + 21.0 / 2.0 * t381 * t187 * t190 + 21.0 / 2.0 * t370 * t374 + 7.0 / 2.0 * t188 * t94 * t96 * t315 + 70.0 * t390 * t187 * t94 * t33 * t76 * t25 + 21.0 * t179 * t318 * t321 + 42.0 * t91 * t621 * t15 * t94 * t96 * t286 - 63.0 / 2.0 * t91 * t318 * param_d * t72 * t95 * t76 * t156 - 3.0 * t349 * t77 - 3.0 * t174 * t157 - t88 * t316 + 6.0 * t174 * t135 - 6.0 * t88 * t287;
        let tv4rho3sigma0 = t725 + t785;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t872 = -7.0 / 6.0 * t378 * t235 + 7.0 / 8.0 * t382 * t235 - 35.0 / 9.0 * t6 * t87 * t92 * t97 + 147.0 / 2.0 * t215 * t621 * t217 * t439 * t26 * t134 - 175.0 / 72.0 * t355 * t92 * t230 + 35.0 / 36.0 * t364 * t235 + 14.0 / 3.0 * t178 * t173 * t92 * t97 - 7.0 / 2.0 * t90 * t348 * t92 * t97 - 140.0 / 3.0 * t425 * t187 * t387 - 70.0 / 3.0 * t83 * t87 * t92 * t393 - 385.0 / 9.0 * t28 * t87 * t63 * t197;
        let t943 = 35.0 / 8.0 * t451 * t94 * t229 * t156 - 35.0 / 2.0 * t458 * t187 * t94 * t460 * t76 + 7.0 / 3.0 * t367 * t469 - 7.0 / 2.0 * t370 * t469 - 7.0 / 4.0 * t188 * t94 * t234 * t156 - 21.0 * t211 * t318 * t321 - 105.0 / 8.0 * t226 * t318 * t94 * t229 * t134 + 21.0 / 4.0 * t319 * t94 * t234 * t134 - 2.0 * t411 * t77 - t209 * t157 + 2.0 * t209 * t135;
        let tv4rho2sigma20 = -2.0 * t208 * t63 * t76 + rho[ip] * (-608.0 / 9.0 * t333 * t201 + 1336.0 / 9.0 * t336 * t401 - 200.0 / 3.0 * param_b * t343 * t270 * t49 * t9 + 64.0 / 9.0 * t118 / t3 / t119 / t110 * t606 * t9) * t43 + 2.0 * t410 * t43 + 1323.0 / 4.0 * t429 * t437 * t441 - 147.0 / 4.0 * t433 * t437 * t441 - 147.0 / 8.0 * t438 * t439 * t26 * t156 + 35.0 / 12.0 * t359 * t92 * t230 - 175.0 / 8.0 * t391 * t461 - 35.0 / 16.0 * t225 * t127 * t92 * t230 + 35.0 / 4.0 * t457 * t58 * t92 * t461 + t872 + 70.0 / 3.0 * t193 * t173 * t63 * t197 - 441.0 / 4.0 * t121 * t58 * t133 * t222 + 49.0 / 8.0 * t168 * t127 * t133 * t222 - 35.0 / 12.0 * t578 * t12 * t63 * t196 * t563 + 2205.0 / 4.0 * t260 * t12 * t133 * t222 - 28.0 / 3.0 * t414 * t187 * t190 + 14.0 * t418 * t187 * t190 + 7.0 * t422 * t374 - 35.0 / 6.0 * t385 * t453 + 35.0 / 4.0 * t447 * t187 * t453 + t943;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t1013 = rho[ip] * (-24.0 * t399 * t473 + 56.0 / 3.0 * param_b * t405 * t270 * sigma[ip] * t9 - 8.0 / 3.0 * t50 / t3 / t593 * t606 * t9) * t43 - 1029.0 / 32.0 * t477 * t58 * t285 * t502 + 1715.0 / 16.0 / t267 * t12 * t285 * t500 * t26 * t151 + 343.0 * t405 * t12 * t285 * t502 + t481 * t43 - 105.0 / 64.0 * t295 * t58 * t92 * t512 + 35.0 / 32.0 * t249 * t12 * t92 * t94 * t562 * t228 + 105.0 / 32.0 * t448 * t517 - 21.0 / 16.0 * t184 * t523 + 7.0 / 2.0 * t178 * t208 * t92 * t97 + 35.0 / 8.0 * t425 * t92 * t230 - 7.0 / 4.0 * t415 * t235 - 35.0 / 32.0 * t459 * t512 - 35.0 / 16.0 * t444 * t517 + 7.0 / 8.0 * t180 * t523 - 21.0 / 4.0 * t90 * t410 * t92 * t97 - 441.0 / 8.0 * t488 * t437 * t441;
        let t1071 = -105.0 / 16.0 * t225 * t173 * t92 * t230 + 105.0 / 8.0 * t457 * t87 * t92 * t461 + 21.0 / 8.0 * t419 * t235 - 1323.0 / 32.0 * t438 * t439 * t25 * t76 + 35.0 / 2.0 * t193 * t208 * t63 * t197 + 147.0 / 8.0 * t168 * t173 * t133 * t222 + 1029.0 / 8.0 * t498 * t621 * t501 * t71 * t76 + 441.0 / 32.0 * t434 * t506 - 1323.0 / 8.0 * t121 * t87 * t133 * t222 - 3969.0 / 32.0 * t430 * t506 + 21.0 / 2.0 * t484 * t187 * t190 + 105.0 / 8.0 * t492 * t187 * t453 - 21.0 / 4.0 * t422 * t469 + 105.0 / 32.0 * t509 * t187 * t94 * t511 * t76 - 105.0 / 16.0 * t451 * t94 * t516 * t76 + 21.0 / 8.0 * t188 * t94 * t522 * t76 - t482 * t77;
        let tv4rhosigma30 = t1013 + t1071;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t1094 = t218 * t218;
        let tv4sigma40 = -1029.0 / 8.0 * t477 * t87 * t285 * t502 - 13377.0 / 128.0 * t498 * t285 * t500 * t71 - 5145.0 / 128.0 * t588 * t12 * t285 * t500 * t25 * t151 + 147.0 / 4.0 * t168 * t208 * t133 * t222 + 7203.0 / 32.0 / t602 / t44 * t12 * t621 * t1094 * t117 * t15 * t23 + 21.0 / 4.0 * t485 * t235 + 105.0 / 8.0 * t493 * t517 - 21.0 / 4.0 * t212 * t523 + 315.0 / 64.0 * t510 * t94 * t33 * t521 - 525.0 / 64.0 * t227 * t94 * t151 / t117 + 105.0 / 32.0 * t93 * t94 * t71 / t25 / t117 - 7.0 * t90 * t481 * t92 * t97 - 105.0 / 8.0 * t225 * t208 * t92 * t230 + 441.0 / 8.0 * t489 * t506 - 105.0 / 16.0 * t295 * t87 * t92 * t512 + 735.0 / 128.0 * t216 * t219 * t220 * t95 - 105.0 / 256.0 * t106 * t12 * t92 * t94 * t562 * t515 + rho[ip] * (-4.0 * param_b * t477 * t270 * t9 + t1 / t3 / t267 * t606 * t9) * t43;
        v4sigma4[ip] += tv4sigma40;
    }
}
