//! MGGA_X_R4SCAN vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 116 shared lines across all orders.
//! Delta: 163 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_r4scan_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_da4: f64,
    param_dp2: f64,
    param_dp4: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (116 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t4 / t5 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t22 = 20.0 / 27.0 + 5.0 / 3.0 * param_eta;
        let t23 = M_CBRT6;
        let t24 = t23 * t23;
        let t25 = M_PI * M_PI;
        let t26 = pow_1_3(t25);
        let t27 = t26 * t25;
        let t28 = 1.0 / t27;
        let t29 = t24 * t28;
        let t30 = sigma[ip] * sigma[ip];
        let t31 = t29 * t30;
        let t32 = M_CBRT2;
        let t33 = rho[ip] * rho[ip];
        let t34 = t33 * t33;
        let t35 = t34 * rho[ip];
        let t37 = 1.0 / t20 / t35;
        let t38 = t32 * t37;
        let t39 = param_dp2 * param_dp2;
        let t40 = t39 * t39;
        let t41 = 1.0 / t40;
        let t45 = f64::exp(-t31 * t38 * t41 / 288.0);
        let t49 = (-0.162742215233874e0 * t22 * t45 + 10.0 / 81.0) * t23;
        let t50 = t26 * t26;
        let t51 = 1.0 / t50;
        let t52 = t49 * t51;
        let t53 = t32 * t32;
        let t54 = sigma[ip] * t53;
        let t55 = t20 * t20;
        let t57 = 1.0 / t55 / t33;
        let t58 = t54 * t57;
        let t61 = param_k1 + t52 * t58 / 24.0;
        let t65 = param_k1 * (1.0 - param_k1 / t61);
        let t66 = tau[ip] * t53;
        let t67 = t55 * rho[ip];
        let t68 = 1.0 / t67;
        let t71 = t66 * t68 - t58 / 8.0;
        let t74 = param_eta * sigma[ip];
        let t75 = t53 * t57;
        let t78 = 3.0 / 10.0 * t24 * t50 + t74 * t75 / 8.0;
        let t79 = 1.0 / t78;
        let t80 = t71 * t79;
        let t81 = t80 <= 0.0;
        let t82 = 0.0 < t80;
        let t83 = piecewise3(t82, 0.0, t80);
        let t84 = param_c1 * t83;
        let t85 = 1.0 - t83;
        let t86 = 1.0 / t85;
        let t88 = f64::exp(-t84 * t86);
        let t89 = t80 <= 0.25e1;
        let t90 = 0.25e1 < t80;
        let t91 = piecewise3(t90, 0.25e1, t80);
        let t93 = t91 * t91;
        let t95 = t93 * t91;
        let t97 = t93 * t93;
        let t99 = t97 * t91;
        let t101 = t97 * t93;
        let t106 = piecewise3(t90, t80, 0.25e1);
        let t107 = 1.0 - t106;
        let t110 = f64::exp(param_c2 / t107);
        let t112 = piecewise5(t81, t88, t89, 1.0 - 0.667e0 * t91 - 0.4445555e0 * t93 - 0.663086601049e0 * t95 + 0.145129704449e1 * t97 - 0.887998041597e0 * t99 + 0.234528941479e0 * t101 - 0.23185843322e-1 * t97 * t95, -param_d * t110);
        let t113 = 0.174e0 - t65;
        let t116 = t22 * t23;
        let t117 = t116 * t51;
        let t120 = 1.0 - t80;
        let t121 = t120 * t120;
        let t125 = (0.40570770199022687793e-1 - 0.30235468026081006357e0 * param_eta) * t23;
        let t126 = t125 * t51;
        let t133 = pow_2(3.0 / 4.0 * param_eta + 2.0 / 3.0);
        let t138 = pow_2(0.290700106132790123e-2 - 0.27123702538979e0 * param_eta);
        let t142 = (146.0 / 2025.0 * t133 - 73.0 / 540.0 * param_eta - 146.0 / 1215.0 + t138 / param_k1) * t24;
        let t143 = t142 * t28;
        let t144 = t30 * t32;
        let t145 = t144 * t37;
        let t148 = -0.162742215233874e0 + 0.162742215233874e0 * t80 + 0.678092563474475e-2 * t117 * t58 - 0.59353125082804e-1 * t121 + t126 * t54 * t57 * t120 / 24.0 + t143 * t145 / 288.0;
        let t149 = t71 * t71;
        let t150 = t148 * t149;
        let t151 = t78 * t78;
        let t152 = 1.0 / t151;
        let t153 = t149 * t149;
        let t154 = t151 * t151;
        let t155 = 1.0 / t154;
        let t157 = t153 * t155 + 1.0;
        let t158 = 1.0 / t157;
        let t159 = t152 * t158;
        let t160 = param_da4 * param_da4;
        let t161 = 1.0 / t160;
        let t163 = param_dp4 * param_dp4;
        let t164 = t163 * t163;
        let t165 = 1.0 / t164;
        let t166 = t38 * t165;
        let t170 = f64::exp(-t121 * t161 - t31 * t166 / 288.0);
        let t171 = t159 * t170;
        let t174 = t112 * t113 + 2.0 * t150 * t171 + t65 + 1.0;
        let t176 = f64::sqrt(3.0);
        let t177 = 1.0 / t26;
        let t178 = t24 * t177;
        let t179 = f64::sqrt(sigma[ip]);
        let t180 = t179 * t32;
        let t182 = 1.0 / t20 / rho[ip];
        let t184 = t178 * t180 * t182;
        let t185 = f64::sqrt(t184);
        let t189 = f64::exp(-0.98958e1 * t176 / t185);
        let t190 = 1.0 - t189;
        let t194 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t174 * t190);
        let tzk0 = 2.0 * t194;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (163 lines) ---
        let t195 = 1.0 / t55;
        let t200 = param_k1 * param_k1;
        let t201 = t61 * t61;
        let t202 = 1.0 / t201;
        let t203 = t200 * t202;
        let t204 = t30 * sigma[ip];
        let t205 = t22 * t204;
        let t206 = t34 * t34;
        let t207 = t206 * rho[ip];
        let t208 = 1.0 / t207;
        let t210 = t208 * t41 * t45;
        let t213 = t33 * rho[ip];
        let t215 = 1.0 / t55 / t213;
        let t216 = t54 * t215;
        let t219 = -0.15469524941471936742e-4 * t205 * t210 - t52 * t216 / 9.0;
        let t224 = -5.0 / 3.0 * t66 * t57 + t216 / 3.0;
        let t225 = t224 * t79;
        let t226 = t71 * t152;
        let t227 = t226 * param_eta;
        let t228 = t227 * t216;
        let t230 = t225 + t228 / 3.0;
        let t231 = piecewise3(t82, 0.0, t230);
        let t234 = t85 * t85;
        let t235 = 1.0 / t234;
        let t236 = t235 * t231;
        let t238 = -param_c1 * t231 * t86 - t84 * t236;
        let t239 = t238 * t88;
        let t240 = piecewise3(t90, 0.0, t230);
        let t242 = t91 * t240;
        let t244 = t93 * t240;
        let t246 = t95 * t240;
        let t248 = t97 * t240;
        let t250 = t99 * t240;
        let t255 = param_d * param_c2;
        let t256 = t107 * t107;
        let t257 = 1.0 / t256;
        let t258 = piecewise3(t90, t230, 0.0);
        let t262 = piecewise5(t81, t239, t89, -0.667e0 * t240 - 0.889111e0 * t242 - 0.1989259803147e1 * t244 + 0.580518817796e1 * t246 - 0.4439990207985e1 * t248 + 0.1407173648874e1 * t250 - 0.162300903254e0 * t101 * t240, -t255 * t257 * t258 * t110);
        let t264 = t112 * t200;
        let t265 = t202 * t219;
        let t271 = -t230;
        let t282 = t34 * t33;
        let t284 = 1.0 / t20 / t282;
        let t288 = 0.162742215233874e0 * t225 + 0.54247405077958e-1 * t228 - 0.18082468359319333333e-1 * t117 * t216 - 0.118706250165608e0 * t120 * t271 - t126 * t54 * t215 * t120 / 9.0 + t126 * t54 * t57 * t271 / 24.0 - t143 * t144 * t284 / 54.0;
        let t289 = t288 * t149;
        let t292 = t148 * t71;
        let t293 = t292 * t152;
        let t294 = t158 * t170;
        let t295 = t294 * t224;
        let t298 = t151 * t78;
        let t299 = 1.0 / t298;
        let t300 = t299 * t158;
        let t301 = t150 * t300;
        let t302 = t170 * param_eta;
        let t303 = t302 * t216;
        let t306 = t150 * t152;
        let t307 = t157 * t157;
        let t308 = 1.0 / t307;
        let t309 = t308 * t170;
        let t310 = t149 * t71;
        let t311 = t310 * t155;
        let t315 = 1.0 / t154 / t78;
        let t316 = t153 * t315;
        let t317 = t316 * param_eta;
        let t320 = 4.0 * t311 * t224 + 4.0 / 3.0 * t317 * t216;
        let t321 = t309 * t320;
        let t324 = t120 * t161;
        let t327 = t32 * t284;
        let t328 = t327 * t165;
        let t331 = -2.0 * t324 * t271 + t31 * t328 / 54.0;
        let t332 = t158 * t331;
        let t333 = t332 * t170;
        let t336 = t203 * t219 + t262 * t113 - t264 * t265 + 2.0 * t289 * t171 + 4.0 * t293 * t295 + 4.0 / 3.0 * t301 * t303 - 2.0 * t306 * t321 + 2.0 * t306 * t333;
        let t341 = f64::powf(3.0, 1.0 / 6.0);
        let t342 = t341 * t341;
        let t343 = t342 * t342;
        let t345 = t343 * t341 * t18;
        let t346 = 1.0 / t33;
        let t347 = t346 * t174;
        let t349 = 1.0 / t185 / t184;
        let t351 = t345 * t347 * t349;
        let t353 = t178 * t180 * t189;
        let t357 = piecewise3(t3, 0.0, -t19 * t195 * t174 * t190 / 8.0 - 3.0 / 8.0 * t19 * t20 * t336 * t190 - 0.16891736332904387511e1 * t351 * t353);
        let tvrho0 = 2.0 * rho[ip] * t357 + 2.0 * t194;
        vrho[ip] += tvrho0;
        let t360 = t22 * t30;
        let t361 = 1.0 / t206;
        let t363 = t361 * t41 * t45;
        let t366 = t51 * t53;
        let t367 = t366 * t57;
        let t370 = 0.58010718530519762783e-5 * t360 * t363 + t49 * t367 / 24.0;
        let t372 = t75 * t79;
        let t373 = param_eta * t53;
        let t374 = t373 * t57;
        let t375 = t226 * t374;
        let t377 = -t372 / 8.0 - t375 / 8.0;
        let t378 = piecewise3(t82, 0.0, t377);
        let t379 = param_c1 * t378;
        let t381 = t235 * t378;
        let t383 = -t379 * t86 - t84 * t381;
        let t384 = t383 * t88;
        let t385 = piecewise3(t90, 0.0, t377);
        let t387 = t91 * t385;
        let t389 = t93 * t385;
        let t391 = t95 * t385;
        let t393 = t97 * t385;
        let t395 = t99 * t385;
        let t400 = piecewise3(t90, t377, 0.0);
        let t404 = piecewise5(t81, t384, t89, -0.667e0 * t385 - 0.889111e0 * t387 - 0.1989259803147e1 * t389 + 0.580518817796e1 * t391 - 0.4439990207985e1 * t393 + 0.1407173648874e1 * t395 - 0.162300903254e0 * t101 * t385, -t255 * t257 * t400 * t110);
        let t406 = t202 * t370;
        let t412 = -t377;
        let t422 = sigma[ip] * t32;
        let t426 = -0.2034277690423425e-1 * t372 - 0.2034277690423425e-1 * t375 + 0.678092563474475e-2 * t116 * t367 - 0.118706250165608e0 * t120 * t412 + t126 * t75 * t120 / 24.0 + t126 * t54 * t57 * t412 / 24.0 + t143 * t422 * t37 / 144.0;
        let t427 = t426 * t149;
        let t430 = t294 * t75;
        let t431 = t293 * t430;
        let t433 = t302 * t75;
        let t439 = -t311 * t75 / 2.0 - t316 * t374 / 2.0;
        let t440 = t309 * t439;
        let t445 = t29 * sigma[ip];
        let t448 = -2.0 * t324 * t412 - t445 * t166 / 144.0;
        let t449 = t158 * t448;
        let t450 = t449 * t170;
        let t453 = t203 * t370 + t404 * t113 - t264 * t406 + 2.0 * t427 * t171 - t431 / 2.0 - t301 * t433 / 2.0 - 2.0 * t306 * t440 + 2.0 * t306 * t450;
        let t458 = 1.0 / rho[ip];
        let t459 = t458 * t174;
        let t461 = t345 * t459 * t349;
        let t462 = 1.0 / t179;
        let t465 = t178 * t462 * t32 * t189;
        let t469 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t453 * t190 + 0.63344011248391453166e0 * t461 * t465);
        let tvsigma0 = 2.0 * rho[ip] * t469;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t471 = t53 * t68;
        let t472 = t471 * t79;
        let t473 = piecewise3(t82, 0.0, t472);
        let t474 = param_c1 * t473;
        let t476 = t235 * t473;
        let t478 = -t474 * t86 - t84 * t476;
        let t479 = t478 * t88;
        let t480 = piecewise3(t90, 0.0, t472);
        let t482 = t91 * t480;
        let t484 = t93 * t480;
        let t486 = t95 * t480;
        let t488 = t97 * t480;
        let t490 = t99 * t480;
        let t495 = piecewise3(t90, t472, 0.0);
        let t499 = piecewise5(t81, t479, t89, -0.667e0 * t480 - 0.889111e0 * t482 - 0.1989259803147e1 * t484 + 0.580518817796e1 * t486 - 0.4439990207985e1 * t488 + 0.1407173648874e1 * t490 - 0.162300903254e0 * t101 * t480, -t255 * t257 * t495 * t110);
        let t502 = t120 * t53;
        let t503 = t68 * t79;
        let t507 = 1.0 / t20 / t34;
        let t512 = 0.162742215233874e0 * t472 + 0.118706250165608e0 * t502 * t503 - t126 * t422 * t507 * t79 / 12.0;
        let t513 = t512 * t149;
        let t516 = t294 * t471;
        let t519 = t153 * t71;
        let t520 = t148 * t519;
        let t521 = t154 * t151;
        let t522 = 1.0 / t521;
        let t523 = t520 * t522;
        let t524 = t309 * t471;
        let t527 = t471 * t170;
        let t528 = t324 * t527;
        let t531 = t499 * t113 + 2.0 * t513 * t171 + 4.0 * t293 * t516 + 4.0 * t301 * t528 - 8.0 * t523 * t524;
        let t536 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t531 * t190);
        let tvtau0 = 2.0 * rho[ip] * t536;
        vtau[ip] += tvtau0;
    }
}
