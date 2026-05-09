//! GGA_C_LYPR fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 55 shared lines across all orders.
//! Delta: 86 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_PI};
use libxc_kernel_math::erf::{erfc_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_lypr_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_m1: f64,
    param_m2: f64,
    param_omega: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (55 lines) ---
        let t2 = pow_1_3(rho[ip]);
        let t3 = 1.0 / t2;
        let t5 = erfc_approx(param_m1 * param_omega * t3);
        let t7 = param_d * t3 + 1.0;
        let t8 = 1.0 / t7;
        let t10 = param_m2 * param_omega;
        let t12 = erfc_approx(t10 * t3);
        let t13 = t12 * param_b;
        let t15 = f64::exp(-param_c * t3);
        let t16 = t15 * t8;
        let t17 = rho[ip] * rho[ip];
        let t18 = t2 * t2;
        let t20 = 1.0 / t18 / t17;
        let t21 = sigma[ip] * t20;
        let t23 = param_d * t8 + param_c;
        let t24 = t23 * t3;
        let t26 = -1.0 / 72.0 - 7.0 / 72.0 * t24;
        let t28 = M_CBRT3;
        let t29 = t28 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t34 = 1.0 <= zeta_threshold;
        let t35 = zeta_threshold * zeta_threshold;
        let t36 = pow_1_3(zeta_threshold);
        let t37 = t36 * t36;
        let t39 = piecewise3(t34, t37 * t35, 1.0);
        let t43 = 5.0 / 2.0 - t24 / 18.0;
        let t44 = t43 * sigma[ip];
        let t45 = t20 * t39;
        let t48 = t24 - 11.0;
        let t49 = t48 * sigma[ip];
        let t52 = piecewise3(t34, t37 * t35 * zeta_threshold, 1.0);
        let t53 = t20 * t52;
        let t56 = M_CBRT2;
        let t57 = t56 * t56;
        let t58 = sigma[ip] * t57;
        let t61 = piecewise3(t34, t35, 1.0);
        let t62 = t61 * sigma[ip];
        let t64 = t57 * t20 * t39;
        let t70 = -t21 * t26 - 3.0 / 10.0 * t29 * t32 * t39 + t44 * t45 / 8.0 + t49 * t53 / 144.0 - t56 * (4.0 / 3.0 * t58 * t45 - t62 * t64 / 2.0) / 8.0;
        let t71 = t16 * t70;
        let t73 = param_b * t15;
        let t74 = f64::sqrt(M_PI);
        let t75 = 1.0 / t74;
        let t76 = t8 * t75;
        let t77 = t73 * t76;
        let t78 = param_m2 * param_m2;
        let t79 = param_omega * param_omega;
        let t81 = 1.0 / t18;
        let t83 = f64::exp(-t78 * t79 * t81);
        let t84 = t17 * rho[ip];
        let t85 = 1.0 / t84;
        let t86 = t83 * t85;
        let tzk0 = param_a * (-t5 * t8 + t13 * t71 + 7.0 / 36.0 * t77 * t10 * t86 * sigma[ip]);
        zk[ip] += tzk0;
        // --- vxc delta (55 lines) ---
        let t92 = rho[ip] * param_a;
        let t93 = param_m1 * param_m1;
        let t96 = f64::exp(-t93 * t79 * t81);
        let t98 = t75 * t96 * param_m1;
        let t100 = 1.0 / t2 / rho[ip];
        let t105 = t7 * t7;
        let t106 = 1.0 / t105;
        let t107 = t5 * t106;
        let t108 = param_d * t100;
        let t111 = t75 * t83;
        let t112 = t111 * t10;
        let t113 = t100 * param_b;
        let t117 = t13 * param_c;
        let t118 = t100 * t15;
        let t119 = t8 * t70;
        let t123 = t13 * t15;
        let t124 = t106 * t70;
        let t129 = 1.0 / t18 / t84;
        let t130 = sigma[ip] * t129;
        let t133 = param_d * param_d;
        let t134 = t133 * t106;
        let t139 = -t134 / t18 / rho[ip] + t23 * t100;
        let t140 = 7.0 / 216.0 * t139;
        let t142 = t139 / 54.0;
        let t143 = t142 * sigma[ip];
        let t146 = t129 * t39;
        let t150 = -t139 / 3.0;
        let t151 = t150 * sigma[ip];
        let t154 = t129 * t52;
        let t160 = t57 * t129 * t39;
        let t166 = 8.0 / 3.0 * t130 * t26 - t21 * t140 + t143 * t45 / 8.0 - t44 * t146 / 3.0 + t151 * t53 / 144.0 - t49 * t154 / 54.0 - t56 * (-32.0 / 9.0 * t58 * t146 + 4.0 / 3.0 * t62 * t160) / 8.0;
        let t167 = t16 * t166;
        let t169 = param_b * param_c;
        let t170 = t17 * t17;
        let t172 = 1.0 / t2 / t170;
        let t173 = t172 * t15;
        let t176 = t75 * param_m2;
        let t177 = param_omega * t83;
        let t179 = t176 * t177 * sigma[ip];
        let t182 = t106 * t75;
        let t184 = t73 * t182 * param_m2;
        let t190 = t78 * param_m2;
        let t191 = t79 * param_omega;
        let t192 = t190 * t191;
        let t194 = 1.0 / t18 / t170;
        let t195 = t194 * t83;
        let t200 = 1.0 / t170;
        let t201 = t83 * t200;
        let t206 = -2.0 / 3.0 * t98 * param_omega * t100 * t8 - t107 * t108 / 3.0 + 2.0 / 3.0 * t112 * t113 * t71 + t117 * t118 * t119 / 3.0 + t123 * t124 * t108 / 3.0 + t13 * t167 + 7.0 / 108.0 * t169 * t173 * t8 * t179 + 7.0 / 108.0 * t184 * t177 * t172 * sigma[ip] * param_d + 7.0 / 54.0 * t77 * t192 * t195 * sigma[ip] - 7.0 / 12.0 * t77 * t10 * t201 * sigma[ip];
        let tvrho0 = t92 * t206 + tzk0;
        vrho[ip] += tvrho0;
        let t216 = t61 * t57;
        let t222 = -t20 * t26 + t43 * t20 * t39 / 8.0 + t48 * t20 * t52 / 144.0 - t56 * (4.0 / 3.0 * t64 - t216 * t45 / 2.0) / 8.0;
        let t223 = t16 * t222;
        let t228 = t13 * t223 + 7.0 / 36.0 * t77 * t10 * t86;
        let tvsigma0 = t92 * t228;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (86 lines) ---
        let t231 = sigma[ip] * t194;
        let t236 = t133 * param_d;
        let t238 = 1.0 / t105 / t7;
        let t239 = t236 * t238;
        let t240 = t239 * t85;
        let t242 = t134 * t20;
        let t245 = 1.0 / t2 / t17;
        let t246 = t23 * t245;
        let t248 = -7.0 / 324.0 * t240 + 7.0 / 108.0 * t242 - 7.0 / 162.0 * t246;
        let t253 = -t240 / 81.0 + t242 / 27.0 - 2.0 / 81.0 * t246;
        let t254 = t253 * sigma[ip];
        let t259 = t194 * t39;
        let t265 = 2.0 / 9.0 * t240 - 2.0 / 3.0 * t242 + 4.0 / 9.0 * t246;
        let t266 = t265 * sigma[ip];
        let t271 = t194 * t52;
        let t277 = t57 * t194 * t39;
        let t283 = -88.0 / 9.0 * t231 * t26 + 16.0 / 3.0 * t130 * t140 - t21 * t248 + t254 * t45 / 8.0 - 2.0 / 3.0 * t143 * t146 + 11.0 / 9.0 * t44 * t259 + t266 * t53 / 144.0 - t151 * t154 / 27.0 + 11.0 / 162.0 * t49 * t271 - t56 * (352.0 / 27.0 * t58 * t259 - 44.0 / 9.0 * t62 * t277) / 8.0;
        let t284 = t16 * t283;
        let t287 = t13 * param_c * t20;
        let t288 = t15 * t106;
        let t290 = t288 * t70 * param_d;
        let t293 = t245 * t15;
        let t297 = param_d * t245;
        let t302 = t106 * param_d;
        let t306 = t8 * t166;
        let t310 = param_c * param_c;
        let t311 = t13 * t310;
        let t312 = t20 * t15;
        let t316 = t106 * t166;
        let t320 = t238 * t70;
        let t321 = t133 * t20;
        let t327 = t5 * t238;
        let t330 = t170 * rho[ip];
        let t332 = 1.0 / t2 / t330;
        let t333 = t332 * t15;
        let t344 = t111 * t10 * t20;
        let t345 = t169 * t71;
        let t348 = t13 * t284 + 2.0 / 9.0 * t287 * t290 - 4.0 / 9.0 * t117 * t293 * t119 - 4.0 / 9.0 * t123 * t124 * t297 - 4.0 / 9.0 * t98 * param_omega * t20 * t302 + 2.0 / 3.0 * t117 * t118 * t306 + t311 * t312 * t119 / 9.0 + 2.0 / 3.0 * t123 * t316 * t108 + 2.0 / 9.0 * t123 * t320 * t321 + 4.0 / 9.0 * t107 * t297 - 2.0 / 9.0 * t327 * t321 - 77.0 / 162.0 * t169 * t333 * t8 * t179 - 77.0 / 162.0 * t184 * t177 * t332 * sigma[ip] * param_d + 4.0 / 9.0 * t344 * t345;
        let t349 = t124 * param_d;
        let t350 = t73 * t349;
        let t353 = param_b * t310;
        let t355 = 1.0 / t18 / t330;
        let t356 = t355 * t15;
        let t361 = t170 * t17;
        let t362 = 1.0 / t361;
        let t363 = t362 * t15;
        let t366 = t75 * t190;
        let t367 = t191 * t83;
        let t369 = t366 * t367 * sigma[ip];
        let t372 = t238 * t75;
        let t374 = t73 * t372 * param_m2;
        let t375 = t355 * sigma[ip];
        let t381 = t73 * t182 * t190;
        let t383 = t83 * sigma[ip];
        let t384 = t383 * param_d;
        let t388 = t93 * param_m1;
        let t390 = t75 * t388 * t191;
        let t399 = t245 * param_b;
        let t403 = t355 * t83;
        let t408 = 1.0 / t330;
        let t409 = t83 * t408;
        let t418 = t366 * t191 * t85;
        let t419 = t83 * param_b;
        let t420 = t419 * t71;
        let t423 = t78 * t78;
        let t424 = t423 * param_m2;
        let t425 = t79 * t79;
        let t426 = t425 * param_omega;
        let t427 = t424 * t426;
        let t429 = 1.0 / t2 / t361;
        let t430 = t429 * t83;
        let t436 = t169 * t356 * t106;
        let t437 = t176 * param_omega;
        let t438 = t437 * t384;
        let t441 = 4.0 / 9.0 * t344 * t350 + 7.0 / 324.0 * t353 * t356 * t8 * t179 + 7.0 / 81.0 * t169 * t363 * t8 * t369 + 7.0 / 162.0 * t374 * t177 * t375 * t133 + 7.0 / 81.0 * t381 * t191 * t362 * t384 - 4.0 / 9.0 * t390 * t85 * t96 * t8 + 8.0 / 9.0 * t98 * param_omega * t245 * t8 - 8.0 / 9.0 * t112 * t399 * t71 - 161.0 / 162.0 * t77 * t192 * t403 * sigma[ip] + 7.0 / 3.0 * t77 * t10 * t409 * sigma[ip] + 4.0 / 3.0 * t112 * t113 * t167 + 4.0 / 9.0 * t418 * t420 + 7.0 / 81.0 * t77 * t427 * t430 * sigma[ip] + 7.0 / 162.0 * t436 * t438;
        let t442 = t348 + t441;
        let tv2rho20 = 2.0 * param_a * t206 + t92 * t442;
        v2rho2[ip] += tv2rho20;
        let t448 = t8 * t222;
        let t452 = t106 * t222;
        let t477 = 8.0 / 3.0 * t129 * t26 - t20 * t140 + t142 * t20 * t39 / 8.0 - t43 * t129 * t39 / 3.0 + t150 * t20 * t52 / 144.0 - t48 * t129 * t52 / 54.0 - t56 * (-32.0 / 9.0 * t160 + 4.0 / 3.0 * t216 * t146) / 8.0;
        let t478 = t16 * t477;
        let t482 = t76 * t10 * t83;
        let t485 = t73 * t182;
        let t497 = 2.0 / 3.0 * t112 * t113 * t223 + t117 * t118 * t448 / 3.0 + t123 * t452 * t108 / 3.0 + t13 * t478 + 7.0 / 108.0 * t169 * t173 * t482 + 7.0 / 108.0 * t485 * t10 * t83 * t172 * param_d + 7.0 / 54.0 * t77 * t192 * t195 - 7.0 / 12.0 * t77 * t10 * t201;
        let tv2rhosigma0 = param_a * t228 + t92 * t497;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
    }
}
