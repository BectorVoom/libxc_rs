//! GGA_K_LLP kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_llp.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_llp_kxc_unpol(
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
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
    }
}
