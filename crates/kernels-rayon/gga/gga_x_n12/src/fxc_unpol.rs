//! GGA_X_N12 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_n12.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_n12_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_CC_0_1: f64,
    param_CC_0_2: f64,
    param_CC_0_3: f64,
    param_CC_1_1: f64,
    param_CC_1_2: f64,
    param_CC_1_3: f64,
    param_CC_1_0: f64,
    param_CC_2_1: f64,
    param_CC_2_2: f64,
    param_CC_2_3: f64,
    param_CC_2_0: f64,
    param_CC_3_1: f64,
    param_CC_3_2: f64,
    param_CC_3_3: f64,
    param_CC_3_0: f64,
    param_CC_0_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t12 = t11 <= zeta_threshold;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t12, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t21 = param_CC_0_1;
        let t22 = t21 * sigma[ip];
        let t23 = M_CBRT2;
        let t24 = t23 * t23;
        let t25 = rho[ip] * rho[ip];
        let t26 = t18 * t18;
        let t28 = 1.0 / t26 / t25;
        let t29 = t24 * t28;
        let t33 = 1.0 + 0.004 * sigma[ip] * t24 * t28;
        let t34 = 1.0 / t33;
        let t35 = t29 * t34;
        let t38 = param_CC_0_2;
        let t39 = sigma[ip] * sigma[ip];
        let t40 = t38 * t39;
        let t41 = t25 * t25;
        let t42 = t41 * rho[ip];
        let t44 = 1.0 / t18 / t42;
        let t46 = t33 * t33;
        let t47 = 1.0 / t46;
        let t48 = t23 * t44 * t47;
        let t51 = param_CC_0_3;
        let t52 = t39 * sigma[ip];
        let t53 = t51 * t52;
        let t54 = t41 * t41;
        let t55 = 1.0 / t54;
        let t56 = t46 * t33;
        let t57 = 1.0 / t56;
        let t58 = t55 * t57;
        let t62 = param_CC_1_1;
        let t63 = t62 * sigma[ip];
        let t66 = param_CC_1_2;
        let t67 = t66 * t39;
        let t70 = param_CC_1_3;
        let t71 = t70 * t52;
        let t74 = param_CC_1_0 + 0.004 * t63 * t35 + 3.2e-05 * t67 * t48 + 2.56e-07 * t71 * t58;
        let t79 = piecewise3(t12, 1.0 / t13, 1.0 / t15);
        let t82 = 1.0 + 0.4 / t18 * t23 * t79;
        let t83 = 1.0 / t82;
        let t86 = param_CC_2_1;
        let t87 = t86 * sigma[ip];
        let t90 = param_CC_2_2;
        let t91 = t90 * t39;
        let t94 = param_CC_2_3;
        let t95 = t94 * t52;
        let t98 = param_CC_2_0 + 0.004 * t87 * t35 + 3.2e-05 * t91 * t48 + 2.56e-07 * t95 * t58;
        let t99 = t82 * t82;
        let t100 = 1.0 / t99;
        let t103 = param_CC_3_1;
        let t104 = t103 * sigma[ip];
        let t107 = param_CC_3_2;
        let t108 = t107 * t39;
        let t111 = param_CC_3_3;
        let t112 = t111 * t52;
        let t115 = param_CC_3_0 + 0.004 * t104 * t35 + 3.2e-05 * t108 * t48 + 2.56e-07 * t112 * t58;
        let t116 = t99 * t82;
        let t117 = 1.0 / t116;
        let t119 = param_CC_0_0 + 0.004 * t22 * t35 + 3.2e-05 * t40 * t48 + 2.56e-07 * t53 * t58 + t74 * t83 + t98 * t100 + t115 * t117;
        let t123 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t119);
        let tzk0 = 2.0 * t123;
        zk[ip] += tzk0;
        let t125 = t17 / t26;
        let t129 = t25 * rho[ip];
        let t131 = 1.0 / t26 / t129;
        let t132 = t24 * t131;
        let t133 = t132 * t34;
        let t136 = t21 * t39;
        let t137 = t41 * t25;
        let t139 = 1.0 / t18 / t137;
        let t141 = t23 * t139 * t47;
        let t146 = t38 * t52;
        let t147 = t54 * rho[ip];
        let t148 = 1.0 / t147;
        let t149 = t148 * t57;
        let t154 = t39 * t39;
        let t155 = t51 * t154;
        let t156 = t54 * t129;
        let t158 = 1.0 / t26 / t156;
        let t159 = t46 * t46;
        let t160 = 1.0 / t159;
        let t162 = t158 * t160 * t24;
        let t167 = t62 * t39;
        let t172 = t66 * t52;
        let t177 = t70 * t154;
        let t180 = -0.010666666666666666 * t63 * t133 + 8.533333333333334e-05 * t167 * t141 - 0.00017066666666666668 * t67 * t141 + 1.3653333333333333e-06 * t172 * t149 - 2.048e-06 * t71 * t149 + 8.192e-09 * t177 * t162;
        let t182 = t74 * t100;
        let t186 = 1.0 / t18 / rho[ip] * t23 * t79;
        let t191 = t86 * t39;
        let t196 = t90 * t52;
        let t201 = t94 * t154;
        let t204 = -0.010666666666666666 * t87 * t133 + 8.533333333333334e-05 * t191 * t141 - 0.00017066666666666668 * t91 * t141 + 1.3653333333333333e-06 * t196 * t149 - 2.048e-06 * t95 * t149 + 8.192e-09 * t201 * t162;
        let t206 = t98 * t117;
        let t211 = t103 * t39;
        let t216 = t107 * t52;
        let t221 = t111 * t154;
        let t224 = -0.010666666666666666 * t104 * t133 + 8.533333333333334e-05 * t211 * t141 - 0.00017066666666666668 * t108 * t141 + 1.3653333333333333e-06 * t216 * t149 - 2.048e-06 * t112 * t149 + 8.192e-09 * t221 * t162;
        let t226 = t99 * t99;
        let t227 = 1.0 / t226;
        let t228 = t115 * t227;
        let t231 = -0.010666666666666666 * t22 * t133 + 8.533333333333334e-05 * t136 * t141 - 0.00017066666666666668 * t40 * t141 + 1.3653333333333333e-06 * t146 * t149 - 2.048e-06 * t53 * t149 + 8.192e-09 * t155 * t162 + t180 * t83 + 0.13333333333333333 * t182 * t186 + t204 * t100 + 0.26666666666666666 * t206 * t186 + t224 * t117 + 0.4 * t228 * t186;
        let t236 = piecewise3(t2, 0.0, -t6 * t125 * t119 / 8.0 - 3.0 / 8.0 * t6 * t19 * t231);
        let tvrho0 = 2.0 * rho[ip] * t236 + 2.0 * t123;
        vrho[ip] += tvrho0;
        let t239 = t21 * t24;
        let t240 = t28 * t34;
        let t245 = t38 * sigma[ip];
        let t250 = t51 * t39;
        let t253 = t54 * t25;
        let t255 = 1.0 / t26 / t253;
        let t257 = t255 * t160 * t24;
        let t260 = t62 * t24;
        let t265 = t66 * sigma[ip];
        let t270 = t70 * t39;
        let t275 = 0.004 * t260 * t240 - 3.2e-05 * t63 * t48 + 6.4e-05 * t265 * t48 - 5.12e-07 * t67 * t58 + 7.68e-07 * t270 * t58 - 3.072e-09 * t71 * t257;
        let t277 = t86 * t24;
        let t282 = t90 * sigma[ip];
        let t287 = t94 * t39;
        let t292 = 0.004 * t277 * t240 - 3.2e-05 * t87 * t48 + 6.4e-05 * t282 * t48 - 5.12e-07 * t91 * t58 + 7.68e-07 * t287 * t58 - 3.072e-09 * t95 * t257;
        let t294 = t103 * t24;
        let t299 = t107 * sigma[ip];
        let t304 = t111 * t39;
        let t309 = 0.004 * t294 * t240 - 3.2e-05 * t104 * t48 + 6.4e-05 * t299 * t48 - 5.12e-07 * t108 * t58 + 7.68e-07 * t304 * t58 - 3.072e-09 * t112 * t257;
        let t311 = 0.004 * t239 * t240 - 3.2e-05 * t22 * t48 + 6.4e-05 * t245 * t48 - 5.12e-07 * t40 * t58 + 7.68e-07 * t250 * t58 - 3.072e-09 * t53 * t257 + t275 * t83 + t292 * t100 + t309 * t117;
        let t315 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t311);
        let tvsigma0 = 2.0 * rho[ip] * t315;
        vsigma[ip] += tvsigma0;
        let t320 = t17 / t26 / rho[ip];
        let t330 = 1.0 / t18 / t25 * t23 * t79;
        let t333 = t21 * t52;
        let t334 = 1.0 / t253;
        let t335 = t334 * t57;
        let t340 = t224 * t227;
        let t344 = 1.0 / t226 / t82;
        let t345 = t115 * t344;
        let t346 = t79 * t79;
        let t347 = t29 * t346;
        let t350 = t74 * t117;
        let t353 = t204 * t117;
        let t356 = t98 * t227;
        let t359 = t41 * t129;
        let t361 = 1.0 / t18 / t359;
        let t363 = t23 * t361 * t47;
        let t366 = t38 * t154;
        let t367 = t54 * t41;
        let t369 = 1.0 / t26 / t367;
        let t371 = t369 * t160 * t24;
        let t379 = t154 * sigma[ip];
        let t380 = t51 * t379;
        let t385 = 1.0 / t159 / t33;
        let t387 = 1.0 / t18 / t54 / t359 * t385 * t23;
        let t390 = t180 * t100;
        let t396 = 1.0 / t26 / t41;
        let t397 = t24 * t396;
        let t398 = t397 * t34;
        let t403 = t62 * t52;
        let t410 = t66 * t154;
        let t417 = t70 * t379;
        let t420 = 0.03911111111111111 * t63 * t398 - 0.000768 * t167 * t363 + 3.6408888888888887e-06 * t403 * t335 + 0.0010808888888888888 * t67 * t363 - 1.956977777777778e-05 * t172 * t335 + 4.369066666666667e-08 * t410 * t371 + 1.8432e-05 * t71 * t335 - 1.6110933333333333e-07 * t177 * t371 + 6.990506666666666e-10 * t417 * t387;
        let t426 = t86 * t52;
        let t433 = t90 * t154;
        let t440 = t94 * t379;
        let t443 = 0.03911111111111111 * t87 * t398 - 0.000768 * t191 * t363 + 3.6408888888888887e-06 * t426 * t335 + 0.0010808888888888888 * t91 * t363 - 1.956977777777778e-05 * t196 * t335 + 4.369066666666667e-08 * t433 * t371 + 1.8432e-05 * t95 * t335 - 1.6110933333333333e-07 * t201 * t371 + 6.990506666666666e-10 * t440 * t387;
        let t455 = t103 * t52;
        let t462 = t107 * t154;
        let t469 = t111 * t379;
        let t472 = 0.03911111111111111 * t104 * t398 - 0.000768 * t211 * t363 + 3.6408888888888887e-06 * t455 * t335 + 0.0010808888888888888 * t108 * t363 - 1.956977777777778e-05 * t216 * t335 + 4.369066666666667e-08 * t462 * t371 + 1.8432e-05 * t112 * t335 - 1.6110933333333333e-07 * t221 * t371 + 6.990506666666666e-10 * t469 * t387;
        let t474 = 1.8432e-05 * t53 * t335 - 1.6110933333333333e-07 * t155 * t371 + 6.990506666666666e-10 * t380 * t387 + 0.26666666666666666 * t390 * t186 - 0.35555555555555557 * t206 * t330 + t420 * t83 + t443 * t100 + 0.03911111111111111 * t22 * t398 + 0.0010808888888888888 * t40 * t363 - 0.17777777777777778 * t182 * t330 + t472 * t117;
        let t475 = -0.5333333333333333 * t228 * t330 + 3.6408888888888887e-06 * t333 * t335 - 1.956977777777778e-05 * t146 * t335 + 0.8 * t340 * t186 + 0.21333333333333335 * t345 * t347 + 0.035555555555555556 * t350 * t347 + 0.5333333333333333 * t353 * t186 + 0.10666666666666667 * t356 * t347 - 0.000768 * t136 * t363 + 4.369066666666667e-08 * t366 * t371 + t474;
        let t480 = piecewise3(t2, 0.0, t6 * t320 * t119 / 12.0 - t6 * t125 * t231 / 4.0 - 3.0 / 8.0 * t6 * t19 * t475);
        let tv2rho20 = 2.0 * rho[ip] * t480 + 4.0 * t236;
        v2rho2[ip] += tv2rho20;
        let t486 = t131 * t34;
        let t489 = t21 * t23;
        let t490 = t139 * t47;
        let t491 = t490 * sigma[ip];
        let t506 = t54 * t137;
        let t510 = 1.0 / t18 / t506 * t385 * t23;
        let t515 = t62 * t23;
        let t532 = -0.010666666666666666 * t260 * t486 + 0.000256 * t515 * t491 - 1.3653333333333333e-06 * t167 * t149 - 0.00034133333333333335 * t265 * t141 + 6.826666666666667e-06 * t67 * t149 - 1.6384e-08 * t172 * t162 - 6.144e-06 * t270 * t149 + 5.7344e-08 * t71 * t162 - 2.62144e-10 * t177 * t510;
        let t534 = t275 * t100;
        let t539 = t86 * t23;
        let t556 = -0.010666666666666666 * t277 * t486 + 0.000256 * t539 * t491 - 1.3653333333333333e-06 * t191 * t149 - 0.00034133333333333335 * t282 * t141 + 6.826666666666667e-06 * t91 * t149 - 1.6384e-08 * t196 * t162 - 6.144e-06 * t287 * t149 + 5.7344e-08 * t95 * t162 - 2.62144e-10 * t201 * t510;
        let t558 = t292 * t117;
        let t563 = t103 * t23;
        let t580 = -0.010666666666666666 * t294 * t486 + 0.000256 * t563 * t491 - 1.3653333333333333e-06 * t211 * t149 - 0.00034133333333333335 * t299 * t141 + 6.826666666666667e-06 * t108 * t149 - 1.6384e-08 * t216 * t162 - 6.144e-06 * t304 * t149 + 5.7344e-08 * t112 * t162 - 2.62144e-10 * t221 * t510;
        let t582 = t309 * t227;
        let t585 = -0.010666666666666666 * t239 * t486 + 0.000256 * t489 * t491 - 1.3653333333333333e-06 * t136 * t149 - 0.00034133333333333335 * t245 * t141 + 6.826666666666667e-06 * t40 * t149 - 1.6384e-08 * t146 * t162 - 6.144e-06 * t250 * t149 + 5.7344e-08 * t53 * t162 - 2.62144e-10 * t155 * t510 + t532 * t83 + 0.13333333333333333 * t534 * t186 + t556 * t100 + 0.26666666666666666 * t558 * t186 + t580 * t117 + 0.4 * t582 * t186;
        let t590 = piecewise3(t2, 0.0, -t6 * t125 * t311 / 8.0 - 3.0 / 8.0 * t6 * t19 * t585);
        let tv2rhosigma0 = 2.0 * rho[ip] * t590 + 2.0 * t315;
        v2rhosigma[ip] += tv2rhosigma0;
        let t593 = t44 * t47;
        let t598 = t38 * t23;
        let t605 = t51 * sigma[ip];
        let t610 = t54 * t42;
        let t614 = 1.0 / t18 / t610 * t385 * t23;
        let t621 = t66 * t23;
        let t628 = t70 * sigma[ip];
        let t635 = -6.4e-05 * t515 * t593 + 5.12e-07 * t63 * t58 + 6.4e-05 * t621 * t593 - 2.048e-06 * t265 * t58 + 6.144e-09 * t67 * t257 + 1.536e-06 * t628 * t58 - 1.8432e-08 * t270 * t257 + 9.8304e-11 * t71 * t614;
        let t641 = t90 * t23;
        let t648 = t94 * sigma[ip];
        let t655 = -6.4e-05 * t539 * t593 + 5.12e-07 * t87 * t58 + 6.4e-05 * t641 * t593 - 2.048e-06 * t282 * t58 + 6.144e-09 * t91 * t257 + 1.536e-06 * t648 * t58 - 1.8432e-08 * t287 * t257 + 9.8304e-11 * t95 * t614;
        let t661 = t107 * t23;
        let t668 = t111 * sigma[ip];
        let t675 = -6.4e-05 * t563 * t593 + 5.12e-07 * t104 * t58 + 6.4e-05 * t661 * t593 - 2.048e-06 * t299 * t58 + 6.144e-09 * t108 * t257 + 1.536e-06 * t668 * t58 - 1.8432e-08 * t304 * t257 + 9.8304e-11 * t112 * t614;
        let t677 = -6.4e-05 * t489 * t593 + 5.12e-07 * t22 * t58 + 6.4e-05 * t598 * t593 - 2.048e-06 * t245 * t58 + 6.144e-09 * t40 * t257 + 1.536e-06 * t605 * t58 - 1.8432e-08 * t250 * t257 + 9.8304e-11 * t53 * t614 + t635 * t83 + t655 * t100 + t675 * t117;
        let t681 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t677);
        let tv2sigma20 = 2.0 * rho[ip] * t681;
        v2sigma2[ip] += tv2sigma20;
    }
}
