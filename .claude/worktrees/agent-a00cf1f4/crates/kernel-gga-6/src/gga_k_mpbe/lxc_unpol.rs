//! GGA_K_MPBE lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_mpbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_mpbe_lxc_unpol(
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
    param_a: f64,
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
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
        let t23 = t20 * t22;
        let t24 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = param_c1 * t24 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t22 / t34;
        let t37 = param_a * t24;
        let t42 = 1.0 + t37 * t29 * t33 * t36 / 24.0;
        let t43 = 1.0 / t42;
        let t48 = t24 * t24;
        let t51 = 1.0 / t27 / t26;
        let t52 = param_c2 * t48 * t51;
        let t53 = sigma[ip] * sigma[ip];
        let t54 = t53 * t31;
        let t55 = t34 * t34;
        let t56 = t55 * rho[ip];
        let t58 = 1.0 / t21 / t56;
        let t59 = t42 * t42;
        let t60 = 1.0 / t59;
        let t61 = t58 * t60;
        let t65 = t26 * t26;
        let t66 = 1.0 / t65;
        let t67 = param_c3 * t66;
        let t68 = t53 * sigma[ip];
        let t69 = t55 * t55;
        let t70 = 1.0 / t69;
        let t72 = t59 * t42;
        let t73 = 1.0 / t72;
        let t77 = 1.0 + t30 * t33 * t36 * t43 / 24.0 + t52 * t54 * t61 / 288.0 + t67 * t68 * t70 * t73 / 576.0;
        let t81 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t77);
        let tzk0 = 2.0 * t81;
        zk[ip] += tzk0;
        let t83 = t20 / t21;
        let t87 = t34 * rho[ip];
        let t89 = 1.0 / t22 / t87;
        let t94 = param_c1 * t48;
        let t96 = t94 * t51 * t53;
        let t97 = t55 * t34;
        let t99 = 1.0 / t21 / t97;
        let t100 = t31 * t99;
        let t101 = t60 * param_a;
        let t102 = t100 * t101;
        let t105 = t99 * t60;
        let t109 = param_c2 * t66;
        let t110 = t109 * t68;
        let t111 = t69 * rho[ip];
        let t112 = 1.0 / t111;
        let t113 = t112 * t73;
        let t114 = t113 * param_a;
        let t121 = t53 * t53;
        let t122 = t69 * t87;
        let t124 = 1.0 / t22 / t122;
        let t127 = t59 * t59;
        let t128 = 1.0 / t127;
        let t131 = t24 * t29 * t32;
        let t132 = t128 * param_a * t131;
        let t135 = -t30 * t33 * t89 * t43 / 9.0 + t96 * t102 / 108.0 - t52 * t54 * t105 / 54.0 + t110 * t114 / 108.0 - t67 * t68 * t112 * t73 / 72.0 + t67 * t121 * t124 * t132 / 1728.0;
        let t140 = piecewise3(t2, 0.0, t7 * t83 * t77 / 10.0 + 3.0 / 20.0 * t7 * t23 * t135);
        let tvrho0 = 2.0 * rho[ip] * t140 + 2.0 * t81;
        vrho[ip] += tvrho0;
        let t149 = t31 * t58;
        let t150 = t149 * t101;
        let t153 = sigma[ip] * t31;
        let t157 = t109 * t53;
        let t158 = t70 * t73;
        let t159 = t158 * param_a;
        let t166 = t69 * t34;
        let t168 = 1.0 / t22 / t166;
        let t173 = t30 * t32 * t36 * t43 / 24.0 - t94 * t51 * sigma[ip] * t150 / 288.0 + t52 * t153 * t61 / 144.0 - t157 * t159 / 288.0 + t67 * t53 * t70 * t73 / 192.0 - t67 * t68 * t168 * t132 / 4608.0;
        let t177 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t173);
        let tvsigma0 = 2.0 * rho[ip] * t177;
        vsigma[ip] += tvsigma0;
        let t182 = t20 / t21 / rho[ip];
        let t190 = 1.0 / t22 / t55;
        let t195 = t55 * t87;
        let t197 = 1.0 / t21 / t195;
        let t198 = t31 * t197;
        let t199 = t198 * t101;
        let t202 = param_c1 * t66;
        let t203 = t202 * t68;
        let t204 = 1.0 / t166;
        let t205 = t204 * t73;
        let t206 = param_a * param_a;
        let t210 = t197 * t60;
        let t214 = t205 * param_a;
        let t217 = t69 * t55;
        let t219 = 1.0 / t22 / t217;
        let t220 = t121 * t219;
        let t223 = t128 * t206 * t131;
        let t233 = t121 * sigma[ip];
        let t236 = 1.0 / t21 / t69 / t195;
        let t240 = 1.0 / t127 / t42;
        let t243 = t48 * t51 * t31;
        let t244 = t240 * t206 * t243;
        let t247 = 11.0 / 27.0 * t30 * t33 * t190 * t43 - t96 * t199 / 12.0 + 2.0 / 81.0 * t203 * t205 * t206 + 19.0 / 162.0 * t52 * t54 * t210 - 43.0 / 324.0 * t110 * t214 + t109 * t220 * t223 / 324.0 + t67 * t68 * t204 * t73 / 8.0 - 59.0 / 5184.0 * t67 * t220 * t132 + t67 * t233 * t236 * t244 / 1944.0;
        let t252 = piecewise3(t2, 0.0, -t7 * t182 * t77 / 30.0 + t7 * t83 * t135 / 5.0 + 3.0 / 20.0 * t7 * t23 * t247);
        let tv2rho20 = 2.0 * rho[ip] * t252 + 4.0 * t140;
        v2rho2[ip] += tv2rho20;
        let t263 = t94 * t51 * t31;
        let t264 = param_a * sigma[ip];
        let t269 = t113 * t206;
        let t277 = t68 * t124;
        let t288 = t69 * t97;
        let t290 = 1.0 / t21 / t288;
        let t295 = -t30 * t32 * t89 * t43 / 9.0 + t263 * t105 * t264 / 36.0 - t202 * t53 * t269 / 108.0 - t52 * t153 * t105 / 27.0 + 5.0 / 108.0 * t157 * t114 - t109 * t277 * t223 / 864.0 - t67 * t53 * t112 * t73 / 24.0 + 7.0 / 1728.0 * t67 * t277 * t132 - t67 * t121 * t290 * t244 / 5184.0;
        let t300 = piecewise3(t2, 0.0, t7 * t83 * t173 / 10.0 + 3.0 / 20.0 * t7 * t23 * t295);
        let tv2rhosigma0 = 2.0 * rho[ip] * t300 + 2.0 * t177;
        v2rhosigma[ip] += tv2rhosigma0;
        let t303 = t94 * t51;
        let t307 = t158 * t206;
        let t316 = t53 * t168;
        let t327 = t69 * t56;
        let t329 = 1.0 / t21 / t327;
        let t334 = -t303 * t150 / 144.0 + t202 * sigma[ip] * t307 / 288.0 + t52 * t149 * t60 / 144.0 - t109 * sigma[ip] * t159 / 72.0 + t109 * t316 * t223 / 2304.0 + t67 * sigma[ip] * t70 * t73 / 96.0 - t67 * t316 * t132 / 768.0 + t67 * t68 * t329 * t244 / 13824.0;
        let t338 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t334);
        let tv2sigma20 = 2.0 * rho[ip] * t338;
        v2sigma2[ip] += tv2sigma20;
        let t343 = t20 / t21 / t34;
        let t354 = 1.0 / t22 / t56;
        let t360 = 1.0 / t21 / t69;
        let t365 = 1.0 / t122;
        let t366 = t365 * t73;
        let t371 = 1.0 / t22 / t327;
        let t372 = t121 * t371;
        let t374 = t206 * param_a;
        let t376 = t128 * t374 * t131;
        let t379 = t360 * t60;
        let t383 = t366 * param_a;
        let t389 = t69 * t69;
        let t391 = 1.0 / t21 / t389;
        let t392 = t233 * t391;
        let t395 = t240 * t374 * t243;
        let t408 = t65 * t65;
        let t409 = 1.0 / t408;
        let t410 = param_c3 * t409;
        let t411 = t121 * t53;
        let t412 = t410 * t411;
        let t413 = t389 * t87;
        let t416 = 1.0 / t127 / t59;
        let t417 = 1.0 / t413 * t416;
        let t418 = t417 * t374;
        let t421 = -154.0 / 81.0 * t30 * t33 * t354 * t43 + 341.0 / 486.0 * t96 * t31 * t360 * t101 - 38.0 / 81.0 * t203 * t366 * t206 + 2.0 / 243.0 * t202 * t372 * t376 - 209.0 / 243.0 * t52 * t54 * t379 + 797.0 / 486.0 * t110 * t383 - t109 * t372 * t223 / 12.0 + 2.0 / 729.0 * t109 * t392 * t395 - 5.0 / 4.0 * t67 * t68 * t365 * t73 + 1445.0 / 7776.0 * t67 * t372 * t132 - 35.0 / 1944.0 * t67 * t392 * t244 + 5.0 / 1458.0 * t412 * t418;
        let t426 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t343 * t77 - t7 * t182 * t135 / 10.0 + 3.0 / 10.0 * t7 * t83 * t247 + 3.0 / 20.0 * t7 * t23 * t421);
        let tv3rho30 = 2.0 * rho[ip] * t426 + 6.0 * t252;
        v3rho3[ip] += tv3rho30;
        let t443 = t202 * t204;
        let t444 = t73 * t206;
        let t445 = t444 * t53;
        let t448 = t68 * t219;
        let t460 = t121 * t236;
        let t474 = t410 * t233;
        let t475 = t389 * t34;
        let t477 = 1.0 / t475 * t416;
        let t478 = t477 * t374;
        let t481 = 11.0 / 27.0 * t30 * t32 * t190 * t43 - 65.0 / 324.0 * t263 * t210 * t264 + 17.0 / 108.0 * t443 * t445 - t202 * t448 * t376 / 324.0 + 19.0 / 81.0 * t52 * t153 * t210 - 167.0 / 324.0 * t157 * t214 + 25.0 / 864.0 * t109 * t448 * t223 - t109 * t460 * t395 / 972.0 + 3.0 / 8.0 * t67 * t53 * t204 * t73 - 317.0 / 5184.0 * t67 * t448 * t132 + 11.0 / 1728.0 * t67 * t460 * t244 - 5.0 / 3888.0 * t474 * t478;
        let t486 = piecewise3(t2, 0.0, -t7 * t182 * t173 / 30.0 + t7 * t83 * t295 / 5.0 + 3.0 / 20.0 * t7 * t23 * t481);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t486 + 4.0 * t300;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t495 = t444 * sigma[ip];
        let t498 = t53 * t124;
        let t507 = t73 * param_a * sigma[ip];
        let t513 = t68 * t290;
        let t527 = t410 * t121;
        let t528 = t389 * rho[ip];
        let t530 = 1.0 / t528 * t416;
        let t531 = t530 * t374;
        let t534 = t303 * t102 / 27.0 - 5.0 / 108.0 * t202 * t112 * t495 + t202 * t498 * t376 / 864.0 - t52 * t100 * t60 / 27.0 + 7.0 / 54.0 * t109 * t112 * t507 - t109 * t498 * t223 / 108.0 + t109 * t513 * t395 / 2592.0 - t67 * sigma[ip] * t112 * t73 / 12.0 + 5.0 / 288.0 * t67 * t498 * t132 - 11.0 / 5184.0 * t67 * t513 * t244 + 5.0 / 10368.0 * t527 * t531;
        let t539 = piecewise3(t2, 0.0, t7 * t83 * t334 / 10.0 + 3.0 / 20.0 * t7 * t23 * t534);
        let tv3rhosigma20 = 2.0 * rho[ip] * t539 + 2.0 * t338;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t544 = sigma[ip] * t168;
        let t553 = t53 * t329;
        let t565 = t410 * t68;
        let t567 = 1.0 / t389 * t416;
        let t568 = t567 * t374;
        let t571 = t202 * t307 / 96.0 - t202 * t544 * t376 / 2304.0 - t109 * t159 / 48.0 + t109 * t544 * t223 / 384.0 - t109 * t553 * t395 / 6912.0 + t67 * t158 / 96.0 - t67 * t544 * t132 / 256.0 + t67 * t553 * t244 / 1536.0 - 5.0 / 27648.0 * t565 * t568;
        let t575 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t571);
        let tv3sigma30 = 2.0 * rho[ip] * t575;
        v3sigma3[ip] += tv3sigma30;
        let t600 = 1.0 / t21 / t111;
        let t605 = 1.0 / t217;
        let t606 = t605 * t73;
        let t612 = t121 / t22 / t288;
        let t618 = t233 / t21 / t528;
        let t620 = t206 * t206;
        let t622 = t240 * t620 * t243;
        let t638 = param_c2 * t409;
        let t640 = t389 * t55;
        let t642 = 1.0 / t640 * t416;
        let t668 = 1.0 / t127 / t72 * t620 * t131;
        let t671 = 2618.0 / 243.0 * t30 * t33 / t22 / t97 * t43 - 3047.0 / 486.0 * t96 * t31 * t600 * t101 + 5126.0 / 729.0 * t203 * t606 * t206 - 196.0 / 729.0 * t202 * t612 * t376 + 16.0 / 2187.0 * t202 * t618 * t622 + 5225.0 / 729.0 * t52 * t54 * t600 * t60 - 29645.0 / 1458.0 * t110 * t606 * param_a + 4915.0 / 2916.0 * t109 * t612 * t223 - 260.0 / 2187.0 * t109 * t618 * t395 + 40.0 / 2187.0 * t638 * t411 * t642 * t620 + 55.0 / 4.0 * t67 * t68 * t605 * t73 - 68965.0 / 23328.0 * t67 * t612 * t132 + 8035.0 / 17496.0 * t67 * t618 * t244 - 5.0 / 27.0 * t412 * t642 * t374 + 5.0 / 2187.0 * t410 * t121 * t68 / t22 / t389 / t97 * t668;
        let t676 = piecewise3(t2, 0.0, -14.0 / 135.0 * t7 * t20 / t21 / t87 * t77 + 8.0 / 45.0 * t7 * t343 * t135 - t7 * t182 * t247 / 5.0 + 2.0 / 5.0 * t7 * t83 * t421 + 3.0 / 20.0 * t7 * t23 * t671);
        let tv4rho40 = 2.0 * rho[ip] * t676 + 8.0 * t426;
        v4rho4[ip] += tv4rho40;
        let t705 = t121 * t391;
        let t714 = t68 * t371;
        let t744 = -154.0 / 81.0 * t30 * t32 * t354 * t43 + 253.0 / 162.0 * t263 * t379 * t264 - 1025.0 / 486.0 * t202 * t365 * t445 + 89.0 / 972.0 * t202 * t371 * t128 * t374 * t68 * t131 - 2.0 / 729.0 * t202 * t705 * t622 - 418.0 / 243.0 * t52 * t153 * t379 + 2809.0 / 486.0 * t157 * t383 - 2093.0 / 3888.0 * t109 * t714 * t223 + 121.0 / 2916.0 * t109 * t705 * t395 - 5.0 / 729.0 * t638 * t233 * t417 * t620 - 15.0 / 4.0 * t67 * t53 * t365 * t73 + 6995.0 / 7776.0 * t67 * t714 * t132 - 3545.0 / 23328.0 * t67 * t705 * t244 + 85.0 / 1296.0 * t474 * t418 - 5.0 / 5832.0 * t410 * t411 / t22 / t389 / t56 * t668;
        let t749 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t343 * t173 - t7 * t182 * t295 / 10.0 + 3.0 / 10.0 * t7 * t83 * t481 + 3.0 / 20.0 * t7 * t23 * t744);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t749 + 6.0 * t486;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t763 = t219 * t128;
        let t769 = t68 * t236;
        let t810 = -19.0 / 81.0 * t303 * t199 + 167.0 / 324.0 * t443 * t495 - 25.0 / 864.0 * t202 * t763 * t374 * t53 * t131 + t202 * t769 * t622 / 972.0 + 19.0 / 81.0 * t52 * t198 * t60 - 205.0 / 162.0 * t109 * t204 * t507 + 49.0 / 324.0 * t109 * t763 * t206 * t53 * t131 - 107.0 / 7776.0 * t109 * t769 * t395 + 5.0 / 1944.0 * t638 * t121 * t477 * t620 + 3.0 / 4.0 * t67 * sigma[ip] * t204 * t73 - 199.0 / 864.0 * t67 * t53 * t219 * t132 + 713.0 / 15552.0 * t67 * t769 * t244 - 695.0 / 31104.0 * t527 * t478 + 5.0 / 15552.0 * t410 * t233 / t22 / t640 * t668;
        let t815 = piecewise3(t2, 0.0, -t7 * t182 * t334 / 30.0 + t7 * t83 * t534 / 5.0 + 3.0 / 20.0 * t7 * t23 * t810);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t815 + 4.0 * t539;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t823 = t124 * t128;
        let t825 = t374 * t24;
        let t827 = t29 * sigma[ip] * t32;
        let t831 = t53 * t290;
        let t838 = t206 * t24;
        let t866 = -t202 * t269 / 12.0 + 7.0 / 864.0 * t202 * t823 * t825 * t827 - t202 * t831 * t622 / 2592.0 + t109 * t114 / 6.0 - 5.0 / 144.0 * t109 * t823 * t838 * t827 + 11.0 / 2592.0 * t109 * t831 * t395 - 5.0 / 5184.0 * t638 * t68 * t530 * t620 - t67 * t113 / 12.0 + 13.0 / 288.0 * t67 * t823 * t37 * t827 - 7.0 / 576.0 * t67 * t831 * t244 + 25.0 / 3456.0 * t565 * t531 - 5.0 / 41472.0 * t410 * t121 / t22 / t413 * t668;
        let t871 = piecewise3(t2, 0.0, t7 * t83 * t571 / 10.0 + 3.0 / 20.0 * t7 * t23 * t866);
        let tv4rhosigma30 = 2.0 * rho[ip] * t871 + 2.0 * t575;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t874 = t168 * t128;
        let t876 = t29 * t32;
        let t880 = sigma[ip] * t329;
        let t915 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * (-t202 * t874 * t825 * t876 / 576.0 + t202 * t880 * t622 / 6912.0 + t109 * t874 * t838 * t876 / 192.0 - t109 * t880 * t395 / 864.0 + 5.0 / 13824.0 * t638 * t53 * t567 * t620 - t67 * t874 * t37 * t876 / 192.0 + t67 * t880 * t244 / 384.0 - 5.0 / 2304.0 * t410 * t53 * t568 + 5.0 / 110592.0 * t410 * t68 / t22 / t475 * t668));
        let tv4sigma40 = 2.0 * rho[ip] * t915;
        v4sigma4[ip] += tv4sigma40;
    }
}
