//! HYB_GGA_XC_CASE21 vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/hyb_gga_xc_case21.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::bspline::{case21_cbspline, case21_xbspline};
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn hyb_gga_xc_case21_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_ax: f64,
    param_gammac: f64,
    param_gammax: f64,
    param_cx_0: f64,
    param_cx_1: f64,
    param_cx_2: f64,
    param_cx_3: f64,
    param_cx_4: f64,
    param_cx_5: f64,
    param_cx_6: f64,
    param_cx_7: f64,
    param_cx_8: f64,
    param_cx_9: f64,
    param_cc_0: f64,
    param_cc_1: f64,
    param_cc_2: f64,
    param_cc_3: f64,
    param_cc_4: f64,
    param_cc_5: f64,
    param_cc_6: f64,
    param_cc_7: f64,
    param_cc_8: f64,
    param_cc_9: f64,
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
        let t1 = 1.0 - param_ax;
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
        let t29 = M_CBRT6;
        let t30 = param_gammax * t29;
        let t31 = M_PI * M_PI;
        let t32 = pow_1_3(t31);
        let t33 = t32 * t32;
        let t34 = 1.0 / t33;
        let t35 = t30 * t34;
        let t36 = rho0 * rho0;
        let t37 = pow_1_3(rho0);
        let t38 = t37 * t37;
        let t40 = 1.0 / t38 / t36;
        let t42 = t34 * sigma0;
        let t46 = 1.0 + t30 * t42 * t40 / 24.0;
        let t47 = 1.0 / t46;
        let t50 = t35 * sigma0 * t40 * t47 / 24.0;
        let t51 = case21_xbspline(t50, 0, param_cx_0, param_cx_1, param_cx_2, param_cx_3, param_cx_4, param_cx_5, param_cx_6, param_cx_7, param_cx_8, param_cx_9);
        let t55 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t51);
        let t56 = rho1 <= dens_threshold;
        let t57 = -t17;
        let t59 = piecewise5(t15, t12, t11, t16, t57 * t8);
        let t60 = 1.0 + t59;
        let t61 = t60 <= zeta_threshold;
        let t62 = pow_1_3(t60);
        let t64 = piecewise3(t61, t23, t62 * t60);
        let t65 = t64 * t27;
        let t66 = rho1 * rho1;
        let t67 = pow_1_3(rho1);
        let t68 = t67 * t67;
        let t70 = 1.0 / t68 / t66;
        let t72 = t34 * sigma2;
        let t76 = 1.0 + t30 * t72 * t70 / 24.0;
        let t77 = 1.0 / t76;
        let t80 = t35 * sigma2 * t70 * t77 / 24.0;
        let t81 = case21_xbspline(t80, 0, param_cx_0, param_cx_1, param_cx_2, param_cx_3, param_cx_4, param_cx_5, param_cx_6, param_cx_7, param_cx_8, param_cx_9);
        let t85 = piecewise3(t56, 0.0, -3.0 / 8.0 * t6 * t65 * t81);
        let t87 = t1 * (t55 + t85);
        let t88 = t18 + 1.0;
        let t89 = t88 <= zeta_threshold;
        let t90 = t22 * t22;
        let t91 = pow_1_3(t88);
        let t92 = t91 * t91;
        let t93 = piecewise3(t89, t90, t92);
        let t94 = 1.0 - t18;
        let t95 = t94 <= zeta_threshold;
        let t96 = pow_1_3(t94);
        let t97 = t96 * t96;
        let t98 = piecewise3(t95, t90, t97);
        let t100 = t93 / 2.0 + t98 / 2.0;
        let t101 = t3 * t3;
        let t102 = t100 * t101;
        let t103 = t102 * t4;
        let t104 = f64::sqrt(sigma0);
        let t105 = f64::sqrt(sigma2);
        let t106 = t104 + t105;
        let t107 = t106 * t106;
        let t108 = t7 * t7;
        let t110 = 1.0 / t27 / t108;
        let t111 = t107 * t110;
        let t112 = t4 * t107;
        let t113 = t112 * t110;
        let t116 = 1.0 / M_PI;
        let t117 = pow_1_3(t116);
        let t118 = t3 * t117;
        let t119 = M_CBRT4;
        let t120 = t119 * t119;
        let t123 = t118 * t120 / t27;
        let t125 = 1.0 + 0.53425e-1 * t123;
        let t126 = f64::sqrt(t123);
        let t129 = pow_3_2(t123);
        let t131 = t117 * t117;
        let t132 = t101 * t131;
        let t133 = t27 * t27;
        let t134 = 1.0 / t133;
        let t136 = t132 * t119 * t134;
        let t138 = 0.379785e1 * t126 + 0.8969e0 * t123 + 0.204775e0 * t129 + 0.123235e0 * t136;
        let t141 = 1.0 + 0.16081979498692535067e2 / t138;
        let t142 = f64::ln(t141);
        let t144 = 0.621814e-1 * t125 * t142;
        let t145 = t17 * t17;
        let t146 = t145 * t145;
        let t147 = t108 * t108;
        let t148 = 1.0 / t147;
        let t149 = t146 * t148;
        let t150 = t91 * t88;
        let t151 = piecewise3(t89, t23, t150);
        let t152 = t96 * t94;
        let t153 = piecewise3(t95, t23, t152);
        let t154 = t151 + t153 - 2.0;
        let t155 = M_CBRT2;
        let t158 = 1.0 / (2.0 * t155 - 2.0);
        let t159 = t154 * t158;
        let t161 = 1.0 + 0.5137e-1 * t123;
        let t166 = 0.705945e1 * t126 + 0.1549425e1 * t123 + 0.420775e0 * t129 + 0.1562925e0 * t136;
        let t169 = 1.0 + 0.32163958997385070134e2 / t166;
        let t170 = f64::ln(t169);
        let t174 = 1.0 + 0.278125e-1 * t123;
        let t179 = 0.51785e1 * t126 + 0.905775e0 * t123 + 0.1100325e0 * t129 + 0.1241775e0 * t136;
        let t182 = 1.0 + 0.29608749977793437516e2 / t179;
        let t183 = f64::ln(t182);
        let t184 = t174 * t183;
        let t186 = -0.310907e-1 * t161 * t170 + t144 - 0.19751673498613801407e-1 * t184;
        let t187 = t159 * t186;
        let t191 = -t144 + t149 * t187 + 0.19751673498613801407e-1 * t159 * t184;
        let t193 = -t102 * t113 / 48.0 + param_gammac * t191;
        let t194 = 1.0 / t193;
        let t195 = t111 * t194;
        let t197 = t103 * t195 / 48.0;
        let t198 = case21_cbspline(-t197, 0, param_cc_0, param_cc_1, param_cc_2, param_cc_3, param_cc_4, param_cc_5, param_cc_6, param_cc_7, param_cc_8, param_cc_9);
        let t199 = t198 * t191;
        let tzk0 = t87 + t199;
        zk[ip] += tzk0;
        let t200 = 1.0 / t108;
        let t201 = t17 * t200;
        let t202 = t8 - t201;
        let t203 = piecewise5(t11, 0.0, t15, 0.0, t202);
        let t206 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t203);
        let t214 = t6 * t26 * t134 * t51 / 8.0;
        let t215 = t6 * t26;
        let t216 = case21_xbspline(t50, 1, param_cx_0, param_cx_1, param_cx_2, param_cx_3, param_cx_4, param_cx_5, param_cx_6, param_cx_7, param_cx_8, param_cx_9);
        let t217 = t27 * t216;
        let t218 = t36 * rho0;
        let t220 = 1.0 / t38 / t218;
        let t225 = param_gammax * param_gammax;
        let t226 = t29 * t29;
        let t227 = t225 * t226;
        let t229 = 1.0 / t32 / t31;
        let t230 = t227 * t229;
        let t231 = sigma0 * sigma0;
        let t232 = t36 * t36;
        let t233 = t232 * t36;
        let t235 = 1.0 / t37 / t233;
        let t237 = t46 * t46;
        let t238 = 1.0 / t237;
        let t242 = -t35 * sigma0 * t220 * t47 / 9.0 + t230 * t231 * t235 * t238 / 216.0;
        let t243 = t217 * t242;
        let t247 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t206 * t27 * t51 - t214 - 3.0 / 8.0 * t215 * t243);
        let t248 = t57 * t200;
        let t250 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t248);
        let t253 = piecewise3(t61, 0.0, 4.0 / 3.0 * t62 * t250);
        let t261 = t6 * t64 * t134 * t81 / 8.0;
        let t263 = piecewise3(t56, 0.0, -3.0 / 8.0 * t6 * t253 * t27 * t81 - t261);
        let t265 = t1 * (t247 + t263);
        let t266 = case21_cbspline(-t197, 1, param_cc_0, param_cc_1, param_cc_2, param_cc_3, param_cc_4, param_cc_5, param_cc_6, param_cc_7, param_cc_8, param_cc_9);
        let t267 = 1.0 / t91;
        let t270 = piecewise3(t89, 0.0, 2.0 / 3.0 * t267 * t202);
        let t271 = 1.0 / t96;
        let t272 = -t202;
        let t275 = piecewise3(t95, 0.0, 2.0 / 3.0 * t271 * t272);
        let t277 = t270 / 2.0 + t275 / 2.0;
        let t278 = t277 * t101;
        let t279 = t278 * t4;
        let t282 = t108 * t7;
        let t284 = 1.0 / t27 / t282;
        let t285 = t107 * t284;
        let t286 = t285 * t194;
        let t288 = 7.0 / 144.0 * t103 * t286;
        let t289 = t193 * t193;
        let t290 = 1.0 / t289;
        let t293 = t112 * t284;
        let t295 = 7.0 / 144.0 * t102 * t293;
        let t297 = 1.0 / t27 / t7;
        let t298 = t120 * t297;
        let t301 = 0.11073470983333333333e-2 * t118 * t298 * t142;
        let t302 = t138 * t138;
        let t303 = 1.0 / t302;
        let t304 = t125 * t303;
        let t306 = 1.0 / t126 * t3;
        let t307 = t117 * t120;
        let t308 = t307 * t297;
        let t309 = t306 * t308;
        let t311 = t118 * t298;
        let t313 = f64::sqrt(t123);
        let t314 = t313 * t3;
        let t315 = t314 * t308;
        let t318 = 1.0 / t133 / t7;
        let t320 = t132 * t119 * t318;
        let t322 = -0.632975e0 * t309 - 0.29896666666666666667e0 * t311 - 0.1023875e0 * t315 - 0.82156666666666666667e-1 * t320;
        let t323 = 1.0 / t141;
        let t324 = t322 * t323;
        let t326 = 1.0 * t304 * t324;
        let t327 = t145 * t17;
        let t328 = t327 * t148;
        let t330 = 4.0 * t328 * t187;
        let t331 = t147 * t7;
        let t332 = 1.0 / t331;
        let t333 = t146 * t332;
        let t335 = 4.0 * t333 * t187;
        let t338 = piecewise3(t89, 0.0, 4.0 / 3.0 * t91 * t202);
        let t341 = piecewise3(t95, 0.0, 4.0 / 3.0 * t96 * t272);
        let t343 = (t338 + t341) * t158;
        let t344 = t343 * t186;
        let t349 = t166 * t166;
        let t350 = 1.0 / t349;
        let t351 = t161 * t350;
        let t356 = -0.1176575e1 * t309 - 0.516475e0 * t311 - 0.2103875e0 * t315 - 0.104195e0 * t320;
        let t357 = 1.0 / t169;
        let t358 = t356 * t357;
        let t364 = t179 * t179;
        let t365 = 1.0 / t364;
        let t366 = t174 * t365;
        let t371 = -0.86308333333333333334e0 * t309 - 0.301925e0 * t311 - 0.5501625e-1 * t315 - 0.82785e-1 * t320;
        let t372 = 1.0 / t182;
        let t373 = t371 * t372;
        let t376 = 0.53237641966666666666e-3 * t118 * t298 * t170 + 1.0 * t351 * t358 - t301 - t326 + 0.18311447306006545054e-3 * t118 * t298 * t183 + 0.5848223622634646207e0 * t366 * t373;
        let t377 = t159 * t376;
        let t378 = t149 * t377;
        let t381 = t159 * t3;
        let t383 = t307 * t297 * t183;
        let t385 = 0.18311447306006545054e-3 * t381 * t383;
        let t386 = t159 * t174;
        let t388 = t365 * t371 * t372;
        let t390 = 0.5848223622634646207e0 * t386 * t388;
        let t391 = t301 + t326 + t330 - t335 + t149 * t344 + t378 + 0.19751673498613801407e-1 * t343 * t184 - t385 - t390;
        let t393 = -t278 * t113 / 48.0 + t295 + param_gammac * t391;
        let t394 = t290 * t393;
        let t395 = t111 * t394;
        let t398 = -t279 * t195 / 48.0 + t288 + t103 * t395 / 48.0;
        let t399 = t266 * t398;
        let t400 = t399 * t191;
        let t401 = t198 * t391;
        let tvrho0 = t87 + t199 + t7 * (t265 + t400 + t401);
        vrho[ip * 2] += tvrho0;
        let t404 = -t8 - t201;
        let t405 = piecewise5(t11, 0.0, t15, 0.0, t404);
        let t408 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t405);
        let t414 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t408 * t27 * t51 - t214);
        let t416 = piecewise5(t15, 0.0, t11, 0.0, t8 - t248);
        let t419 = piecewise3(t61, 0.0, 4.0 / 3.0 * t62 * t416);
        let t424 = t6 * t64;
        let t425 = case21_xbspline(t80, 1, param_cx_0, param_cx_1, param_cx_2, param_cx_3, param_cx_4, param_cx_5, param_cx_6, param_cx_7, param_cx_8, param_cx_9);
        let t426 = t27 * t425;
        let t427 = t66 * rho1;
        let t429 = 1.0 / t68 / t427;
        let t434 = sigma2 * sigma2;
        let t435 = t66 * t66;
        let t436 = t435 * t66;
        let t438 = 1.0 / t67 / t436;
        let t440 = t76 * t76;
        let t441 = 1.0 / t440;
        let t445 = -t35 * sigma2 * t429 * t77 / 9.0 + t230 * t434 * t438 * t441 / 216.0;
        let t446 = t426 * t445;
        let t450 = piecewise3(t56, 0.0, -3.0 / 8.0 * t6 * t419 * t27 * t81 - t261 - 3.0 / 8.0 * t424 * t446);
        let t452 = t1 * (t414 + t450);
        let t455 = piecewise3(t89, 0.0, 2.0 / 3.0 * t267 * t404);
        let t456 = -t404;
        let t459 = piecewise3(t95, 0.0, 2.0 / 3.0 * t271 * t456);
        let t461 = t455 / 2.0 + t459 / 2.0;
        let t462 = t461 * t101;
        let t463 = t462 * t4;
        let t470 = piecewise3(t89, 0.0, 4.0 / 3.0 * t91 * t404);
        let t473 = piecewise3(t95, 0.0, 4.0 / 3.0 * t96 * t456);
        let t475 = (t470 + t473) * t158;
        let t476 = t475 * t186;
        let t480 = t301 + t326 - t330 - t335 + t149 * t476 + t378 + 0.19751673498613801407e-1 * t475 * t184 - t385 - t390;
        let t482 = -t462 * t113 / 48.0 + t295 + param_gammac * t480;
        let t483 = t290 * t482;
        let t484 = t111 * t483;
        let t487 = -t463 * t195 / 48.0 + t288 + t103 * t484 / 48.0;
        let t488 = t266 * t487;
        let t489 = t488 * t191;
        let t490 = t198 * t480;
        let tvrho1 = t87 + t199 + t7 * (t452 + t489 + t490);
        vrho[ip * 2 + 1] += tvrho1;
        let t497 = t232 * rho0;
        let t499 = 1.0 / t37 / t497;
        let t504 = t30 * t34 * t40 * t47 / 24.0 - t230 * sigma0 * t499 * t238 / 576.0;
        let t505 = t217 * t504;
        let t508 = piecewise3(t2, 0.0, -3.0 / 8.0 * t215 * t505);
        let t509 = t1 * t508;
        let t510 = t106 * t110;
        let t511 = 1.0 / t104;
        let t512 = t194 * t511;
        let t513 = t510 * t512;
        let t516 = t100 * t100;
        let t517 = t516 * t3;
        let t518 = t4 * t4;
        let t519 = t517 * t518;
        let t520 = t107 * t106;
        let t522 = 1.0 / t133 / t147;
        let t523 = t520 * t522;
        let t524 = t290 * t511;
        let t525 = t523 * t524;
        let t528 = -t103 * t513 / 48.0 - t519 * t525 / 768.0;
        let t529 = t266 * t528;
        let t530 = t529 * t191;
        let tvsigma0 = t7 * (t509 + t530);
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t536 = t435 * rho1;
        let t538 = 1.0 / t67 / t536;
        let t543 = t30 * t34 * t70 * t77 / 24.0 - t230 * sigma2 * t538 * t441 / 576.0;
        let t544 = t426 * t543;
        let t547 = piecewise3(t56, 0.0, -3.0 / 8.0 * t424 * t544);
        let t548 = t1 * t547;
        let t549 = 1.0 / t105;
        let t550 = t194 * t549;
        let t551 = t510 * t550;
        let t554 = t290 * t549;
        let t555 = t523 * t554;
        let t558 = -t103 * t551 / 48.0 - t519 * t555 / 768.0;
        let t559 = t266 * t558;
        let t560 = t559 * t191;
        let tvsigma2 = t7 * (t548 + t560);
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
