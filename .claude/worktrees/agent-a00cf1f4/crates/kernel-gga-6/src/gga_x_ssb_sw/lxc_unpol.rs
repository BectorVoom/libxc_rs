//! GGA_X_SSB_SW lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ssb_sw.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ssb_sw_lxc_unpol(
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
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = param_B * t20 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t39 = 1.0 + param_C * t20 * t25 * t29 * t33 / 24.0;
        let t40 = 1.0 / t39;
        let t46 = param_D * t20 * t25;
        let t47 = t20 * t20;
        let t50 = 1.0 / t23 / t22;
        let t52 = sigma[ip] * sigma[ip];
        let t54 = t30 * t30;
        let t55 = t54 * rho[ip];
        let t57 = 1.0 / t18 / t55;
        let t61 = 1.0 + param_E * t47 * t50 * t52 * t27 * t57 / 288.0;
        let t62 = 1.0 / t61;
        let t67 = param_A + t26 * t29 * t33 * t40 / 24.0 - t46 * t29 * t33 * t62 / 24.0;
        let t71 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t67);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
        let t73 = t17 / t31;
        let t77 = t30 * rho[ip];
        let t79 = 1.0 / t31 / t77;
        let t84 = param_B * t47;
        let t86 = t84 * t50 * t52;
        let t87 = t54 * t30;
        let t89 = 1.0 / t18 / t87;
        let t91 = t39 * t39;
        let t92 = 1.0 / t91;
        let t93 = t92 * param_C;
        let t94 = t27 * t89 * t93;
        let t101 = t22 * t22;
        let t102 = 1.0 / t101;
        let t103 = param_D * t102;
        let t104 = t52 * sigma[ip];
        let t105 = t103 * t104;
        let t106 = t54 * t54;
        let t107 = t106 * rho[ip];
        let t108 = 1.0 / t107;
        let t109 = t61 * t61;
        let t110 = 1.0 / t109;
        let t112 = t108 * t110 * param_E;
        let t115 = -t26 * t29 * t79 * t40 / 9.0 + t86 * t94 / 108.0 + t46 * t29 * t79 * t62 / 9.0 - t105 * t112 / 108.0;
        let t120 = piecewise3(t2, 0.0, -t6 * t73 * t67 / 8.0 - 3.0 / 8.0 * t6 * t19 * t115);
        let tvrho0 = 2.0 * rho[ip] * t120 + 2.0 * t71;
        vrho[ip] += tvrho0;
        let t123 = t28 * t33;
        let t130 = t27 * t57 * t93;
        let t137 = 1.0 / t106;
        let t139 = t137 * t110 * param_E;
        let t142 = t26 * t123 * t40 / 24.0 - t84 * t50 * sigma[ip] * t130 / 288.0 - t46 * t123 * t62 / 24.0 + t103 * t52 * t139 / 288.0;
        let t146 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t142);
        let tvsigma0 = 2.0 * rho[ip] * t146;
        vsigma[ip] += tvsigma0;
        let t151 = t17 / t31 / rho[ip];
        let t159 = 1.0 / t31 / t54;
        let t164 = t54 * t77;
        let t166 = 1.0 / t18 / t164;
        let t168 = t27 * t166 * t93;
        let t171 = param_B * t102;
        let t172 = t171 * t104;
        let t173 = t106 * t30;
        let t174 = 1.0 / t173;
        let t176 = 1.0 / t91 / t39;
        let t178 = param_C * param_C;
        let t190 = t52 * t52;
        let t191 = t190 * sigma[ip];
        let t194 = 1.0 / t18 / t106 / t164;
        let t198 = 1.0 / t109 / t61;
        let t199 = param_E * param_E;
        let t202 = t47 * t50 * t27;
        let t203 = t198 * t199 * t202;
        let t206 = 11.0 / 27.0 * t26 * t29 * t159 * t40 - t86 * t168 / 12.0 + 2.0 / 81.0 * t172 * t174 * t176 * t178 - 11.0 / 27.0 * t46 * t29 * t159 * t62 + 35.0 / 324.0 * t105 * t174 * t110 * param_E - t103 * t191 * t194 * t203 / 2916.0;
        let t211 = piecewise3(t2, 0.0, t6 * t151 * t67 / 12.0 - t6 * t73 * t115 / 4.0 - 3.0 / 8.0 * t6 * t19 * t206);
        let tv2rho20 = 2.0 * rho[ip] * t211 + 4.0 * t120;
        v2rho2[ip] += tv2rho20;
        let t217 = t28 * t79;
        let t222 = t84 * t50 * t27;
        let t224 = param_C * sigma[ip];
        let t230 = t108 * t176 * t178;
        let t236 = t103 * t108;
        let t237 = t110 * param_E;
        let t238 = t237 * t52;
        let t241 = t106 * t87;
        let t243 = 1.0 / t18 / t241;
        let t248 = -t26 * t217 * t40 / 9.0 + t222 * t89 * t92 * t224 / 36.0 - t171 * t52 * t230 / 108.0 + t46 * t217 * t62 / 9.0 - t236 * t238 / 27.0 + t103 * t190 * t243 * t203 / 7776.0;
        let t253 = piecewise3(t2, 0.0, -t6 * t73 * t142 / 8.0 - 3.0 / 8.0 * t6 * t19 * t248);
        let tv2rhosigma0 = 2.0 * rho[ip] * t253 + 2.0 * t146;
        v2rhosigma[ip] += tv2rhosigma0;
        let t256 = t84 * t50;
        let t261 = t137 * t176 * t178;
        let t265 = t237 * sigma[ip];
        let t268 = t106 * t55;
        let t270 = 1.0 / t18 / t268;
        let t275 = -t256 * t130 / 144.0 + t171 * sigma[ip] * t261 / 288.0 + t103 * t137 * t265 / 96.0 - t103 * t104 * t270 * t203 / 20736.0;
        let t279 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t275);
        let tv2sigma20 = 2.0 * rho[ip] * t279;
        v2sigma2[ip] += tv2sigma20;
        let t282 = t17 * t33;
        let t293 = 1.0 / t31 / t55;
        let t299 = 1.0 / t18 / t106;
        let t304 = t106 * t77;
        let t305 = 1.0 / t304;
        let t311 = 1.0 / t31 / t268;
        let t314 = t91 * t91;
        let t315 = 1.0 / t314;
        let t316 = t178 * param_C;
        let t319 = t20 * t25 * t28;
        let t320 = t315 * t316 * t319;
        let t331 = t106 * t106;
        let t333 = 1.0 / t18 / t331;
        let t338 = t190 * t104;
        let t341 = 1.0 / t31 / t331 / t55;
        let t344 = t109 * t109;
        let t345 = 1.0 / t344;
        let t346 = t199 * param_E;
        let t351 = t20 / t24 / t101 * t28;
        let t352 = t345 * t346 * t351;
        let t355 = -154.0 / 81.0 * t26 * t29 * t293 * t40 + 341.0 / 486.0 * t86 * t27 * t299 * t93 - 38.0 / 81.0 * t172 * t305 * t176 * t178 + 2.0 / 243.0 * t171 * t190 * t311 * t320 + 154.0 / 81.0 * t46 * t29 * t293 * t62 - 569.0 / 486.0 * t105 * t305 * t110 * param_E + t103 * t191 * t333 * t203 / 108.0 - t103 * t338 * t341 * t352 / 8748.0;
        let t360 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t282 * t67 + t6 * t151 * t115 / 4.0 - 3.0 / 8.0 * t6 * t73 * t206 - 3.0 / 8.0 * t6 * t19 * t355);
        let tv3rho30 = 2.0 * rho[ip] * t360 + 6.0 * t211;
        v3rho3[ip] += tv3rho30;
        let t370 = t28 * t159;
        let t378 = t171 * t174;
        let t379 = t176 * t178;
        let t380 = t379 * t52;
        let t383 = t106 * t54;
        let t385 = 1.0 / t31 / t383;
        let t393 = t103 * t174;
        let t397 = t103 * t194 * t198;
        let t399 = t199 * t190 * t202;
        let t402 = t190 * t52;
        let t405 = 1.0 / t31 / t331 / t54;
        let t410 = 11.0 / 27.0 * t26 * t370 * t40 - 65.0 / 324.0 * t222 * t166 * t92 * t224 + 17.0 / 108.0 * t378 * t380 - t171 * t104 * t385 * t320 / 324.0 - 11.0 / 27.0 * t46 * t370 * t62 + 29.0 / 81.0 * t393 * t238 - 25.0 / 7776.0 * t397 * t399 + t103 * t402 * t405 * t352 / 23328.0;
        let t415 = piecewise3(t2, 0.0, t6 * t151 * t142 / 12.0 - t6 * t73 * t248 / 4.0 - 3.0 / 8.0 * t6 * t19 * t410);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t415 + 4.0 * t253;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t424 = t379 * sigma[ip];
        let t428 = 1.0 / t31 / t304;
        let t436 = t103 * t243 * t198;
        let t438 = t199 * t104 * t202;
        let t443 = 1.0 / t31 / t331 / t77;
        let t448 = t256 * t94 / 27.0 - 5.0 / 108.0 * t171 * t108 * t424 + t171 * t52 * t428 * t320 / 864.0 - t236 * t265 / 12.0 + t436 * t438 / 972.0 - t103 * t191 * t443 * t352 / 62208.0;
        let t453 = piecewise3(t2, 0.0, -t6 * t73 * t275 / 8.0 - 3.0 / 8.0 * t6 * t19 * t448);
        let tv3rhosigma20 = 2.0 * rho[ip] * t453 + 2.0 * t279;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t459 = 1.0 / t31 / t173;
        let t465 = t103 * t270 * t198;
        let t467 = t199 * t52 * t202;
        let t474 = 1.0 / t31 / t331 / t30;
        let t479 = t171 * t261 / 96.0 - t171 * sigma[ip] * t459 * t320 / 2304.0 - t465 * t467 / 3456.0 + t103 * t139 / 96.0 + t103 * t190 * t474 * t352 / 165888.0;
        let t483 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t479);
        let tv3sigma30 = 2.0 * rho[ip] * t483;
        v3sigma3[ip] += tv3sigma30;
        let t500 = 1.0 / t31 / t87;
        let t511 = 1.0 / t383;
        let t525 = t191 / t18 / t331 / rho[ip];
        let t529 = t178 * t178;
        let t531 = 1.0 / t314 / t39 * t529 * t202;
        let t552 = t101 * t101;
        let t555 = param_D / t552 / t101;
        let t556 = t190 * t190;
        let t562 = 1.0 / t344 / t61;
        let t564 = t199 * t199;
        let t573 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 * t79 * t67 - 5.0 / 9.0 * t6 * t282 * t115 + t6 * t151 * t206 / 2.0 - t6 * t73 * t355 / 2.0 - 3.0 / 8.0 * t6 * t19 * (2618.0 / 243.0 * t26 * t29 * t500 * t40 - 3047.0 / 486.0 * t86 * t27 / t18 / t107 * t93 + 5126.0 / 729.0 * t172 * t511 * t176 * t178 - 196.0 / 729.0 * t171 * t190 / t31 / t241 * t320 + 16.0 / 2187.0 * t171 * t525 * t531 - 2618.0 / 243.0 * t46 * t29 * t500 * t62 + 19393.0 / 1458.0 * t105 * t511 * t110 * param_E - 5107.0 / 26244.0 * t103 * t525 * t203 + 73.0 / 13122.0 * t103 * t338 / t31 / t331 / t87 * t352 - 2.0 / 19683.0 * t555 * t556 * sigma[ip] / t331 / t383 * t562 * t564));
        let tv4rho40 = 2.0 * rho[ip] * t573 + 8.0 * t360;
        v4rho4[ip] += tv4rho40;
        let t586 = t28 * t293;
        let t635 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t282 * t142 + t6 * t151 * t248 / 4.0 - 3.0 / 8.0 * t6 * t73 * t410 - 3.0 / 8.0 * t6 * t19 * (-154.0 / 81.0 * t26 * t586 * t40 + 253.0 / 162.0 * t222 * t299 * t92 * t224 - 1025.0 / 486.0 * t171 * t305 * t380 + 89.0 / 972.0 * t171 * t311 * t315 * t316 * t104 * t319 - 2.0 / 729.0 * t171 * t190 * t333 * t531 + 154.0 / 81.0 * t46 * t586 * t62 - 892.0 / 243.0 * t103 * t305 * t238 + 2189.0 / 34992.0 * t103 * t333 * t198 * t399 - 137.0 / 69984.0 * t103 * t341 * t345 * t346 * t402 * t351 + t555 * t556 / t331 / t304 * t562 * t564 / 26244.0));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t635 + 6.0 * t415;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t681 = piecewise3(t2, 0.0, t6 * t151 * t275 / 12.0 - t6 * t73 * t448 / 4.0 - 3.0 / 8.0 * t6 * t19 * (-19.0 / 81.0 * t256 * t168 + 167.0 / 324.0 * t378 * t424 - 25.0 / 864.0 * t171 * t385 * t315 * t316 * t52 * t319 + t171 * t104 * t194 * t531 / 972.0 + 3.0 / 4.0 * t393 * t265 - 13.0 / 729.0 * t397 * t438 + 41.0 / 62208.0 * t103 * t405 * t345 * t346 * t191 * t351 - t555 * t338 / t331 / t173 * t562 * t564 / 69984.0));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t681 + 4.0 * t453;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t691 = t316 * t20;
        let t723 = piecewise3(t2, 0.0, -t6 * t73 * t479 / 8.0 - 3.0 / 8.0 * t6 * t19 * (-t171 * t230 / 12.0 + 7.0 / 864.0 * t171 * t428 * t315 * t691 * t25 * sigma[ip] * t28 - t171 * t52 * t243 * t531 / 2592.0 + 11.0 / 2592.0 * t436 * t467 - 13.0 / 62208.0 * t103 * t443 * t345 * t346 * t190 * t351 - t103 * t112 / 12.0 + t555 * t402 / t331 / t107 * t562 * t564 / 186624.0));
        let tv4rhosigma30 = 2.0 * rho[ip] * t723 + 2.0 * t483;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t757 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * (-t171 * t459 * t315 * t691 * t25 * t28 / 576.0 + t171 * sigma[ip] * t270 * t531 / 6912.0 + 5.0 / 82944.0 * t103 * t474 * t345 * t346 * t104 * t351 - 5.0 / 6912.0 * t465 * t199 * sigma[ip] * t202 - t555 * t191 / t331 / t106 * t562 * t564 / 497664.0));
        let tv4sigma40 = 2.0 * rho[ip] * t757;
        v4sigma4[ip] += tv4sigma40;
    }
}
