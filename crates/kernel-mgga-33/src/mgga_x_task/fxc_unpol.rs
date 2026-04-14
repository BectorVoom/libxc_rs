//! MGGA_X_TASK fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 108 shared lines across all orders.
//! Delta: 222 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_task_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    param_task_anu_0: f64,
    param_task_anu_1: f64,
    param_task_anu_2: f64,
    param_task_bnu_0: f64,
    param_task_bnu_1: f64,
    param_task_bnu_2: f64,
    param_task_bnu_3: f64,
    param_task_bnu_4: f64,
    param_task_c: f64,
    param_task_d: f64,
    param_task_h0x: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (108 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = t11 + 1.0;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t26 = t21 / t24;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t32 = t31 * t30;
        let t33 = 1.0 / t32;
        let t36 = t26 * t29 * t33 / 24.0;
        let t37 = 0.0 < t36;
        let t38 = piecewise3(t37, t36, 0.0);
        let t39 = pow_1_4(t38);
        let t42 = f64::exp(-param_task_c / t39);
        let t44 = piecewise3(t37, 1.0 - t42, 0.0);
        let t46 = tau[ip] * tau[ip];
        let t47 = t46 * t46;
        let t48 = t47 * t4;
        let t49 = param_task_bnu_0;
        let t50 = param_task_bnu_1;
        let t51 = param_task_bnu_2;
        let t52 = param_task_bnu_3;
        let t53 = param_task_bnu_4;
        let t54 = t49 + t50 + t51 + t52 + t53;
        let t55 = rho[ip] * tau[ip];
        let t59 = 1.0 / rho[ip];
        let t61 = 1.0 / tau[ip];
        let t63 = 0.0 < (0.9999999999e0 * t55 - 0.125e0 * sigma[ip]) * t59 * t61;
        let t65 = 8.0 * t55 - sigma[ip];
        let t66 = t65 * t59;
        let t69 = piecewise3(t63, t66 * t61 / 8.0, 0.1e-9);
        let t70 = t69 * t69;
        let t71 = t70 * t70;
        let t72 = t54 * t71;
        let t75 = t5 * M_PI;
        let t76 = t50 / 2.0;
        let t77 = 7.0 / 2.0 * t52;
        let t78 = 7.0 * t53;
        let t80 = t75 * (t49 + t76 - t51 - t77 - t78);
        let t81 = t31 * rho[ip];
        let t82 = t46 * tau[ip];
        let t83 = t81 * t82;
        let t84 = t70 * t69;
        let t88 = t30 * rho[ip];
        let t89 = t19 * t88;
        let t90 = t5 * t5;
        let t91 = t90 * t22;
        let t92 = t89 * t91;
        let t93 = t4 * t4;
        let t94 = t92 * t93;
        let t97 = t49 - 5.0 / 3.0 * t51 + 35.0 / 3.0 * t53;
        let t98 = t97 * t46;
        let t99 = t98 * t70;
        let t102 = t30 * t30;
        let t103 = t102 * rho[ip];
        let t104 = t22 * t22;
        let t105 = t103 * t104;
        let t106 = t49 - t76 - t51 + t77 - t78;
        let t107 = t105 * t106;
        let t108 = tau[ip] * t4;
        let t109 = t108 * t69;
        let t113 = t31 * t102 * t30;
        let t115 = t5 * t104 * M_PI;
        let t116 = t113 * t115;
        let t117 = t49 - t50 + t51 - t52 + t53;
        let t120 = 108000.0 * t80 * t83 * t84 + 29160.0 * t107 * t109 + 6561.0 * t116 * t117 + 30000.0 * t48 * t72 + 48600.0 * t94 * t99;
        let t121 = t81 * t75;
        let t124 = 9.0 * t121 + 10.0 * t109;
        let t125 = t124 * t124;
        let t126 = t125 * t125;
        let t127 = 1.0 / t126;
        let t129 = 1.0 - t120 * t127;
        let t130 = param_task_anu_0;
        let t131 = param_task_anu_1;
        let t132 = param_task_anu_2;
        let t134 = t91 * (t130 - t131 + t132);
        let t138 = t4 * t75;
        let t140 = t130 - 3.0 * t132;
        let t143 = 24.0 * t138 * t140 * t32;
        let t145 = t130 + t131 + t132;
        let t146 = sigma[ip] * t93 * t145;
        let t149 = 144.0 * t134 * t19 * t103 + (t143 + t146) * sigma[ip];
        let t153 = 12.0 * t75 * t32 + t4 * sigma[ip];
        let t154 = t153 * t153;
        let t155 = 1.0 / t154;
        let t157 = t149 * t155 - param_task_h0x;
        let t158 = t129 * t157;
        let t159 = f64::powf(t44, param_task_d);
        let t160 = t158 * t159;
        let t161 = param_task_h0x * t44 + t160;
        let t165 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t161);
        let tzk0 = 2.0 * t165;
        zk[ip] += tzk0;
        // --- vxc delta (87 lines) ---
        let t166 = 1.0 / t31;
        let t167 = t18 * t166;
        let t173 = param_task_c / t39 / t38;
        let t174 = t31 * t88;
        let t175 = 1.0 / t174;
        let t179 = piecewise3(t37, -t26 * t29 * t175 / 9.0, 0.0);
        let t180 = t179 * t42;
        let t183 = piecewise3(t37, -t173 * t180 / 4.0, 0.0);
        let t185 = t54 * t84;
        let t186 = 1.0 / t30;
        let t187 = t65 * t186;
        let t191 = piecewise3(t63, t59 - t187 * t61 / 8.0, 0.0);
        let t192 = t185 * t191;
        let t195 = t31 * t82;
        let t199 = t80 * t81;
        let t200 = t82 * t70;
        let t201 = t200 * t191;
        let t204 = t19 * t30;
        let t205 = t204 * t91;
        let t206 = t205 * t93;
        let t209 = t69 * t191;
        let t210 = t98 * t209;
        let t213 = t102 * t104;
        let t214 = t213 * t106;
        let t217 = t108 * t191;
        let t220 = t31 * t103;
        let t224 = 43740.0 * t220 * t115 * t117 + 180000.0 * t80 * t195 * t84 + 29160.0 * t107 * t217 + 145800.0 * t214 * t109 + 120000.0 * t48 * t192 + 324000.0 * t199 * t201 + 162000.0 * t206 * t99 + 97200.0 * t94 * t210;
        let t227 = 1.0 / t126 / t124;
        let t228 = t120 * t227;
        let t229 = t31 * t75;
        let t232 = 15.0 * t229 + 10.0 * t217;
        let t235 = -t224 * t127 + 4.0 * t228 * t232;
        let t237 = t235 * t157 * t159;
        let t241 = t140 * t81;
        let t245 = 768.0 * t134 * t19 * t102 + 64.0 * t138 * t241 * sigma[ip];
        let t248 = 1.0 / t154 / t153;
        let t249 = t149 * t248;
        let t252 = -64.0 * t249 * t121 + t245 * t155;
        let t254 = t129 * t252 * t159;
        let t256 = 1.0 / t44;
        let t257 = param_task_d * t183 * t256;
        let t259 = t160 * t257 + param_task_h0x * t183 + t237 + t254;
        let t264 = piecewise3(t3, 0.0, -t7 * t167 * t161 / 8.0 - 3.0 / 8.0 * t7 * t20 * t259);
        let tvrho0 = 2.0 * rho[ip] * t264 + 2.0 * t165;
        vrho[ip] += tvrho0;
        let t270 = piecewise3(t37, t26 * t28 * t33 / 24.0, 0.0);
        let t271 = t270 * t42;
        let t274 = piecewise3(t37, -t173 * t271 / 4.0, 0.0);
        let t276 = t59 * t61;
        let t278 = piecewise3(t63, -t276 / 8.0, 0.0);
        let t279 = t185 * t278;
        let t282 = t200 * t278;
        let t285 = t69 * t278;
        let t286 = t98 * t285;
        let t289 = t108 * t278;
        let t292 = 29160.0 * t107 * t289 + 324000.0 * t199 * t282 + 120000.0 * t48 * t279 + 97200.0 * t94 * t286;
        let t296 = -t292 * t127 + 40.0 * t228 * t289;
        let t298 = t296 * t157 * t159;
        let t300 = 2.0 * t146 + t143;
        let t304 = t300 * t155 - 2.0 * t249 * t4;
        let t306 = t129 * t304 * t159;
        let t307 = param_task_d * t274;
        let t308 = t307 * t256;
        let t310 = t160 * t308 + param_task_h0x * t274 + t298 + t306;
        let t314 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t310);
        let tvsigma0 = 2.0 * rho[ip] * t314;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t316 = t7 * t18;
        let t317 = t82 * t4;
        let t320 = 1.0 / t46;
        let t324 = piecewise3(t63, t61 - t66 * t320 / 8.0, 0.0);
        let t325 = t185 * t324;
        let t328 = t81 * t46;
        let t332 = t200 * t324;
        let t335 = t97 * tau[ip];
        let t336 = t335 * t70;
        let t339 = t69 * t324;
        let t340 = t98 * t339;
        let t343 = t106 * t4;
        let t344 = t343 * t69;
        let t347 = t108 * t324;
        let t350 = 324000.0 * t80 * t328 * t84 + 29160.0 * t105 * t344 + 29160.0 * t107 * t347 + 324000.0 * t199 * t332 + 120000.0 * t317 * t72 + 120000.0 * t48 * t325 + 97200.0 * t94 * t336 + 97200.0 * t94 * t340;
        let t354 = 10.0 * t4 * t69 + 10.0 * t347;
        let t357 = -t350 * t127 + 4.0 * t228 * t354;
        let t358 = t19 * t357;
        let t359 = t157 * t159;
        let t363 = piecewise3(t3, 0.0, -3.0 / 8.0 * t316 * t358 * t359);
        let tvtau0 = 2.0 * rho[ip] * t363;
        vtau[ip] += tvtau0;
        // --- fxc delta (this level) (222 lines) ---
        let t366 = 1.0 / t81;
        let t367 = t18 * t366;
        let t374 = t38 * t38;
        let t377 = param_task_c / t39 / t374;
        let t378 = t179 * t179;
        let t379 = t378 * t42;
        let t382 = t31 * t102;
        let t383 = 1.0 / t382;
        let t387 = piecewise3(t37, 11.0 / 27.0 * t26 * t29 * t383, 0.0);
        let t391 = param_task_c * param_task_c;
        let t392 = f64::sqrt(t38);
        let t395 = t391 / t392 / t374;
        let t399 = piecewise3(t37, 5.0 / 16.0 * t377 * t379 - t173 * t387 * t42 / 4.0 - t395 * t379 / 16.0, 0.0);
        let t401 = t54 * t70;
        let t402 = t191 * t191;
        let t403 = t401 * t402;
        let t407 = 1.0 / t88;
        let t408 = t65 * t407;
        let t412 = piecewise3(t63, -2.0 * t186 + t408 * t61 / 4.0, 0.0);
        let t413 = t185 * t412;
        let t416 = 1.0 / t19;
        let t421 = t80 * t31;
        let t424 = t82 * t69;
        let t425 = t424 * t402;
        let t428 = t200 * t412;
        let t431 = t19 * rho[ip];
        let t432 = t431 * t91;
        let t433 = t432 * t93;
        let t438 = t98 * t402;
        let t441 = t69 * t412;
        let t442 = t98 * t441;
        let t445 = t88 * t104;
        let t446 = t445 * t106;
        let t451 = t108 * t412;
        let t457 = 120000.0 * t80 * t416 * t82 * t84 + 247860.0 * t382 * t115 * t117 + 29160.0 * t107 * t451 + 583200.0 * t446 * t109 + 648000.0 * t199 * t425 + 324000.0 * t199 * t428 + 1080000.0 * t421 * t201 + 648000.0 * t206 * t210 + 291600.0 * t214 * t217 + 360000.0 * t48 * t403 + 120000.0 * t48 * t413 + 378000.0 * t433 * t99 + 97200.0 * t94 * t438 + 97200.0 * t94 * t442;
        let t459 = t224 * t227;
        let t463 = 1.0 / t126 / t125;
        let t464 = t120 * t463;
        let t465 = t232 * t232;
        let t468 = t416 * t75;
        let t470 = 10.0 * t468 + 10.0 * t451;
        let t473 = -t457 * t127 + 4.0 * t228 * t470 + 8.0 * t459 * t232 - 20.0 * t464 * t465;
        let t475 = t473 * t157 * t159;
        let t477 = t235 * t252 * t159;
        let t483 = t140 * t31;
        let t487 = 3328.0 * t134 * t89 + 320.0 / 3.0 * t138 * t483 * sigma[ip];
        let t489 = t245 * t248;
        let t492 = t154 * t154;
        let t493 = 1.0 / t492;
        let t494 = t149 * t493;
        let t499 = t487 * t155 - 128.0 * t489 * t121 + 6144.0 * t494 * t92 - 320.0 / 3.0 * t249 * t229;
        let t501 = t129 * t499 * t159;
        let t504 = param_task_d * param_task_d;
        let t505 = t183 * t183;
        let t506 = t504 * t505;
        let t507 = t44 * t44;
        let t508 = 1.0 / t507;
        let t509 = t506 * t508;
        let t511 = param_task_d * t399;
        let t512 = t511 * t256;
        let t515 = param_task_d * t505 * t508;
        let t517 = t160 * t509 + t160 * t512 - t160 * t515 + 2.0 * t237 * t257 + 2.0 * t254 * t257 + param_task_h0x * t399 + t475 + 2.0 * t477 + t501;
        let t522 = piecewise3(t3, 0.0, t7 * t367 * t161 / 12.0 - t7 * t167 * t259 / 4.0 - 3.0 / 8.0 * t7 * t20 * t517);
        let tv2rho20 = 2.0 * rho[ip] * t522 + 4.0 * t264;
        v2rho2[ip] += tv2rho20;
        let t528 = t271 * t179;
        let t534 = piecewise3(t37, -t26 * t28 * t175 / 9.0, 0.0);
        let t535 = t534 * t42;
        let t541 = piecewise3(t37, 5.0 / 16.0 * t377 * t528 - t173 * t535 / 4.0 - t395 * t528 / 16.0, 0.0);
        let t543 = t48 * t54;
        let t544 = t70 * t278;
        let t545 = t544 * t191;
        let t548 = t186 * t61;
        let t550 = piecewise3(t63, t548 / 8.0, 0.0);
        let t551 = t185 * t550;
        let t556 = t278 * t191;
        let t557 = t424 * t556;
        let t560 = t200 * t550;
        let t565 = t98 * t556;
        let t568 = t69 * t550;
        let t569 = t98 * t568;
        let t574 = t108 * t550;
        let t577 = 29160.0 * t107 * t574 + 648000.0 * t199 * t557 + 324000.0 * t199 * t560 + 324000.0 * t206 * t286 + 145800.0 * t214 * t289 + 540000.0 * t421 * t282 + 120000.0 * t48 * t551 + 360000.0 * t543 * t545 + 97200.0 * t94 * t565 + 97200.0 * t94 * t569;
        let t579 = t292 * t227;
        let t584 = t464 * tau[ip];
        let t585 = t4 * t278;
        let t586 = t585 * t232;
        let t591 = -t577 * t127 + 40.0 * t228 * t574 + 4.0 * t579 * t232 + 40.0 * t459 * t289 - 200.0 * t584 * t586;
        let t593 = t591 * t157 * t159;
        let t595 = t296 * t252 * t159;
        let t598 = t235 * t304 * t159;
        let t602 = t300 * t248;
        let t608 = t4 * t81 * t75;
        let t611 = 64.0 * t138 * t241 * t155 - 64.0 * t602 * t121 - 2.0 * t489 * t4 + 192.0 * t494 * t608;
        let t613 = t129 * t611 * t159;
        let t617 = t504 * t183;
        let t618 = t508 * t274;
        let t619 = t617 * t618;
        let t621 = param_task_d * t541;
        let t622 = t621 * t256;
        let t624 = t508 * t183;
        let t625 = t307 * t624;
        let t627 = t160 * t619 + t160 * t622 - t160 * t625 + t237 * t308 + t254 * t308 + t298 * t257 + t306 * t257 + param_task_h0x * t541 + t593 + t595 + t598 + t613;
        let t632 = piecewise3(t3, 0.0, -t7 * t167 * t310 / 8.0 - 3.0 / 8.0 * t7 * t20 * t627);
        let tv2rhosigma0 = 2.0 * rho[ip] * t632 + 2.0 * t314;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t635 = t166 * t357;
        let t641 = t70 * t324;
        let t642 = t641 * t191;
        let t648 = piecewise3(t63, -t276 + t187 * t320 / 8.0, 0.0);
        let t649 = t185 * t648;
        let t656 = t46 * t70;
        let t657 = t656 * t191;
        let t662 = t324 * t191;
        let t663 = t424 * t662;
        let t666 = t200 * t648;
        let t671 = t335 * t209;
        let t676 = t98 * t662;
        let t679 = t69 * t648;
        let t680 = t98 * t679;
        let t685 = t343 * t191;
        let t690 = t108 * t648;
        let t693 = 480000.0 * t317 * t192 + 360000.0 * t543 * t642 + 120000.0 * t48 * t649 + 540000.0 * t80 * t31 * t46 * t84 + 972000.0 * t199 * t657 + 540000.0 * t421 * t332 + 648000.0 * t199 * t663 + 324000.0 * t199 * t666 + 324000.0 * t206 * t336 + 194400.0 * t94 * t671 + 324000.0 * t206 * t340 + 97200.0 * t94 * t676 + 97200.0 * t94 * t680 + 145800.0 * t213 * t344 + 29160.0 * t105 * t685 + 145800.0 * t214 * t347 + 29160.0 * t107 * t690;
        let t695 = t350 * t227;
        let t700 = t354 * t232;
        let t705 = 10.0 * t4 * t191 + 10.0 * t690;
        let t708 = -t693 * t127 + 4.0 * t228 * t705 + 4.0 * t695 * t232 + 4.0 * t459 * t354 - 20.0 * t464 * t700;
        let t709 = t19 * t708;
        let t713 = t252 * t159;
        let t718 = t7 * t20 * t357;
        let t719 = t359 * t257;
        let t723 = piecewise3(t3, 0.0, -t316 * t635 * t359 / 8.0 - 3.0 / 8.0 * t316 * t709 * t359 - 3.0 / 8.0 * t316 * t358 * t713 - 3.0 / 8.0 * t718 * t719);
        let tv2rhotau0 = 2.0 * rho[ip] * t723 + 2.0 * t363;
        v2rhotau[ip] += tv2rhotau0;
        let t726 = t270 * t270;
        let t727 = t726 * t42;
        let t730 = piecewise3(t37, 0.0, 0.0);
        let t731 = t730 * t42;
        let t733 = t173 * t731 / 4.0;
        let t737 = piecewise3(t37, 5.0 / 16.0 * t377 * t727 - t733 - t395 * t727 / 16.0, 0.0);
        let t739 = t278 * t278;
        let t740 = t401 * t739;
        let t743 = piecewise3(t63, 0.0, 0.0);
        let t744 = t185 * t743;
        let t746 = 120000.0 * t48 * t744;
        let t747 = t424 * t739;
        let t750 = t200 * t743;
        let t752 = 324000.0 * t199 * t750;
        let t753 = t98 * t739;
        let t756 = t69 * t743;
        let t757 = t98 * t756;
        let t759 = 97200.0 * t94 * t757;
        let t760 = t108 * t743;
        let t762 = 29160.0 * t107 * t760;
        let t763 = 648000.0 * t199 * t747 + 360000.0 * t48 * t740 + 97200.0 * t94 * t753 + t746 + t752 + t759 + t762;
        let t767 = t46 * t93;
        let t768 = t767 * t739;
        let t772 = 40.0 * t228 * t760;
        let t773 = -t763 * t127 + 80.0 * t579 * t289 - 2000.0 * t464 * t768 + t772;
        let t775 = t773 * t157 * t159;
        let t777 = t296 * t304 * t159;
        let t781 = t93 * t145;
        let t788 = 2.0 * t781 * t155 - 4.0 * t602 * t4 + 6.0 * t494 * t93;
        let t790 = t129 * t788 * t159;
        let t793 = t274 * t274;
        let t794 = t504 * t793;
        let t795 = t794 * t508;
        let t797 = param_task_d * t737;
        let t798 = t797 * t256;
        let t800 = param_task_d * t793;
        let t801 = t800 * t508;
        let t803 = t160 * t795 + t160 * t798 - t160 * t801 + 2.0 * t298 * t308 + 2.0 * t306 * t308 + param_task_h0x * t737 + t775 + 2.0 * t777 + t790;
        let t807 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t803);
        let tv2sigma20 = 2.0 * rho[ip] * t807;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t811 = t641 * t278;
        let t814 = t59 * t320;
        let t816 = piecewise3(t63, t814 / 8.0, 0.0);
        let t817 = t185 * t816;
        let t820 = t656 * t278;
        let t823 = t324 * t278;
        let t824 = t424 * t823;
        let t827 = t200 * t816;
        let t830 = t335 * t285;
        let t833 = t98 * t823;
        let t836 = t69 * t816;
        let t837 = t98 * t836;
        let t840 = t343 * t278;
        let t843 = t108 * t816;
        let t846 = 29160.0 * t105 * t840 + 29160.0 * t107 * t843 + 972000.0 * t199 * t820 + 648000.0 * t199 * t824 + 324000.0 * t199 * t827 + 480000.0 * t317 * t279 + 120000.0 * t48 * t817 + 360000.0 * t543 * t811 + 194400.0 * t94 * t830 + 97200.0 * t94 * t833 + 97200.0 * t94 * t837;
        let t852 = t464 * t354;
        let t856 = 10.0 * t585 + 10.0 * t843;
        let t859 = -t846 * t127 + 4.0 * t228 * t856 + 40.0 * t695 * t289 - 200.0 * t852 * t289 + 4.0 * t579 * t354;
        let t860 = t19 * t859;
        let t863 = t304 * t159;
        let t866 = t359 * t308;
        let t870 = piecewise3(t3, 0.0, -3.0 / 8.0 * t316 * t358 * t863 - 3.0 / 8.0 * t316 * t860 * t359 - 3.0 / 8.0 * t718 * t866);
        let tv2sigmatau0 = 2.0 * rho[ip] * t870;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t872 = t46 * t4;
        let t877 = t324 * t324;
        let t878 = t401 * t877;
        let t882 = 1.0 / t82;
        let t886 = piecewise3(t63, -2.0 * t320 + t66 * t882 / 4.0, 0.0);
        let t887 = t185 * t886;
        let t894 = t656 * t324;
        let t897 = t424 * t877;
        let t900 = t200 * t886;
        let t903 = t93 * t97;
        let t904 = t903 * t70;
        let t907 = t335 * t339;
        let t910 = t98 * t877;
        let t913 = t69 * t886;
        let t914 = t98 * t913;
        let t917 = t343 * t324;
        let t920 = t108 * t886;
        let t923 = 648000.0 * t80 * t81 * tau[ip] * t84 + 58320.0 * t105 * t917 + 29160.0 * t107 * t920 + 1944000.0 * t199 * t894 + 648000.0 * t199 * t897 + 324000.0 * t199 * t900 + 960000.0 * t317 * t325 + 360000.0 * t48 * t878 + 120000.0 * t48 * t887 + 360000.0 * t872 * t72 + 97200.0 * t92 * t904 + 388800.0 * t94 * t907 + 97200.0 * t94 * t910 + 97200.0 * t94 * t914;
        let t927 = t354 * t354;
        let t933 = 20.0 * t4 * t324 + 10.0 * t920;
        let t936 = -t923 * t127 + 4.0 * t228 * t933 + 8.0 * t695 * t354 - 20.0 * t464 * t927;
        let t937 = t19 * t936;
        let t941 = piecewise3(t3, 0.0, -3.0 / 8.0 * t316 * t937 * t359);
        let tv2tau20 = 2.0 * rho[ip] * t941;
        v2tau2[ip] += tv2tau20;
    }
}
