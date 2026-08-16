//! GGA_X_B86 lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_b86.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_b86_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    param_omega: f64,
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
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = param_beta * sigma[ip];
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = rho[ip] * rho[ip];
        let t24 = t18 * t18;
        let t26 = 1.0 / t24 / t23;
        let t27 = t22 * t26;
        let t30 = param_gamma * sigma[ip] * t27 + 1.0;
        let t31 = f64::powf(t30, param_omega);
        let t32 = 1.0 / t31;
        let t35 = t20 * t27 * t32 + 1.0;
        let t39 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t35);
        let tzk0 = 2.0 * t39;
        zk[ip] += tzk0;
        let t41 = t17 / t24;
        let t45 = t23 * rho[ip];
        let t47 = 1.0 / t24 / t45;
        let t52 = sigma[ip] * sigma[ip];
        let t53 = param_beta * t52;
        let t54 = t23 * t23;
        let t55 = t54 * t23;
        let t57 = 1.0 / t18 / t55;
        let t60 = t32 * param_omega;
        let t61 = 1.0 / t30;
        let t63 = t60 * param_gamma * t61;
        let t66 = -8.0 / 3.0 * t20 * t22 * t47 * t32 + 16.0 / 3.0 * t53 * t21 * t57 * t63;
        let t71 = piecewise3(t2, 0.0, -t6 * t41 * t35 / 8.0 - 3.0 / 8.0 * t6 * t19 * t66);
        let tvrho0 = 2.0 * rho[ip] * t71 + 2.0 * t39;
        vrho[ip] += tvrho0;
        let t74 = param_beta * t22;
        let t77 = t54 * rho[ip];
        let t79 = 1.0 / t18 / t77;
        let t84 = -2.0 * t20 * t21 * t79 * t63 + t74 * t26 * t32;
        let t88 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t84);
        let tvsigma0 = 2.0 * rho[ip] * t88;
        vsigma[ip] += tvsigma0;
        let t93 = t17 / t24 / rho[ip];
        let t101 = 1.0 / t24 / t54;
        let t106 = t54 * t45;
        let t108 = 1.0 / t18 / t106;
        let t113 = t52 * sigma[ip];
        let t114 = param_beta * t113;
        let t115 = t54 * t54;
        let t116 = t115 * t23;
        let t117 = 1.0 / t116;
        let t118 = t114 * t117;
        let t119 = param_omega * param_omega;
        let t120 = t32 * t119;
        let t121 = param_gamma * param_gamma;
        let t122 = t30 * t30;
        let t123 = 1.0 / t122;
        let t124 = t121 * t123;
        let t125 = t120 * t124;
        let t128 = t60 * t124;
        let t131 = 88.0 / 9.0 * t20 * t22 * t101 * t32 - 48.0 * t53 * t21 * t108 * t63 + 256.0 / 9.0 * t118 * t125 + 256.0 / 9.0 * t118 * t128;
        let t136 = piecewise3(t2, 0.0, t6 * t93 * t35 / 12.0 - t6 * t41 * t66 / 4.0 - 3.0 / 8.0 * t6 * t19 * t131);
        let tv2rho20 = 2.0 * rho[ip] * t136 + 4.0 * t71;
        v2rho2[ip] += tv2rho20;
        let t145 = param_beta * t21;
        let t150 = param_omega * param_gamma * sigma[ip] * t61;
        let t153 = t115 * rho[ip];
        let t154 = 1.0 / t153;
        let t155 = t53 * t154;
        let t160 = -8.0 / 3.0 * t74 * t47 * t32 + 16.0 * t145 * t57 * t32 * t150 - 32.0 / 3.0 * t155 * t125 - 32.0 / 3.0 * t155 * t128;
        let t165 = piecewise3(t2, 0.0, -t6 * t41 * t84 / 8.0 - 3.0 / 8.0 * t6 * t19 * t160);
        let tv2rhosigma0 = 2.0 * rho[ip] * t165 + 2.0 * t88;
        v2rhosigma[ip] += tv2rhosigma0;
        let t170 = 1.0 / t115;
        let t171 = t20 * t170;
        let t175 = -4.0 * t145 * t79 * t63 + 4.0 * t171 * t125 + 4.0 * t171 * t128;
        let t179 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t175);
        let tv2sigma20 = 2.0 * rho[ip] * t179;
        v2sigma2[ip] += tv2sigma20;
        let t182 = t17 * t26;
        let t193 = 1.0 / t24 / t77;
        let t199 = 1.0 / t18 / t115;
        let t204 = t115 * t45;
        let t205 = 1.0 / t204;
        let t206 = t114 * t205;
        let t211 = t52 * t52;
        let t212 = param_beta * t211;
        let t213 = t115 * t77;
        let t215 = 1.0 / t24 / t213;
        let t217 = t212 * t215 * t32;
        let t218 = t119 * param_omega;
        let t219 = t121 * param_gamma;
        let t222 = 1.0 / t122 / t30;
        let t223 = t222 * t22;
        let t224 = t218 * t219 * t223;
        let t228 = t119 * t219 * t223;
        let t232 = param_omega * t219 * t223;
        let t235 = -1232.0 / 27.0 * t20 * t22 * t193 * t32 + 10912.0 / 27.0 * t53 * t21 * t199 * t63 - 4864.0 / 9.0 * t206 * t125 - 4864.0 / 9.0 * t206 * t128 + 2048.0 / 27.0 * t217 * t224 + 2048.0 / 9.0 * t217 * t228 + 4096.0 / 27.0 * t217 * t232;
        let t240 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t182 * t35 + t6 * t93 * t66 / 4.0 - 3.0 / 8.0 * t6 * t41 * t131 - 3.0 / 8.0 * t6 * t19 * t235);
        let tv3rho30 = 2.0 * rho[ip] * t240 + 6.0 * t136;
        v3rho3[ip] += tv3rho30;
        let t258 = param_beta * t117 * t32;
        let t259 = t119 * t121;
        let t260 = t52 * t123;
        let t261 = t259 * t260;
        let t264 = param_omega * t121;
        let t265 = t264 * t260;
        let t268 = t115 * t54;
        let t270 = 1.0 / t24 / t268;
        let t272 = t114 * t270 * t32;
        let t279 = 88.0 / 9.0 * t74 * t101 * t32 - 1040.0 / 9.0 * t145 * t108 * t32 * t150 + 544.0 / 3.0 * t258 * t261 + 544.0 / 3.0 * t258 * t265 - 256.0 / 9.0 * t272 * t224 - 256.0 / 3.0 * t272 * t228 - 512.0 / 9.0 * t272 * t232;
        let t284 = piecewise3(t2, 0.0, t6 * t93 * t84 / 12.0 - t6 * t41 * t160 / 4.0 - 3.0 / 8.0 * t6 * t19 * t279);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t284 + 4.0 * t165;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t294 = param_beta * t154 * t32;
        let t295 = t123 * sigma[ip];
        let t296 = t259 * t295;
        let t299 = t264 * t295;
        let t303 = 1.0 / t24 / t204;
        let t305 = t53 * t303 * t32;
        let t312 = 64.0 / 3.0 * t145 * t57 * t63 - 160.0 / 3.0 * t294 * t296 - 160.0 / 3.0 * t294 * t299 + 32.0 / 3.0 * t305 * t224 + 32.0 * t305 * t228 + 64.0 / 3.0 * t305 * t232;
        let t317 = piecewise3(t2, 0.0, -t6 * t41 * t175 / 8.0 - 3.0 / 8.0 * t6 * t19 * t312);
        let tv3rhosigma20 = 2.0 * rho[ip] * t317 + 2.0 * t179;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t321 = param_beta * t170 * t32;
        let t322 = t259 * t123;
        let t325 = t264 * t123;
        let t329 = 1.0 / t24 / t116;
        let t331 = t20 * t329 * t32;
        let t338 = -4.0 * t331 * t224 - 12.0 * t331 * t228 - 8.0 * t331 * t232 + 12.0 * t321 * t322 + 12.0 * t321 * t325;
        let t342 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t338);
        let tv3sigma30 = 2.0 * rho[ip] * t342;
        v3sigma3[ip] += tv3sigma30;
        let t371 = t114 / t268;
        let t376 = t115 * t55;
        let t380 = t212 / t24 / t376 * t32;
        let t389 = t115 * t115;
        let t394 = param_beta * t211 * sigma[ip] / t18 / t389 / rho[ip] * t32;
        let t395 = t119 * t119;
        let t396 = t121 * t121;
        let t398 = t122 * t122;
        let t400 = 1.0 / t398 * t21;
        let t401 = t395 * t396 * t400;
        let t405 = t218 * t396 * t400;
        let t409 = t119 * t396 * t400;
        let t413 = param_omega * t396 * t400;
        let t416 = 20944.0 / 81.0 * t20 * t22 / t24 / t55 * t32 - 97504.0 / 27.0 * t53 * t21 / t18 / t153 * t63 + 656128.0 / 81.0 * t371 * t125 + 656128.0 / 81.0 * t371 * t128 - 200704.0 / 81.0 * t380 * t224 - 200704.0 / 27.0 * t380 * t228 - 401408.0 / 81.0 * t380 * t232 + 32768.0 / 81.0 * t394 * t401 + 65536.0 / 27.0 * t394 * t405 + 360448.0 / 81.0 * t394 * t409 + 65536.0 / 27.0 * t394 * t413;
        let t421 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 * t47 * t35 - 5.0 / 9.0 * t6 * t182 * t66 + t6 * t93 * t131 / 2.0 - t6 * t41 * t235 / 2.0 - 3.0 / 8.0 * t6 * t19 * t416);
        let tv4rho40 = 2.0 * rho[ip] * t421 + 8.0 * t240;
        v4rho4[ip] += tv4rho40;
        let t442 = param_beta * t205 * t32;
        let t447 = param_beta * t215;
        let t448 = t32 * t218;
        let t451 = t219 * t113 * t223;
        let t463 = t212 / t18 / t389 * t32;
        let t472 = -1232.0 / 27.0 * t74 * t193 * t32 + 8096.0 / 9.0 * t145 * t199 * t32 * t150 - 65600.0 / 27.0 * t442 * t261 - 65600.0 / 27.0 * t442 * t265 + 22784.0 / 27.0 * t447 * t448 * t451 + 22784.0 / 9.0 * t447 * t120 * t451 + 45568.0 / 27.0 * t447 * t60 * t451 - 4096.0 / 27.0 * t463 * t401 - 8192.0 / 9.0 * t463 * t405 - 45056.0 / 27.0 * t463 * t409 - 8192.0 / 9.0 * t463 * t413;
        let t477 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t182 * t84 + t6 * t93 * t160 / 4.0 - 3.0 / 8.0 * t6 * t41 * t279 - 3.0 / 8.0 * t6 * t19 * t472);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t477 + 6.0 * t284;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t494 = param_beta * t270;
        let t496 = t219 * t222;
        let t498 = t496 * t52 * t22;
        let t511 = t114 / t18 / t115 / t106 * t32;
        let t525 = piecewise3(t2, 0.0, t6 * t93 * t175 / 12.0 - t6 * t41 * t312 / 4.0 - 3.0 / 8.0 * t6 * t19 * (-1216.0 / 9.0 * t145 * t108 * t63 + 5344.0 / 9.0 * t258 * t296 + 5344.0 / 9.0 * t258 * t299 - 800.0 / 3.0 * t494 * t448 * t498 - 800.0 * t494 * t120 * t498 - 1600.0 / 3.0 * t494 * t60 * t498 + 512.0 / 9.0 * t511 * t401 + 1024.0 / 3.0 * t511 * t405 + 5632.0 / 9.0 * t511 * t409 + 1024.0 / 3.0 * t511 * t413));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t525 + 4.0 * t317;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t533 = param_beta * t303;
        let t536 = t496 * sigma[ip] * t22;
        let t550 = t53 / t18 / t376 * t32;
        let t564 = piecewise3(t2, 0.0, -t6 * t41 * t338 / 8.0 - 3.0 / 8.0 * t6 * t19 * (-96.0 * t294 * t322 + 224.0 / 3.0 * t533 * t448 * t536 + 224.0 * t533 * t120 * t536 - 96.0 * t294 * t325 + 448.0 / 3.0 * t533 * t60 * t536 - 64.0 / 3.0 * t550 * t401 - 128.0 * t550 * t405 - 704.0 / 3.0 * t550 * t409 - 128.0 * t550 * t413));
        let tv4rhosigma30 = 2.0 * rho[ip] * t564 + 2.0 * t342;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t568 = param_beta * t329 * t32;
        let t578 = t20 / t18 / t213 * t32;
        let t591 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * (-16.0 * t568 * t224 - 48.0 * t568 * t228 - 32.0 * t568 * t232 + 8.0 * t578 * t401 + 48.0 * t578 * t405 + 88.0 * t578 * t409 + 48.0 * t578 * t413));
        let tv4sigma40 = 2.0 * rho[ip] * t591;
        v4sigma4[ip] += tv4sigma40;
    }
}
