//! GGA_K_LC94 lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lc94.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lc94_lxc_unpol(
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
    param_a: f64,
    param_alpha: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_expo: f64,
    param_f: f64,
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
        let t24 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t22 / t34;
        let t37 = t33 * t36;
        let t40 = f64::exp(-param_alpha * t24 * t29 * t37 / 24.0);
        let t43 = (t40 * param_d + param_c) * t24;
        let t44 = t43 * t29;
        let t47 = t24 * t24;
        let t48 = 1.0 / t27;
        let t49 = t47 * t48;
        let t50 = f64::sqrt(sigma[ip]);
        let t53 = 1.0 / t21 / rho[ip];
        let t54 = t50 * t31 * t53;
        let t57 = f64::powf(t49 * t54 / 12.0, param_expo);
        let t58 = param_f * t57;
        let t59 = t44 * t37 / 24.0 - t58;
        let t60 = t49 * t50;
        let t66 = f64::ln(param_b * t47 * t48 * t54 / 12.0 + f64::sqrt(pow_2(param_b * t47 * t48 * t54 / 12.0) + 1.0));
        let t67 = param_a * t66;
        let t68 = t31 * t53 * t67;
        let t71 = 1.0 + t60 * t68 / 12.0 + t58;
        let t72 = 1.0 / t71;
        let t74 = t59 * t72 + 1.0;
        let t78 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        let t80 = t20 / t21;
        let t84 = param_d * param_alpha;
        let t86 = 1.0 / t27 / t26;
        let t87 = t47 * t86;
        let t88 = t84 * t87;
        let t89 = sigma[ip] * sigma[ip];
        let t90 = t89 * t31;
        let t91 = t34 * t34;
        let t92 = t91 * t34;
        let t94 = 1.0 / t21 / t92;
        let t95 = t94 * t40;
        let t99 = t34 * rho[ip];
        let t101 = 1.0 / t22 / t99;
        let t105 = 1.0 / rho[ip];
        let t108 = 4.0 / 3.0 * t58 * param_expo * t105;
        let t109 = t88 * t90 * t95 / 108.0 - t44 * t33 * t101 / 9.0 + t108;
        let t111 = t71 * t71;
        let t112 = 1.0 / t111;
        let t113 = t59 * t112;
        let t115 = 1.0 / t21 / t34;
        let t117 = t31 * t115 * t67;
        let t120 = t24 * t29;
        let t121 = t120 * t33;
        let t123 = param_b * param_b;
        let t128 = 6.0 * t123 * t24 * t29 * t37 + 144.0;
        let t129 = f64::sqrt(t128);
        let t131 = param_b / t129;
        let t132 = t101 * param_a * t131;
        let t135 = -t60 * t117 / 9.0 - 2.0 / 3.0 * t121 * t132 - t108;
        let t137 = t109 * t72 - t113 * t135;
        let t142 = piecewise3(t2, 0.0, t7 * t80 * t74 / 10.0 + 3.0 / 20.0 * t7 * t23 * t137);
        let tvrho0 = 2.0 * rho[ip] * t142 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t145 = t91 * rho[ip];
        let t147 = 1.0 / t21 / t145;
        let t148 = t31 * t147;
        let t149 = t40 * sigma[ip];
        let t153 = t29 * t32;
        let t157 = 1.0 / sigma[ip];
        let t160 = t58 * param_expo * t157 / 2.0;
        let t161 = -t88 * t148 * t149 / 288.0 + t43 * t153 * t36 / 24.0 - t160;
        let t164 = t49 / t50;
        let t167 = t120 * t32;
        let t169 = t36 * param_a * t131;
        let t172 = t164 * t68 / 24.0 + t167 * t169 / 4.0 + t160;
        let t174 = -t113 * t172 + t161 * t72;
        let t178 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t174);
        let tvsigma0 = 2.0 * rho[ip] * t178;
        vsigma[ip] += tvsigma0;
        let t181 = t20 * t53;
        let t188 = t91 * t99;
        let t190 = 1.0 / t21 / t188;
        let t191 = t190 * t40;
        let t195 = param_alpha * param_alpha;
        let t196 = param_d * t195;
        let t197 = t26 * t26;
        let t198 = 1.0 / t197;
        let t199 = t196 * t198;
        let t200 = t89 * sigma[ip];
        let t201 = t91 * t91;
        let t202 = t201 * t34;
        let t203 = 1.0 / t202;
        let t209 = 1.0 / t22 / t91;
        let t213 = param_expo * param_expo;
        let t214 = 1.0 / t34;
        let t215 = t213 * t214;
        let t217 = 16.0 / 9.0 * t58 * t215;
        let t220 = 4.0 / 3.0 * t58 * param_expo * t214;
        let t221 = -t88 * t90 * t191 / 12.0 + t199 * t200 * t203 * t40 / 81.0 + 11.0 / 27.0 * t44 * t33 * t209 - t217 - t220;
        let t223 = t109 * t112;
        let t227 = 1.0 / t111 / t71;
        let t228 = t59 * t227;
        let t229 = t135 * t135;
        let t233 = 1.0 / t21 / t99;
        let t235 = t31 * t233 * t67;
        let t239 = t209 * param_a * t131;
        let t242 = t87 * t90;
        let t244 = t123 * param_b;
        let t246 = 1.0 / t129 / t128;
        let t247 = t244 * t246;
        let t248 = t190 * param_a * t247;
        let t251 = 7.0 / 27.0 * t60 * t235 + 10.0 / 3.0 * t121 * t239 - 32.0 / 3.0 * t242 * t248 + t217 + t220;
        let t253 = -t113 * t251 - 2.0 * t135 * t223 + t221 * t72 + 2.0 * t228 * t229;
        let t258 = piecewise3(t2, 0.0, -t7 * t181 * t74 / 30.0 + t7 * t80 * t137 / 5.0 + 3.0 / 20.0 * t7 * t23 * t253);
        let tv2rho20 = 2.0 * rho[ip] * t258 + 4.0 * t142;
        v2rho2[ip] += tv2rho20;
        let t264 = t31 * t94;
        let t268 = t201 * rho[ip];
        let t269 = 1.0 / t268;
        let t277 = t213 * t105;
        let t280 = 2.0 / 3.0 * t58 * t277 * t157;
        let t281 = t88 * t264 * t149 / 36.0 - t199 * t269 * t89 * t40 / 216.0 - t43 * t153 * t101 / 9.0 + t280;
        let t283 = t161 * t112;
        let t286 = t172 * t135;
        let t295 = param_a * t244 * t246 * sigma[ip];
        let t298 = -t164 * t117 / 18.0 - t167 * t132 + 4.0 * t87 * t264 * t295 - t280;
        let t300 = -t113 * t298 - t135 * t283 - t172 * t223 + 2.0 * t228 * t286 + t281 * t72;
        let t305 = piecewise3(t2, 0.0, t7 * t80 * t174 / 10.0 + 3.0 / 20.0 * t7 * t23 * t300);
        let tv2rhosigma0 = 2.0 * rho[ip] * t305 + 2.0 * t178;
        v2rhosigma[ip] += tv2rhosigma0;
        let t308 = 1.0 / t201;
        let t313 = t84 * t47;
        let t314 = t86 * t31;
        let t319 = 1.0 / t89;
        let t322 = t58 * t213 * t319 / 4.0;
        let t325 = t58 * param_expo * t319 / 2.0;
        let t326 = t199 * t308 * t40 * sigma[ip] / 576.0 - t313 * t314 * t147 * t40 / 144.0 - t322 + t325;
        let t330 = t172 * t172;
        let t335 = t49 / t50 / sigma[ip];
        let t339 = t120 * t157 * t32;
        let t342 = t87 * t31;
        let t344 = t147 * param_a * t247;
        let t347 = -t335 * t68 / 48.0 + t339 * t169 / 8.0 - 3.0 / 2.0 * t342 * t344 + t322 - t325;
        let t349 = -t113 * t347 - 2.0 * t172 * t283 + 2.0 * t228 * t330 + t326 * t72;
        let t353 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t349);
        let tv2sigma20 = 2.0 * rho[ip] * t353;
        v2sigma2[ip] += tv2sigma20;
        let t356 = t20 * t115;
        let t367 = 1.0 / t21 / t201;
        let t372 = t201 * t99;
        let t373 = 1.0 / t372;
        let t379 = param_d * t195 * param_alpha;
        let t380 = t89 * t89;
        let t381 = t198 * t380;
        let t382 = t379 * t381;
        let t383 = t201 * t145;
        let t385 = 1.0 / t22 / t383;
        let t387 = t153 * t40;
        let t392 = 1.0 / t22 / t145;
        let t396 = t213 * param_expo;
        let t397 = 1.0 / t99;
        let t398 = t396 * t397;
        let t400 = 64.0 / 27.0 * t58 * t398;
        let t401 = t213 * t397;
        let t403 = 16.0 / 3.0 * t58 * t401;
        let t406 = 8.0 / 3.0 * t58 * param_expo * t397;
        let t407 = 341.0 / 486.0 * t88 * t90 * t367 * t40 - 19.0 / 81.0 * t199 * t200 * t373 * t40 + t382 * t385 * t24 * t387 / 729.0 - 154.0 / 81.0 * t44 * t33 * t392 + t400 + t403 + t406;
        let t409 = t221 * t112;
        let t412 = t109 * t227;
        let t417 = t111 * t111;
        let t418 = 1.0 / t417;
        let t419 = t59 * t418;
        let t420 = t229 * t135;
        let t423 = t135 * t251;
        let t429 = t31 / t21 / t91 * t67;
        let t433 = t392 * param_a * t131;
        let t440 = t198 * t200;
        let t442 = t123 * t123;
        let t443 = t442 * param_b;
        let t445 = t128 * t128;
        let t447 = 1.0 / t129 / t445;
        let t448 = param_a * t443 * t447;
        let t451 = -70.0 / 81.0 * t60 * t429 - 476.0 / 27.0 * t121 * t433 + 1184.0 / 9.0 * t242 * t367 * param_a * t247 - 3072.0 * t440 * t373 * t448 - t400 - t403 - t406;
        let t453 = -t113 * t451 - 3.0 * t135 * t409 - 3.0 * t223 * t251 + 6.0 * t228 * t423 + 6.0 * t229 * t412 + t407 * t72 - 6.0 * t419 * t420;
        let t458 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t356 * t74 - t7 * t181 * t137 / 10.0 + 3.0 / 10.0 * t7 * t80 * t253 + 3.0 / 20.0 * t7 * t23 * t453);
        let tv3rho30 = 2.0 * rho[ip] * t458 + 6.0 * t258;
        v3rho3[ip] += tv3rho30;
        let t468 = t31 * t190;
        let t476 = t201 * t91;
        let t479 = t198 / t22 / t476;
        let t480 = t379 * t479;
        let t482 = t200 * t24 * t387;
        let t488 = t396 * t214;
        let t491 = 8.0 / 9.0 * t58 * t488 * t157;
        let t494 = 2.0 / 3.0 * t58 * t215 * t157;
        let t495 = -65.0 / 324.0 * t88 * t468 * t149 + 17.0 / 216.0 * t199 * t203 * t89 * t40 - t480 * t482 / 1944.0 + 11.0 / 27.0 * t43 * t153 * t209 - t491 - t494;
        let t497 = t281 * t112;
        let t500 = t161 * t227;
        let t509 = t172 * t229;
        let t512 = t298 * t135;
        let t515 = t172 * t251;
        let t526 = t198 * t203 * param_a;
        let t527 = t443 * t447;
        let t528 = t527 * t89;
        let t531 = 7.0 / 54.0 * t164 * t235 + 37.0 / 9.0 * t167 * t239 - 124.0 / 3.0 * t87 * t468 * t295 + 1152.0 * t526 * t528 + t491 + t494;
        let t533 = -t113 * t531 - 2.0 * t135 * t497 - t172 * t409 - 2.0 * t223 * t298 + 4.0 * t228 * t512 + 2.0 * t228 * t515 + 2.0 * t229 * t500 - t251 * t283 + 4.0 * t286 * t412 - 6.0 * t419 * t509 + t495 * t72;
        let t538 = piecewise3(t2, 0.0, -t7 * t181 * t174 / 30.0 + t7 * t80 * t300 / 5.0 + 3.0 / 20.0 * t7 * t23 * t533);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t538 + 4.0 * t305;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t550 = t198 / t22 / t372;
        let t551 = t379 * t550;
        let t554 = t120 * t89 * t32 * t40;
        let t560 = t396 * t105;
        let t563 = t58 * t560 * t319 / 3.0;
        let t566 = 2.0 / 3.0 * t58 * t277 * t319;
        let t567 = -5.0 / 216.0 * t199 * t269 * t40 * sigma[ip] + t551 * t554 / 5184.0 + t313 * t314 * t95 / 27.0 + t563 - t566;
        let t569 = t326 * t112;
        let t579 = t330 * t135;
        let t582 = t172 * t298;
        let t586 = t347 * t135;
        let t594 = t94 * param_a * t247;
        let t597 = t198 * t269;
        let t599 = t527 * sigma[ip];
        let t602 = t335 * t117 / 36.0 - t339 * t132 / 6.0 + 10.0 * t342 * t594 - 432.0 * t597 * param_a * t599 - t563 + t566;
        let t604 = -t113 * t602 - t135 * t569 - 2.0 * t172 * t497 - t223 * t347 + 4.0 * t228 * t582 + 2.0 * t228 * t586 - 2.0 * t283 * t298 + 4.0 * t286 * t500 + 2.0 * t330 * t412 - 6.0 * t419 * t579 + t567 * t72;
        let t609 = piecewise3(t2, 0.0, t7 * t80 * t349 / 10.0 + 3.0 / 20.0 * t7 * t23 * t604);
        let tv3rhosigma20 = 2.0 * rho[ip] * t609 + 2.0 * t353;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t614 = t198 / t22 / t202;
        let t615 = t379 * t614;
        let t616 = t32 * t40;
        let t618 = t120 * t616 * sigma[ip];
        let t621 = t198 * t308;
        let t625 = 1.0 / t200;
        let t628 = t58 * t396 * t625 / 8.0;
        let t631 = 3.0 / 4.0 * t58 * t213 * t625;
        let t633 = t58 * param_expo * t625;
        let t634 = -t615 * t618 / 13824.0 + t196 * t621 * t40 / 192.0 - t628 + t631 - t633;
        let t642 = t330 * t172;
        let t645 = t172 * t347;
        let t650 = t49 / t50 / t89;
        let t654 = t120 * t319 * t32;
        let t658 = t87 * t157 * t31;
        let t663 = t650 * t68 / 32.0 - 3.0 / 16.0 * t654 * t169 - 3.0 / 4.0 * t658 * t344 + 162.0 * t621 * t448 + t628 - t631 + t633;
        let t665 = -t113 * t663 - 3.0 * t172 * t569 + 6.0 * t228 * t645 - 3.0 * t283 * t347 + 6.0 * t330 * t500 - 6.0 * t419 * t642 + t634 * t72;
        let t669 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t665);
        let tv3sigma30 = 2.0 * rho[ip] * t669;
        v3sigma3[ip] += tv3sigma30;
        let t686 = 1.0 / t21 / t268;
        let t691 = 1.0 / t476;
        let t696 = t201 * t92;
        let t698 = 1.0 / t22 / t696;
        let t703 = t195 * t195;
        let t704 = param_d * t703;
        let t708 = t201 * t201;
        let t713 = t314 * t40;
        let t718 = 1.0 / t22 / t92;
        let t722 = t213 * t213;
        let t723 = 1.0 / t91;
        let t726 = 256.0 / 81.0 * t58 * t722 * t723;
        let t729 = 128.0 / 9.0 * t58 * t396 * t723;
        let t732 = 176.0 / 9.0 * t58 * t213 * t723;
        let t735 = 8.0 * t58 * param_expo * t723;
        let t738 = t407 * t112;
        let t741 = t221 * t227;
        let t746 = t109 * t418;
        let t755 = t59 / t417 / t71;
        let t756 = t229 * t229;
        let t762 = t251 * t251;
        let t784 = t442 * t244;
        let t787 = 1.0 / t129 / t445 / t128;
        let t794 = (-3047.0 / 486.0 * t88 * t90 * t686 * t40 + 2563.0 / 729.0 * t199 * t200 * t691 * t40 - 98.0 / 2187.0 * t382 * t698 * t24 * t387 + 2.0 / 6561.0 * t704 * t198 * t380 * sigma[ip] / t21 / t708 / rho[ip] * t47 * t713 + 2618.0 / 243.0 * t44 * t33 * t718 - t726 - t729 - t732 - t735) * t72 - 4.0 * t738 * t135 + 12.0 * t741 * t229 - 6.0 * t409 * t251 - 24.0 * t746 * t420 + 24.0 * t412 * t423 - 4.0 * t223 * t451 + 24.0 * t755 * t756 - 36.0 * t419 * t229 * t251 + 6.0 * t228 * t762 + 8.0 * t228 * t135 * t451 - t113 * (910.0 / 243.0 * t60 * t148 * t67 + 2884.0 / 27.0 * t121 * t718 * param_a * t131 - 37216.0 / 27.0 * t242 * t686 * param_a * t247 + 71680.0 * t440 * t691 * t448 - 122880.0 * t381 * t698 * param_a * t784 * t787 * t167 + t726 + t729 + t732 + t735);
        let t799 = piecewise3(t2, 0.0, -14.0 / 135.0 * t7 * t20 * t233 * t74 + 8.0 / 45.0 * t7 * t356 * t137 - t7 * t181 * t253 / 5.0 + 2.0 / 5.0 * t7 * t80 * t453 + 3.0 / 20.0 * t7 * t23 * t794);
        let tv4rho40 = 2.0 * rho[ip] * t799 + 8.0 * t458;
        v4rho4[ip] += tv4rho40;
        let t839 = t281 * t227;
        let t842 = t161 * t418;
        let t847 = t31 * t367;
        let t855 = t198 * t385;
        let t873 = 32.0 / 27.0 * t58 * t722 * t397 * t157;
        let t876 = 8.0 / 3.0 * t58 * t398 * t157;
        let t879 = 4.0 / 3.0 * t58 * t401 * t157;
        let t882 = t495 * t112;
        let t904 = param_a * t784;
        let t912 = 6.0 * t839 * t229 - 6.0 * t842 * t420 + 6.0 * t500 * t423 + (253.0 / 162.0 * t88 * t847 * t149 - 1025.0 / 972.0 * t199 * t373 * t89 * t40 + 89.0 / 5832.0 * t379 * t855 * t482 - t704 * t198 / t21 / t708 * t380 * t47 * t713 / 8748.0 - 154.0 / 81.0 * t43 * t153 * t392 + t873 + t876 + t879) * t72 - 3.0 * t882 * t135 - 3.0 * t497 * t251 - t283 * t451 - t738 * t172 - 3.0 * t409 * t298 - 3.0 * t223 * t531 - t113 * (-35.0 / 81.0 * t164 * t429 - 182.0 / 9.0 * t167 * t433 + 3320.0 / 9.0 * t87 * t847 * t295 - 23424.0 * t198 * t373 * param_a * t528 + 46080.0 * t855 * t904 * t787 * t200 * t167 - t873 - t876 - t879);
        let t918 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t356 * t174 - t7 * t181 * t300 / 10.0 + 3.0 / 10.0 * t7 * t80 * t533 + 3.0 / 20.0 * t7 * t23 * (6.0 * t228 * t531 * t135 + 2.0 * t228 * t172 * t451 + 24.0 * t755 * t172 * t420 + 6.0 * t228 * t298 * t251 - 18.0 * t419 * t298 * t229 - 18.0 * t419 * t286 * t251 + 6.0 * t741 * t286 + 12.0 * t412 * t512 + 6.0 * t412 * t515 - 18.0 * t746 * t509 + t912));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t918 + 6.0 * t538;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t928 = t298 * t298;
        let t931 = t326 * t227;
        let t962 = 4.0 * t135 * t228 * t602 + 4.0 * t172 * t228 * t531 + 2.0 * t228 * t251 * t347 - 6.0 * t229 * t347 * t419 - 6.0 * t251 * t330 * t419 - 24.0 * t286 * t298 * t419 + 4.0 * t228 * t928 + 2.0 * t229 * t931 + 8.0 * t286 * t839 + 4.0 * t412 * t586 + 8.0 * t500 * t512 + 4.0 * t500 * t515 - 12.0 * t579 * t746;
        let t992 = 4.0 / 9.0 * t58 * t722 * t214 * t319;
        let t995 = 5.0 / 9.0 * t58 * t488 * t319;
        let t998 = 2.0 / 3.0 * t58 * t215 * t319;
        let t1001 = t567 * t112;
        let t1031 = 8.0 * t412 * t582 - 12.0 * t842 * t509 + 24.0 * t755 * t330 * t229 + (167.0 / 648.0 * t199 * t203 * t40 * sigma[ip] - 25.0 / 5184.0 * t480 * t554 + t704 * t198 / t21 / t201 / t188 * t87 * t200 * t31 * t40 / 23328.0 - 19.0 / 81.0 * t313 * t314 * t191 - t992 + t995 + t998) * t72 - 2.0 * t1001 * t135 - t569 * t251 - 2.0 * t882 * t172 - 4.0 * t497 * t298 - 2.0 * t283 * t531 + 2.0 * t741 * t330 - t409 * t347 - 2.0 * t223 * t602 - t113 * (-7.0 / 108.0 * t335 * t235 + 7.0 / 18.0 * t339 * t239 - 66.0 * t342 * t248 + 6768.0 * t526 * t599 - 17280.0 * t479 * t904 * t787 * t89 * t167 + t992 - t995 - t998);
        let t1037 = piecewise3(t2, 0.0, -t7 * t181 * t349 / 30.0 + t7 * t80 * t604 / 5.0 + 3.0 / 20.0 * t7 * t23 * (t962 + t1031));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t1037 + 4.0 * t609;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t1059 = t58 * t722 * t105 * t625 / 6.0;
        let t1061 = t58 * t560 * t625;
        let t1064 = 4.0 / 3.0 * t58 * t277 * t625;
        let t1067 = t634 * t112;
        let t1119 = t787 * t24;
        let t1127 = -3.0 * t283 * t602 - 6.0 * t746 * t642 + 24.0 * t755 * t642 * t135 - 18.0 * t419 * t330 * t298 + 6.0 * t412 * t645 - 18.0 * t419 * t645 * t135 + 6.0 * t228 * t298 * t347 + 6.0 * t228 * t172 * t602 - t223 * t663 + 2.0 * t228 * t663 * t135 - t113 * (-t650 * t117 / 24.0 + t654 * t132 / 4.0 + t658 * t594 - 1512.0 * t597 * t448 + 6480.0 * t550 * t904 * t1119 * t29 * sigma[ip] * t32 - t1059 + t1061 - t1064);
        let t1133 = piecewise3(t2, 0.0, t7 * t80 * t665 / 10.0 + 3.0 / 20.0 * t7 * t23 * ((7.0 / 5184.0 * t551 * t618 - t704 * t198 / t21 / t696 * t87 * t90 * t40 / 62208.0 - t196 * t597 * t40 / 24.0 + t1059 - t1061 + t1064) * t72 - t1067 * t135 - 3.0 * t1001 * t172 + 6.0 * t931 * t286 - 3.0 * t569 * t298 + 6.0 * t839 * t330 - 18.0 * t842 * t579 + 12.0 * t500 * t582 - 3.0 * t497 * t347 + 6.0 * t500 * t586 + t1127));
        let tv4rhosigma30 = 2.0 * rho[ip] * t1133 + 2.0 * t669;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t1148 = 1.0 / t380;
        let t1151 = t58 * t722 * t1148 / 16.0;
        let t1154 = 3.0 / 4.0 * t58 * t396 * t1148;
        let t1157 = 11.0 / 4.0 * t58 * t213 * t1148;
        let t1160 = 3.0 * t58 * param_expo * t1148;
        let t1175 = t330 * t330;
        let t1181 = t347 * t347;
        let t1210 = (t704 * t198 / t21 / t383 * t87 * t31 * t40 * sigma[ip] / 165888.0 - t615 * t120 * t616 / 3456.0 - t1151 + t1154 - t1157 + t1160) * t72 - 4.0 * t1067 * t172 + 12.0 * t931 * t330 - 6.0 * t569 * t347 - 24.0 * t842 * t642 + 24.0 * t500 * t645 - 4.0 * t283 * t663 + 24.0 * t755 * t1175 - 36.0 * t419 * t330 * t347 + 6.0 * t228 * t1181 + 8.0 * t228 * t172 * t663 - t113 * (-5.0 / 64.0 * t49 / t50 / t200 * t68 + 15.0 / 32.0 * t120 * t625 * t32 * t169 + 15.0 / 8.0 * t87 * t319 * t31 * t344 + 81.0 * t198 * t157 * t308 * t448 - 2430.0 * t614 * t904 * t1119 * t153 + t1151 - t1154 + t1157 - t1160);
        let t1214 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t1210);
        let tv4sigma40 = 2.0 * rho[ip] * t1214;
        v4sigma4[ip] += tv4sigma40;
    }
}
