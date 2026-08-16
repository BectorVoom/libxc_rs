//! MGGA_X_TAU_HCTH fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tau_hcth.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_tau_hcth_fxc_unpol(
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
    param_cx_local_0: f64,
    param_cx_local_1: f64,
    param_cx_local_2: f64,
    param_cx_local_3: f64,
    param_cx_nlocal_0: f64,
    param_cx_nlocal_1: f64,
    param_cx_nlocal_2: f64,
    param_cx_nlocal_3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t16 = pow_1_3::<f64>(t12);
        let t18 = piecewise3::<f64>(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3::<f64>(rho[ip]);
        let t20 = t18 * t19;
        let t22 = param_cx_local_1;
        let t23 = t22 * sigma[ip];
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = rho[ip] * rho[ip];
        let t27 = t19 * t19;
        let t29 = 1.0 / t27 / t26;
        let t30 = t25 * t29;
        let t34 = 1.0 + 0.4e-2 * sigma[ip] * t25 * t29;
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
        let t75 = param_cx_nlocal_0 + 0.4e-2 * t64 * t36 + 0.32e-4 * t68 * t49 + 0.256e-6 * t72 * t59;
        let t76 = M_CBRT6;
        let t77 = t76 * t76;
        let t78 = M_PI * M_PI;
        let t79 = pow_1_3::<f64>(t78);
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
        let t106 = param_cx_local_0 + 0.4e-2 * t23 * t36 + 0.32e-4 * t41 * t49 + 0.256e-6 * t54 * t59 + t75 * t104;
        let t110 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t106);
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
        let t167 = -0.10666666666666666667e-1 * t64 * t120 + 0.85333333333333333336e-4 * t154 * t128 - 0.17066666666666666667e-3 * t68 * t128 + 0.13653333333333333334e-5 * t159 * t136 - 0.2048e-5 * t72 * t136 + 0.81920000000000000003e-8 * t164 * t149;
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
        let t194 = -0.10666666666666666667e-1 * t23 * t120 + 0.85333333333333333336e-4 * t123 * t128 - 0.17066666666666666667e-3 * t41 * t128 + 0.13653333333333333334e-5 * t133 * t136 - 0.2048e-5 * t54 * t136 + 0.81920000000000000003e-8 * t142 * t149 + t167 * t104 + t75 * t192;
        let t199 = piecewise3::<f64>(t3, 0.0, -t7 * t112 * t106 / 8.0 - 3.0 / 8.0 * t7 * t20 * t194);
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
        let t238 = 0.4e-2 * t223 * t203 - 0.32e-4 * t64 * t49 + 0.64e-4 * t228 * t49 - 0.512e-6 * t68 * t59 + 0.768e-6 * t233 * t59 - 0.3072e-8 * t72 * t220;
        let t240 = 0.4e-2 * t202 * t203 - 0.32e-4 * t23 * t49 + 0.64e-4 * t208 * t49 - 0.512e-6 * t41 * t59 + 0.768e-6 * t213 * t59 - 0.3072e-8 * t54 * t220 + t238 * t104;
        let t244 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t240);
        let tvsigma0 = 2.0 * rho[ip] * t244;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t246 = t7 * t18;
        let t247 = t19 * t75;
        let t248 = t25 * t85;
        let t259 = -t173 * t248 + 6.0 * t177 * t248 + 6.0 * t181 * t248 - 5.0 * t184 * t248 - 5.0 * t189 * t248 - t248 * t89;
        let t263 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t246 * t247 * t259);
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
        let t339 = 0.39111111111111111112e-1 * t64 * t276 - 0.76800000000000000003e-3 * t154 * t283 + 0.36408888888888888891e-5 * t322 * t288 + 0.10808888888888888889e-2 * t68 * t283 - 0.19569777777777777779e-4 * t159 * t288 + 0.4369066666666666667e-7 * t329 * t300 + 0.18432e-4 * t72 * t288 - 0.16110933333333333334e-6 * t164 * t300 + 0.69905066666666666671e-9 * t336 * t315;
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
        let t382 = 0.39111111111111111112e-1 * t23 * t276 - 0.76800000000000000003e-3 * t123 * t283 + 0.36408888888888888891e-5 * t286 * t288 + 0.10808888888888888889e-2 * t41 * t283 - 0.19569777777777777779e-4 * t133 * t288 + 0.4369066666666666667e-7 * t295 * t300 + 0.18432e-4 * t54 * t288 - 0.16110933333333333334e-6 * t142 * t300 + 0.69905066666666666671e-9 * t308 * t315 + t339 * t104 + 2.0 * t167 * t192 + t75 * t380;
        let t387 = piecewise3::<f64>(t3, 0.0, t7 * t266 * t106 / 12.0 - t7 * t112 * t194 / 4.0 - 3.0 / 8.0 * t7 * t20 * t382);
        let tv2rho20 = 2.0 * rho[ip] * t387 + 4.0 * t199;
        v2rho2[ip] += tv2rho20;
        let t393 = t118 * t35;
        let t396 = t22 * t24;
        let t397 = t126 * t48;
        let t398 = t397 * sigma[ip];
        let t413 = t55 * t124;
        let t417 = 1.0 / t19 / t413 * t313 * t24;
        let t422 = t63 * t24;
        let t439 = -0.10666666666666666667e-1 * t223 * t393 + 0.25600000000000000001e-3 * t422 * t398 - 0.13653333333333333334e-5 * t154 * t136 - 0.34133333333333333333e-3 * t228 * t128 + 0.68266666666666666668e-5 * t68 * t136 - 0.16384000000000000001e-7 * t159 * t149 - 0.6144e-5 * t233 * t136 + 0.57344000000000000001e-7 * t72 * t149 - 0.26214400000000000001e-9 * t164 * t417;
        let t442 = -0.10666666666666666667e-1 * t202 * t393 + 0.25600000000000000001e-3 * t396 * t398 - 0.13653333333333333334e-5 * t123 * t136 - 0.34133333333333333333e-3 * t208 * t128 + 0.68266666666666666668e-5 * t41 * t136 - 0.16384000000000000001e-7 * t133 * t149 - 0.6144e-5 * t213 * t136 + 0.57344000000000000001e-7 * t54 * t149 - 0.26214400000000000001e-9 * t142 * t417 + t439 * t104 + t238 * t192;
        let t447 = piecewise3::<f64>(t3, 0.0, -t7 * t112 * t240 / 8.0 - 3.0 / 8.0 * t7 * t20 * t442);
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
        let t492 = piecewise3::<f64>(t3, 0.0, -t246 * t450 * t259 / 8.0 - 3.0 / 8.0 * t246 * t454 * t259 - 3.0 / 8.0 * t246 * t247 * t487);
        let tv2rhotau0 = 2.0 * rho[ip] * t492 + 2.0 * t263;
        v2rhotau[ip] += tv2rhotau0;
        let t495 = t45 * t48;
        let t500 = t39 * t24;
        let t507 = t52 * sigma[ip];
        let t512 = t55 * t43;
        let t516 = 1.0 / t19 / t512 * t313 * t24;
        let t523 = t67 * t24;
        let t530 = t71 * sigma[ip];
        let t537 = -0.64e-4 * t422 * t495 + 0.512e-6 * t64 * t59 + 0.64e-4 * t523 * t495 - 0.2048e-5 * t228 * t59 + 0.6144e-8 * t68 * t220 + 0.1536e-5 * t530 * t59 - 0.18432e-7 * t233 * t220 + 0.98304e-10 * t72 * t516;
        let t539 = -0.64e-4 * t396 * t495 + 0.512e-6 * t23 * t59 + 0.64e-4 * t500 * t495 - 0.2048e-5 * t208 * t59 + 0.6144e-8 * t41 * t220 + 0.1536e-5 * t507 * t59 - 0.18432e-7 * t213 * t220 + 0.98304e-10 * t54 * t516 + t537 * t104;
        let t543 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t539);
        let tv2sigma20 = 2.0 * rho[ip] * t543;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t545 = t19 * t238;
        let t549 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t246 * t545 * t259);
        let tv2sigmatau0 = 2.0 * rho[ip] * t549;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t553 = t24 / t19 / t116;
        let t566 = 4.0 * t553 * t172 - 20.0 * t351 * t553 - 72.0 * t358 * t553 - 8.0 * t363 * t553 + 100.0 * t368 * t553 + 60.0 * t375 * t553;
        let t570 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t246 * t247 * t566);
        let tv2tau20 = 2.0 * rho[ip] * t570;
        v2tau2[ip] += tv2tau20;
    }
}
