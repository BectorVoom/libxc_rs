//! GGA_X_S12 lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_s12.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_s12_lxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    param_bx: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t3 / t4 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t19 * param_bx;
        let t21 = param_C * sigma[ip];
        let t22 = M_CBRT2;
        let t23 = t22 * t22;
        let t24 = rho[ip] * rho[ip];
        let t25 = t19 * t19;
        let t27 = 1.0 / t25 / t24;
        let t28 = t23 * t27;
        let t30 = sigma[ip] * sigma[ip];
        let t31 = param_D * t30;
        let t32 = t24 * t24;
        let t33 = t32 * rho[ip];
        let t35 = 1.0 / t19 / t33;
        let t36 = t22 * t35;
        let t39 = t21 * t28 + 2.0 * t31 * t36 + 1.0;
        let t42 = param_B * (1.0 - 1.0 / t39);
        let t43 = param_E * sigma[ip];
        let t45 = t43 * t28 + 1.0;
        let t47 = 1.0 - 1.0 / t45;
        let t49 = t42 * t47 + param_A;
        let t53 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t20 * t49);
        let tzk0 = 2.0 * t53;
        zk[ip] += tzk0;
        let t55 = 1.0 / t25 * param_bx;
        let t59 = t39 * t39;
        let t61 = param_B / t59;
        let t62 = t24 * rho[ip];
        let t64 = 1.0 / t25 / t62;
        let t65 = t23 * t64;
        let t68 = t32 * t24;
        let t70 = 1.0 / t19 / t68;
        let t71 = t22 * t70;
        let t74 = -8.0 / 3.0 * t21 * t65 - 32.0 / 3.0 * t31 * t71;
        let t75 = t74 * t47;
        let t77 = t45 * t45;
        let t78 = 1.0 / t77;
        let t79 = t42 * t78;
        let t80 = t43 * t65;
        let t83 = t61 * t75 - 8.0 / 3.0 * t79 * t80;
        let t88 = piecewise3(t2, 0.0, -t18 * t55 * t49 / 8.0 - 3.0 / 8.0 * t18 * t20 * t83);
        let tvrho0 = 2.0 * rho[ip] * t88 + 2.0 * t53;
        vrho[ip] += tvrho0;
        let t91 = param_C * t23;
        let t93 = param_D * sigma[ip];
        let t96 = t91 * t27 + 4.0 * t93 * t36;
        let t97 = t96 * t47;
        let t99 = param_E * t23;
        let t102 = t79 * t99 * t27 + t61 * t97;
        let t106 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t20 * t102);
        let tvsigma0 = 2.0 * rho[ip] * t106;
        vsigma[ip] += tvsigma0;
        let t111 = 1.0 / t25 / rho[ip] * param_bx;
        let t120 = param_B / t59 / t39;
        let t121 = t74 * t74;
        let t122 = t121 * t47;
        let t126 = 1.0 / t25 / t32;
        let t127 = t23 * t126;
        let t132 = 1.0 / t19 / t32 / t62;
        let t133 = t22 * t132;
        let t136 = 88.0 / 9.0 * t21 * t127 + 608.0 / 9.0 * t31 * t133;
        let t137 = t136 * t47;
        let t139 = t74 * t78;
        let t140 = t61 * t139;
        let t144 = 1.0 / t77 / t45;
        let t145 = t42 * t144;
        let t146 = param_E * param_E;
        let t147 = t146 * t30;
        let t148 = t147 * t133;
        let t151 = t43 * t127;
        let t154 = -2.0 * t120 * t122 + t61 * t137 - 16.0 / 3.0 * t140 * t80 - 256.0 / 9.0 * t145 * t148 + 88.0 / 9.0 * t79 * t151;
        let t159 = piecewise3(t2, 0.0, t18 * t111 * t49 / 12.0 - t18 * t55 * t83 / 4.0 - 3.0 / 8.0 * t18 * t20 * t154);
        let tv2rho20 = 2.0 * rho[ip] * t159 + 4.0 * t88;
        v2rho2[ip] += tv2rho20;
        let t172 = -8.0 / 3.0 * t91 * t64 - 64.0 / 3.0 * t93 * t71;
        let t173 = t172 * t47;
        let t175 = t96 * t78;
        let t176 = t61 * t175;
        let t179 = t61 * t74;
        let t180 = t78 * param_E;
        let t181 = t180 * t28;
        let t183 = t146 * t22;
        let t185 = t183 * t70 * sigma[ip];
        let t191 = -2.0 * t120 * t97 * t74 + t61 * t173 - 8.0 / 3.0 * t176 * t80 + t179 * t181 + 32.0 / 3.0 * t145 * t185 - 8.0 / 3.0 * t79 * t99 * t64;
        let t196 = piecewise3(t2, 0.0, -t18 * t55 * t102 / 8.0 - 3.0 / 8.0 * t18 * t20 * t191);
        let tv2rhosigma0 = 2.0 * rho[ip] * t196 + 2.0 * t106;
        v2rhosigma[ip] += tv2rhosigma0;
        let t199 = t96 * t96;
        let t200 = t199 * t47;
        let t203 = t61 * param_D;
        let t207 = t61 * t96;
        let t213 = -4.0 * t145 * t183 * t35 + 4.0 * t203 * t36 * t47 - 2.0 * t120 * t200 + 2.0 * t207 * t181;
        let t217 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t20 * t213);
        let tv2sigma20 = 2.0 * rho[ip] * t217;
        v2sigma2[ip] += tv2sigma20;
        let t220 = t27 * param_bx;
        let t230 = t59 * t59;
        let t232 = param_B / t230;
        let t233 = t121 * t74;
        let t237 = t75 * t136;
        let t241 = t120 * t121 * t78;
        let t245 = 1.0 / t25 / t33;
        let t246 = t23 * t245;
        let t249 = t32 * t32;
        let t251 = 1.0 / t19 / t249;
        let t252 = t22 * t251;
        let t255 = -1232.0 / 27.0 * t21 * t246 - 13376.0 / 27.0 * t31 * t252;
        let t259 = t61 * t136 * t78;
        let t263 = t61 * t74 * t144;
        let t268 = t77 * t77;
        let t269 = 1.0 / t268;
        let t270 = t42 * t269;
        let t271 = t146 * param_E;
        let t272 = t30 * sigma[ip];
        let t273 = t271 * t272;
        let t274 = t249 * t62;
        let t275 = 1.0 / t274;
        let t279 = t147 * t252;
        let t282 = t43 * t246;
        let t285 = 6.0 * t232 * t233 * t47 - 6.0 * t120 * t237 + 16.0 * t241 * t80 + t61 * t255 * t47 - 8.0 * t259 * t80 - 256.0 / 3.0 * t263 * t148 + 88.0 / 3.0 * t140 * t151 - 4096.0 / 9.0 * t270 * t273 * t275 + 2816.0 / 9.0 * t145 * t279 - 1232.0 / 27.0 * t79 * t282;
        let t290 = piecewise3(t2, 0.0, -5.0 / 36.0 * t18 * t220 * t49 + t18 * t111 * t83 / 4.0 - 3.0 / 8.0 * t18 * t55 * t154 - 3.0 / 8.0 * t18 * t20 * t285);
        let tv3rho30 = 2.0 * rho[ip] * t290 + 6.0 * t159;
        v3rho3[ip] += tv3rho30;
        let t303 = t173 * t74;
        let t306 = t120 * t175;
        let t308 = t43 * t65 * t74;
        let t318 = 88.0 / 9.0 * t91 * t126 + 1216.0 / 9.0 * t93 * t133;
        let t319 = t318 * t47;
        let t321 = t172 * t78;
        let t322 = t61 * t321;
        let t325 = t96 * t144;
        let t326 = t61 * t325;
        let t331 = t120 * t121;
        let t334 = t61 * t136;
        let t338 = t180 * t65;
        let t341 = t249 * t24;
        let t342 = 1.0 / t341;
        let t343 = t271 * t342;
        let t348 = t183 * t132 * sigma[ip];
        let t354 = 6.0 * t232 * t97 * t121 - 4.0 * t120 * t303 + 32.0 / 3.0 * t306 * t308 - 2.0 * t120 * t97 * t136 + t61 * t319 - 16.0 / 3.0 * t322 * t80 - 256.0 / 9.0 * t326 * t148 + 88.0 / 9.0 * t176 * t151 - 2.0 * t331 * t181 + t334 * t181 + 64.0 / 3.0 * t263 * t185 - 16.0 / 3.0 * t179 * t338 + 512.0 / 3.0 * t270 * t343 * t30 - 96.0 * t145 * t348 + 88.0 / 9.0 * t79 * t99 * t126;
        let t359 = piecewise3(t2, 0.0, t18 * t111 * t102 / 12.0 - t18 * t55 * t191 / 4.0 - 3.0 / 8.0 * t18 * t20 * t354);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t359 + 4.0 * t196;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t371 = t199 * t78;
        let t372 = t120 * t371;
        let t375 = t120 * param_D;
        let t382 = t249 * rho[ip];
        let t383 = 1.0 / t382;
        let t384 = t383 * t78;
        let t389 = t99 * t27 * t74;
        let t392 = t61 * t172;
        let t399 = t144 * t146;
        let t400 = t399 * t36;
        let t410 = 6.0 * t232 * t200 * t74 - 4.0 * t120 * t97 * t172 + 16.0 / 3.0 * t372 * t80 - 8.0 * t375 * t36 * t75 - 64.0 / 3.0 * t203 * t71 * t47 - 64.0 / 3.0 * t203 * t384 * t43 - 4.0 * t306 * t389 + 2.0 * t392 * t181 + 64.0 / 3.0 * t326 * t185 - 16.0 / 3.0 * t207 * t338 - 4.0 * t179 * t400 - 64.0 * t270 * t271 * t383 * sigma[ip] + 64.0 / 3.0 * t145 * t183 * t70;
        let t415 = piecewise3(t2, 0.0, -t18 * t55 * t213 / 8.0 - 3.0 / 8.0 * t18 * t20 * t410);
        let tv3rhosigma20 = 2.0 * rho[ip] * t415 + 2.0 * t217;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t418 = t199 * t96;
        let t419 = t418 * t47;
        let t422 = t120 * t96;
        let t423 = t47 * param_D;
        let t424 = t423 * t36;
        let t427 = t120 * t199;
        let t430 = 1.0 / t249;
        let t431 = t430 * t78;
        let t437 = t269 * t271;
        let t438 = t437 * t430;
        let t441 = 24.0 * t203 * t431 * param_E - 6.0 * t427 * t181 - 12.0 * t207 * t400 + 6.0 * t232 * t419 + 24.0 * t42 * t438 - 24.0 * t422 * t424;
        let t445 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t20 * t441);
        let tv3sigma30 = 2.0 * rho[ip] * t445;
        v3sigma3[ip] += tv3sigma30;
        let t463 = t23 / t25 / t68;
        let t468 = t22 / t19 / t382;
        let t485 = t120 * t121 * t144;
        let t493 = t61 * t136 * t144;
        let t497 = t437 * t272 * t275;
        let t500 = t249 * t32;
        let t507 = t42 / t268 / t45;
        let t508 = t146 * t146;
        let t509 = t30 * t30;
        let t520 = param_B / t230 / t39;
        let t521 = t121 * t121;
        let t528 = t136 * t136;
        let t535 = t120 * t139;
        let t537 = t43 * t65 * t136;
        let t548 = t61 * (20944.0 / 81.0 * t21 * t463 + 334400.0 / 81.0 * t31 * t468) * t47 - 250624.0 / 81.0 * t145 * t147 * t468 + 20944.0 / 81.0 * t79 * t43 * t463 - 64.0 * t232 * t233 * t78 * t80 + 1024.0 / 3.0 * t485 * t148 - 32.0 / 3.0 * t61 * t255 * t78 * t80 - 512.0 / 3.0 * t493 * t148 - 16384.0 / 9.0 * t179 * t497 + 90112.0 / 9.0 * t270 * t273 / t500 - 131072.0 / 27.0 * t507 * t508 * t509 / t25 / t249 / t68 * t23 - 24.0 * t520 * t521 * t47 + 36.0 * t232 * t122 * t136 - 6.0 * t120 * t528 * t47 - 8.0 * t120 * t75 * t255 + 64.0 * t535 * t537 - 352.0 / 3.0 * t241 * t151 + 176.0 / 3.0 * t259 * t151 + 11264.0 / 9.0 * t263 * t279 - 4928.0 / 27.0 * t140 * t282;
        let t553 = piecewise3(t2, 0.0, 10.0 / 27.0 * t18 * t64 * param_bx * t49 - 5.0 / 9.0 * t18 * t220 * t83 + t18 * t111 * t154 / 2.0 - t18 * t55 * t285 / 2.0 - 3.0 / 8.0 * t18 * t20 * t548);
        let tv4rho40 = 2.0 * rho[ip] * t553 + 8.0 * t290;
        v4rho4[ip] += tv4rho40;
        let t590 = t180 * t127;
        let t607 = t437 * t342 * t30;
        let t620 = t61 * (-1232.0 / 27.0 * t91 * t245 - 26752.0 / 27.0 * t93 * t252) * t47 - 2.0 * t120 * t97 * t255 - 24.0 * t520 * t97 * t233 + 18.0 * t232 * t173 * t121 - 6.0 * t120 * t319 * t74 - 6.0 * t120 * t173 * t136 - 8.0 * t334 * t338 + 88.0 / 3.0 * t179 * t590 + 21824.0 / 27.0 * t145 * t183 * t251 * sigma[ip] + 6.0 * t232 * t233 * t181 + 16.0 * t331 * t338 - 4096.0 / 9.0 * t207 * t497 + t61 * t255 * t181 + 512.0 * t179 * t607 + 16384.0 / 9.0 * t507 * t508 / t25 / t249 / t33 * t272 * t23 + 32.0 * t493 * t185;
        let t626 = t61 * t172 * t144;
        let t630 = t99 * t27 * t136;
        let t647 = t232 * t175;
        let t652 = t120 * t321;
        let t657 = t120 * t325;
        let t662 = t232 * t96;
        let t672 = -8.0 * t61 * t318 * t78 * t80 - 256.0 / 3.0 * t626 * t148 - 6.0 * t535 * t630 - 64.0 * t485 * t185 + 88.0 / 3.0 * t322 * t151 + 2816.0 / 9.0 * t326 * t279 - 1232.0 / 27.0 * t176 * t282 - 288.0 * t263 * t348 - 176.0 / 3.0 * t306 * t43 * t127 * t74 - 48.0 * t647 * t43 * t65 * t121 + 32.0 * t652 * t308 + 16.0 * t306 * t537 + 512.0 / 3.0 * t657 * t147 * t133 * t74 + 18.0 * t662 * t237 - 9728.0 / 3.0 * t270 * t271 * t275 * t30 - 1232.0 / 27.0 * t79 * t99 * t245;
        let t678 = piecewise3(t2, 0.0, -5.0 / 36.0 * t18 * t220 * t102 + t18 * t111 * t191 / 4.0 - 3.0 / 8.0 * t18 * t55 * t354 - 3.0 / 8.0 * t18 * t20 * (t620 + t672));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t678 + 6.0 * t359;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t688 = t172 * t172;
        let t726 = t437 * t383 * sigma[ip];
        let t730 = 1.0 / t25 / t500;
        let t732 = t30 * t23;
        let t739 = t399 * t71;
        let t742 = -4.0 * t120 * t688 * t47 + 6.0 * t232 * t200 * t136 - 4.0 * t120 * t97 * t318 - 24.0 * t520 * t200 * t121 - 32.0 / 3.0 * t392 * t338 + 176.0 / 9.0 * t207 * t590 + 8.0 * t331 * t400 + 24.0 * t232 * param_D * t36 * t122 + 256.0 / 3.0 * t375 * t71 * t75 + 2752.0 / 9.0 * t203 * t342 * t78 * t43 + 2.0 * t61 * t318 * t181 + 1024.0 / 3.0 * t207 * t607 - 4.0 * t334 * t400 - 128.0 * t179 * t726 - 2048.0 / 3.0 * t507 * t508 * t730 * t732 - 8.0 * t375 * t36 * t137 + 128.0 / 3.0 * t179 * t739;
        let t755 = t120 * t199 * t144;
        let t758 = param_D * t383;
        let t781 = t232 * t371;
        let t799 = -1024.0 / 9.0 * t61 * param_D * t730 * t399 * t732 - 8.0 * t652 * t389 - 4.0 * t306 * t630 + 128.0 / 3.0 * t626 * t185 + 512.0 / 9.0 * t755 * t148 + 256.0 / 3.0 * t120 * t758 * t180 * sigma[ip] * t74 - 176.0 / 9.0 * t372 * t151 + 12.0 * t647 * t99 * t27 * t121 + 64.0 / 3.0 * t306 * t99 * t64 * t74 - 192.0 * t326 * t348 - 256.0 / 3.0 * t657 * t183 * t70 * t74 * sigma[ip] - 32.0 * t781 * t308 + 64.0 / 3.0 * t306 * t43 * t65 * t172 + 24.0 * t662 * t303 + 1216.0 / 9.0 * t203 * t133 * t47 + 2752.0 / 3.0 * t270 * t343 * sigma[ip] - 1216.0 / 9.0 * t145 * t183 * t132;
        let t805 = piecewise3(t2, 0.0, t18 * t111 * t213 / 12.0 - t18 * t55 * t410 / 4.0 - 3.0 / 8.0 * t18 * t20 * (t742 + t799));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t805 + 4.0 * t415;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t823 = t35 * t74;
        let t855 = 1.0 / t25 / t274;
        let t858 = sigma[ip] * t23;
        let t880 = 128.0 * t61 * param_D * t855 * t399 * t858 - 48.0 * t375 * t431 * param_E * t74 + 256.0 * t507 * t508 * t855 * t858 + 24.0 * t657 * t183 * t823 - 192.0 * t203 * t384 * param_E - 192.0 * t42 * t437 * t383 + 24.0 * t179 * t438 - 192.0 * t207 * t726 + 64.0 * t207 * t739 + 16.0 * t427 * t338 - 12.0 * t392 * t400;
        let t886 = piecewise3(t2, 0.0, -t18 * t55 * t441 / 8.0 - 3.0 / 8.0 * t18 * t20 * (72.0 * t232 * t97 * param_D * t22 * t823 - 12.0 * t306 * t99 * t27 * t172 - 16.0 * t232 * t418 * t78 * t80 - 24.0 * t120 * t172 * t424 + 18.0 * t232 * t200 * t172 + 128.0 * t306 * t43 * t758 - 24.0 * t520 * t419 * t74 + 128.0 * t422 * t423 * t71 - 64.0 * t755 * t185 + 18.0 * t781 * t389 + t880));
        let tv4rhosigma30 = 2.0 * rho[ip] * t886 + 2.0 * t445;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t889 = t199 * t199;
        let t899 = param_D * param_D;
        let t902 = 1.0 / t25 / t341;
        let t928 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t20 * (-96.0 * t120 * t899 * t23 * t902 * t47 - 96.0 * t203 * t902 * t144 * t146 * t23 - 192.0 * t422 * t180 * t430 * param_D - 96.0 * t507 * t508 * t902 * t23 + 24.0 * t232 * t418 * t181 + 144.0 * t232 * t199 * t424 - 24.0 * t520 * t889 * t47 + 96.0 * t207 * t438 + 48.0 * t427 * t400));
        let tv4sigma40 = 2.0 * rho[ip] * t928;
        v4sigma4[ip] += tv4sigma40;
    }
}
