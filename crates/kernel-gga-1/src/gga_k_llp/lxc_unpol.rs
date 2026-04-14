//! GGA_K_LLP lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 40 shared lines across all orders.
//! Delta: 31 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_llp_lxc_unpol(
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
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (40 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = param_beta * t4;
        let t26 = pow_1_3(1.0 / M_PI);
        let t27 = 1.0 / t26;
        let t28 = M_CBRT4;
        let t29 = t27 * t28;
        let t30 = t24 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t22 / t34;
        let t37 = param_gamma * param_beta;
        let t38 = f64::sqrt(sigma[ip]);
        let t39 = t37 * t38;
        let t41 = 1.0 / t21 / rho[ip];
        let t45 = f64::ln(t38 * t31 * t41 + f64::sqrt(pow_2(t38 * t31 * t41) + 1.0));
        let t46 = t31 * t41 * t45;
        let t48 = 1.0 + t39 * t46;
        let t49 = 1.0 / t48;
        let t50 = t36 * t49;
        let t54 = 1.0 + 2.0 / 9.0 * t30 * t33 * t50;
        let t58 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t54);
        let tzk0 = 2.0 * t58;
        zk[ip] += tzk0;
        // --- vxc delta (28 lines) ---
        let t60 = t20 / t21;
        let t64 = t34 * rho[ip];
        let t66 = 1.0 / t22 / t64;
        let t67 = t66 * t49;
        let t71 = t48 * t48;
        let t72 = 1.0 / t71;
        let t73 = t36 * t72;
        let t75 = 1.0 / t21 / t34;
        let t77 = t31 * t75 * t45;
        let t79 = t37 * sigma[ip];
        let t80 = t32 * t66;
        let t82 = t33 * t36 + 1.0;
        let t83 = f64::sqrt(t82);
        let t84 = 1.0 / t83;
        let t85 = t80 * t84;
        let t88 = -4.0 / 3.0 * t39 * t77 - 4.0 / 3.0 * t79 * t85;
        let t93 = -16.0 / 27.0 * t30 * t33 * t67 - 2.0 / 9.0 * t30 * t33 * t73 * t88;
        let t98 = piecewise3(t2, 0.0, t7 * t60 * t54 / 10.0 + 3.0 / 20.0 * t7 * t23 * t93);
        let tvrho0 = 2.0 * rho[ip] * t98 + 2.0 * t58;
        vrho[ip] += tvrho0;
        let t101 = t24 * t27;
        let t102 = t28 * t32;
        let t106 = t37 / t38;
        let t108 = t32 * t36;
        let t109 = t108 * t84;
        let t112 = t106 * t46 / 2.0 + t37 * t109 / 2.0;
        let t117 = -2.0 / 9.0 * t30 * t33 * t73 * t112 + 2.0 / 9.0 * t101 * t102 * t50;
        let t121 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t117);
        let tvsigma0 = 2.0 * rho[ip] * t121;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (45 lines) ---
        let t124 = t20 * t41;
        let t131 = t34 * t34;
        let t133 = 1.0 / t22 / t131;
        let t134 = t133 * t49;
        let t138 = t66 * t72;
        let t144 = 1.0 / t71 / t48;
        let t145 = t36 * t144;
        let t146 = t88 * t88;
        let t152 = 1.0 / t21 / t64;
        let t154 = t31 * t152 * t45;
        let t157 = t32 * t133;
        let t158 = t157 * t84;
        let t161 = sigma[ip] * sigma[ip];
        let t162 = t37 * t161;
        let t165 = 1.0 / t21 / t131 / t64;
        let t168 = 1.0 / t83 / t82;
        let t169 = t31 * t165 * t168;
        let t172 = 28.0 / 9.0 * t39 * t154 + 20.0 / 3.0 * t79 * t158 - 32.0 / 9.0 * t162 * t169;
        let t177 = 176.0 / 81.0 * t30 * t33 * t134 + 32.0 / 27.0 * t30 * t33 * t138 * t88 + 4.0 / 9.0 * t30 * t33 * t145 * t146 - 2.0 / 9.0 * t30 * t33 * t73 * t172;
        let t182 = piecewise3(t2, 0.0, -t7 * t124 * t54 / 30.0 + t7 * t60 * t93 / 5.0 + 3.0 / 20.0 * t7 * t23 * t177);
        let tv2rho20 = 2.0 * rho[ip] * t182 + 4.0 * t98;
        v2rho2[ip] += tv2rho20;
        let t191 = t72 * t88;
        let t200 = t24 * t29 * sigma[ip];
        let t201 = t144 * t112;
        let t202 = t201 * t88;
        let t203 = t108 * t202;
        let t210 = t37 * t31;
        let t211 = t131 * t34;
        let t213 = 1.0 / t21 / t211;
        let t218 = -2.0 / 3.0 * t106 * t77 - 2.0 * t37 * t85 + 4.0 / 3.0 * t210 * t213 * t168 * sigma[ip];
        let t223 = -16.0 / 27.0 * t101 * t102 * t67 - 2.0 / 9.0 * t30 * t108 * t191 + 16.0 / 27.0 * t30 * t33 * t138 * t112 + 4.0 / 9.0 * t200 * t203 - 2.0 / 9.0 * t30 * t33 * t73 * t218;
        let t228 = piecewise3(t2, 0.0, t7 * t60 * t117 / 10.0 + 3.0 / 20.0 * t7 * t23 * t223);
        let tv2rhosigma0 = 2.0 * rho[ip] * t228 + 2.0 * t121;
        v2rhosigma[ip] += tv2rhosigma0;
        let t231 = t72 * t112;
        let t235 = t112 * t112;
        let t242 = t37 / t38 / sigma[ip];
        let t245 = 1.0 / sigma[ip];
        let t246 = t37 * t245;
        let t249 = t131 * rho[ip];
        let t252 = t31 / t21 / t249;
        let t253 = t252 * t168;
        let t256 = -t242 * t46 / 4.0 + t246 * t109 / 4.0 - t37 * t253 / 2.0;
        let t261 = -4.0 / 9.0 * t30 * t108 * t231 + 4.0 / 9.0 * t30 * t33 * t145 * t235 - 2.0 / 9.0 * t30 * t33 * t73 * t256;
        let t265 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t261);
        let tv2sigma20 = 2.0 * rho[ip] * t265;
        v2sigma2[ip] += tv2sigma20;
        // --- kxc delta (69 lines) ---
        let t268 = t20 * t75;
        let t279 = 1.0 / t22 / t249;
        let t280 = t279 * t49;
        let t284 = t133 * t72;
        let t289 = t66 * t144;
        let t298 = t71 * t71;
        let t299 = 1.0 / t298;
        let t300 = t36 * t299;
        let t301 = t146 * t88;
        let t306 = t144 * t88;
        let t307 = t306 * t172;
        let t308 = t108 * t307;
        let t314 = t31 / t21 / t131 * t45;
        let t318 = t32 * t279 * t84;
        let t321 = t131 * t131;
        let t323 = 1.0 / t21 / t321;
        let t328 = t161 * sigma[ip];
        let t329 = t321 * t64;
        let t330 = 1.0 / t329;
        let t332 = t82 * t82;
        let t334 = 1.0 / t83 / t332;
        let t338 = -280.0 / 27.0 * t39 * t314 - 952.0 / 27.0 * t79 * t318 + 1184.0 / 27.0 * t162 * t31 * t323 * t168 - 256.0 / 9.0 * t37 * t328 * t330 * t334;
        let t343 = -2464.0 / 243.0 * t30 * t33 * t280 - 176.0 / 27.0 * t30 * t33 * t284 * t88 - 32.0 / 9.0 * t30 * t33 * t289 * t146 + 16.0 / 9.0 * t30 * t33 * t138 * t172 - 4.0 / 3.0 * t30 * t33 * t300 * t301 + 4.0 / 3.0 * t200 * t308 - 2.0 / 9.0 * t30 * t33 * t73 * t338;
        let t348 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t268 * t54 - t7 * t124 * t93 / 10.0 + 3.0 / 10.0 * t7 * t60 * t177 + 3.0 / 20.0 * t7 * t23 * t343);
        let tv3rho30 = 2.0 * rho[ip] * t348 + 6.0 * t182;
        v3rho3[ip] += tv3rho30;
        let t364 = t144 * t146;
        let t368 = t72 * t172;
        let t376 = t80 * t202;
        let t384 = t299 * t112 * t146;
        let t385 = t108 * t384;
        let t388 = t144 * t218;
        let t389 = t388 * t88;
        let t390 = t108 * t389;
        let t393 = t201 * t172;
        let t394 = t108 * t393;
        let t405 = t321 * t34;
        let t407 = 1.0 / t405 * t334;
        let t411 = 14.0 / 9.0 * t106 * t154 + 74.0 / 9.0 * t37 * t158 - 124.0 / 9.0 * t210 * t165 * t168 * sigma[ip] + 32.0 / 3.0 * t37 * t407 * t161;
        let t416 = 176.0 / 81.0 * t101 * t102 * t134 + 32.0 / 27.0 * t30 * t80 * t191 + 4.0 / 9.0 * t30 * t108 * t364 - 2.0 / 9.0 * t30 * t108 * t368 - 176.0 / 81.0 * t30 * t33 * t284 * t112 - 64.0 / 27.0 * t200 * t376 + 32.0 / 27.0 * t30 * t33 * t138 * t218 - 4.0 / 3.0 * t200 * t385 + 8.0 / 9.0 * t200 * t390 + 4.0 / 9.0 * t200 * t394 - 2.0 / 9.0 * t30 * t33 * t73 * t411;
        let t421 = piecewise3(t2, 0.0, -t7 * t124 * t117 / 30.0 + t7 * t60 * t223 / 5.0 + 3.0 / 20.0 * t7 * t23 * t416);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t421 + 4.0 * t228;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t432 = t72 * t218;
        let t440 = t299 * t235;
        let t441 = t440 * t88;
        let t442 = t108 * t441;
        let t445 = t201 * t218;
        let t446 = t108 * t445;
        let t453 = t144 * t256;
        let t454 = t453 * t88;
        let t455 = t108 * t454;
        let t463 = t31 * t213 * t168;
        let t466 = t321 * rho[ip];
        let t468 = 1.0 / t466 * t334;
        let t472 = t242 * t77 / 3.0 - t246 * t85 / 3.0 + 10.0 / 3.0 * t37 * t463 - 4.0 * t37 * t468 * sigma[ip];
        let t477 = 32.0 / 27.0 * t30 * t80 * t231 + 8.0 / 9.0 * t30 * t203 - 4.0 / 9.0 * t30 * t108 * t432 - 32.0 / 27.0 * t30 * t33 * t289 * t235 - 4.0 / 3.0 * t200 * t442 + 8.0 / 9.0 * t200 * t446 + 16.0 / 27.0 * t30 * t33 * t138 * t256 + 4.0 / 9.0 * t200 * t455 - 2.0 / 9.0 * t30 * t33 * t73 * t472;
        let t482 = piecewise3(t2, 0.0, t7 * t60 * t261 / 10.0 + 3.0 / 20.0 * t7 * t23 * t477);
        let tv3rhosigma20 = 2.0 * rho[ip] * t482 + 2.0 * t265;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t485 = t144 * t235;
        let t489 = t72 * t256;
        let t493 = t235 * t112;
        let t498 = t201 * t256;
        let t499 = t108 * t498;
        let t504 = t37 / t38 / t161;
        let t508 = t37 / t161;
        let t513 = 1.0 / t321;
        let t517 = 3.0 / 8.0 * t504 * t46 - 3.0 / 8.0 * t508 * t109 - t246 * t253 / 4.0 + 3.0 / 2.0 * t37 * t513 * t334;
        let t522 = 4.0 / 3.0 * t30 * t108 * t485 - 2.0 / 3.0 * t30 * t108 * t489 - 4.0 / 3.0 * t30 * t33 * t300 * t493 + 4.0 / 3.0 * t200 * t499 - 2.0 / 9.0 * t30 * t33 * t73 * t517;
        let t526 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t522);
        let tv3sigma30 = 2.0 * rho[ip] * t526;
        v3sigma3[ip] += tv3sigma30;
        // --- lxc delta (this level) (31 lines) ---
        let t543 = 1.0 / t22 / t211;
        let t548 = t279 * t72;
        let t553 = t133 * t144;
        let t562 = t66 * t299;
        let t575 = 1.0 / t298 / t48;
        let t576 = t36 * t575;
        let t577 = t146 * t146;
        let t587 = t172 * t172;
        let t609 = t321 * t131;
        let t615 = t161 * t161;
        let t622 = 1.0 / t83 / t332 / t82;
        let t632 = 41888.0 / 729.0 * t30 * t33 * t543 * t49 + 9856.0 / 243.0 * t30 * t33 * t548 * t88 + 704.0 / 27.0 * t30 * t33 * t553 * t146 - 352.0 / 27.0 * t30 * t33 * t284 * t172 + 128.0 / 9.0 * t30 * t33 * t562 * t301 - 128.0 / 9.0 * t200 * t80 * t307 + 64.0 / 27.0 * t30 * t33 * t138 * t338 + 16.0 / 3.0 * t30 * t33 * t576 * t577 - 8.0 * t200 * t108 * t299 * t146 * t172 + 4.0 / 3.0 * t30 * t33 * t145 * t587 + 16.0 / 9.0 * t200 * t108 * t306 * t338 - 2.0 / 9.0 * t30 * t33 * t73 * (3640.0 / 81.0 * t39 * t252 * t45 + 5768.0 / 27.0 * t79 * t32 * t543 * t84 - 37216.0 / 81.0 * t162 * t31 / t21 / t466 * t168 + 17920.0 / 27.0 * t37 * t328 / t609 * t334 - 5120.0 / 27.0 * t37 * t615 / t22 / t321 / t211 * t622 * t32);
        let t637 = piecewise3(t2, 0.0, -14.0 / 135.0 * t7 * t20 * t152 * t54 + 8.0 / 45.0 * t7 * t268 * t93 - t7 * t124 * t177 / 5.0 + 2.0 / 5.0 * t7 * t60 * t343 + 3.0 / 20.0 * t7 * t23 * t632);
        let tv4rho40 = 2.0 * rho[ip] * t637 + 8.0 * t348;
        v4rho4[ip] += tv4rho40;
        let t745 = t108 * t299;
        let t746 = t112 * t88;
        let t751 = 352.0 / 27.0 * t200 * t157 * t202 - 64.0 / 9.0 * t200 * t80 * t389 - 2.0 / 9.0 * t30 * t33 * t73 * (-140.0 / 27.0 * t106 * t314 - 364.0 / 9.0 * t37 * t318 + 3320.0 / 27.0 * t210 * t323 * t168 * sigma[ip] - 1952.0 / 9.0 * t37 * t330 * t334 * t161 + 640.0 / 9.0 * t37 / t22 / t321 / t249 * t622 * t328 * t32) - 4.0 / 3.0 * t30 * t108 * t299 * t301 + 4.0 / 3.0 * t30 * t308 - 176.0 / 27.0 * t30 * t33 * t284 * t218 + 16.0 / 9.0 * t30 * t33 * t138 * t411 - 32.0 / 9.0 * t30 * t80 * t364 + 32.0 / 3.0 * t200 * t80 * t384 + 16.0 / 3.0 * t200 * t108 * t575 * t112 * t301 - 4.0 * t200 * t745 * t746 * t172;
        let t757 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t268 * t117 - t7 * t124 * t223 / 10.0 + 3.0 / 10.0 * t7 * t60 * t416 + 3.0 / 20.0 * t7 * t23 * (-2.0 / 9.0 * t30 * t108 * t72 * t338 - 2464.0 / 243.0 * t101 * t102 * t280 - 176.0 / 27.0 * t30 * t157 * t191 + 16.0 / 9.0 * t30 * t80 * t368 + 4.0 / 3.0 * t200 * t108 * t144 * t411 * t88 + 4.0 / 3.0 * t200 * t108 * t388 * t172 + 4.0 / 9.0 * t200 * t108 * t201 * t338 - 32.0 / 9.0 * t200 * t80 * t393 - 4.0 * t200 * t108 * t299 * t218 * t146 + 2464.0 / 243.0 * t30 * t33 * t548 * t112 + t751));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t757 + 6.0 * t421;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t822 = t218 * t218;
        let t827 = 32.0 / 27.0 * t30 * t33 * t138 * t472 - 2.0 / 9.0 * t30 * t33 * t73 * (-7.0 / 9.0 * t242 * t154 + 7.0 / 9.0 * t246 * t158 - 22.0 * t37 * t169 + 188.0 / 3.0 * t37 * t407 * sigma[ip] - 80.0 / 3.0 * t37 / t22 / t609 * t622 * t161 * t32) - 176.0 / 81.0 * t30 * t33 * t284 * t256 - 64.0 / 27.0 * t200 * t80 * t454 + 8.0 / 9.0 * t200 * t108 * t144 * t472 * t88 + 4.0 / 9.0 * t200 * t108 * t453 * t172 + 352.0 / 81.0 * t30 * t33 * t553 * t235 + 64.0 / 9.0 * t200 * t80 * t441 - 128.0 / 27.0 * t200 * t80 * t445 - 4.0 / 3.0 * t200 * t108 * t440 * t172 + 8.0 / 9.0 * t30 * t33 * t145 * t822;
        let t864 = 8.0 / 9.0 * t200 * t108 * t201 * t411 - 8.0 / 3.0 * t30 * t385 - 4.0 / 9.0 * t30 * t108 * t72 * t411 + 64.0 / 27.0 * t30 * t80 * t432 - 352.0 / 81.0 * t30 * t157 * t231 - 128.0 / 27.0 * t30 * t376 + 16.0 / 9.0 * t30 * t390 + 8.0 / 9.0 * t30 * t394 - 4.0 / 3.0 * t200 * t108 * t299 * t256 * t146 + 16.0 / 3.0 * t200 * t108 * t575 * t235 * t146 - 16.0 / 3.0 * t200 * t745 * t746 * t218;
        let t870 = piecewise3(t2, 0.0, -t7 * t124 * t261 / 30.0 + t7 * t60 * t477 / 5.0 + 3.0 / 20.0 * t7 * t23 * (t827 + t864));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t870 + 4.0 * t482;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t950 = -32.0 / 9.0 * t30 * t80 * t485 - 4.0 * t30 * t442 + 8.0 / 3.0 * t30 * t446 + 16.0 / 9.0 * t30 * t80 * t489 + 4.0 / 3.0 * t30 * t455 - 2.0 / 3.0 * t30 * t108 * t72 * t472 + 32.0 / 9.0 * t30 * t33 * t562 * t493 + 16.0 / 3.0 * t200 * t108 * t575 * t493 * t88 - 4.0 * t200 * t108 * t440 * t218 - 32.0 / 9.0 * t200 * t80 * t498 - 4.0 * t200 * t745 * t112 * t256 * t88 + 4.0 / 3.0 * t200 * t108 * t388 * t256 + 4.0 / 3.0 * t200 * t108 * t201 * t472 + 16.0 / 27.0 * t30 * t33 * t138 * t517 + 4.0 / 9.0 * t200 * t108 * t144 * t517 * t88 - 2.0 / 9.0 * t30 * t33 * t73 * (-t504 * t77 / 2.0 + t508 * t85 / 2.0 + t246 * t463 / 3.0 - 14.0 * t37 * t468 + 10.0 * t37 / t22 / t329 * t622 * sigma[ip] * t32);
        let t955 = piecewise3(t2, 0.0, t7 * t60 * t522 / 10.0 + 3.0 / 20.0 * t7 * t23 * t950);
        let tv4rhosigma30 = 2.0 * rho[ip] * t955 + 2.0 * t526;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t968 = t235 * t235;
        let t977 = t256 * t256;
        let t1016 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * (-16.0 / 3.0 * t30 * t108 * t299 * t493 + 16.0 / 3.0 * t30 * t499 - 8.0 / 9.0 * t30 * t108 * t72 * t517 + 16.0 / 3.0 * t30 * t33 * t576 * t968 - 8.0 * t200 * t108 * t440 * t256 + 4.0 / 3.0 * t30 * t33 * t145 * t977 + 16.0 / 9.0 * t200 * t108 * t201 * t517 - 2.0 / 9.0 * t30 * t33 * t73 * (-15.0 / 16.0 * t37 / t38 / t328 * t46 + 15.0 / 16.0 * t37 / t328 * t109 + 5.0 / 8.0 * t508 * t253 + 3.0 / 4.0 * t37 * t245 * t513 * t334 - 15.0 / 4.0 * t37 / t22 / t405 * t622 * t32)));
        let tv4sigma40 = 2.0 * rho[ip] * t1016;
        v4sigma4[ip] += tv4sigma40;
    }
}
