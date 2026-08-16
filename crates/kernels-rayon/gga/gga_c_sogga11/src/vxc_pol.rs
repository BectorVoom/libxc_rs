//! GGA_C_SOGGA11 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_sogga11.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_sogga11_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_sogga11_a_0: f64,
    param_sogga11_a_1: f64,
    param_sogga11_a_2: f64,
    param_sogga11_a_3: f64,
    param_sogga11_a_4: f64,
    param_sogga11_a_5: f64,
    param_sogga11_b_0: f64,
    param_sogga11_b_1: f64,
    param_sogga11_b_2: f64,
    param_sogga11_b_3: f64,
    param_sogga11_b_4: f64,
    param_sogga11_b_5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t11 = t4 * t6 / t8;
        let t13 = 1.0 + 0.53425e-1 * t11;
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t1 * t1;
        let t20 = t3 * t3;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t5 / t22;
        let t27 = 0.379785e1 * t14 + 0.8969e0 * t11 + 0.204775e0 * t17 + 0.123235e0 * t25;
        let t30 = 1.0 + 0.16081979498692535067e2 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.621814e-1 * t13 * t31;
        let t34 = rho0 - rho1;
        let t35 = t34 * t34;
        let t36 = t35 * t35;
        let t37 = t7 * t7;
        let t38 = t37 * t37;
        let t39 = 1.0 / t38;
        let t40 = t36 * t39;
        let t41 = 1.0 / t7;
        let t42 = t34 * t41;
        let t43 = 1.0 + t42;
        let t44 = t43 <= zeta_threshold;
        let t45 = pow_1_3(zeta_threshold);
        let t46 = t45 * zeta_threshold;
        let t47 = pow_1_3(t43);
        let t48 = t47 * t43;
        let t49 = piecewise3(t44, t46, t48);
        let t50 = 1.0 - t42;
        let t51 = t50 <= zeta_threshold;
        let t52 = pow_1_3(t50);
        let t53 = t52 * t50;
        let t54 = piecewise3(t51, t46, t53);
        let t55 = t49 + t54 - 2.0;
        let t56 = M_CBRT2;
        let t59 = 1.0 / (2.0 * t56 - 2.0);
        let t60 = t55 * t59;
        let t62 = 1.0 + 0.5137e-1 * t11;
        let t67 = 0.705945e1 * t14 + 0.1549425e1 * t11 + 0.420775e0 * t17 + 0.1562925e0 * t25;
        let t70 = 1.0 + 0.32163958997385070134e2 / t67;
        let t71 = f64::ln(t70);
        let t75 = 1.0 + 0.278125e-1 * t11;
        let t80 = 0.51785e1 * t14 + 0.905775e0 * t11 + 0.1100325e0 * t17 + 0.1241775e0 * t25;
        let t83 = 1.0 + 0.29608749977793437516e2 / t80;
        let t84 = f64::ln(t83);
        let t85 = t75 * t84;
        let t87 = -0.310907e-1 * t62 * t71 + t33 - 0.19751673498613801407e-1 * t85;
        let t88 = t60 * t87;
        let t92 = -t33 + t40 * t88 + 0.19751673498613801407e-1 * t60 * t85;
        let t94 = param_sogga11_a_1;
        let t95 = t45 * t45;
        let t96 = t47 * t47;
        let t97 = piecewise3(t44, t95, t96);
        let t98 = t52 * t52;
        let t99 = piecewise3(t51, t95, t98);
        let t101 = t97 / 2.0 + t99 / 2.0;
        let t102 = t56 * t101;
        let t104 = sigma0 + 2.0 * sigma1 + sigma2;
        let t106 = 1.0 / t8 / t37;
        let t107 = t104 * t106;
        let t108 = t102 * t107;
        let t109 = 1.0 / t3;
        let t110 = t19 * t109;
        let t111 = 1.0 / t92;
        let t112 = t5 * t111;
        let t113 = t110 * t112;
        let t115 = 0.69506584583333333332e-3 * t108 * t113;
        let t116 = 1.0 - t115;
        let t118 = 1.0 - 1.0 / t116;
        let t120 = param_sogga11_a_2;
        let t121 = t118 * t118;
        let t123 = param_sogga11_a_3;
        let t124 = t121 * t118;
        let t126 = param_sogga11_a_4;
        let t127 = t121 * t121;
        let t129 = param_sogga11_a_5;
        let t133 = param_sogga11_b_1;
        let t134 = f64::exp(t115);
        let t135 = 1.0 - t134;
        let t137 = param_sogga11_b_2;
        let t138 = t135 * t135;
        let t140 = param_sogga11_b_3;
        let t141 = t138 * t135;
        let t143 = param_sogga11_b_4;
        let t144 = t138 * t138;
        let t146 = param_sogga11_b_5;
        let t149 = t129 * t127 * t118 + t146 * t144 * t135 + t94 * t118 + t120 * t121 + t123 * t124 + t126 * t127 + t133 * t135 + t137 * t138 + t140 * t141 + t143 * t144 + param_sogga11_a_0 + param_sogga11_b_0;
        let tzk0 = t92 * t149;
        zk[ip] += tzk0;
        let t151 = 1.0 / t8 / t7;
        let t152 = t6 * t151;
        let t155 = 0.11073470983333333333e-2 * t4 * t152 * t31;
        let t156 = t27 * t27;
        let t157 = 1.0 / t156;
        let t158 = t13 * t157;
        let t160 = 1.0 / t14 * t1;
        let t161 = t3 * t6;
        let t162 = t161 * t151;
        let t163 = t160 * t162;
        let t165 = t4 * t152;
        let t167 = f64::sqrt(t11);
        let t168 = t167 * t1;
        let t169 = t168 * t162;
        let t174 = t21 * t5 / t22 / t7;
        let t176 = -0.632975e0 * t163 - 0.29896666666666666667e0 * t165 - 0.1023875e0 * t169 - 0.82156666666666666667e-1 * t174;
        let t177 = 1.0 / t30;
        let t178 = t176 * t177;
        let t180 = 1.0 * t158 * t178;
        let t181 = t35 * t34;
        let t182 = t181 * t39;
        let t184 = 4.0 * t182 * t88;
        let t185 = t38 * t7;
        let t186 = 1.0 / t185;
        let t187 = t36 * t186;
        let t189 = 4.0 * t187 * t88;
        let t190 = 1.0 / t37;
        let t191 = t34 * t190;
        let t192 = t41 - t191;
        let t195 = piecewise3(t44, 0.0, 4.0 / 3.0 * t47 * t192);
        let t196 = -t192;
        let t199 = piecewise3(t51, 0.0, 4.0 / 3.0 * t52 * t196);
        let t201 = (t195 + t199) * t59;
        let t202 = t201 * t87;
        let t207 = t67 * t67;
        let t208 = 1.0 / t207;
        let t209 = t62 * t208;
        let t214 = -0.1176575e1 * t163 - 0.516475e0 * t165 - 0.2103875e0 * t169 - 0.104195e0 * t174;
        let t215 = 1.0 / t70;
        let t216 = t214 * t215;
        let t222 = t80 * t80;
        let t223 = 1.0 / t222;
        let t224 = t75 * t223;
        let t229 = -0.86308333333333333334e0 * t163 - 0.301925e0 * t165 - 0.5501625e-1 * t169 - 0.82785e-1 * t174;
        let t230 = 1.0 / t83;
        let t231 = t229 * t230;
        let t234 = 0.53237641966666666666e-3 * t4 * t152 * t71 + 1.0 * t209 * t216 - t155 - t180 + 0.18311447306006545054e-3 * t4 * t152 * t84 + 0.5848223622634646207e0 * t224 * t231;
        let t235 = t60 * t234;
        let t236 = t40 * t235;
        let t239 = t60 * t1;
        let t241 = t161 * t151 * t84;
        let t243 = 0.18311447306006545054e-3 * t239 * t241;
        let t244 = t60 * t75;
        let t246 = t223 * t229 * t230;
        let t248 = 0.5848223622634646207e0 * t244 * t246;
        let t249 = t155 + t180 + t184 - t189 + t40 * t202 + t236 + 0.19751673498613801407e-1 * t201 * t85 - t243 - t248;
        let t250 = t7 * t249;
        let t252 = t7 * t92;
        let t253 = t116 * t116;
        let t254 = 1.0 / t253;
        let t255 = t94 * t254;
        let t256 = 1.0 / t47;
        let t259 = piecewise3(t44, 0.0, 2.0 / 3.0 * t256 * t192);
        let t260 = 1.0 / t52;
        let t263 = piecewise3(t51, 0.0, 2.0 / 3.0 * t260 * t196);
        let t265 = t259 / 2.0 + t263 / 2.0;
        let t266 = t56 * t265;
        let t267 = t266 * t107;
        let t270 = t37 * t7;
        let t272 = 1.0 / t8 / t270;
        let t273 = t104 * t272;
        let t274 = t102 * t273;
        let t276 = 0.16218203069444444444e-2 * t274 * t113;
        let t277 = t92 * t92;
        let t278 = 1.0 / t277;
        let t279 = t5 * t278;
        let t280 = t279 * t249;
        let t281 = t110 * t280;
        let t284 = -0.69506584583333333332e-3 * t267 * t113 + t276 + 0.69506584583333333332e-3 * t108 * t281;
        let t286 = t120 * t118;
        let t287 = t254 * t284;
        let t290 = t123 * t121;
        let t293 = t126 * t124;
        let t296 = t129 * t127;
        let t299 = -t284;
        let t300 = t133 * t299;
        let t302 = t137 * t135;
        let t303 = t299 * t134;
        let t306 = t140 * t138;
        let t309 = t143 * t141;
        let t312 = t146 * t144;
        let t315 = -t300 * t134 + t255 * t284 + 2.0 * t286 * t287 + 3.0 * t290 * t287 + 4.0 * t293 * t287 + 5.0 * t296 * t287 - 2.0 * t302 * t303 - 3.0 * t306 * t303 - 4.0 * t309 * t303 - 5.0 * t312 * t303;
        let tvrho0 = t250 * t149 + t252 * t315 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t317 = -t41 - t191;
        let t320 = piecewise3(t44, 0.0, 4.0 / 3.0 * t47 * t317);
        let t321 = -t317;
        let t324 = piecewise3(t51, 0.0, 4.0 / 3.0 * t52 * t321);
        let t326 = (t320 + t324) * t59;
        let t327 = t326 * t87;
        let t331 = t155 + t180 - t184 - t189 + t40 * t327 + t236 + 0.19751673498613801407e-1 * t326 * t85 - t243 - t248;
        let t332 = t7 * t331;
        let t336 = piecewise3(t44, 0.0, 2.0 / 3.0 * t256 * t317);
        let t339 = piecewise3(t51, 0.0, 2.0 / 3.0 * t260 * t321);
        let t341 = t336 / 2.0 + t339 / 2.0;
        let t342 = t56 * t341;
        let t343 = t342 * t107;
        let t346 = t279 * t331;
        let t347 = t110 * t346;
        let t350 = -0.69506584583333333332e-3 * t343 * t113 + t276 + 0.69506584583333333332e-3 * t108 * t347;
        let t352 = t254 * t350;
        let t361 = -t350;
        let t362 = t133 * t361;
        let t364 = t361 * t134;
        let t373 = -t362 * t134 + t255 * t350 + 2.0 * t286 * t352 + 3.0 * t290 * t352 + 4.0 * t293 * t352 + 5.0 * t296 * t352 - 2.0 * t302 * t364 - 3.0 * t306 * t364 - 4.0 * t309 * t364 - 5.0 * t312 * t364;
        let tvrho1 = t332 * t149 + t252 * t373 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t375 = t255 * t102;
        let t376 = t106 * t19;
        let t377 = t109 * t5;
        let t378 = t377 * t111;
        let t379 = t376 * t378;
        let t380 = t375 * t379;
        let t382 = t254 * t56;
        let t383 = t382 * t101;
        let t384 = t286 * t383;
        let t385 = t384 * t379;
        let t387 = t290 * t383;
        let t388 = t387 * t379;
        let t390 = t293 * t383;
        let t391 = t390 * t379;
        let t393 = t296 * t383;
        let t394 = t393 * t379;
        let t396 = t133 * t56;
        let t397 = t101 * t106;
        let t400 = t110 * t112 * t134;
        let t401 = t396 * t397 * t400;
        let t403 = t102 * t106;
        let t404 = t302 * t403;
        let t405 = t404 * t400;
        let t407 = t306 * t403;
        let t408 = t407 * t400;
        let t410 = t309 * t403;
        let t411 = t410 * t400;
        let t413 = t312 * t403;
        let t414 = t413 * t400;
        let t416 = -0.69506584583333333332e-3 * t380 - 0.13901316916666666666e-2 * t385 - 0.20851975375e-2 * t388 - 0.27802633833333333333e-2 * t391 - 0.34753292291666666666e-2 * t394 - 0.69506584583333333332e-3 * t401 - 0.13901316916666666666e-2 * t405 - 0.20851975375e-2 * t408 - 0.27802633833333333333e-2 * t411 - 0.34753292291666666666e-2 * t414;
        let tvsigma0 = t252 * t416;
        vsigma[ip * 3] += tvsigma0;
        let t427 = -0.13901316916666666666e-2 * t380 - 0.27802633833333333332e-2 * t385 - 0.41703950749999999998e-2 * t388 - 0.55605267666666666664e-2 * t391 - 0.6950658458333333333e-2 * t394 - 0.13901316916666666666e-2 * t401 - 0.27802633833333333332e-2 * t405 - 0.41703950749999999998e-2 * t408 - 0.55605267666666666664e-2 * t411 - 0.6950658458333333333e-2 * t414;
        let tvsigma1 = t252 * t427;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
