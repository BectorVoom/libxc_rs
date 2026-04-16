//! GGA_X_VMT lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_vmt.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_vmt_lxc_unpol(
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
    param_alpha: f64,
    param_mu: f64,
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
        let t21 = param_mu * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t25 * sigma[ip];
        let t27 = t21 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t38 = sigma[ip] * t29 * t33;
        let t41 = f64::exp(-param_alpha * t20 * t25 * t38 / 24.0);
        let t42 = t21 * t25;
        let t45 = 1.0 + t42 * t38 / 24.0;
        let t46 = 1.0 / t45;
        let t47 = t41 * t46;
        let t48 = t29 * t33 * t47;
        let t51 = 1.0 + t27 * t48 / 24.0;
        let t55 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t51);
        let tzk0 = 2.0 * t55;
        zk[ip] += tzk0;
        let t57 = t17 / t31;
        let t61 = t30 * rho[ip];
        let t63 = 1.0 / t31 / t61;
        let t65 = t29 * t63 * t47;
        let t68 = t20 * t20;
        let t69 = param_mu * t68;
        let t71 = 1.0 / t23 / t22;
        let t72 = sigma[ip] * sigma[ip];
        let t73 = t71 * t72;
        let t74 = t69 * t73;
        let t75 = t30 * t30;
        let t76 = t75 * t30;
        let t78 = 1.0 / t18 / t76;
        let t79 = t28 * t78;
        let t80 = param_alpha * t41;
        let t81 = t80 * t46;
        let t85 = param_mu * param_mu;
        let t86 = t85 * t68;
        let t87 = t86 * t73;
        let t88 = t45 * t45;
        let t89 = 1.0 / t88;
        let t90 = t41 * t89;
        let t91 = t79 * t90;
        let t94 = -t27 * t65 / 9.0 + t74 * t79 * t81 / 108.0 + t87 * t91 / 108.0;
        let t99 = piecewise3(t2, 0.0, -t6 * t57 * t51 / 8.0 - 3.0 / 8.0 * t6 * t19 * t94);
        let tvrho0 = 2.0 * rho[ip] * t99 + 2.0 * t55;
        vrho[ip] += tvrho0;
        let t104 = t71 * sigma[ip];
        let t106 = t75 * rho[ip];
        let t108 = 1.0 / t18 / t106;
        let t109 = t28 * t108;
        let t114 = t109 * t90;
        let t117 = t42 * t48 / 24.0 - t69 * t104 * t109 * t81 / 288.0 - t86 * t104 * t114 / 288.0;
        let t121 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t117);
        let tvsigma0 = 2.0 * rho[ip] * t121;
        vsigma[ip] += tvsigma0;
        let t126 = t17 / t31 / rho[ip];
        let t136 = t29 / t31 / t75 * t47;
        let t139 = t75 * t61;
        let t141 = 1.0 / t18 / t139;
        let t142 = t28 * t141;
        let t146 = t142 * t90;
        let t149 = t22 * t22;
        let t150 = 1.0 / t149;
        let t151 = param_mu * t150;
        let t152 = t72 * sigma[ip];
        let t153 = t151 * t152;
        let t154 = t75 * t75;
        let t155 = t154 * t30;
        let t156 = 1.0 / t155;
        let t157 = param_alpha * param_alpha;
        let t162 = t85 * t150;
        let t163 = t162 * t152;
        let t169 = t85 * param_mu * t150;
        let t170 = t169 * t152;
        let t173 = 1.0 / t88 / t45;
        let t177 = 11.0 / 27.0 * t27 * t136 - t74 * t142 * t81 / 12.0 - t87 * t146 / 12.0 + t153 * t156 * t157 * t47 / 81.0 + 2.0 / 81.0 * t163 * t156 * param_alpha * t90 + 2.0 / 81.0 * t170 * t156 * t41 * t173;
        let t182 = piecewise3(t2, 0.0, t6 * t126 * t51 / 12.0 - t6 * t57 * t94 / 4.0 - 3.0 / 8.0 * t6 * t19 * t177);
        let tv2rho20 = 2.0 * rho[ip] * t182 + 4.0 * t99;
        v2rho2[ip] += tv2rho20;
        let t190 = t71 * t28;
        let t191 = t69 * t190;
        let t192 = t78 * param_alpha;
        let t194 = sigma[ip] * t41 * t46;
        let t198 = t86 * t190;
        let t200 = t89 * sigma[ip];
        let t205 = t154 * rho[ip];
        let t206 = 1.0 / t205;
        let t218 = t206 * t41 * t173;
        let t221 = -t42 * t65 / 9.0 + t191 * t192 * t194 / 36.0 + t198 * t78 * t41 * t200 / 36.0 - t151 * t72 * t206 * t157 * t47 / 216.0 - t162 * t72 * t206 * param_alpha * t90 / 108.0 - t169 * t72 * t218 / 108.0;
        let t226 = piecewise3(t2, 0.0, -t6 * t57 * t117 / 8.0 - 3.0 / 8.0 * t6 * t19 * t221);
        let tv2rhosigma0 = 2.0 * rho[ip] * t226 + 2.0 * t121;
        v2rhosigma[ip] += tv2rhosigma0;
        let t233 = t86 * t71;
        let t237 = 1.0 / t154;
        let t249 = t237 * t41 * t173;
        let t252 = -t191 * t108 * param_alpha * t47 / 144.0 - t233 * t114 / 144.0 + t151 * sigma[ip] * t237 * t157 * t47 / 576.0 + t162 * sigma[ip] * t237 * param_alpha * t90 / 288.0 + t169 * sigma[ip] * t249 / 288.0;
        let t256 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t252);
        let tv2sigma20 = 2.0 * rho[ip] * t256;
        v2sigma2[ip] += tv2sigma20;
        let t259 = t17 * t33;
        let t272 = t29 / t31 / t106 * t47;
        let t276 = 1.0 / t18 / t154;
        let t277 = t28 * t276;
        let t284 = t154 * t61;
        let t285 = 1.0 / t284;
        let t298 = t72 * t72;
        let t299 = t154 * t106;
        let t301 = 1.0 / t31 / t299;
        let t302 = t298 * t301;
        let t303 = t157 * param_alpha;
        let t306 = t20 * t25;
        let t308 = t29 * t41 * t46;
        let t309 = t306 * t308;
        let t314 = t306 * t29;
        let t315 = t90 * t314;
        let t320 = t41 * t173;
        let t321 = t320 * t314;
        let t324 = t85 * t85;
        let t325 = t324 * t150;
        let t327 = t88 * t88;
        let t328 = 1.0 / t327;
        let t329 = t41 * t328;
        let t330 = t329 * t314;
        let t333 = -154.0 / 81.0 * t27 * t272 + 341.0 / 486.0 * t74 * t277 * t81 + 341.0 / 486.0 * t87 * t277 * t90 - 19.0 / 81.0 * t153 * t285 * t157 * t47 - 38.0 / 81.0 * t163 * t285 * param_alpha * t90 - 38.0 / 81.0 * t170 * t285 * t41 * t173 + t151 * t302 * t303 * t309 / 729.0 + t162 * t302 * t157 * t315 / 243.0 + 2.0 / 243.0 * t169 * t302 * param_alpha * t321 + 2.0 / 243.0 * t325 * t302 * t330;
        let t338 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t259 * t51 + t6 * t126 * t94 / 4.0 - 3.0 / 8.0 * t6 * t57 * t177 - 3.0 / 8.0 * t6 * t19 * t333);
        let tv3rho30 = 2.0 * rho[ip] * t338 + 6.0 * t182;
        v3rho3[ip] += tv3rho30;
        let t350 = t141 * param_alpha;
        let t358 = t151 * t156;
        let t360 = t157 * t72 * t47;
        let t363 = t162 * t156;
        let t365 = param_alpha * t72 * t90;
        let t368 = t169 * t156;
        let t369 = t320 * t72;
        let t372 = t154 * t75;
        let t374 = 1.0 / t31 / t372;
        let t375 = t152 * t374;
        let t391 = 11.0 / 27.0 * t42 * t136 - 65.0 / 324.0 * t191 * t350 * t194 - 65.0 / 324.0 * t198 * t141 * t41 * t200 + 17.0 / 216.0 * t358 * t360 + 17.0 / 108.0 * t363 * t365 + 17.0 / 108.0 * t368 * t369 - t151 * t375 * t303 * t309 / 1944.0 - t162 * t375 * t157 * t315 / 648.0 - t169 * t375 * param_alpha * t321 / 324.0 - t325 * t375 * t330 / 324.0;
        let t396 = piecewise3(t2, 0.0, t6 * t126 * t117 / 12.0 - t6 * t57 * t221 / 4.0 - 3.0 / 8.0 * t6 * t19 * t391);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t396 + 4.0 * t226;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t405 = t151 * t206;
        let t407 = t157 * sigma[ip] * t47;
        let t410 = t162 * t206;
        let t411 = t80 * t200;
        let t417 = t320 * sigma[ip];
        let t421 = 1.0 / t31 / t284;
        let t422 = t72 * t421;
        let t438 = t191 * t192 * t47 / 27.0 - 5.0 / 216.0 * t405 * t407 - 5.0 / 108.0 * t410 * t411 + t233 * t91 / 27.0 - 5.0 / 108.0 * t169 * t206 * t417 + t151 * t422 * t303 * t309 / 5184.0 + t162 * t422 * t157 * t315 / 1728.0 + t169 * t422 * param_alpha * t321 / 864.0 + t325 * t422 * t330 / 864.0;
        let t443 = piecewise3(t2, 0.0, -t6 * t57 * t252 / 8.0 - 3.0 / 8.0 * t6 * t19 * t438);
        let tv3rhosigma20 = 2.0 * rho[ip] * t443 + 2.0 * t256;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t448 = t157 * t41 * t46;
        let t452 = t80 * t89;
        let t458 = 1.0 / t31 / t155;
        let t459 = sigma[ip] * t458;
        let t475 = t151 * t237 * t448 / 192.0 + t162 * t237 * t452 / 96.0 + t169 * t249 / 96.0 - t151 * t459 * t303 * t309 / 13824.0 - t162 * t459 * t157 * t315 / 4608.0 - t169 * t459 * param_alpha * t321 / 2304.0 - t325 * t459 * t330 / 2304.0;
        let t479 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t475);
        let tv3sigma30 = 2.0 * rho[ip] * t479;
        v3sigma3[ip] += tv3sigma30;
        let t495 = 1.0 / t372;
        let t502 = t28 / t18 / t205;
        let t507 = t154 * t154;
        let t511 = t298 * sigma[ip] / t18 / t507 / rho[ip];
        let t512 = t157 * t157;
        let t515 = t68 * t71;
        let t516 = t28 * t41;
        let t518 = t515 * t516 * t46;
        let t524 = t515 * t516 * t89;
        let t529 = t515 * t28;
        let t530 = t320 * t529;
        let t535 = t329 * t529;
        let t538 = t154 * t76;
        let t541 = t298 / t31 / t538;
        let t569 = t324 * param_mu * t150;
        let t574 = t41 / t327 / t45 * t529;
        let t583 = 5126.0 / 729.0 * t170 * t495 * t41 * t173 - 3047.0 / 486.0 * t87 * t502 * t90 + 2.0 / 6561.0 * t151 * t511 * t512 * t518 + 8.0 / 6561.0 * t162 * t511 * t303 * t524 + 8.0 / 2187.0 * t169 * t511 * t157 * t530 + 16.0 / 2187.0 * t325 * t511 * param_alpha * t535 - 98.0 / 2187.0 * t151 * t541 * t303 * t309 - 98.0 / 729.0 * t162 * t541 * t157 * t315 - 196.0 / 729.0 * t169 * t541 * param_alpha * t321 + 2563.0 / 729.0 * t153 * t495 * t157 * t47 + 5126.0 / 729.0 * t163 * t495 * param_alpha * t90 - 3047.0 / 486.0 * t74 * t502 * t81 - 196.0 / 729.0 * t325 * t541 * t330 + 16.0 / 2187.0 * t569 * t511 * t574 + 2618.0 / 243.0 * t27 * t29 / t31 / t76 * t47;
        let t588 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 * t63 * t51 - 5.0 / 9.0 * t6 * t259 * t94 + t6 * t126 * t177 / 2.0 - t6 * t57 * t333 / 2.0 - 3.0 / 8.0 * t6 * t19 * t583);
        let tv4rho40 = 2.0 * rho[ip] * t588 + 8.0 * t338;
        v4rho4[ip] += tv4rho40;
        let t609 = t298 / t18 / t507;
        let t663 = 89.0 / 972.0 * t325 * t301 * t41 * t328 * t152 * t314 - 2.0 / 729.0 * t569 * t609 * t574 + 253.0 / 162.0 * t191 * t276 * param_alpha * t194 - 1025.0 / 486.0 * t169 * t285 * t369 - 154.0 / 81.0 * t42 * t272 + 253.0 / 162.0 * t198 * t276 * t41 * t200 + 89.0 / 972.0 * t169 * t301 * param_alpha * t152 * t321 - t151 * t609 * t512 * t518 / 8748.0 - t162 * t609 * t303 * t524 / 2187.0 - t169 * t609 * t157 * t530 / 729.0 - 2.0 / 729.0 * t325 * t609 * param_alpha * t535 + 89.0 / 5832.0 * t151 * t301 * t303 * t152 * t309 + 89.0 / 1944.0 * t162 * t301 * t157 * t152 * t315 - 1025.0 / 972.0 * t151 * t285 * t360 - 1025.0 / 486.0 * t162 * t285 * t365;
        let t668 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t259 * t117 + t6 * t126 * t221 / 4.0 - 3.0 / 8.0 * t6 * t57 * t391 - 3.0 / 8.0 * t6 * t19 * t663);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t668 + 6.0 * t396;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t683 = t152 / t18 / t154 / t139;
        let t735 = 167.0 / 324.0 * t368 * t417 + t569 * t683 * t574 / 972.0 - 19.0 / 81.0 * t191 * t350 * t47 - 25.0 / 864.0 * t325 * t374 * t41 * t328 * t72 * t314 - 19.0 / 81.0 * t233 * t146 + t325 * t683 * param_alpha * t535 / 972.0 - 25.0 / 5184.0 * t151 * t374 * t303 * t72 * t309 - 25.0 / 1728.0 * t162 * t374 * t157 * t72 * t315 - 25.0 / 864.0 * t169 * t374 * param_alpha * t41 * t173 * t72 * t314 + t151 * t683 * t512 * t518 / 23328.0 + t162 * t683 * t303 * t524 / 5832.0 + t169 * t683 * t157 * t530 / 1944.0 + 167.0 / 648.0 * t358 * t407 + 167.0 / 324.0 * t363 * t411;
        let t740 = piecewise3(t2, 0.0, t6 * t126 * t252 / 12.0 - t6 * t57 * t438 / 4.0 - 3.0 / 8.0 * t6 * t19 * t735);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t740 + 4.0 * t443;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t758 = t26 * t29;
        let t775 = t328 * t20;
        let t781 = t72 / t18 / t538;
        let t801 = -t405 * t448 / 24.0 + 7.0 / 5184.0 * t151 * t421 * t303 * t20 * t26 * t308 + 7.0 / 1728.0 * t162 * t421 * t157 * t41 * t89 * t20 * t758 - t410 * t452 / 12.0 + 7.0 / 864.0 * t169 * t421 * param_alpha * t41 * t173 * t20 * t758 - t169 * t218 / 12.0 + 7.0 / 864.0 * t325 * t421 * t41 * t775 * t758 - t151 * t781 * t512 * t518 / 62208.0 - t162 * t781 * t303 * t524 / 15552.0 - t169 * t781 * t157 * t530 / 5184.0 - t325 * t781 * param_alpha * t535 / 2592.0 - t569 * t781 * t574 / 2592.0;
        let t806 = piecewise3(t2, 0.0, -t6 * t57 * t475 / 8.0 - 3.0 / 8.0 * t6 * t19 * t801);
        let tv4rhosigma30 = 2.0 * rho[ip] * t806 + 2.0 * t479;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t829 = sigma[ip] / t18 / t299;
        let t853 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * (-t151 * t458 * t303 * t309 / 3456.0 - t162 * t458 * t157 * t315 / 1152.0 - t169 * t458 * param_alpha * t321 / 576.0 - t325 * t458 * t41 * t775 * t25 * t29 / 576.0 + t151 * t829 * t512 * t518 / 165888.0 + t162 * t829 * t303 * t524 / 41472.0 + t169 * t829 * t157 * t530 / 13824.0 + t325 * t829 * param_alpha * t535 / 6912.0 + t569 * t829 * t574 / 6912.0));
        let tv4sigma40 = 2.0 * rho[ip] * t853;
        v4sigma4[ip] += tv4sigma40;
    }
}
