//! MGGA_X_TAU_HCTH fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tau_hcth.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_tau_hcth_fxc_unpol(
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
    param_cx_local_1: f64,
    param_cx_local_2: f64,
    param_cx_local_3: f64,
    param_cx_nlocal_1: f64,
    param_cx_nlocal_2: f64,
    param_cx_nlocal_3: f64,
    param_cx_nlocal_0: f64,
    param_cx_local_0: f64,
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
        let t22 = param_cx_local_1;
        let t23 = t22 * sigma[ip];
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = rho[ip] * rho[ip];
        let t27 = t19 * t19;
        let t29 = 1.0 / t27 / t26;
        let t30 = t25 * t29;
        let t34 = 1.0 + 0.004 * sigma[ip] * t25 * t29;
        let t35 = 1.0 / t34;
        let t36 = t30 * t35;
        let t39 = param_cx_local_2;
        let t40 = sigma[ip] * sigma[ip];
        let t41 = t39 * t40;
        let t42 = t26 * t26;
        let t43 = t42 * rho[ip];
        let t45 = 1.0 / t19 / t43;
        let t46 = t24 * t45;
        let t47 = t34 * t34;
        let t48 = 1.0 / t47;
        let t49 = t46 * t48;
        let t52 = param_cx_local_3;
        let t53 = t40 * sigma[ip];
        let t54 = t52 * t53;
        let t55 = t42 * t42;
        let t56 = 1.0 / t55;
        let t57 = t47 * t34;
        let t58 = 1.0 / t57;
        let t59 = t56 * t58;
        let t63 = param_cx_nlocal_1;
        let t64 = t63 * sigma[ip];
        let t67 = param_cx_nlocal_2;
        let t68 = t67 * t40;
        let t71 = param_cx_nlocal_3;
        let t72 = t71 * t53;
        let t75 = param_cx_nlocal_0 + 0.004 * t64 * t36 + 3.2e-05 * t68 * t49 + 2.56e-07 * t72 * t59;
        let t76 = M_CBRT6;
        let t77 = t76 * t76;
        let t78 = M_PI * M_PI;
        let t79 = pow_1_3(t78);
        let t80 = t79 * t79;
        let t82 = 3.0 / 10.0 * t77 * t80;
        let t83 = tau[ip] * t25;
        let t85 = 1.0 / t27 / rho[ip];
        let t86 = t83 * t85;
        let t87 = t82 - t86;
        let t88 = t82 + t86;
        let t89 = 1.0 / t88;
        let t91 = t87 * t87;
        let t92 = t91 * t87;
        let t93 = t88 * t88;
        let t94 = t93 * t88;
        let t95 = 1.0 / t94;
        let t98 = t91 * t91;
        let t99 = t98 * t87;
        let t100 = t93 * t93;
        let t102 = 1.0 / t100 / t88;
        let t104 = t99 * t102 + t87 * t89 - 2.0 * t92 * t95;
        let t106 = param_cx_local_0 + 0.004 * t23 * t36 + 3.2e-05 * t41 * t49 + 2.56e-07 * t54 * t59 + t75 * t104;
        let t110 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t106);
        let tzk0 = 2.0 * t110;
        zk[ip] += tzk0;
        let t111 = 1.0 / t27;
        let t112 = t18 * t111;
        let t116 = t26 * rho[ip];
        let t118 = 1.0 / t27 / t116;
        let t119 = t25 * t118;
        let t120 = t119 * t35;
        let t123 = t22 * t40;
        let t124 = t42 * t26;
        let t126 = 1.0 / t19 / t124;
        let t127 = t24 * t126;
        let t128 = t127 * t48;
        let t133 = t39 * t53;
        let t134 = t55 * rho[ip];
        let t135 = 1.0 / t134;
        let t136 = t135 * t58;
        let t141 = t40 * t40;
        let t142 = t52 * t141;
        let t143 = t55 * t116;
        let t145 = 1.0 / t27 / t143;
        let t146 = t47 * t47;
        let t147 = 1.0 / t146;
        let t149 = t145 * t147 * t25;
        let t154 = t63 * t40;
        let t159 = t67 * t53;
        let t164 = t71 * t141;
        let t167 = -0.010666666666666666 * t64 * t120 + 8.533333333333334e-05 * t154 * t128 - 0.00017066666666666668 * t68 * t128 + 1.3653333333333333e-06 * t159 * t136 - 2.048e-06 * t72 * t136 + 8.192e-09 * t164 * t149;
        let t172 = 1.0 / t93;
        let t173 = t87 * t172;
        let t174 = t83 * t29;
        let t177 = t91 * t95;
        let t180 = 1.0 / t100;
        let t181 = t92 * t180;
        let t184 = t98 * t102;
        let t188 = 1.0 / t100 / t93;
        let t189 = t99 * t188;
        let t192 = 5.0 / 3.0 * t83 * t29 * t89 + 5.0 / 3.0 * t173 * t174 - 10.0 * t177 * t174 - 10.0 * t181 * t174 + 25.0 / 3.0 * t184 * t174 + 25.0 / 3.0 * t189 * t174;
        let t194 = -0.010666666666666666 * t23 * t120 + 8.533333333333334e-05 * t123 * t128 - 0.00017066666666666668 * t41 * t128 + 1.3653333333333333e-06 * t133 * t136 - 2.048e-06 * t54 * t136 + 8.192e-09 * t142 * t149 + t167 * t104 + t75 * t192;
        let t199 = piecewise3(t3, 0.0, -t7 * t112 * t106 / 8.0 - 3.0 / 8.0 * t7 * t20 * t194);
        let tvrho0 = 2.0 * rho[ip] * t199 + 2.0 * t110;
        vrho[ip] += tvrho0;
        let t202 = t22 * t25;
        let t203 = t29 * t35;
        let t208 = t39 * sigma[ip];
        let t213 = t52 * t40;
        let t216 = t55 * t26;
        let t218 = 1.0 / t27 / t216;
        let t220 = t218 * t147 * t25;
        let t223 = t63 * t25;
        let t228 = t67 * sigma[ip];
        let t233 = t71 * t40;
        let t238 = 0.004 * t223 * t203 - 3.2e-05 * t64 * t49 + 6.4e-05 * t228 * t49 - 5.12e-07 * t68 * t59 + 7.68e-07 * t233 * t59 - 3.072e-09 * t72 * t220;
        let t240 = 0.004 * t202 * t203 - 3.2e-05 * t23 * t49 + 6.4e-05 * t208 * t49 - 5.12e-07 * t41 * t59 + 7.68e-07 * t213 * t59 - 3.072e-09 * t54 * t220 + t238 * t104;
        let t244 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t240);
        let tvsigma0 = 2.0 * rho[ip] * t244;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t246 = t7 * t18;
        let t247 = t19 * t75;
        let t248 = t25 * t85;
        let t259 = -t173 * t248 + 6.0 * t177 * t248 + 6.0 * t181 * t248 - 5.0 * t184 * t248 - 5.0 * t189 * t248 - t248 * t89;
        let t263 = piecewise3(t3, 0.0, -3.0 / 8.0 * t246 * t247 * t259);
        let tvtau0 = 2.0 * rho[ip] * t263;
        vtau[ip] += tvtau0;
        let t266 = t18 * t85;
        let t274 = 1.0 / t27 / t42;
        let t275 = t25 * t274;
        let t276 = t275 * t35;
        let t279 = t42 * t116;
        let t281 = 1.0 / t19 / t279;
        let t283 = t24 * t281 * t48;
        let t286 = t22 * t53;
        let t287 = 1.0 / t216;
        let t288 = t287 * t58;
        let t295 = t39 * t141;
        let t296 = t55 * t42;
        let t298 = 1.0 / t27 / t296;
        let t300 = t298 * t147 * t25;
        let t307 = t141 * sigma[ip];
        let t308 = t52 * t307;
        let t313 = 1.0 / t146 / t34;
        let t315 = 1.0 / t19 / t55 / t279 * t313 * t24;
        let t322 = t63 * t53;
        let t329 = t67 * t141;
        let t336 = t71 * t307;
        let t339 = 0.03911111111111111 * t64 * t276 - 0.000768 * t154 * t283 + 3.6408888888888887e-06 * t322 * t288 + 0.0010808888888888888 * t68 * t283 - 1.956977777777778e-05 * t159 * t288 + 4.369066666666667e-08 * t329 * t300 + 1.8432e-05 * t72 * t288 - 1.6110933333333333e-07 * t164 * t300 + 6.990506666666666e-10 * t336 * t315;
        let t346 = tau[ip] * tau[ip];
        let t347 = t346 * t24;
        let t351 = t87 * t95;
        let t352 = t347 * t45;
        let t355 = t83 * t118;
        let t358 = t91 * t180;
        let t363 = t92 * t102;
        let t368 = t98 * t188;
        let t374 = 1.0 / t100 / t94;
        let t375 = t99 * t374;
        let t380 = -40.0 / 9.0 * t83 * t118 * t89 + 100.0 / 9.0 * t347 * t45 * t172 - 500.0 / 9.0 * t351 * t352 - 40.0 / 9.0 * t173 * t355 - 200.0 * t358 * t352 + 80.0 / 3.0 * t177 * t355 - 200.0 / 9.0 * t363 * t352 + 80.0 / 3.0 * t181 * t355 + 2500.0 / 9.0 * t368 * t352 - 200.0 / 9.0 * t184 * t355 + 500.0 / 3.0 * t375 * t352 - 200.0 / 9.0 * t189 * t355;
        let t382 = 0.03911111111111111 * t23 * t276 - 0.000768 * t123 * t283 + 3.6408888888888887e-06 * t286 * t288 + 0.0010808888888888888 * t41 * t283 - 1.956977777777778e-05 * t133 * t288 + 4.369066666666667e-08 * t295 * t300 + 1.8432e-05 * t54 * t288 - 1.6110933333333333e-07 * t142 * t300 + 6.990506666666666e-10 * t308 * t315 + t339 * t104 + 2.0 * t167 * t192 + t75 * t380;
        let t387 = piecewise3(t3, 0.0, t7 * t266 * t106 / 12.0 - t7 * t112 * t194 / 4.0 - 3.0 / 8.0 * t7 * t20 * t382);
        let tv2rho20 = 2.0 * rho[ip] * t387 + 4.0 * t199;
        v2rho2[ip] += tv2rho20;
        let t393 = t118 * t35;
        let t396 = t22 * t24;
        let t397 = t126 * t48;
        let t398 = t397 * sigma[ip];
        let t413 = t55 * t124;
        let t417 = 1.0 / t19 / t413 * t313 * t24;
        let t422 = t63 * t24;
        let t439 = -0.010666666666666666 * t223 * t393 + 0.000256 * t422 * t398 - 1.3653333333333333e-06 * t154 * t136 - 0.00034133333333333335 * t228 * t128 + 6.826666666666667e-06 * t68 * t136 - 1.6384e-08 * t159 * t149 - 6.144e-06 * t233 * t136 + 5.7344e-08 * t72 * t149 - 2.62144e-10 * t164 * t417;
        let t442 = -0.010666666666666666 * t202 * t393 + 0.000256 * t396 * t398 - 1.3653333333333333e-06 * t123 * t136 - 0.00034133333333333335 * t208 * t128 + 6.826666666666667e-06 * t41 * t136 - 1.6384e-08 * t133 * t149 - 6.144e-06 * t213 * t136 + 5.7344e-08 * t54 * t149 - 2.62144e-10 * t142 * t417 + t439 * t104 + t238 * t192;
        let t447 = piecewise3(t3, 0.0, -t7 * t112 * t240 / 8.0 - 3.0 / 8.0 * t7 * t20 * t442);
        let tv2rhosigma0 = 2.0 * rho[ip] * t447 + 2.0 * t244;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t450 = t111 * t75;
        let t454 = t19 * t167;
        let t462 = t24 / t19 / t42;
        let t463 = t172 * tau[ip];
        let t466 = t462 * tau[ip];
        let t487 = 5.0 / 3.0 * t30 * t89 - 20.0 / 3.0 * t462 * t463 + 100.0 / 3.0 * t351 * t466 + 5.0 / 3.0 * t173 * t30 + 120.0 * t358 * t466 - 10.0 * t177 * t30 + 40.0 / 3.0 * t363 * t466 - 10.0 * t181 * t30 - 500.0 / 3.0 * t368 * t466 + 25.0 / 3.0 * t184 * t30 - 100.0 * t375 * t466 + 25.0 / 3.0 * t189 * t30;
        let t492 = piecewise3(t3, 0.0, -t246 * t450 * t259 / 8.0 - 3.0 / 8.0 * t246 * t454 * t259 - 3.0 / 8.0 * t246 * t247 * t487);
        let tv2rhotau0 = 2.0 * rho[ip] * t492 + 2.0 * t263;
        v2rhotau[ip] += tv2rhotau0;
        let t495 = t45 * t48;
        let t500 = t39 * t24;
        let t507 = t52 * sigma[ip];
        let t512 = t55 * t43;
        let t516 = 1.0 / t19 / t512 * t313 * t24;
        let t523 = t67 * t24;
        let t530 = t71 * sigma[ip];
        let t537 = -6.4e-05 * t422 * t495 + 5.12e-07 * t64 * t59 + 6.4e-05 * t523 * t495 - 2.048e-06 * t228 * t59 + 6.144e-09 * t68 * t220 + 1.536e-06 * t530 * t59 - 1.8432e-08 * t233 * t220 + 9.8304e-11 * t72 * t516;
        let t539 = -6.4e-05 * t396 * t495 + 5.12e-07 * t23 * t59 + 6.4e-05 * t500 * t495 - 2.048e-06 * t208 * t59 + 6.144e-09 * t41 * t220 + 1.536e-06 * t507 * t59 - 1.8432e-08 * t213 * t220 + 9.8304e-11 * t54 * t516 + t537 * t104;
        let t543 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t539);
        let tv2sigma20 = 2.0 * rho[ip] * t543;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t545 = t19 * t238;
        let t549 = piecewise3(t3, 0.0, -3.0 / 8.0 * t246 * t545 * t259);
        let tv2sigmatau0 = 2.0 * rho[ip] * t549;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t553 = t24 / t19 / t116;
        let t566 = 4.0 * t553 * t172 - 20.0 * t351 * t553 - 72.0 * t358 * t553 - 8.0 * t363 * t553 + 100.0 * t368 * t553 + 60.0 * t375 * t553;
        let t570 = piecewise3(t3, 0.0, -3.0 / 8.0 * t246 * t247 * t566);
        let tv2tau20 = 2.0 * rho[ip] * t570;
        v2tau2[ip] += tv2tau20;
    }
}
