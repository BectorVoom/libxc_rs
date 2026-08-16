//! GGA_X_CAP kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_cap.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_cap_kxc_unpol(
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
    param_alphaoAx: f64,
    param_c: f64,
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
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3::<f64>(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = t20 * t20;
        let t22 = param_alphaoAx * t21;
        let t23 = M_PI * M_PI;
        let t24 = pow_1_3::<f64>(t23);
        let t25 = 1.0 / t24;
        let t26 = f64::sqrt(sigma[ip]);
        let t28 = t22 * t25 * t26;
        let t29 = M_CBRT2;
        let t31 = 1.0 / t18 / rho[ip];
        let t33 = t21 * t25;
        let t38 = 1.0 + t33 * t26 * t29 * t31 / 12.0;
        let t39 = f64::ln(t38);
        let t41 = param_c * t39 + 1.0;
        let t42 = 1.0 / t41;
        let t43 = t39 * t42;
        let t44 = t29 * t31 * t43;
        let t47 = 1.0 - t28 * t44 / 12.0;
        let t51 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t47);
        let tzk0 = 2.0 * t51;
        zk[ip] += tzk0;
        let t52 = t18 * t18;
        let t54 = t17 / t52;
        let t58 = rho[ip] * rho[ip];
        let t62 = t29 / t18 / t58 * t43;
        let t65 = param_alphaoAx * t20;
        let t66 = t24 * t24;
        let t67 = 1.0 / t66;
        let t68 = t67 * sigma[ip];
        let t69 = t65 * t68;
        let t70 = t29 * t29;
        let t71 = t58 * rho[ip];
        let t73 = 1.0 / t52 / t71;
        let t75 = 1.0 / t38;
        let t76 = t75 * t42;
        let t77 = t70 * t73 * t76;
        let t81 = t65 * t68 * t70;
        let t83 = t41 * t41;
        let t84 = 1.0 / t83;
        let t85 = t84 * param_c;
        let t86 = t85 * t75;
        let t87 = t73 * t39 * t86;
        let t90 = t28 * t62 / 9.0 + t69 * t77 / 18.0 - t81 * t87 / 18.0;
        let t95 = piecewise3::<f64>(t2, 0.0, -t6 * t54 * t47 / 8.0 - 3.0 / 8.0 * t6 * t19 * t90);
        let tvrho0 = 2.0 * rho[ip] * t95 + 2.0 * t51;
        vrho[ip] += tvrho0;
        let t98 = 1.0 / t26;
        let t100 = t22 * t25 * t98;
        let t103 = t65 * t67;
        let t105 = 1.0 / t52 / t58;
        let t107 = t70 * t105 * t76;
        let t110 = t67 * t70;
        let t111 = t65 * t110;
        let t113 = t105 * t39 * t86;
        let t116 = -t100 * t44 / 24.0 - t103 * t107 / 48.0 + t111 * t113 / 48.0;
        let t120 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t116);
        let tvsigma0 = 2.0 * rho[ip] * t120;
        vsigma[ip] += tvsigma0;
        let t125 = t17 / t52 / rho[ip];
        let t135 = t29 / t18 / t71 * t43;
        let t138 = t58 * t58;
        let t140 = 1.0 / t52 / t138;
        let t142 = t70 * t140 * t76;
        let t146 = t140 * t39 * t86;
        let t150 = param_alphaoAx / t23;
        let t151 = t26 * sigma[ip];
        let t152 = t150 * t151;
        let t153 = t138 * t58;
        let t154 = 1.0 / t153;
        let t155 = t38 * t38;
        let t156 = 1.0 / t155;
        let t157 = t154 * t156;
        let t158 = t157 * t42;
        let t161 = t157 * t85;
        let t165 = t150 * t151 * t154;
        let t167 = 1.0 / t83 / t41;
        let t169 = param_c * param_c;
        let t171 = t39 * t167 * t169 * t156;
        let t176 = t39 * t84 * param_c * t156;
        let t179 = -7.0 / 27.0 * t28 * t135 - 5.0 / 18.0 * t69 * t142 + 5.0 / 18.0 * t81 * t146 + 2.0 / 27.0 * t152 * t158 + 4.0 / 27.0 * t152 * t161 - 4.0 / 27.0 * t165 * t171 - 2.0 / 27.0 * t165 * t176;
        let t184 = piecewise3::<f64>(t2, 0.0, t6 * t125 * t47 / 12.0 - t6 * t54 * t90 / 4.0 - 3.0 / 8.0 * t6 * t19 * t179);
        let tv2rho20 = 2.0 * rho[ip] * t184 + 4.0 * t95;
        v2rho2[ip] += tv2rho20;
        let t196 = t138 * rho[ip];
        let t197 = 1.0 / t196;
        let t198 = t150 * t197;
        let t199 = t156 * t42;
        let t200 = t199 * t26;
        let t203 = t156 * t84;
        let t205 = t203 * param_c * t26;
        let t209 = t150 * t197 * t39;
        let t210 = t167 * t169;
        let t212 = t210 * t156 * t26;
        let t217 = t100 * t62 / 18.0 + t103 * t77 / 12.0 - t111 * t87 / 12.0 - t198 * t200 / 36.0 - t198 * t205 / 18.0 + t209 * t212 / 18.0 + t209 * t205 / 36.0;
        let t222 = piecewise3::<f64>(t2, 0.0, -t6 * t54 * t116 / 8.0 - 3.0 / 8.0 * t6 * t19 * t217);
        let tv2rhosigma0 = 2.0 * rho[ip] * t222 + 2.0 * t120;
        v2rhosigma[ip] += tv2rhosigma0;
        let t225 = 1.0 / t151;
        let t227 = t22 * t25 * t225;
        let t230 = 1.0 / sigma[ip];
        let t231 = t67 * t230;
        let t232 = t65 * t231;
        let t236 = t65 * t231 * t70;
        let t239 = 1.0 / t138;
        let t240 = t150 * t239;
        let t245 = t203 * param_c * t98;
        let t249 = t150 * t239 * t39;
        let t256 = t227 * t44 / 48.0 - t232 * t107 / 96.0 + t236 * t113 / 96.0 + t240 * t199 * t98 / 96.0 + t240 * t245 / 48.0 - t249 * t210 * t156 * t98 / 48.0 - t249 * t245 / 96.0;
        let t260 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t256);
        let tv2sigma20 = 2.0 * rho[ip] * t260;
        v2sigma2[ip] += tv2sigma20;
        let t263 = t17 * t105;
        let t276 = t29 / t18 / t138 * t43;
        let t280 = 1.0 / t52 / t196;
        let t282 = t70 * t280 * t76;
        let t286 = t280 * t39 * t86;
        let t289 = t138 * t71;
        let t290 = 1.0 / t289;
        let t291 = t290 * t156;
        let t299 = t150 * t151 * t290;
        let t304 = sigma[ip] * sigma[ip];
        let t305 = t138 * t138;
        let t307 = 1.0 / t18 / t305;
        let t308 = t304 * t307;
        let t311 = 1.0 / t155 / t38;
        let t313 = t33 * t29;
        let t314 = t311 * t42 * t313;
        let t318 = t150 * t308 * t311;
        let t319 = t85 * t313;
        let t322 = t210 * t313;
        let t326 = t150 * t308 * t39;
        let t327 = t83 * t83;
        let t328 = 1.0 / t327;
        let t329 = t169 * param_c;
        let t330 = t328 * t329;
        let t332 = t330 * t311 * t313;
        let t336 = t210 * t311 * t313;
        let t340 = t85 * t311 * t313;
        let t343 = 70.0 / 81.0 * t28 * t276 + 119.0 / 81.0 * t69 * t282 - 119.0 / 81.0 * t81 * t286 - 22.0 / 27.0 * t152 * t291 * t42 - 44.0 / 27.0 * t152 * t291 * t85 + 44.0 / 27.0 * t299 * t171 + 22.0 / 27.0 * t299 * t176 + 4.0 / 243.0 * t150 * t308 * t314 + 4.0 / 81.0 * t318 * t319 + 4.0 / 81.0 * t318 * t322 - 4.0 / 81.0 * t326 * t332 - 4.0 / 81.0 * t326 * t336 - 4.0 / 243.0 * t326 * t340;
        let t348 = piecewise3::<f64>(t2, 0.0, -5.0 / 36.0 * t6 * t263 * t47 + t6 * t125 * t90 / 4.0 - 3.0 / 8.0 * t6 * t54 * t179 - 3.0 / 8.0 * t6 * t19 * t343);
        let tv3rho30 = 2.0 * rho[ip] * t348 + 6.0 * t184;
        v3rho3[ip] += tv3rho30;
        let t364 = t150 * t154;
        let t370 = t150 * t154 * t39;
        let t376 = 1.0 / t18 / t289;
        let t377 = t376 * t311;
        let t378 = t150 * t377;
        let t380 = t42 * sigma[ip] * t313;
        let t386 = sigma[ip] * param_c * t313;
        let t392 = t169 * sigma[ip] * t313;
        let t395 = t376 * t39;
        let t397 = t150 * t395 * t328;
        let t398 = t329 * t311;
        let t400 = t398 * sigma[ip] * t313;
        let t404 = t150 * t395 * t167;
        let t405 = t169 * t311;
        let t407 = t405 * sigma[ip] * t313;
        let t411 = t150 * t395 * t84;
        let t412 = param_c * t311;
        let t414 = t412 * sigma[ip] * t313;
        let t417 = -7.0 / 54.0 * t100 * t135 - 37.0 / 108.0 * t103 * t142 + 37.0 / 108.0 * t111 * t146 + t364 * t200 / 4.0 + t364 * t205 / 2.0 - t370 * t212 / 2.0 - t370 * t205 / 4.0 - t378 * t380 / 162.0 - t150 * t377 * t84 * t386 / 54.0 - t150 * t377 * t167 * t392 / 54.0 + t397 * t400 / 54.0 + t404 * t407 / 54.0 + t411 * t414 / 162.0;
        let t422 = piecewise3::<f64>(t2, 0.0, t6 * t125 * t116 / 12.0 - t6 * t54 * t217 / 4.0 - 3.0 / 8.0 * t6 * t19 * t417);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t422 + 4.0 * t222;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t434 = t150 * t98;
        let t435 = t197 * t156;
        let t436 = t435 * t42;
        let t439 = t435 * t85;
        let t443 = t150 * t98 * t197;
        let t449 = 1.0 / t18 / t153;
        let t450 = t449 * t311;
        let t451 = t150 * t450;
        let t454 = t42 * t21 * t25 * t29;
        let t461 = t449 * t39;
        let t463 = t150 * t461 * t328;
        let t464 = t398 * t313;
        let t468 = t150 * t461 * t167;
        let t469 = t405 * t313;
        let t473 = t150 * t461 * t84;
        let t474 = t412 * t313;
        let t477 = -t227 * t62 / 36.0 + t232 * t77 / 72.0 - t236 * t87 / 72.0 - t434 * t436 / 18.0 - t434 * t439 / 9.0 + t443 * t171 / 9.0 + t443 * t176 / 18.0 + t451 * t454 / 432.0 + t451 * t319 / 144.0 + t451 * t322 / 144.0 - t463 * t464 / 144.0 - t468 * t469 / 144.0 - t473 * t474 / 432.0;
        let t482 = piecewise3::<f64>(t2, 0.0, -t6 * t54 * t256 / 8.0 - 3.0 / 8.0 * t6 * t19 * t477);
        let tv3rhosigma20 = 2.0 * rho[ip] * t482 + 2.0 * t260;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t485 = t26 * t304;
        let t486 = 1.0 / t485;
        let t488 = t22 * t25 * t486;
        let t491 = 1.0 / t304;
        let t492 = t67 * t491;
        let t493 = t65 * t492;
        let t497 = t65 * t492 * t70;
        let t501 = 1.0 / t18 / t196;
        let t502 = t501 * t311;
        let t503 = t150 * t502;
        let t505 = t42 * t230 * t313;
        let t509 = t150 * t502 * t84;
        let t511 = t230 * param_c * t313;
        let t515 = t150 * t502 * t167;
        let t517 = t169 * t230 * t313;
        let t520 = t501 * t39;
        let t522 = t150 * t520 * t328;
        let t524 = t398 * t230 * t313;
        let t528 = t150 * t520 * t167;
        let t530 = t405 * t230 * t313;
        let t534 = t150 * t520 * t84;
        let t536 = t412 * t230 * t313;
        let t539 = -t488 * t44 / 32.0 + t493 * t107 / 64.0 - t497 * t113 / 64.0 - t503 * t505 / 1152.0 - t509 * t511 / 384.0 - t515 * t517 / 384.0 + t522 * t524 / 384.0 + t528 * t530 / 384.0 + t534 * t536 / 1152.0;
        let t543 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t539);
        let tv3sigma30 = 2.0 * rho[ip] * t543;
        v3sigma3[ip] += tv3sigma30;
    }
}
