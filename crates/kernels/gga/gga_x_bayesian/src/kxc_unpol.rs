//! GGA_X_BAYESIAN kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_bayesian.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_bayesian_kxc_unpol(
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
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3::<f64>(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = t25 * sigma[ip];
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t33 = t28 * t32;
        let t34 = t20 * t20;
        let t35 = 1.0 / t22;
        let t36 = t34 * t35;
        let t37 = f64::sqrt(sigma[ip]);
        let t44 = 1.0 + t36 * t37 * t27 / t18 / rho[ip] / 12.0;
        let t45 = t44 * t44;
        let t46 = 1.0 / t45;
        let t47 = t33 * t46;
        let t50 = 0.1926e0 + 0.79008333333333333333e-1 * t26 * t47;
        let t51 = t46 * t50;
        let t55 = 0.10008e1 + t26 * t33 * t51 / 24.0;
        let t59 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t55);
        let tzk0 = 2.0 * t59;
        zk[ip] += tzk0;
        let t61 = t17 / t30;
        let t65 = t29 * rho[ip];
        let t67 = 1.0 / t30 / t65;
        let t68 = t28 * t67;
        let t72 = 1.0 / t21;
        let t73 = t37 * sigma[ip];
        let t74 = t72 * t73;
        let t75 = t29 * t29;
        let t76 = t75 * rho[ip];
        let t77 = 1.0 / t76;
        let t79 = 1.0 / t45 / t44;
        let t80 = t77 * t79;
        let t84 = t68 * t46;
        let t89 = -0.21068888888888888889e0 * t26 * t84 + 0.21068888888888888889e0 * t74 * t80;
        let t90 = t46 * t89;
        let t94 = -t26 * t68 * t51 / 9.0 + t74 * t80 * t50 / 9.0 + t26 * t33 * t90 / 24.0;
        let t99 = piecewise3::<f64>(t2, 0.0, -t6 * t61 * t55 / 8.0 - 3.0 / 8.0 * t6 * t19 * t94);
        let tvrho0 = 2.0 * rho[ip] * t99 + 2.0 * t59;
        vrho[ip] += tvrho0;
        let t102 = t25 * t28;
        let t103 = t32 * t46;
        let t106 = t72 * t37;
        let t107 = 1.0 / t75;
        let t108 = t107 * t79;
        let t115 = 0.79008333333333333333e-1 * t25 * t47 - 0.79008333333333333333e-1 * t106 * t108;
        let t116 = t46 * t115;
        let t120 = t102 * t103 * t50 / 24.0 - t106 * t108 * t50 / 24.0 + t26 * t33 * t116 / 24.0;
        let t124 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t120);
        let tvsigma0 = 2.0 * rho[ip] * t124;
        vsigma[ip] += tvsigma0;
        let t129 = t17 / t30 / rho[ip];
        let t137 = 1.0 / t30 / t75;
        let t138 = t28 * t137;
        let t142 = t75 * t29;
        let t143 = 1.0 / t142;
        let t144 = t143 * t79;
        let t151 = sigma[ip] * sigma[ip];
        let t152 = t72 * t151;
        let t153 = t75 * t65;
        let t155 = 1.0 / t18 / t153;
        let t156 = t45 * t45;
        let t157 = 1.0 / t156;
        let t159 = t152 * t155 * t157;
        let t161 = t35 * t27;
        let t162 = t50 * t34 * t161;
        let t168 = t138 * t46;
        let t175 = t157 * t34 * t161;
        let t178 = 0.77252592592592592593e0 * t26 * t168 - 0.16152814814814814814e1 * t74 * t144 + 0.7022962962962962963e-1 * t152 * t155 * t175;
        let t179 = t46 * t178;
        let t183 = 11.0 / 27.0 * t26 * t138 * t51 - 23.0 / 27.0 * t74 * t144 * t50 - 2.0 / 9.0 * t26 * t68 * t90 + t159 * t162 / 27.0 + 2.0 / 9.0 * t74 * t80 * t89 + t26 * t33 * t179 / 24.0;
        let t188 = piecewise3::<f64>(t2, 0.0, t6 * t129 * t55 / 12.0 - t6 * t61 * t94 / 4.0 - 3.0 / 8.0 * t6 * t19 * t183);
        let tv2rho20 = 2.0 * rho[ip] * t188 + 4.0 * t99;
        v2rho2[ip] += tv2rho20;
        let t194 = t67 * t46;
        let t198 = t72 * t77;
        let t199 = t79 * t50;
        let t200 = t199 * t37;
        let t206 = t72 * sigma[ip];
        let t208 = 1.0 / t18 / t142;
        let t210 = t206 * t208 * t157;
        let t224 = t79 * t37;
        let t230 = -0.21068888888888888889e0 * t25 * t84 + 0.52672222222222222222e0 * t198 * t224 - 0.26336111111111111111e-1 * t206 * t208 * t175;
        let t231 = t46 * t230;
        let t235 = -t102 * t194 * t50 / 9.0 + 5.0 / 18.0 * t198 * t200 + t102 * t103 * t89 / 24.0 - t210 * t162 / 72.0 - t106 * t108 * t89 / 24.0 - t26 * t68 * t116 / 9.0 + t74 * t80 * t115 / 9.0 + t26 * t33 * t231 / 24.0;
        let t240 = piecewise3::<f64>(t2, 0.0, -t6 * t61 * t120 / 8.0 - 3.0 / 8.0 * t6 * t19 * t235);
        let tv2rhosigma0 = 2.0 * rho[ip] * t240 + 2.0 * t124;
        v2rhosigma[ip] += tv2rhosigma0;
        let t243 = t72 * t107;
        let t244 = 1.0 / t37;
        let t245 = t199 * t244;
        let t253 = t72 / t18 / t76;
        let t254 = t253 * t157;
        let t260 = t79 * t244;
        let t263 = t36 * t27;
        let t266 = -0.1185125e0 * t243 * t260 + 0.98760416666666666666e-2 * t254 * t263;
        let t267 = t46 * t266;
        let t271 = -t243 * t245 / 16.0 + t102 * t103 * t115 / 12.0 + t254 * t162 / 192.0 - t106 * t108 * t115 / 12.0 + t26 * t33 * t267 / 24.0;
        let t275 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t271);
        let tv2sigma20 = 2.0 * rho[ip] * t275;
        v2sigma2[ip] += tv2sigma20;
        let t278 = t17 * t32;
        let t289 = 1.0 / t30 / t76;
        let t290 = t28 * t289;
        let t294 = 1.0 / t153;
        let t295 = t294 * t79;
        let t302 = t75 * t75;
        let t304 = 1.0 / t18 / t302;
        let t306 = t152 * t304 * t157;
        let t315 = t37 * t151;
        let t316 = t72 * t315;
        let t317 = t302 * rho[ip];
        let t319 = 1.0 / t30 / t317;
        let t321 = 1.0 / t156 / t44;
        let t323 = t316 * t319 * t321;
        let t325 = t24 * t28;
        let t326 = t50 * t20 * t325;
        let t330 = t89 * t34 * t161;
        let t336 = t290 * t46;
        let t346 = t321 * t20 * t325;
        let t349 = -0.36051209876543209877e1 * t26 * t336 + 0.11751758024691358024e2 * t74 * t295 - 0.10534444444444444444e1 * t152 * t304 * t175 + 0.18727901234567901235e0 * t316 * t319 * t346;
        let t350 = t46 * t349;
        let t354 = -154.0 / 81.0 * t26 * t290 * t51 + 502.0 / 81.0 * t74 * t295 * t50 + 11.0 / 9.0 * t26 * t138 * t90 - 5.0 / 9.0 * t306 * t162 - 23.0 / 9.0 * t74 * t144 * t89 - t26 * t68 * t179 / 3.0 + 8.0 / 81.0 * t323 * t326 + t159 * t330 / 9.0 + t74 * t80 * t178 / 3.0 + t26 * t33 * t350 / 24.0;
        let t359 = piecewise3::<f64>(t2, 0.0, -5.0 / 36.0 * t6 * t278 * t55 + t6 * t129 * t94 / 4.0 - 3.0 / 8.0 * t6 * t61 * t183 - 3.0 / 8.0 * t6 * t19 * t354);
        let tv3rho30 = 2.0 * rho[ip] * t359 + 6.0 * t188;
        v3rho3[ip] += tv3rho30;
        let t369 = t137 * t46;
        let t373 = t72 * t143;
        let t379 = t72 * t155;
        let t380 = t157 * t50;
        let t383 = sigma[ip] * t34 * t161;
        let t386 = t79 * t89;
        let t387 = t386 * t37;
        let t394 = 1.0 / t30 / t302;
        let t396 = t74 * t394 * t321;
        let t414 = t115 * t34 * t161;
        let t424 = t379 * t157;
        let t430 = 0.77252592592592592593e0 * t25 * t168 - 0.31954481481481481481e1 * t373 * t224 + 0.34236944444444444444e0 * t424 * t383 - 0.70229629629629629629e-1 * t74 * t394 * t346;
        let t431 = t46 * t430;
        let t435 = 11.0 / 27.0 * t102 * t369 * t50 - 91.0 / 54.0 * t373 * t200 - 2.0 / 9.0 * t102 * t194 * t89 + 13.0 / 72.0 * t379 * t380 * t383 + 5.0 / 9.0 * t198 * t387 + t102 * t103 * t178 / 24.0 - t396 * t326 / 27.0 - t210 * t330 / 36.0 - t106 * t108 * t178 / 24.0 + 11.0 / 27.0 * t26 * t138 * t116 - 23.0 / 27.0 * t74 * t144 * t115 - 2.0 / 9.0 * t26 * t68 * t231 + t159 * t414 / 27.0 + 2.0 / 9.0 * t74 * t80 * t230 + t26 * t33 * t431 / 24.0;
        let t440 = piecewise3::<f64>(t2, 0.0, t6 * t129 * t120 / 12.0 - t6 * t61 * t235 / 4.0 - 3.0 / 8.0 * t6 * t19 * t435);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t440 + 4.0 * t240;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t448 = t72 * t208;
        let t449 = t448 * t157;
        let t452 = t386 * t244;
        let t458 = t79 * t115;
        let t459 = t458 * t37;
        let t467 = t72 / t30 / t153;
        let t468 = t321 * t50;
        let t469 = t467 * t468;
        let t471 = t25 * t28 * t37;
        let t491 = t467 * t321;
        let t494 = 0.47405e0 * t198 * t260 - 0.92176388888888888889e-1 * t449 * t263 + 0.26336111111111111111e-1 * t491 * t471;
        let t495 = t46 * t494;
        let t499 = t198 * t245 / 4.0 - 7.0 / 144.0 * t449 * t162 - t243 * t452 / 16.0 - 2.0 / 9.0 * t102 * t194 * t115 + 5.0 / 9.0 * t198 * t459 + t102 * t103 * t230 / 12.0 + t469 * t471 / 72.0 + t254 * t330 / 192.0 - t210 * t414 / 36.0 - t106 * t108 * t230 / 12.0 - t26 * t68 * t267 / 9.0 + t74 * t80 * t266 / 9.0 + t26 * t33 * t495 / 24.0;
        let t504 = piecewise3::<f64>(t2, 0.0, -t6 * t61 * t271 / 8.0 - 3.0 / 8.0 * t6 * t19 * t499);
        let tv3rhosigma20 = 2.0 * rho[ip] * t504 + 2.0 * t275;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t507 = t253 * t380;
        let t508 = 1.0 / sigma[ip];
        let t510 = t508 * t34 * t161;
        let t513 = t458 * t244;
        let t516 = 1.0 / t73;
        let t517 = t199 * t516;
        let t524 = 1.0 / t30 / t142;
        let t525 = t72 * t524;
        let t526 = t525 * t468;
        let t528 = t25 * t28 * t244;
        let t538 = t79 * t516;
        let t541 = t525 * t321;
        let t544 = 0.148140625e-1 * t254 * t510 + 0.5925625e-1 * t243 * t538 - 0.98760416666666666666e-2 * t541 * t528;
        let t545 = t46 * t544;
        let t549 = t507 * t510 / 128.0 - 3.0 / 16.0 * t243 * t513 + t243 * t517 / 32.0 + t102 * t103 * t266 / 8.0 - t526 * t528 / 192.0 + t254 * t414 / 64.0 - t106 * t108 * t266 / 8.0 + t26 * t33 * t545 / 24.0;
        let t553 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t549);
        let tv3sigma30 = 2.0 * rho[ip] * t553;
        v3sigma3[ip] += tv3sigma30;
    }
}
