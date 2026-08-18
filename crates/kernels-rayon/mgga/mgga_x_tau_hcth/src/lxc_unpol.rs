//! MGGA_X_TAU_HCTH lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tau_hcth.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_tau_hcth_lxc_unpol(
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
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rho2lapl: &mut [f64],
    v3rho2tau: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3rhosigmalapl: &mut [f64],
    v3rhosigmatau: &mut [f64],
    v3rholapl2: &mut [f64],
    v3rholapltau: &mut [f64],
    v3rhotau2: &mut [f64],
    v3sigma3: &mut [f64],
    v3sigma2lapl: &mut [f64],
    v3sigma2tau: &mut [f64],
    v3sigmalapl2: &mut [f64],
    v3sigmalapltau: &mut [f64],
    v3sigmatau2: &mut [f64],
    v3lapl3: &mut [f64],
    v3lapl2tau: &mut [f64],
    v3lapltau2: &mut [f64],
    v3tau3: &mut [f64],
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho3lapl: &mut [f64],
    v4rho3tau: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rho2sigmalapl: &mut [f64],
    v4rho2sigmatau: &mut [f64],
    v4rho2lapl2: &mut [f64],
    v4rho2lapltau: &mut [f64],
    v4rho2tau2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4rhosigma2lapl: &mut [f64],
    v4rhosigma2tau: &mut [f64],
    v4rhosigmalapl2: &mut [f64],
    v4rhosigmalapltau: &mut [f64],
    v4rhosigmatau2: &mut [f64],
    v4rholapl3: &mut [f64],
    v4rholapl2tau: &mut [f64],
    v4rholapltau2: &mut [f64],
    v4rhotau3: &mut [f64],
    v4sigma4: &mut [f64],
    v4sigma3lapl: &mut [f64],
    v4sigma3tau: &mut [f64],
    v4sigma2lapl2: &mut [f64],
    v4sigma2lapltau: &mut [f64],
    v4sigma2tau2: &mut [f64],
    v4sigmalapl3: &mut [f64],
    v4sigmalapl2tau: &mut [f64],
    v4sigmalapltau2: &mut [f64],
    v4sigmatau3: &mut [f64],
    v4lapl4: &mut [f64],
    v4lapl3tau: &mut [f64],
    v4lapl2tau2: &mut [f64],
    v4lapltau3: &mut [f64],
    v4tau4: &mut [f64],
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
        let t573 = t18 * t29;
        let t584 = 1.0 / t27 / t43;
        let t586 = t25 * t584 * t35;
        let t590 = 1.0 / t19 / t55;
        let t592 = t24 * t590 * t48;
        let t595 = 1.0 / t143;
        let t596 = t595 * t58;
        let t599 = t22 * t141;
        let t601 = 1.0 / t27 / t512;
        let t603 = t601 * t147 * t25;
        let t612 = t39 * t307;
        let t613 = t55 * t55;
        let t617 = 1.0 / t19 / t613 * t313 * t24;
        let t626 = t141 * t40;
        let t627 = t52 * t626;
        let t628 = t613 * t116;
        let t631 = 1.0 / t146 / t47;
        let t632 = 1.0 / t628 * t631;
        let t641 = t63 * t141;
        let t650 = t67 * t307;
        let t659 = t71 * t626;
        let t662 = -0.18251851851851852 * t64 * t586 + 0.006466370370370371 * t154 * t592 - 6.917688888888889e-05 * t322 * t596 + 1.1650844444444444e-07 * t641 * t603 - 0.007926518518518519 * t68 * t592 + 0.0002418157037037037 * t159 * t596 - 1.179648e-06 * t329 * t603 + 3.7282702222222225e-09 * t650 * t617 - 0.00018432 * t72 * t596 + 2.630542222222222e-06 * t164 * t603 - 2.4466773333333333e-08 * t336 * t617 + 7.456540444444444e-11 * t659 * t632;
        let t668 = t83 * t274;
        let t673 = t347 * t126;
        let t693 = t100 * t100;
        let t694 = 1.0 / t693;
        let t695 = t99 * t694;
        let t696 = t346 * tau[ip];
        let t697 = t696 * t56;
        let t703 = t87 * t180;
        let t706 = t91 * t102;
        let t709 = t92 * t188;
        let t712 = t98 * t374;
        let t717 = 2200.0 / 27.0 * t189 * t668 + 440.0 / 27.0 * t173 * t668 + 1600.0 * t358 * t673 - 880.0 / 9.0 * t177 * t668 + 1600.0 / 9.0 * t363 * t673 - 880.0 / 9.0 * t181 * t668 - 20000.0 / 9.0 * t368 * t673 + 2200.0 / 27.0 * t184 * t668 - 4000.0 / 3.0 * t375 * t673 + 4000.0 / 9.0 * t351 * t673 + 440.0 / 27.0 * t83 * t274 * t89 + 35000.0 / 9.0 * t695 * t697 - 800.0 / 9.0 * t347 * t126 * t172 - 17000.0 / 9.0 * t703 * t697 - 26000.0 / 9.0 * t706 * t697 + 10000.0 / 3.0 * t709 * t697 + 25000.0 / 3.0 * t712 * t697 - 1000.0 / 9.0 * t697 * t95;
        let t719 = -0.18251851851851852 * t23 * t586 + 0.006466370370370371 * t123 * t592 - 6.917688888888889e-05 * t286 * t596 + 1.1650844444444444e-07 * t599 * t603 - 0.007926518518518519 * t41 * t592 + 0.0002418157037037037 * t133 * t596 - 1.179648e-06 * t295 * t603 + 3.7282702222222225e-09 * t612 * t617 - 0.00018432 * t54 * t596 + 2.630542222222222e-06 * t142 * t603 - 2.4466773333333333e-08 * t308 * t617 + 7.456540444444444e-11 * t627 * t632 + t662 * t104 + 3.0 * t339 * t192 + 3.0 * t167 * t380 + t75 * t717;
        let t724 = piecewise3(t3, 0.0, -5.0 / 36.0 * t7 * t573 * t106 + t7 * t266 * t194 / 4.0 - 3.0 / 8.0 * t7 * t112 * t382 - 3.0 / 8.0 * t7 * t20 * t719);
        let tv3rho30 = 2.0 * rho[ip] * t724 + 6.0 * t387;
        v3rho3[ip] += tv3rho30;
        let t734 = t274 * t35;
        let t737 = t281 * t48;
        let t738 = t737 * sigma[ip];
        let t741 = t22 * t287;
        let t742 = t58 * t40;
        let t761 = t613 * t26;
        let t763 = 1.0 / t761 * t631;
        let t770 = t63 * t287;
        let t791 = 0.03911111111111111 * t223 * t734 - 0.0018488888888888888 * t422 * t738 + 2.3210666666666668e-05 * t770 * t742 - 4.369066666666667e-08 * t322 * t300 + 0.0021617777777777777 * t228 * t283 - 7.600355555555555e-05 * t68 * t288 + 4.096e-07 * t159 * t300 - 1.3981013333333332e-09 * t329 * t315 + 5.5296e-05 * t233 * t288 - 8.656213333333333e-07 * t72 * t300 + 8.650752e-09 * t164 * t315 - 2.7962026666666667e-11 * t336 * t763;
        let t796 = 0.03911111111111111 * t202 * t734 - 0.0018488888888888888 * t396 * t738 + 2.3210666666666668e-05 * t741 * t742 - 4.369066666666667e-08 * t286 * t300 + 0.0021617777777777777 * t208 * t283 - 7.600355555555555e-05 * t41 * t288 + 4.096e-07 * t133 * t300 - 1.3981013333333332e-09 * t295 * t315 + 5.5296e-05 * t213 * t288 - 8.656213333333333e-07 * t54 * t300 + 8.650752e-09 * t142 * t315 - 2.7962026666666667e-11 * t308 * t763 + t791 * t104 + 2.0 * t439 * t192 + t238 * t380;
        let t801 = piecewise3(t3, 0.0, t7 * t266 * t240 / 12.0 - t7 * t112 * t442 / 4.0 - 3.0 / 8.0 * t7 * t20 * t796);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t801 + 4.0 * t447;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rho2lapl0 = 0.0;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t805 = t85 * t75;
        let t809 = t111 * t167;
        let t816 = t19 * t339;
        let t823 = 1.0 / t279;
        let t824 = t823 * t346;
        let t839 = t823 * t95;
        let t852 = t46 * tau[ip];
        let t863 = -7000.0 / 3.0 * t695 * t824 - 40.0 / 9.0 * t119 * t89 + 40.0 * t46 * t463 + 3400.0 / 3.0 * t703 * t824 + 5200.0 / 3.0 * t706 * t824 - 2000.0 * t709 * t824 - 5000.0 * t712 * t824 + 200.0 / 3.0 * t839 * t346 + 80.0 / 3.0 * t177 * t119 + 80.0 / 3.0 * t181 * t119 - 200.0 / 9.0 * t184 * t119 - 200.0 / 9.0 * t189 * t119 - 40.0 / 9.0 * t173 * t119 + 1000.0 * t368 * t852 + 600.0 * t375 * t852 - 200.0 * t351 * t852 - 720.0 * t358 * t852 - 80.0 * t363 * t852;
        let t868 = piecewise3(t3, 0.0, t246 * t805 * t259 / 12.0 - t246 * t809 * t259 / 4.0 - t246 * t450 * t487 / 4.0 - 3.0 / 8.0 * t246 * t816 * t259 - 3.0 / 4.0 * t246 * t454 * t487 - 3.0 / 8.0 * t246 * t247 * t863);
        let tv3rho2tau0 = 2.0 * rho[ip] * t868 + 4.0 * t492;
        v3rho2tau[ip] += tv3rho2tau0;
        let t876 = t22 * t135;
        let t877 = t58 * sigma[ip];
        let t884 = t39 * t135;
        let t897 = t613 * rho[ip];
        let t899 = 1.0 / t897 * t631;
        let t904 = t63 * t135;
        let t911 = t67 * t135;
        let t926 = 0.00034133333333333335 * t422 * t397 - 6.826666666666667e-06 * t904 * t877 + 1.6384e-08 * t154 * t149 - 0.00034133333333333335 * t523 * t397 + 1.9114666666666665e-05 * t911 * t877 - 1.31072e-07 * t68 * t149 + 5.24288e-10 * t159 * t417 - 1.2288e-05 * t530 * t136 + 2.4576e-07 * t233 * t149 - 2.883584e-09 * t72 * t417 + 1.048576e-11 * t164 * t899;
        let t929 = 0.00034133333333333335 * t396 * t397 - 6.826666666666667e-06 * t876 * t877 + 1.6384e-08 * t123 * t149 - 0.00034133333333333335 * t500 * t397 + 1.9114666666666665e-05 * t884 * t877 - 1.31072e-07 * t41 * t149 + 5.24288e-10 * t133 * t417 - 1.2288e-05 * t507 * t136 + 2.4576e-07 * t213 * t149 - 2.883584e-09 * t54 * t417 + 1.048576e-11 * t142 * t899 + t926 * t104 + t537 * t192;
        let t934 = piecewise3(t3, 0.0, -t7 * t112 * t539 / 8.0 - 3.0 / 8.0 * t7 * t20 * t929);
        let tv3rhosigma20 = 2.0 * rho[ip] * t934 + 2.0 * t543;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let t937 = t111 * t238;
        let t941 = t19 * t439;
        let t949 = piecewise3(t3, 0.0, -t246 * t937 * t259 / 8.0 - 3.0 / 8.0 * t246 * t941 * t259 - 3.0 / 8.0 * t246 * t545 * t487);
        let tv3rhosigmatau0 = 2.0 * rho[ip] * t949 + 2.0 * t549;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let tv3rholapl20 = 0.0;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let t960 = 1.0 / t124;
        let t961 = t960 * t95;
        let t964 = t960 * tau[ip];
        let t985 = -40.0 / 3.0 * t462 * t172 - 40.0 * t961 * tau[ip] - 680.0 * t703 * t964 + 200.0 / 3.0 * t351 * t462 - 1040.0 * t706 * t964 + 240.0 * t358 * t462 + 1200.0 * t709 * t964 + 80.0 / 3.0 * t363 * t462 + 3000.0 * t712 * t964 - 1000.0 / 3.0 * t368 * t462 + 1400.0 * t695 * t964 - 200.0 * t375 * t462;
        let t990 = piecewise3(t3, 0.0, -t246 * t450 * t566 / 8.0 - 3.0 / 8.0 * t246 * t454 * t566 - 3.0 / 8.0 * t246 * t247 * t985);
        let tv3rhotau20 = 2.0 * rho[ip] * t990 + 2.0 * t570;
        v3rhotau2[ip] += tv3rhotau20;
        let t1013 = 1.0 / t613 * t631;
        let t1037 = 1.536e-06 * t63 * t56 * t58 - 6.144e-09 * t64 * t220 - 3.072e-06 * t67 * t56 * t58 + 3.6864e-08 * t228 * t220 - 1.96608e-10 * t68 * t516 + 1.536e-06 * t71 * t56 * t58 - 5.5296e-08 * t530 * t220 + 8.84736e-10 * t233 * t516 - 3.93216e-12 * t72 * t1013;
        let t1039 = 1.536e-06 * t22 * t56 * t58 - 6.144e-09 * t23 * t220 - 3.072e-06 * t39 * t56 * t58 + 3.6864e-08 * t208 * t220 - 1.96608e-10 * t41 * t516 + 1.536e-06 * t52 * t56 * t58 - 5.5296e-08 * t507 * t220 + 8.84736e-10 * t213 * t516 - 3.93216e-12 * t54 * t1013 + t1037 * t104;
        let t1043 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t1039);
        let tv3sigma30 = 2.0 * rho[ip] * t1043;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let t1045 = t19 * t537;
        let t1049 = piecewise3(t3, 0.0, -3.0 / 8.0 * t246 * t1045 * t259);
        let tv3sigma2tau0 = 2.0 * rho[ip] * t1049;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let tv3sigmalapl20 = 0.0;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let t1054 = piecewise3(t3, 0.0, -3.0 / 8.0 * t246 * t545 * t566);
        let tv3sigmatau20 = 2.0 * rho[ip] * t1054;
        v3sigmatau2[ip] += tv3sigmatau20;
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let t1056 = 1.0 / t43;
        let t1069 = -840.0 * t695 * t1056 + 408.0 * t703 * t1056 + 624.0 * t706 * t1056 - 720.0 * t709 * t1056 - 1800.0 * t712 * t1056 + 24.0 * t1056 * t95;
        let t1073 = piecewise3(t3, 0.0, -3.0 / 8.0 * t246 * t247 * t1069);
        let tv3tau30 = 2.0 * rho[ip] * t1073;
        v3tau3[ip] += tv3tau30;
        let t1092 = 1.0 / t19 / t897 * t313 * t24;
        let t1096 = 1.0 / t296 * t58;
        let t1102 = 1.0 / t27 / t413 * t147 * t25;
        let t1107 = t141 * t53;
        let t1113 = 1.0 / t146 / t57;
        let t1115 = 1.0 / t27 / t613 / t124 * t1113 * t25;
        let t1119 = 1.0 / t27 / t124;
        let t1120 = t25 * t1119;
        let t1121 = t1120 * t35;
        let t1127 = t24 / t19 / t134 * t48;
        let t1133 = t98 * t694;
        let t1134 = t346 * t346;
        let t1136 = t1134 * t218 * t25;
        let t1141 = t99 / t693 / t88;
        let t1144 = t87 * t102;
        let t1147 = t91 * t188;
        let t1150 = t92 * t374;
        let t1153 = t347 * t281;
        let t1160 = t696 * t135;
        let t1163 = t83 * t584;
        let t1168 = -6160.0 / 81.0 * t83 * t584 * t89 + 3500000.0 / 27.0 * t1133 * t1136 + 1400000.0 / 27.0 * t1141 * t1136 - 200000.0 / 9.0 * t1144 * t1136 - 200000.0 / 27.0 * t1147 * t1136 + 800000.0 / 9.0 * t1150 * t1136 + 1360000.0 / 81.0 * t368 * t1153 + 272000.0 / 27.0 * t375 * t1153 - 272000.0 / 81.0 * t351 * t1153 + 16000.0 / 9.0 * t1160 * t95 - 30800.0 / 81.0 * t184 * t1163 - 30800.0 / 81.0 * t189 * t1163;
        let t1196 = -6160.0 / 81.0 * t173 * t1163 - 108800.0 / 9.0 * t358 * t1153 + 12320.0 / 27.0 * t177 * t1163 - 108800.0 / 81.0 * t363 * t1153 + 12320.0 / 27.0 * t181 * t1163 + 54400.0 / 81.0 * t347 * t281 * t172 + 272000.0 / 9.0 * t703 * t1160 + 416000.0 / 9.0 * t706 * t1160 - 160000.0 / 3.0 * t709 * t1160 - 400000.0 / 3.0 * t712 * t1160 - 560000.0 / 9.0 * t695 * t1160 - 100000.0 / 27.0 * t1134 * t25 * t218 * t180;
        let t1225 = t613 * t42;
        let t1227 = 1.0 / t1225 * t631;
        let t1241 = 1.0342716049382716 * t64 * t1121 - 0.05778014814814815 * t154 * t1127 + 0.0010368442469135802 * t322 * t1096 - 3.8059425185185186e-06 * t641 * t1102 + 9.942053925925926e-09 * t63 * t307 * t1092 + 0.06605432098765432 * t68 * t1127 - 0.002998170864197531 * t159 * t1096 + 2.3859958518518517e-05 * t329 * t1102 - 1.615583762962963e-07 * t650 * t1092 + 3.9768215703703705e-10 * t67 * t626 * t1227 + 0.00202752 * t72 * t1096 - 4.18489837037037e-05 * t164 * t1102 + 6.240969007407408e-07 * t336 * t1092 - 4.02653184e-09 * t659 * t1227 + 4.772185884444445e-12 * t71 * t1107 * t1115;
        let t1261 = -1.615583762962963e-07 * t612 * t1092 + 0.00202752 * t54 * t1096 - 4.18489837037037e-05 * t142 * t1102 + 6.240969007407408e-07 * t308 * t1092 + 4.772185884444445e-12 * t52 * t1107 * t1115 + 1.0342716049382716 * t23 * t1121 + 0.06605432098765432 * t41 * t1127 + t75 * (t1168 + t1196) + 4.0 * t167 * t717 + 6.0 * t339 * t380 + 4.0 * t662 * t192 + t1241 * t104 - 0.05778014814814815 * t123 * t1127 - 3.8059425185185186e-06 * t599 * t1102 + 9.942053925925926e-09 * t22 * t307 * t1092 + 2.3859958518518517e-05 * t295 * t1102 + 0.0010368442469135802 * t286 * t1096 - 0.002998170864197531 * t133 * t1096 + 3.9768215703703705e-10 * t39 * t626 * t1227 - 4.02653184e-09 * t627 * t1227;
        let t1266 = piecewise3(t3, 0.0, 10.0 / 27.0 * t7 * t18 * t118 * t106 - 5.0 / 9.0 * t7 * t573 * t194 + t7 * t266 * t382 / 2.0 - t7 * t112 * t719 / 2.0 - 3.0 / 8.0 * t7 * t20 * t1261);
        let tv4rho40 = 2.0 * rho[ip] * t1266 + 8.0 * t724;
        v4rho4[ip] += tv4rho40;
        let t1284 = t584 * t35;
        let t1288 = t590 * t48 * sigma[ip];
        let t1296 = t147 * t53 * t25;
        let t1323 = 1.0 / t27 / t613 / t43 * t1113 * t25;
        let t1326 = -0.18251851851851852 * t223 * t1284 + 0.014392888888888889 * t422 * t1288 - 0.00031099259259259257 * t63 * t595 * t742 + 1.2961564444444445e-06 * t63 * t601 * t1296 - 3.7282702222222225e-09 * t641 * t617 - 0.015853037037037037 * t228 * t592 + 0.0008522714074074074 * t68 * t596 - 7.620380444444445e-06 * t159 * t603 + 5.639008711111111e-08 * t329 * t617 - 1.491308088888889e-10 * t650 * t632 - 0.00055296 * t233 * t596 + 1.273400888888889e-05 * t72 * t603 - 2.0651121777777777e-07 * t164 * t617 + 1.42606336e-09 * t336 * t632 - 1.7895697066666666e-12 * t659 * t1323;
        let t1360 = t238 * t717 + 3.0 * t439 * t380 + 3.0 * t791 * t192 + t1326 * t104 - 0.18251851851851852 * t202 * t1284 + 0.014392888888888889 * t396 * t1288 + 1.2961564444444445e-06 * t22 * t601 * t1296 - 3.7282702222222225e-09 * t599 * t617 - 7.620380444444445e-06 * t133 * t603 + 5.639008711111111e-08 * t295 * t617 - 0.00055296 * t213 * t596 + 1.273400888888889e-05 * t54 * t603 - 2.0651121777777777e-07 * t142 * t617 - 1.7895697066666666e-12 * t627 * t1323 - 0.015853037037037037 * t208 * t592 - 0.00031099259259259257 * t22 * t595 * t742 + 0.0008522714074074074 * t41 * t596 - 1.491308088888889e-10 * t612 * t632 + 1.42606336e-09 * t308 * t632;
        let t1365 = piecewise3(t3, 0.0, -5.0 / 36.0 * t7 * t573 * t240 + t7 * t266 * t442 / 4.0 - 3.0 / 8.0 * t7 * t112 * t796 - 3.0 / 8.0 * t7 * t20 * t1360);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t1365 + 6.0 * t801;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho3lapl0 = 0.0;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let t1400 = t127 * tau[ip];
        let t1405 = t56 * t346;
        let t1409 = 1.0 / t27 / t134;
        let t1411 = t1409 * t696 * t25;
        let t1430 = -32800.0 / 9.0 * t375 * t1400 + 440.0 / 27.0 * t173 * t275 - 44200.0 / 3.0 * t703 * t1405 + 40000.0 / 3.0 * t1144 * t1411 - 67600.0 / 3.0 * t706 * t1405 + 40000.0 / 9.0 * t1147 * t1411 + 26000.0 * t709 * t1405 - 160000.0 / 3.0 * t1150 * t1411 + 65000.0 * t712 * t1405 - 880.0 / 9.0 * t177 * t275 + 32800.0 / 27.0 * t351 * t1400 + 13120.0 / 3.0 * t358 * t1400;
        let t1458 = -880.0 / 9.0 * t181 * t275 + 13120.0 / 27.0 * t363 * t1400 + 2200.0 / 27.0 * t184 * t275 - 164000.0 / 27.0 * t368 * t1400 + 2200.0 / 27.0 * t189 * t275 + 91000.0 / 3.0 * t695 * t1405 - 700000.0 / 9.0 * t1133 * t1411 - 280000.0 / 9.0 * t1141 * t1411 + 440.0 / 27.0 * t275 * t89 - 6560.0 / 27.0 * t127 * t463 + 20000.0 / 9.0 * t696 * t25 * t1409 * t180 - 2600.0 / 3.0 * t56 * t95 * t346;
        let t1464 = piecewise3(t3, 0.0, -5.0 / 36.0 * t246 * t29 * t75 * t259 + t246 * t85 * t167 * t259 / 4.0 + t246 * t805 * t487 / 4.0 - 3.0 / 8.0 * t246 * t111 * t339 * t259 - 3.0 / 4.0 * t246 * t809 * t487 - 3.0 / 8.0 * t246 * t450 * t863 - 3.0 / 8.0 * t246 * t19 * t662 * t259 - 9.0 / 8.0 * t246 * t816 * t487 - 9.0 / 8.0 * t246 * t454 * t863 - 3.0 / 8.0 * t246 * t247 * (t1430 + t1458));
        let tv4rho3tau0 = 2.0 * rho[ip] * t1464 + 6.0 * t868;
        v4rho3tau[ip] += tv4rho3tau0;
        let t1485 = t147 * t40 * t25;
        let t1513 = 1.0 / t27 / t1225 * t1113 * t25;
        let t1516 = -0.0021617777777777777 * t422 * t737 + 7.600355555555555e-05 * t770 * t877 - 4.096e-07 * t63 * t298 * t1485 + 1.3981013333333332e-09 * t322 * t315 + 0.0021617777777777777 * t523 * t737 - 0.00018659555555555556 * t67 * t287 * t877 + 2.1408426666666665e-06 * t67 * t298 * t1485 - 1.8699605333333334e-08 * t159 * t315 + 5.592405333333333e-11 * t329 * t763 + 0.000110592 * t530 * t288 - 3.260416e-06 * t233 * t300 + 6.230289066666666e-08 * t72 * t315 - 4.858402133333333e-10 * t164 * t763 + 6.7108864e-13 * t336 * t1513;
        let t1547 = -4.858402133333333e-10 * t142 * t763 + t537 * t380 + 2.0 * t926 * t192 + t1516 * t104 - 0.0021617777777777777 * t396 * t737 - 4.096e-07 * t22 * t298 * t1485 + 1.3981013333333332e-09 * t286 * t315 + 0.0021617777777777777 * t500 * t737 + 2.1408426666666665e-06 * t39 * t298 * t1485 - 1.8699605333333334e-08 * t133 * t315 + 0.000110592 * t507 * t288 - 3.260416e-06 * t213 * t300 + 6.230289066666666e-08 * t54 * t315 + 6.7108864e-13 * t308 * t1513 + 7.600355555555555e-05 * t741 * t877 - 0.00018659555555555556 * t39 * t287 * t877 + 5.592405333333333e-11 * t295 * t763;
        let t1552 = piecewise3(t3, 0.0, t7 * t266 * t539 / 12.0 - t7 * t112 * t929 / 4.0 - 3.0 / 8.0 * t7 * t20 * t1547);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t1552 + 4.0 * t934;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rho2sigmalapl0 = 0.0;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let t1578 = piecewise3(t3, 0.0, t246 * t85 * t238 * t259 / 12.0 - t246 * t111 * t439 * t259 / 4.0 - t246 * t937 * t487 / 4.0 - 3.0 / 8.0 * t246 * t19 * t791 * t259 - 3.0 / 4.0 * t246 * t941 * t487 - 3.0 / 8.0 * t246 * t545 * t863);
        let tv4rho2sigmatau0 = 2.0 * rho[ip] * t1578 + 4.0 * t949;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let tv4rho2lapl20 = 0.0;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let tv4rho2lapltau0 = 0.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let t1600 = 1.0 / t27 / t55;
        let t1607 = t823 * tau[ip];
        let t1611 = t1600 * t346 * t25;
        let t1640 = 520.0 / 9.0 * t46 * t172 - 4000.0 / 3.0 * t1600 * t180 * t346 * t25 - 1040.0 / 9.0 * t363 * t46 - 28000.0 * t712 * t1607 + 140000.0 / 3.0 * t1133 * t1611 + 13000.0 / 9.0 * t368 * t46 - 39200.0 / 3.0 * t695 * t1607 + 56000.0 / 3.0 * t1141 * t1611 + 2600.0 / 3.0 * t375 * t46 + 19040.0 / 3.0 * t703 * t1607 - 8000.0 * t1144 * t1611 - 2600.0 / 9.0 * t351 * t46 + 29120.0 / 3.0 * t706 * t1607 - 8000.0 / 3.0 * t1147 * t1611 - 1040.0 * t358 * t46 - 11200.0 * t709 * t1607 + 32000.0 * t1150 * t1611 + 1120.0 / 3.0 * t839 * tau[ip];
        let t1645 = piecewise3(t3, 0.0, t246 * t805 * t566 / 12.0 - t246 * t809 * t566 / 4.0 - t246 * t450 * t985 / 4.0 - 3.0 / 8.0 * t246 * t816 * t566 - 3.0 / 4.0 * t246 * t454 * t985 - 3.0 / 8.0 * t246 * t247 * t1640);
        let tv4rho2tau20 = 2.0 * rho[ip] * t1645 + 4.0 * t990;
        v4rho2tau2[ip] += tv4rho2tau20;
        let t1655 = t147 * sigma[ip] * t25;
        let t1682 = 1.0 / t27 / t628 * t1113 * t25;
        let t1713 = -1.2288e-05 * t904 * t58 + 1.14688e-07 * t63 * t145 * t1655 - 5.24288e-10 * t154 * t417 + 2.4576e-05 * t911 * t58 - 4.9152e-07 * t67 * t145 * t1655 + 5.767168e-09 * t68 * t417 - 2.097152e-11 * t159 * t899 - 1.2288e-05 * t71 * t135 * t58 + 6.38976e-07 * t71 * t145 * t1655 - 1.6515072e-08 * t233 * t417 + 1.572864e-10 * t72 * t899 - 2.5165824e-13 * t164 * t1682;
        let t1716 = -1.2288e-05 * t876 * t58 + 1.14688e-07 * t22 * t145 * t1655 - 5.24288e-10 * t123 * t417 + 2.4576e-05 * t884 * t58 - 4.9152e-07 * t39 * t145 * t1655 + 5.767168e-09 * t41 * t417 - 2.097152e-11 * t133 * t899 - 1.2288e-05 * t52 * t135 * t58 + 6.38976e-07 * t52 * t145 * t1655 - 1.6515072e-08 * t213 * t417 + 1.572864e-10 * t54 * t899 - 2.5165824e-13 * t142 * t1682 + t1713 * t104 + t1037 * t192;
        let t1721 = piecewise3(t3, 0.0, -t7 * t112 * t1039 / 8.0 - 3.0 / 8.0 * t7 * t20 * t1716);
        let tv4rhosigma30 = 2.0 * rho[ip] * t1721 + 2.0 * t1043;
        v4rhosigma3[ip] += tv4rhosigma30;
        let tv4rhosigma2lapl0 = 0.0;
        v4rhosigma2lapl[ip] += tv4rhosigma2lapl0;
        let t1736 = piecewise3(t3, 0.0, -t246 * t111 * t537 * t259 / 8.0 - 3.0 / 8.0 * t246 * t19 * t926 * t259 - 3.0 / 8.0 * t246 * t1045 * t487);
        let tv4rhosigma2tau0 = 2.0 * rho[ip] * t1736 + 2.0 * t1049;
        v4rhosigma2tau[ip] += tv4rhosigma2tau0;
        let tv4rhosigmalapl20 = 0.0;
        v4rhosigmalapl2[ip] += tv4rhosigmalapl20;
        let tv4rhosigmalapltau0 = 0.0;
        v4rhosigmalapltau[ip] += tv4rhosigmalapltau0;
        let t1749 = piecewise3(t3, 0.0, -t246 * t937 * t566 / 8.0 - 3.0 / 8.0 * t246 * t941 * t566 - 3.0 / 8.0 * t246 * t545 * t985);
        let tv4rhosigmatau20 = 2.0 * rho[ip] * t1749 + 2.0 * t1054;
        v4rhosigmatau2[ip] += tv4rhosigmatau20;
        let tv4rholapl30 = 0.0;
        v4rholapl3[ip] += tv4rholapl30;
        let tv4rholapl2tau0 = 0.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let tv4rholapltau20 = 0.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let t1760 = 1.0 / t27 / t279;
        let t1765 = t1760 * tau[ip] * t25;
        let t1786 = 800.0 * t1760 * t180 * t83 - 28000.0 * t1133 * t1765 - 11200.0 * t1141 * t1765 + 4800.0 * t1144 * t1765 + 1600.0 * t1147 * t1765 - 19200.0 * t1150 * t1765 + 4200.0 * t695 * t960 - 2040.0 * t703 * t960 - 3120.0 * t706 * t960 + 3600.0 * t709 * t960 + 9000.0 * t712 * t960 - 120.0 * t961;
        let t1791 = piecewise3(t3, 0.0, -t246 * t450 * t1069 / 8.0 - 3.0 / 8.0 * t246 * t454 * t1069 - 3.0 / 8.0 * t246 * t247 * t1786);
        let tv4rhotau30 = 2.0 * rho[ip] * t1791 + 2.0 * t1073;
        v4rhotau3[ip] += tv4rhotau30;
        let t1795 = t147 * t25;
        let t1817 = 1.0 / t27 / t761 * t1113 * t25;
        let t1847 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * (-2.4576e-08 * t22 * t218 * t1795 + 1.96608e-10 * t23 * t516 + 7.3728e-08 * t39 * t218 * t1795 - 1.572864e-09 * t208 * t516 + 7.86432e-12 * t41 * t1013 - 7.3728e-08 * t52 * t218 * t1795 + 3.538944e-09 * t507 * t516 - 4.718592e-11 * t213 * t1013 + 9.437184e-14 * t54 * t1817 + (-2.4576e-08 * t63 * t218 * t1795 + 1.96608e-10 * t64 * t516 + 7.3728e-08 * t67 * t218 * t1795 - 1.572864e-09 * t228 * t516 + 7.86432e-12 * t68 * t1013 - 7.3728e-08 * t71 * t218 * t1795 + 3.538944e-09 * t530 * t516 - 4.718592e-11 * t233 * t1013 + 9.437184e-14 * t72 * t1817) * t104));
        let tv4sigma40 = 2.0 * rho[ip] * t1847;
        v4sigma4[ip] += tv4sigma40;
        let tv4sigma3lapl0 = 0.0;
        v4sigma3lapl[ip] += tv4sigma3lapl0;
        let t1853 = piecewise3(t3, 0.0, -3.0 / 8.0 * t246 * t19 * t1037 * t259);
        let tv4sigma3tau0 = 2.0 * rho[ip] * t1853;
        v4sigma3tau[ip] += tv4sigma3tau0;
        let tv4sigma2lapl20 = 0.0;
        v4sigma2lapl2[ip] += tv4sigma2lapl20;
        let tv4sigma2lapltau0 = 0.0;
        v4sigma2lapltau[ip] += tv4sigma2lapltau0;
        let t1858 = piecewise3(t3, 0.0, -3.0 / 8.0 * t246 * t1045 * t566);
        let tv4sigma2tau20 = 2.0 * rho[ip] * t1858;
        v4sigma2tau2[ip] += tv4sigma2tau20;
        let tv4sigmalapl30 = 0.0;
        v4sigmalapl3[ip] += tv4sigmalapl30;
        let tv4sigmalapl2tau0 = 0.0;
        v4sigmalapl2tau[ip] += tv4sigmalapl2tau0;
        let tv4sigmalapltau20 = 0.0;
        v4sigmalapltau2[ip] += tv4sigmalapltau20;
        let t1863 = piecewise3(t3, 0.0, -3.0 / 8.0 * t246 * t545 * t1069);
        let tv4sigmatau30 = 2.0 * rho[ip] * t1863;
        v4sigmatau3[ip] += tv4sigmatau30;
        let tv4lapl40 = 0.0;
        v4lapl4[ip] += tv4lapl40;
        let tv4lapl3tau0 = 0.0;
        v4lapl3tau[ip] += tv4lapl3tau0;
        let tv4lapl2tau20 = 0.0;
        v4lapl2tau2[ip] += tv4lapl2tau20;
        let tv4lapltau30 = 0.0;
        v4lapltau3[ip] += tv4lapltau30;
        let t1882 = piecewise3(t3, 0.0, -3.0 / 8.0 * t246 * t247 * (-480.0 * t1119 * t180 * t25 + 16800.0 * t1133 * t1120 + 6720.0 * t1141 * t1120 - 2880.0 * t1144 * t1120 - 960.0 * t1147 * t1120 + 11520.0 * t1150 * t1120));
        let tv4tau40 = 2.0 * rho[ip] * t1882;
        v4tau4[ip] += tv4tau40;
    }
}
