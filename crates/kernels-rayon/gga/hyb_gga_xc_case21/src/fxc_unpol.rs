//! HYB_GGA_XC_CASE21 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/hyb_gga_xc_case21.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::bspline::{case21_cbspline, case21_xbspline};
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_gga_xc_case21_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
    for ip in 0..zk.len() {
        let t1 = 1.0 - param_ax;
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * zeta_threshold;
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t15, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT6;
        let t23 = M_PI * M_PI;
        let t24 = pow_1_3(t23);
        let t25 = t24 * t24;
        let t26 = 1.0 / t25;
        let t27 = param_gammax * t21 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = sigma[ip] * t29;
        let t31 = rho[ip] * rho[ip];
        let t32 = t19 * t19;
        let t34 = 1.0 / t32 / t31;
        let t38 = 1.0 + t27 * t30 * t34 / 24.0;
        let t39 = 1.0 / t38;
        let t43 = t27 * t30 * t34 * t39 / 24.0;
        let t44 = case21_xbspline(t43, 0, param_cx_0, param_cx_1, param_cx_2, param_cx_3, param_cx_4, param_cx_5, param_cx_6, param_cx_7, param_cx_8, param_cx_9);
        let t48 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t44);
        let t50 = 2.0 * t1 * t48;
        let t51 = t14 * t14;
        let t52 = piecewise3(t8, t51, 1.0);
        let t53 = t4 * t4;
        let t54 = t52 * t53;
        let t55 = t54 * t5;
        let t57 = 1.0 / t19 / t31;
        let t58 = sigma[ip] * t57;
        let t59 = t5 * sigma[ip];
        let t63 = 1.0 / M_PI;
        let t64 = pow_1_3(t63);
        let t65 = t4 * t64;
        let t66 = M_CBRT4;
        let t67 = t66 * t66;
        let t70 = t65 * t67 / t19;
        let t72 = 1.0 + 0.53425e-1 * t70;
        let t73 = f64::sqrt(t70);
        let t76 = pow_3_2(t70);
        let t78 = t64 * t64;
        let t79 = t53 * t78;
        let t80 = 1.0 / t32;
        let t82 = t79 * t66 * t80;
        let t84 = 0.379785e1 * t73 + 0.8969e0 * t70 + 0.204775e0 * t76 + 0.123235e0 * t82;
        let t87 = 1.0 + 0.16081979498692535067e2 / t84;
        let t88 = f64::ln(t87);
        let t91 = piecewise3(t8, t15, 1.0);
        let t97 = (2.0 * t91 - 2.0) / (2.0 * t28 - 2.0);
        let t99 = 1.0 + 0.278125e-1 * t70;
        let t104 = 0.51785e1 * t73 + 0.905775e0 * t70 + 0.1100325e0 * t76 + 0.1241775e0 * t82;
        let t107 = 1.0 + 0.29608749977793437516e2 / t104;
        let t108 = f64::ln(t107);
        let t112 = -0.621814e-1 * t72 * t88 + 0.19751673498613801407e-1 * t97 * t99 * t108;
        let t114 = -t54 * t59 * t57 / 48.0 + param_gammac * t112;
        let t115 = 1.0 / t114;
        let t118 = t55 * t58 * t115 / 48.0;
        let t119 = case21_cbspline(-t118, 0, param_cc_0, param_cc_1, param_cc_2, param_cc_3, param_cc_4, param_cc_5, param_cc_6, param_cc_7, param_cc_8, param_cc_9);
        let t120 = t119 * t112;
        let tzk0 = t50 + t120;
        zk[ip] += tzk0;
        let t125 = t7 * t18;
        let t126 = case21_xbspline(t43, 1, param_cx_0, param_cx_1, param_cx_2, param_cx_3, param_cx_4, param_cx_5, param_cx_6, param_cx_7, param_cx_8, param_cx_9);
        let t127 = t19 * t126;
        let t128 = t31 * rho[ip];
        let t130 = 1.0 / t32 / t128;
        let t135 = param_gammax * param_gammax;
        let t136 = t21 * t21;
        let t139 = 1.0 / t24 / t23;
        let t140 = t135 * t136 * t139;
        let t141 = sigma[ip] * sigma[ip];
        let t142 = t141 * t28;
        let t143 = t31 * t31;
        let t144 = t143 * t31;
        let t146 = 1.0 / t19 / t144;
        let t147 = t38 * t38;
        let t148 = 1.0 / t147;
        let t153 = -t27 * t30 * t130 * t39 / 9.0 + t140 * t142 * t146 * t148 / 108.0;
        let t158 = piecewise3(t3, 0.0, -t7 * t18 * t80 * t44 / 8.0 - 3.0 / 8.0 * t125 * t127 * t153);
        let t159 = t1 * t158;
        let t161 = case21_cbspline(-t118, 1, param_cc_0, param_cc_1, param_cc_2, param_cc_3, param_cc_4, param_cc_5, param_cc_6, param_cc_7, param_cc_8, param_cc_9);
        let t163 = 1.0 / t19 / t128;
        let t164 = sigma[ip] * t163;
        let t168 = t114 * t114;
        let t169 = 1.0 / t168;
        let t174 = 1.0 / t19 / rho[ip];
        let t175 = t67 * t174;
        let t179 = t84 * t84;
        let t180 = 1.0 / t179;
        let t181 = t72 * t180;
        let t183 = 1.0 / t73 * t4;
        let t184 = t64 * t67;
        let t185 = t184 * t174;
        let t186 = t183 * t185;
        let t188 = t65 * t175;
        let t190 = f64::sqrt(t70);
        let t191 = t190 * t4;
        let t192 = t191 * t185;
        let t195 = 1.0 / t32 / rho[ip];
        let t197 = t79 * t66 * t195;
        let t199 = -0.632975e0 * t186 - 0.29896666666666666667e0 * t188 - 0.1023875e0 * t192 - 0.82156666666666666667e-1 * t197;
        let t200 = 1.0 / t87;
        let t201 = t199 * t200;
        let t204 = t97 * t4;
        let t209 = t97 * t99;
        let t210 = t104 * t104;
        let t211 = 1.0 / t210;
        let t216 = -0.86308333333333333334e0 * t186 - 0.301925e0 * t188 - 0.5501625e-1 * t192 - 0.82785e-1 * t197;
        let t218 = 1.0 / t107;
        let t219 = t211 * t216 * t218;
        let t222 = 0.11073470983333333333e-2 * t65 * t175 * t88 + 1.0 * t181 * t201 - 0.18311447306006545054e-3 * t204 * t184 * t174 * t108 - 0.5848223622634646207e0 * t209 * t219;
        let t224 = 7.0 / 144.0 * t54 * t59 * t163 + param_gammac * t222;
        let t225 = t169 * t224;
        let t229 = 7.0 / 144.0 * t55 * t164 * t115 + t55 * t58 * t225 / 48.0;
        let t230 = t161 * t229;
        let t231 = t230 * t112;
        let t232 = t119 * t222;
        let tvrho0 = t50 + t120 + rho[ip] * (2.0 * t159 + t231 + t232);
        vrho[ip] += tvrho0;
        let t240 = t143 * rho[ip];
        let t242 = 1.0 / t19 / t240;
        let t247 = t27 * t29 * t34 * t39 / 24.0 - t140 * sigma[ip] * t28 * t242 * t148 / 288.0;
        let t251 = piecewise3(t3, 0.0, -3.0 / 8.0 * t125 * t127 * t247);
        let t253 = 2.0 * t1 * t251;
        let t258 = t52 * t52;
        let t259 = t258 * t4;
        let t260 = t5 * t5;
        let t261 = t259 * t260;
        let t263 = 1.0 / t32 / t143;
        let t264 = sigma[ip] * t263;
        let t268 = -t54 * t5 * t57 * t115 / 48.0 - t261 * t264 * t169 / 768.0;
        let t269 = t161 * t268;
        let t270 = t269 * t112;
        let tvsigma0 = rho[ip] * (t253 + t270);
        vsigma[ip] += tvsigma0;
        let t279 = t80 * t126;
        let t283 = case21_xbspline(t43, 2, param_cx_0, param_cx_1, param_cx_2, param_cx_3, param_cx_4, param_cx_5, param_cx_6, param_cx_7, param_cx_8, param_cx_9);
        let t284 = t19 * t283;
        let t285 = t153 * t153;
        let t293 = t143 * t128;
        let t295 = 1.0 / t19 / t293;
        let t301 = t23 * t23;
        let t302 = 1.0 / t301;
        let t303 = t135 * param_gammax * t302;
        let t304 = t141 * sigma[ip];
        let t305 = t143 * t143;
        let t306 = t305 * t31;
        let t307 = 1.0 / t306;
        let t310 = 1.0 / t147 / t38;
        let t314 = 11.0 / 27.0 * t27 * t30 * t263 * t39 - t140 * t142 * t295 * t148 / 12.0 + 2.0 / 81.0 * t303 * t304 * t307 * t310;
        let t319 = piecewise3(t3, 0.0, t7 * t18 * t195 * t44 / 12.0 - t125 * t279 * t153 / 4.0 - 3.0 / 8.0 * t125 * t284 * t285 - 3.0 / 8.0 * t125 * t127 * t314);
        let t320 = t1 * t319;
        let t322 = case21_cbspline(-t118, 2, param_cc_0, param_cc_1, param_cc_2, param_cc_3, param_cc_4, param_cc_5, param_cc_6, param_cc_7, param_cc_8, param_cc_9);
        let t323 = t229 * t229;
        let t324 = t322 * t323;
        let t325 = t324 * t112;
        let t327 = 1.0 / t19 / t143;
        let t328 = sigma[ip] * t327;
        let t336 = 1.0 / t168 / t114;
        let t337 = t224 * t224;
        let t338 = t336 * t337;
        let t345 = t67 * t57;
        let t349 = t65 * t67;
        let t350 = t174 * t180;
        let t354 = t179 * t84;
        let t355 = 1.0 / t354;
        let t356 = t72 * t355;
        let t357 = t199 * t199;
        let t358 = t357 * t200;
        let t363 = 1.0 / t73 / t70 * t53;
        let t364 = t78 * t66;
        let t365 = t364 * t34;
        let t366 = t363 * t365;
        let t368 = t184 * t57;
        let t369 = t183 * t368;
        let t371 = t65 * t345;
        let t373 = 1.0/f64::sqrt(t70);
        let t374 = t373 * t53;
        let t375 = t374 * t365;
        let t377 = t191 * t368;
        let t380 = t79 * t66 * t34;
        let t382 = -0.42198333333333333333e0 * t366 + 0.84396666666666666666e0 * t369 + 0.39862222222222222223e0 * t371 + 0.68258333333333333333e-1 * t375 + 0.13651666666666666667e0 * t377 + 0.13692777777777777778e0 * t380;
        let t383 = t382 * t200;
        let t386 = t179 * t179;
        let t387 = 1.0 / t386;
        let t388 = t72 * t387;
        let t389 = t87 * t87;
        let t390 = 1.0 / t389;
        let t391 = t357 * t390;
        let t398 = t97 * t65;
        let t402 = t210 * t104;
        let t403 = 1.0 / t402;
        let t404 = t216 * t216;
        let t406 = t403 * t404 * t218;
        let t415 = -0.57538888888888888889e0 * t366 + 0.11507777777777777778e1 * t369 + 0.40256666666666666667e0 * t371 + 0.366775e-1 * t375 + 0.73355e-1 * t377 + 0.137975e0 * t380;
        let t417 = t211 * t415 * t218;
        let t420 = t210 * t210;
        let t421 = 1.0 / t420;
        let t422 = t421 * t404;
        let t423 = t107 * t107;
        let t424 = 1.0 / t423;
        let t425 = t422 * t424;
        let t428 = -0.14764627977777777777e-2 * t65 * t345 * t88 - 0.35616666666666666666e-1 * t349 * t350 * t201 - 2.0 * t356 * t358 + 1.0 * t181 * t383 + 0.16081979498692535067e2 * t388 * t391 + 0.24415263074675393405e-3 * t204 * t184 * t57 * t108 + 0.10843581300301739842e-1 * t398 * t175 * t219 + 0.11696447245269292414e1 * t209 * t406 - 0.5848223622634646207e0 * t209 * t417 - 0.17315859105681463759e2 * t209 * t425;
        let t430 = -35.0 / 216.0 * t54 * t59 * t327 + param_gammac * t428;
        let t431 = t169 * t430;
        let t435 = -35.0 / 216.0 * t55 * t328 * t115 - 7.0 / 72.0 * t55 * t164 * t225 - t55 * t58 * t338 / 24.0 + t55 * t58 * t431 / 48.0;
        let t436 = t161 * t435;
        let t437 = t436 * t112;
        let t438 = t230 * t222;
        let t440 = t119 * t428;
        let tv2rho20 = 4.0 * t159 + 2.0 * t231 + 2.0 * t232 + rho[ip] * (2.0 * t320 + t325 + t437 + 2.0 * t438 + t440);
        v2rho2[ip] += tv2rho20;
        let t446 = t153 * t247;
        let t454 = t28 * t146;
        let t455 = t148 * sigma[ip];
        let t459 = t305 * rho[ip];
        let t460 = 1.0 / t459;
        let t465 = -t27 * t29 * t130 * t39 / 9.0 + t140 * t454 * t455 / 36.0 - t303 * t141 * t460 * t310 / 108.0;
        let t470 = piecewise3(t3, 0.0, -t125 * t279 * t247 / 8.0 - 3.0 / 8.0 * t125 * t284 * t446 - 3.0 / 8.0 * t125 * t127 * t465);
        let t471 = t1 * t470;
        let t473 = t322 * t229;
        let t474 = t268 * t112;
        let t475 = t473 * t474;
        let t480 = t57 * t169;
        let t485 = 1.0 / t32 / t240;
        let t486 = sigma[ip] * t485;
        let t490 = t336 * t224;
        let t494 = 7.0 / 144.0 * t54 * t5 * t163 * t115 + t55 * t480 * t224 / 48.0 + 7.0 / 1152.0 * t261 * t486 * t169 + t261 * t264 * t490 / 384.0;
        let t495 = t161 * t494;
        let t496 = t495 * t112;
        let t497 = t269 * t222;
        let tv2rhosigma0 = t253 + t270 + rho[ip] * (2.0 * t471 + t475 + t496 + t497);
        v2rhosigma[ip] += tv2rhosigma0;
        let t500 = t247 * t247;
        let t507 = 1.0 / t305;
        let t508 = sigma[ip] * t507;
        let t512 = -t140 * t28 * t242 * t148 / 144.0 + t303 * t508 * t310 / 288.0;
        let t517 = piecewise3(t3, 0.0, -3.0 / 8.0 * t125 * t127 * t512 - 3.0 / 8.0 * t125 * t284 * t500);
        let t519 = 2.0 * t1 * t517;
        let t520 = t268 * t268;
        let t521 = t322 * t520;
        let t522 = t521 * t112;
        let t528 = t258 * t52 * M_PI;
        let t529 = 1.0 / t293;
        let t534 = -t259 * t260 * t263 * t169 / 384.0 - t528 * sigma[ip] * t529 * t336 / 6144.0;
        let t535 = t161 * t534;
        let t536 = t535 * t112;
        let tv2sigma20 = rho[ip] * (t519 + t522 + t536);
        v2sigma2[ip] += tv2sigma20;
    }
}
