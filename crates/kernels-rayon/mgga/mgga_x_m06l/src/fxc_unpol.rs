//! MGGA_X_M06L fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_m06l.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_m06l_fxc_unpol(
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
    param_d_0: f64,
    param_d_1: f64,
    param_d_2: f64,
    param_d_3: f64,
    param_d_4: f64,
    param_d_5: f64,
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
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t37 = 0.804 + 0.009146457198521547 * t26 * t34;
        let t40 = 1.804 - 0.646416 / t37;
        let t42 = param_a_1;
        let t43 = t21 * t21;
        let t44 = t43 * t24;
        let t45 = 3.0 / 10.0 * t44;
        let t46 = tau[ip] * t28;
        let t48 = 1.0 / t31 / rho[ip];
        let t49 = t46 * t48;
        let t50 = t45 - t49;
        let t51 = t42 * t50;
        let t52 = t45 + t49;
        let t53 = 1.0 / t52;
        let t55 = param_a_2;
        let t56 = t50 * t50;
        let t57 = t55 * t56;
        let t58 = t52 * t52;
        let t59 = 1.0 / t58;
        let t61 = param_a_3;
        let t62 = t56 * t50;
        let t63 = t61 * t62;
        let t64 = t58 * t52;
        let t65 = 1.0 / t64;
        let t67 = param_a_4;
        let t68 = t56 * t56;
        let t69 = t67 * t68;
        let t70 = t58 * t58;
        let t71 = 1.0 / t70;
        let t73 = param_a_5;
        let t74 = t68 * t50;
        let t75 = t73 * t74;
        let t76 = t70 * t52;
        let t77 = 1.0 / t76;
        let t79 = param_a_6;
        let t80 = t68 * t56;
        let t81 = t79 * t80;
        let t82 = t70 * t58;
        let t83 = 1.0 / t82;
        let t85 = param_a_7;
        let t86 = t68 * t62;
        let t87 = t85 * t86;
        let t88 = t70 * t64;
        let t89 = 1.0 / t88;
        let t91 = param_a_8;
        let t92 = t68 * t68;
        let t93 = t91 * t92;
        let t94 = t70 * t70;
        let t95 = 1.0 / t94;
        let t97 = param_a_9;
        let t98 = t92 * t50;
        let t99 = t97 * t98;
        let t101 = 1.0 / t94 / t52;
        let t103 = param_a_10;
        let t104 = t92 * t56;
        let t105 = t103 * t104;
        let t107 = 1.0 / t94 / t58;
        let t109 = param_a_11;
        let t111 = t109 * t92 * t62;
        let t113 = 1.0 / t94 / t64;
        let t115 = t99 * t101 + t105 * t107 + t111 * t113 + t51 * t53 + t57 * t59 + t63 * t65 + t69 * t71 + t75 * t77 + t81 * t83 + t87 * t89 + t93 * t95 + param_a_0;
        let t117 = param_d_0;
        let t121 = 1.0 + 0.00186726 * t34 + 0.00373452 * t49 - 0.001120356 * t44;
        let t124 = param_d_1;
        let t125 = t124 * sigma[ip];
        let t126 = t28 * t33;
        let t128 = param_d_2;
        let t131 = 2.0 * t49 - 3.0 / 5.0 * t44;
        let t133 = t125 * t126 + t128 * t131;
        let t134 = t121 * t121;
        let t135 = 1.0 / t134;
        let t137 = param_d_3;
        let t138 = sigma[ip] * sigma[ip];
        let t139 = t137 * t138;
        let t140 = t30 * t30;
        let t141 = t140 * rho[ip];
        let t143 = 1.0 / t19 / t141;
        let t144 = t27 * t143;
        let t147 = param_d_4;
        let t148 = t147 * sigma[ip];
        let t151 = param_d_5;
        let t152 = t131 * t131;
        let t154 = t148 * t126 * t131 + 2.0 * t139 * t144 + t151 * t152;
        let t155 = t134 * t121;
        let t156 = 1.0 / t155;
        let t158 = t40 * t115 + t117 / t121 + t133 * t135 + t154 * t156;
        let t162 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t158);
        let tzk0 = 2.0 * t162;
        zk[ip] += tzk0;
        let t164 = t18 / t31;
        let t168 = t37 * t37;
        let t171 = 1.0 / t168 * t21 * t25;
        let t172 = t30 * rho[ip];
        let t174 = 1.0 / t31 / t172;
        let t179 = t42 * tau[ip];
        let t183 = t51 * t59;
        let t184 = t46 * t33;
        let t187 = t55 * t50;
        let t188 = t187 * t59;
        let t191 = t57 * t65;
        let t194 = t61 * t56;
        let t195 = t194 * t65;
        let t198 = t63 * t71;
        let t201 = t67 * t62;
        let t202 = t201 * t71;
        let t205 = t69 * t77;
        let t208 = t73 * t68;
        let t209 = t208 * t77;
        let t212 = t75 * t83;
        let t215 = t79 * t74;
        let t216 = t215 * t83;
        let t219 = 5.0 / 3.0 * t179 * t126 * t53 + 5.0 / 3.0 * t183 * t184 + 10.0 / 3.0 * t188 * t184 + 10.0 / 3.0 * t191 * t184 + 5.0 * t195 * t184 + 5.0 * t198 * t184 + 20.0 / 3.0 * t202 * t184 + 20.0 / 3.0 * t205 * t184 + 25.0 / 3.0 * t209 * t184 + 25.0 / 3.0 * t212 * t184 + 10.0 * t216 * t184;
        let t220 = t81 * t89;
        let t223 = t85 * t80;
        let t224 = t223 * t89;
        let t227 = t87 * t95;
        let t230 = t91 * t86;
        let t231 = t230 * t95;
        let t234 = t93 * t101;
        let t237 = t97 * t92;
        let t238 = t237 * t101;
        let t241 = t99 * t107;
        let t244 = t103 * t98;
        let t245 = t244 * t107;
        let t248 = t105 * t113;
        let t251 = t109 * t104;
        let t252 = t251 * t113;
        let t256 = 1.0 / t94 / t70;
        let t257 = t111 * t256;
        let t260 = 10.0 * t220 * t184 + 35.0 / 3.0 * t224 * t184 + 35.0 / 3.0 * t227 * t184 + 40.0 / 3.0 * t231 * t184 + 40.0 / 3.0 * t234 * t184 + 15.0 * t238 * t184 + 15.0 * t241 * t184 + 50.0 / 3.0 * t245 * t184 + 50.0 / 3.0 * t248 * t184 + 55.0 / 3.0 * t252 * t184 + 55.0 / 3.0 * t257 * t184;
        let t261 = t219 + t260;
        let t263 = t117 * t135;
        let t267 = -0.00497936 * t29 * t174 - 0.0062242 * t184;
        let t269 = t28 * t174;
        let t272 = t128 * tau[ip];
        let t275 = -8.0 / 3.0 * t125 * t269 - 10.0 / 3.0 * t272 * t126;
        let t277 = t133 * t156;
        let t280 = t140 * t30;
        let t282 = 1.0 / t19 / t280;
        let t283 = t27 * t282;
        let t289 = t144 * tau[ip];
        let t292 = t151 * t131;
        let t295 = -32.0 / 3.0 * t139 * t283 - 8.0 / 3.0 * t148 * t269 * t131 - 20.0 / 3.0 * t148 * t289 - 20.0 / 3.0 * t292 * t184;
        let t297 = t134 * t134;
        let t298 = 1.0 / t297;
        let t299 = t154 * t298;
        let t302 = -0.015766443403838676 * t171 * t29 * t174 * t115 + t40 * t261 - t263 * t267 + t275 * t135 - 2.0 * t277 * t267 + t295 * t156 - 3.0 * t299 * t267;
        let t307 = piecewise3(t3, 0.0, -t7 * t164 * t158 / 8.0 - 3.0 / 8.0 * t7 * t20 * t302);
        let tvrho0 = 2.0 * rho[ip] * t307 + 2.0 * t162;
        vrho[ip] += tvrho0;
        let t313 = t263 * t126;
        let t315 = t124 * t28;
        let t316 = t33 * t135;
        let t318 = t277 * t126;
        let t320 = t137 * sigma[ip];
        let t323 = t147 * t28;
        let t326 = t323 * t33 * t131 + 4.0 * t320 * t144;
        let t328 = t299 * t126;
        let t330 = 0.005912416276439503 * t171 * t126 * t115 - 0.00186726 * t313 + t315 * t316 - 0.00373452 * t318 + t326 * t156 - 0.00560178 * t328;
        let t334 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t330);
        let tvsigma0 = 2.0 * rho[ip] * t334;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t336 = t42 * t28;
        let t339 = t59 * t28;
        let t340 = t339 * t48;
        let t344 = t65 * t28;
        let t345 = t344 * t48;
        let t350 = t71 * t28;
        let t351 = t350 * t48;
        let t356 = t77 * t28;
        let t357 = t356 * t48;
        let t362 = t83 * t28;
        let t363 = t362 * t48;
        let t368 = -t336 * t48 * t53 - 2.0 * t187 * t340 - 3.0 * t194 * t345 - 4.0 * t201 * t351 - 5.0 * t208 * t357 - 6.0 * t215 * t363 - t51 * t340 - 2.0 * t57 * t345 - 3.0 * t63 * t351 - 4.0 * t69 * t357 - 5.0 * t75 * t363;
        let t369 = t89 * t28;
        let t370 = t369 * t48;
        let t375 = t95 * t28;
        let t376 = t375 * t48;
        let t381 = t101 * t28;
        let t382 = t381 * t48;
        let t387 = t107 * t28;
        let t388 = t387 * t48;
        let t393 = t113 * t28;
        let t394 = t393 * t48;
        let t399 = t256 * t28;
        let t403 = -11.0 * t111 * t399 * t48 - 10.0 * t105 * t394 - 7.0 * t223 * t370 - 8.0 * t230 * t376 - 9.0 * t237 * t382 - 10.0 * t244 * t388 - 11.0 * t251 * t394 - 6.0 * t81 * t370 - 7.0 * t87 * t376 - 8.0 * t93 * t382 - 9.0 * t99 * t388;
        let t404 = t368 + t403;
        let t406 = t28 * t48;
        let t409 = t128 * t28;
        let t416 = 1.0 / t19 / t140;
        let t417 = t27 * t416;
        let t421 = 4.0 * t148 * t417 + 4.0 * t292 * t406;
        let t425 = t40 * t404 - 0.00373452 * t263 * t406 + 2.0 * t409 * t48 * t135 - 0.00746904 * t277 * t406 + t421 * t156 - 0.01120356 * t299 * t406;
        let t429 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t425);
        let tvtau0 = 2.0 * rho[ip] * t429;
        vtau[ip] += tvtau0;
        let t432 = t18 * t48;
        let t444 = 1.0 / t168 / t37 * t43 / t23 / t22;
        let t445 = t138 * t27;
        let t446 = t140 * t172;
        let t448 = 1.0 / t19 / t446;
        let t454 = 1.0 / t31 / t140;
        let t466 = tau[ip] * tau[ip];
        let t467 = t42 * t466;
        let t468 = t144 * t59;
        let t471 = t55 * t466;
        let t474 = t244 * t113;
        let t475 = t466 * t27;
        let t476 = t475 * t143;
        let t479 = t105 * t256;
        let t482 = t109 * t98;
        let t483 = t482 * t113;
        let t486 = t63 * t77;
        let t489 = t67 * t56;
        let t490 = t489 * t71;
        let t493 = t201 * t77;
        let t496 = t69 * t83;
        let t499 = t73 * t62;
        let t500 = t499 * t77;
        let t503 = t208 * t83;
        let t506 = t75 * t89;
        let t509 = -40.0 / 9.0 * t179 * t269 * t53 + 100.0 / 9.0 * t467 * t468 + 100.0 / 9.0 * t471 * t468 + 10000.0 / 9.0 * t474 * t476 + 5500.0 / 9.0 * t479 * t476 + 5500.0 / 9.0 * t483 * t476 + 200.0 / 3.0 * t486 * t476 + 200.0 / 3.0 * t490 * t476 + 1600.0 / 9.0 * t493 * t476 + 1000.0 / 9.0 * t496 * t476 + 1000.0 / 9.0 * t500 * t476 + 2500.0 / 9.0 * t503 * t476 + 500.0 / 3.0 * t506 * t476;
        let t510 = t79 * t68;
        let t511 = t510 * t83;
        let t514 = t215 * t89;
        let t517 = t81 * t95;
        let t520 = t85 * t74;
        let t521 = t520 * t89;
        let t524 = t223 * t95;
        let t527 = t51 * t65;
        let t530 = t187 * t65;
        let t533 = t57 * t71;
        let t536 = t61 * t50;
        let t537 = t536 * t65;
        let t540 = t194 * t71;
        let t543 = t46 * t174;
        let t552 = 500.0 / 3.0 * t511 * t476 + 400.0 * t514 * t476 + 700.0 / 3.0 * t517 * t476 + 700.0 / 3.0 * t521 * t476 + 4900.0 / 9.0 * t524 * t476 + 100.0 / 9.0 * t527 * t476 + 400.0 / 9.0 * t530 * t476 + 100.0 / 3.0 * t533 * t476 + 100.0 / 3.0 * t537 * t476 + 100.0 * t540 * t476 - 400.0 / 9.0 * t245 * t543 - 400.0 / 9.0 * t248 * t543 - 440.0 / 9.0 * t252 * t543 - 440.0 / 9.0 * t257 * t543;
        let t580 = -80.0 / 3.0 * t216 * t543 - 80.0 / 3.0 * t220 * t543 - 280.0 / 9.0 * t224 * t543 - 280.0 / 9.0 * t227 * t543 - 320.0 / 9.0 * t231 * t543 - 320.0 / 9.0 * t234 * t543 - 40.0 * t238 * t543 - 40.0 * t241 * t543 - 80.0 / 9.0 * t188 * t543 - 80.0 / 9.0 * t191 * t543 - 40.0 / 3.0 * t195 * t543 - 40.0 / 3.0 * t198 * t543 - 160.0 / 9.0 * t202 * t543;
        let t589 = t251 * t256;
        let t593 = 1.0 / t94 / t76;
        let t594 = t111 * t593;
        let t597 = t87 * t101;
        let t600 = t91 * t80;
        let t601 = t600 * t95;
        let t604 = t230 * t101;
        let t607 = t93 * t107;
        let t610 = t97 * t86;
        let t611 = t610 * t101;
        let t614 = t237 * t107;
        let t617 = t99 * t113;
        let t620 = t103 * t92;
        let t621 = t620 * t107;
        let t624 = -160.0 / 9.0 * t205 * t543 - 200.0 / 9.0 * t209 * t543 - 200.0 / 9.0 * t212 * t543 - 40.0 / 9.0 * t183 * t543 + 12100.0 / 9.0 * t589 * t476 + 2200.0 / 3.0 * t594 * t476 + 2800.0 / 9.0 * t597 * t476 + 2800.0 / 9.0 * t601 * t476 + 6400.0 / 9.0 * t604 * t476 + 400.0 * t607 * t476 + 400.0 * t611 * t476 + 900.0 * t614 * t476 + 500.0 * t617 * t476 + 500.0 * t621 * t476;
        let t626 = t509 + t552 + t580 + t624;
        let t628 = t117 * t156;
        let t629 = t267 * t267;
        let t635 = 0.018257653333333332 * t29 * t454 + 0.016597866666666666 * t543;
        let t637 = t28 * t454;
        let t642 = 88.0 / 9.0 * t125 * t637 + 80.0 / 9.0 * t272 * t269;
        let t644 = t275 * t156;
        let t647 = t133 * t298;
        let t652 = t27 * t448;
        let t658 = t283 * tau[ip];
        let t661 = t151 * t466;
        let t666 = 608.0 / 9.0 * t139 * t652 + 88.0 / 9.0 * t148 * t637 * t131 + 160.0 / 3.0 * t148 * t658 + 400.0 / 9.0 * t661 * t144 + 160.0 / 9.0 * t292 * t543;
        let t668 = t295 * t298;
        let t672 = 1.0 / t297 / t121;
        let t673 = t154 * t672;
        let t678 = -0.0015382090641719766 * t444 * t445 * t448 * t115 + 0.057810292480741814 * t171 * t29 * t454 * t115 - 0.03153288680767735 * t171 * t29 * t174 * t261 + t40 * t626 + 2.0 * t628 * t629 - t263 * t635 + t642 * t135 - 4.0 * t644 * t267 + 6.0 * t647 * t629 - 2.0 * t277 * t635 + t666 * t156 - 6.0 * t668 * t267 + 12.0 * t673 * t629 - 3.0 * t299 * t635;
        let t683 = piecewise3(t3, 0.0, t7 * t432 * t158 / 12.0 - t7 * t164 * t302 / 4.0 - 3.0 / 8.0 * t7 * t20 * t678);
        let tv2rho20 = 2.0 * rho[ip] * t683 + 4.0 * t307;
        v2rho2[ip] += tv2rho20;
        let t689 = t115 * sigma[ip];
        let t699 = t126 * t267;
        let t700 = t628 * t699;
        let t702 = t263 * t269;
        let t704 = t174 * t135;
        let t707 = t33 * t156;
        let t708 = t707 * t267;
        let t711 = t644 * t126;
        let t713 = t647 * t699;
        let t715 = t277 * t269;
        let t722 = t147 * t27;
        let t726 = -64.0 / 3.0 * t320 * t283 - 8.0 / 3.0 * t323 * t174 * t131 - 20.0 / 3.0 * t722 * t143 * tau[ip];
        let t728 = t326 * t298;
        let t731 = t668 * t126;
        let t733 = t673 * t699;
        let t735 = t299 * t269;
        let t737 = 0.0005768283990644912 * t444 * t283 * t689 - 0.015766443403838676 * t171 * t269 * t115 + 0.005912416276439503 * t171 * t126 * t261 + 0.00373452 * t700 + 0.00497936 * t702 - 8.0 / 3.0 * t315 * t704 - 2.0 * t315 * t708 - 0.00373452 * t711 + 0.01120356 * t713 + 0.00995872 * t715 + t726 * t156 - 3.0 * t728 * t267 - 0.00560178 * t731 + 0.02240712 * t733 + 0.01493808 * t735;
        let t742 = piecewise3(t3, 0.0, -t7 * t164 * t330 / 8.0 - 3.0 / 8.0 * t7 * t20 * t737);
        let tv2rhosigma0 = 2.0 * rho[ip] * t742 + 2.0 * t334;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t752 = t381 * t33;
        let t755 = t387 * t33;
        let t760 = t393 * t33;
        let t768 = t362 * t33;
        let t771 = t369 * t33;
        let t776 = t375 * t33;
        let t783 = t344 * t33;
        let t786 = 15.0 * t237 * t752 + 15.0 * t99 * t755 + 50.0 / 3.0 * t244 * t755 + 50.0 / 3.0 * t105 * t760 + 55.0 / 3.0 * t251 * t760 + 55.0 / 3.0 * t111 * t399 * t33 + 10.0 * t215 * t768 + 10.0 * t81 * t771 + 35.0 / 3.0 * t223 * t771 + 35.0 / 3.0 * t87 * t776 + 40.0 / 3.0 * t230 * t776 + 40.0 / 3.0 * t93 * t752 + 10.0 / 3.0 * t57 * t783;
        let t789 = t350 * t33;
        let t794 = t356 * t33;
        let t801 = t339 * t33;
        let t806 = t42 * t27;
        let t807 = t416 * t59;
        let t811 = t55 * tau[ip];
        let t815 = t417 * tau[ip];
        let t824 = 5.0 * t194 * t783 + 5.0 * t63 * t789 + 20.0 / 3.0 * t201 * t789 + 20.0 / 3.0 * t69 * t794 + 25.0 / 3.0 * t208 * t794 + 25.0 / 3.0 * t75 * t768 + 5.0 / 3.0 * t51 * t801 + 10.0 / 3.0 * t187 * t801 - 20.0 / 3.0 * t806 * t807 * tau[ip] - 20.0 / 3.0 * t811 * t417 * t59 - 20.0 / 3.0 * t527 * t815 - 80.0 / 3.0 * t530 * t815 - 440.0 * t594 * t815 - 100.0 * t511 * t815;
        let t852 = -240.0 * t514 * t815 - 140.0 * t517 * t815 - 140.0 * t521 * t815 - 980.0 / 3.0 * t524 * t815 - 560.0 / 3.0 * t597 * t815 - 560.0 / 3.0 * t601 * t815 - 1280.0 / 3.0 * t604 * t815 - 240.0 * t607 * t815 - 20.0 * t533 * t815 - 20.0 * t537 * t815 - 60.0 * t540 * t815 - 40.0 * t486 * t815 - 40.0 * t490 * t815;
        let t882 = -320.0 / 3.0 * t493 * t815 - 200.0 / 3.0 * t496 * t815 - 200.0 / 3.0 * t500 * t815 - 500.0 / 3.0 * t503 * t815 - 100.0 * t506 * t815 - 240.0 * t611 * t815 - 540.0 * t614 * t815 - 300.0 * t617 * t815 - 300.0 * t621 * t815 - 2000.0 / 3.0 * t474 * t815 - 1100.0 / 3.0 * t479 * t815 - 1100.0 / 3.0 * t483 * t815 - 2420.0 / 3.0 * t589 * t815 + 5.0 / 3.0 * t336 * t33 * t53;
        let t884 = t786 + t824 + t852 + t882;
        let t886 = t406 * t267;
        let t892 = t48 * t156;
        let t903 = t151 * tau[ip];
        let t908 = -52.0 / 3.0 * t148 * t144 - 80.0 / 3.0 * t903 * t417 - 20.0 / 3.0 * t292 * t126;
        let t910 = t421 * t298;
        let t918 = -0.015766443403838676 * t171 * t29 * t174 * t404 + t40 * t884 + 0.00746904 * t628 * t886 + 0.0062242 * t313 - 10.0 / 3.0 * t409 * t316 - 4.0 * t409 * t892 * t267 - 0.00746904 * t644 * t406 + 0.02240712 * t647 * t886 + 0.0124484 * t318 + t908 * t156 - 3.0 * t910 * t267 - 0.01120356 * t668 * t406 + 0.04481424 * t673 * t886 + 0.0186726 * t328;
        let t923 = piecewise3(t3, 0.0, -t7 * t164 * t425 / 8.0 - 3.0 / 8.0 * t7 * t20 * t918);
        let tv2rhotau0 = 2.0 * rho[ip] * t923 + 2.0 * t429;
        v2rhotau[ip] += tv2rhotau0;
        let t929 = t628 * t144;
        let t931 = t124 * t27;
        let t932 = t143 * t156;
        let t933 = t931 * t932;
        let t935 = t647 * t144;
        let t937 = t137 * t27;
        let t940 = t728 * t126;
        let t942 = t673 * t144;
        let t944 = -0.00021631064964918421 * t444 * t144 * t115 + 1.39466396304e-05 * t929 - 0.01493808 * t933 + 4.18399188912e-05 * t935 + 4.0 * t937 * t932 - 0.01120356 * t940 + 8.36798377824e-05 * t942;
        let t948 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t944);
        let tv2sigma20 = 2.0 * rho[ip] * t948;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t953 = t628 * t417;
        let t955 = t128 * t27;
        let t956 = t416 * t156;
        let t957 = t955 * t956;
        let t961 = t647 * t417;
        let t965 = t910 * t126;
        let t969 = t673 * t417;
        let t971 = 0.005912416276439503 * t171 * t126 * t404 + 2.78932792608e-05 * t953 - 0.01493808 * t957 - 0.01493808 * t931 * t956 + 8.36798377824e-05 * t961 + 4.0 * t722 * t956 - 0.00560178 * t965 - 0.01120356 * t728 * t406 + 0.0001673596755648 * t969;
        let t975 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t971);
        let tv2sigmatau0 = 2.0 * rho[ip] * t975;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t977 = t89 * t27;
        let t979 = 1.0 / t19 / t172;
        let t980 = t977 * t979;
        let t983 = t95 * t27;
        let t984 = t983 * t979;
        let t991 = t101 * t27;
        let t992 = t991 * t979;
        let t999 = t107 * t27;
        let t1000 = t999 * t979;
        let t1005 = t65 * t27;
        let t1006 = t1005 * t979;
        let t1011 = t71 * t27;
        let t1012 = t1011 * t979;
        let t1021 = t113 * t27;
        let t1022 = t1021 * t979;
        let t1025 = 324.0 * t237 * t1000 + 144.0 * t93 * t1000 + 16.0 * t187 * t1006 + 4.0 * t51 * t1006 + 12.0 * t536 * t1006 + 36.0 * t194 * t1012 + 12.0 * t57 * t1012 + 180.0 * t99 * t1022 + 144.0 * t215 * t980 + 196.0 * t223 * t984 + 256.0 * t230 * t992 + 84.0 * t520 * t980 + 112.0 * t600 * t984 + 144.0 * t610 * t992 + 84.0 * t81 * t984 + 112.0 * t87 * t992;
        let t1030 = t256 * t27;
        let t1031 = t1030 * t979;
        let t1038 = t593 * t27;
        let t1042 = t77 * t27;
        let t1043 = t1042 * t979;
        let t1050 = t83 * t27;
        let t1051 = t1050 * t979;
        let t1062 = t979 * t59;
        let t1065 = t55 * t27;
        let t1068 = 264.0 * t111 * t1038 * t979 + 180.0 * t620 * t1000 + 24.0 * t489 * t1012 + 400.0 * t244 * t1022 + 220.0 * t482 * t1022 + 220.0 * t105 * t1031 + 484.0 * t251 * t1031 + 64.0 * t201 * t1043 + 40.0 * t499 * t1043 + 24.0 * t63 * t1043 + 100.0 * t208 * t1051 + 60.0 * t510 * t1051 + 40.0 * t69 * t1051 + 4.0 * t1065 * t1062 + 4.0 * t806 * t1062 + 60.0 * t75 * t980;
        let t1069 = t1025 + t1068;
        let t1071 = t27 * t979;
        let t1074 = t979 * t156;
        let t1079 = t151 * t27;
        let t1086 = t40 * t1069 + 5.57865585216e-05 * t628 * t1071 - 0.05975232 * t955 * t1074 + 0.0001673596755648 * t647 * t1071 + 16.0 * t1079 * t1074 - 0.02240712 * t910 * t406 + 0.0003347193511296 * t673 * t1071;
        let t1090 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t1086);
        let tv2tau20 = 2.0 * rho[ip] * t1090;
        v2tau2[ip] += tv2tau20;
    }
}
