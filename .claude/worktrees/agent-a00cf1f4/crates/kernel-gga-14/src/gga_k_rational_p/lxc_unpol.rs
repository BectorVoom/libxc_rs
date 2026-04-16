//! GGA_K_RATIONAL_P lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_rational_p.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_rational_p_lxc_unpol(
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
    param_C2: f64,
    param_p: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t24 = 1.0 / param_p;
        let t26 = M_CBRT6;
        let t28 = M_PI * M_PI;
        let t29 = pow_1_3(t28);
        let t30 = t29 * t29;
        let t31 = 1.0 / t30;
        let t32 = t31 * sigma[ip];
        let t33 = M_CBRT2;
        let t34 = t33 * t33;
        let t35 = rho[ip] * rho[ip];
        let t42 = 1.0 + param_C2 * t24 * t26 * t32 * t34 / t22 / t35 / 24.0;
        let t43 = f64::powf(t42, -param_p);
        let t47 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t20 * t22 * t43);
        let tzk0 = 2.0 * t47;
        zk[ip] += tzk0;
        let t53 = t35 * rho[ip];
        let t57 = t7 * t20 / t53 * t43;
        let t58 = param_C2 * t26;
        let t60 = sigma[ip] * t34;
        let t61 = 1.0 / t42;
        let t63 = t58 * t31 * t60 * t61;
        let t67 = piecewise3(t2, 0.0, t7 * t20 / t21 * t43 / 10.0 + t57 * t63 / 60.0);
        let tvrho0 = 2.0 * rho[ip] * t67 + 2.0 * t47;
        vrho[ip] += tvrho0;
        let t74 = t31 * t34;
        let t76 = t58 * t74 * t61;
        let t79 = piecewise3(t2, 0.0, -t7 * t20 / t35 * t43 * t76 / 160.0);
        let tvsigma0 = 2.0 * rho[ip] * t79;
        vsigma[ip] += tvsigma0;
        let t88 = t35 * t35;
        let t92 = t7 * t20 / t88 * t43;
        let t95 = t88 * t35;
        let t97 = 1.0 / t22 / t95;
        let t100 = t7 * t20 * t97 * t43;
        let t101 = param_C2 * param_C2;
        let t102 = t26 * t26;
        let t103 = t101 * t102;
        let t105 = 1.0 / t29 / t28;
        let t106 = t103 * t105;
        let t107 = sigma[ip] * sigma[ip];
        let t109 = t42 * t42;
        let t110 = 1.0 / t109;
        let t112 = t106 * t107 * t33 * t110;
        let t115 = t7 * t20;
        let t118 = t115 * t97 * t43 * t101;
        let t119 = t102 * t105;
        let t122 = t33 * t110 * t24;
        let t123 = t119 * t107 * t122;
        let t127 = piecewise3(t2, 0.0, -t7 * t20 / t21 / rho[ip] * t43 / 30.0 - 7.0 / 180.0 * t92 * t63 + t100 * t112 / 270.0 + t118 * t123 / 270.0);
        let tv2rho20 = 2.0 * rho[ip] * t127 + 4.0 * t67;
        v2rho2[ip] += tv2rho20;
        let t132 = t88 * rho[ip];
        let t134 = 1.0 / t22 / t132;
        let t137 = t7 * t20 * t134 * t43;
        let t140 = t106 * sigma[ip] * t33 * t110;
        let t149 = t119 * t33 * t110 * t24 * sigma[ip];
        let t153 = piecewise3(t2, 0.0, t57 * t76 / 80.0 - t137 * t140 / 720.0 - t115 * t134 * t43 * t101 * t149 / 720.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t153 + 2.0 * t79;
        v2rhosigma[ip] += tv2rhosigma0;
        let t160 = t7 * t20 / t22 / t88 * t43;
        let t163 = t103 * t105 * t33 * t110;
        let t165 = t106 * t122;
        let t169 = piecewise3(t2, 0.0, t160 * t163 / 1920.0 + t160 * t165 / 1920.0);
        let tv2sigma20 = 2.0 * rho[ip] * t169;
        v2sigma2[ip] += tv2sigma20;
        let t181 = t7 * t20 / t132 * t43;
        let t184 = t88 * t53;
        let t186 = 1.0 / t22 / t184;
        let t189 = t7 * t20 * t186 * t43;
        let t194 = t115 * t186 * t43 * t101;
        let t197 = t5 * t5;
        let t200 = t4 / t197 / t28;
        let t201 = t88 * t88;
        let t202 = t201 * t35;
        let t206 = t200 * t20 / t21 / t202;
        let t207 = t101 * param_C2;
        let t208 = t43 * t207;
        let t209 = t107 * sigma[ip];
        let t211 = 1.0 / t109 / t42;
        let t212 = t209 * t211;
        let t213 = t208 * t212;
        let t217 = t208 * t212 * t24;
        let t220 = param_p * param_p;
        let t221 = 1.0 / t220;
        let t223 = t208 * t212 * t221;
        let t227 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t20 / t21 / t35 * t43 + 41.0 / 270.0 * t181 * t63 - t189 * t112 / 30.0 - t194 * t123 / 30.0 + 2.0 / 405.0 * t206 * t213 + 2.0 / 135.0 * t206 * t217 + 4.0 / 405.0 * t206 * t223);
        let tv3rho30 = 2.0 * rho[ip] * t227 + 6.0 * t127;
        v3rho3[ip] += tv3rho30;
        let t241 = t200 * t20 / t21 / t201 / rho[ip];
        let t242 = t107 * t211;
        let t243 = t208 * t242;
        let t247 = t208 * t242 * t24;
        let t250 = t211 * t221;
        let t252 = t208 * t250 * t107;
        let t256 = piecewise3(t2, 0.0, -3.0 / 80.0 * t92 * t76 + 23.0 / 2160.0 * t100 * t140 + 23.0 / 2160.0 * t118 * t149 - t241 * t243 / 540.0 - t241 * t247 / 180.0 - t241 * t252 / 270.0);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t256 + 4.0 * t153;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t262 = 1.0 / t21 / t201;
        let t264 = t200 * t20 * t262;
        let t266 = t208 * sigma[ip] * t211;
        let t269 = t211 * t24;
        let t271 = t208 * t269 * sigma[ip];
        let t277 = t208 * t250 * sigma[ip];
        let t281 = piecewise3(t2, 0.0, -7.0 / 2880.0 * t137 * t163 + t264 * t266 / 1440.0 + t264 * t271 / 480.0 - 7.0 / 2880.0 * t137 * t165 + t264 * t277 / 720.0);
        let tv3rhosigma20 = 2.0 * rho[ip] * t281 + 2.0 * t169;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t284 = t200 * t20;
        let t286 = 1.0 / t21 / t184;
        let t288 = t207 * t211;
        let t293 = t200 * t20 * t286;
        let t294 = t208 * t269;
        let t297 = t208 * t250;
        let t301 = piecewise3(t2, 0.0, -t284 * t286 * t43 * t288 / 3840.0 - t293 * t294 / 1280.0 - t293 * t297 / 1920.0);
        let tv3sigma30 = 2.0 * rho[ip] * t301;
        v3sigma3[ip] += tv3sigma30;
        let t317 = 1.0 / t22 / t201;
        let t328 = t201 * t53;
        let t332 = t200 * t20 / t21 / t328;
        let t340 = 1.0 / t201 / t95;
        let t344 = t101 * t101;
        let t345 = t344 * t26;
        let t346 = t345 * t31;
        let t347 = t107 * t107;
        let t349 = t109 * t109;
        let t350 = 1.0 / t349;
        let t357 = t284 * t340 * t43 * t344;
        let t358 = t347 * t350;
        let t361 = t26 * t31 * t34;
        let t370 = 1.0 / t220 / param_p;
        let t375 = -14.0 / 135.0 * t7 * t20 / t21 / t53 * t43 - 611.0 / 810.0 * t7 * t20 / t95 * t43 * t63 + 703.0 / 2430.0 * t7 * t20 * t317 * t43 * t112 + 703.0 / 2430.0 * t115 * t317 * t43 * t101 * t123 - 116.0 / 1215.0 * t332 * t213 - 116.0 / 405.0 * t332 * t217 - 232.0 / 1215.0 * t332 * t223 + 2.0 / 3645.0 * t200 * t20 * t340 * t43 * t346 * t347 * t34 * t350 + 4.0 / 1215.0 * t357 * t358 * t24 * t361 + 22.0 / 3645.0 * t357 * t358 * t221 * t361 + 4.0 / 1215.0 * t357 * t358 * t370 * t361;
        let t376 = piecewise3(t2, 0.0, t375);
        let tv4rho40 = 2.0 * rho[ip] * t376 + 8.0 * t227;
        v4rho4[ip] += tv4rho40;
        let t393 = 1.0 / t201 / t132;
        let t404 = t284 * t393 * t43 * t344;
        let t405 = t209 * t350;
        let t414 = t350 * t370;
        let t420 = piecewise3(t2, 0.0, 3.0 / 20.0 * t181 * t76 - 257.0 / 3240.0 * t189 * t140 - 257.0 / 3240.0 * t194 * t149 + 17.0 / 540.0 * t206 * t243 + 17.0 / 180.0 * t206 * t247 + 17.0 / 270.0 * t206 * t252 - t200 * t20 * t393 * t43 * t346 * t209 * t34 * t350 / 4860.0 - t404 * t405 * t24 * t361 / 810.0 - 11.0 / 4860.0 * t404 * t405 * t221 * t361 - t404 * t414 * t209 * t361 / 810.0);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t420 + 6.0 * t256;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t431 = 1.0 / t201 / t88;
        let t442 = t284 * t431 * t43 * t344;
        let t448 = t350 * t221;
        let t462 = piecewise3(t2, 0.0, 119.0 / 8640.0 * t100 * t163 - 13.0 / 1440.0 * t241 * t266 - 13.0 / 480.0 * t241 * t271 + t200 * t20 * t431 * t43 * t346 * t107 * t34 * t350 / 12960.0 + t442 * t107 * t350 * t24 * t361 / 2160.0 + 11.0 / 12960.0 * t442 * t448 * t107 * t361 + 119.0 / 8640.0 * t100 * t165 - 13.0 / 720.0 * t241 * t277 + t442 * t414 * t107 * t361 / 2160.0);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t462 + 4.0 * t281;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t469 = 1.0 / t328;
        let t479 = t284 * t469 * t43 * t344;
        let t482 = t32 * t34;
        let t499 = piecewise3(t2, 0.0, 11.0 / 5760.0 * t284 * t262 * t43 * t288 - t200 * t20 * t469 * t43 * t346 * t60 * t350 / 34560.0 - t479 * t350 * t24 * t26 * t482 / 5760.0 + 11.0 / 1920.0 * t264 * t294 - 11.0 / 34560.0 * t479 * t448 * t26 * t482 + 11.0 / 2880.0 * t264 * t297 - t479 * t414 * t26 * t482 / 5760.0);
        let tv4rhosigma30 = 2.0 * rho[ip] * t499 + 2.0 * t301;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t505 = t200 * t20 / t202 * t43;
        let t510 = t344 * t350;
        let t524 = piecewise3(t2, 0.0, t505 * t345 * t74 * t350 / 92160.0 + t505 * t510 * t24 * t361 / 15360.0 + 11.0 / 92160.0 * t505 * t510 * t221 * t361 + t505 * t510 * t370 * t361 / 15360.0);
        let tv4sigma40 = 2.0 * rho[ip] * t524;
        v4sigma4[ip] += tv4sigma40;
    }
}
