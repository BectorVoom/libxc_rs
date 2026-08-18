//! MGGA_C_M05 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_m05.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_m05_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_css_1: f64,
    param_gamma_ss: f64,
    param_css_2: f64,
    param_css_3: f64,
    param_css_4: f64,
    param_css_0: f64,
    param_Fermi_D_cnst: f64,
    param_cab_1: f64,
    param_gamma_ab: f64,
    param_cab_2: f64,
    param_cab_3: f64,
    param_cab_4: f64,
    param_cab_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t4 = 1.0 <= zeta_threshold;
        let t5 = rho[ip] / 2.0 <= dens_threshold || t4;
        let t6 = piecewise3(t4, zeta_threshold, 1.0);
        let t7 = M_CBRT3;
        let t8 = 1.0 / M_PI;
        let t9 = pow_1_3(t8);
        let t10 = t7 * t9;
        let t11 = M_CBRT4;
        let t12 = t11 * t11;
        let t13 = t10 * t12;
        let t14 = pow_1_3(rho[ip]);
        let t15 = 1.0 / t14;
        let t16 = M_CBRT2;
        let t18 = pow_1_3(zeta_threshold);
        let t20 = piecewise3(t4, 1.0 / t18, 1.0);
        let t22 = t13 * t15 * t16 * t20;
        let t24 = 1.0 + 0.053425 * t22;
        let t25 = f64::sqrt(t22);
        let t28 = pow_3_2(t22);
        let t30 = t7 * t7;
        let t31 = t9 * t9;
        let t32 = t30 * t31;
        let t33 = t32 * t11;
        let t34 = t14 * t14;
        let t35 = 1.0 / t34;
        let t36 = t16 * t16;
        let t38 = t20 * t20;
        let t40 = t33 * t35 * t36 * t38;
        let t42 = 3.79785 * t25 + 0.8969 * t22 + 0.204775 * t28 + 0.123235 * t40;
        let t45 = 1.0 + 16.081979498692537 / t42;
        let t46 = f64::ln(t45);
        let t48 = 0.0621814 * t24 * t46;
        let t50 = t18 * zeta_threshold;
        let t52 = piecewise3(2.0 <= zeta_threshold, t50, 2.0 * t16);
        let t54 = piecewise3(0.0 <= zeta_threshold, t50, 0.0);
        let t58 = 1.0 / (2.0 * t16 - 2.0);
        let t59 = (t52 + t54 - 2.0) * t58;
        let t61 = 1.0 + 0.05137 * t22;
        let t66 = 7.05945 * t25 + 1.549425 * t22 + 0.420775 * t28 + 0.1562925 * t40;
        let t69 = 1.0 + 32.16395899738507 / t66;
        let t70 = f64::ln(t69);
        let t74 = 1.0 + 0.0278125 * t22;
        let t79 = 5.1785 * t25 + 0.905775 * t22 + 0.1100325 * t28 + 0.1241775 * t40;
        let t82 = 1.0 + 29.608749977793437 / t79;
        let t83 = f64::ln(t82);
        let t84 = t74 * t83;
        let t93 = piecewise3(t5, 0.0, t6 * (-t48 + t59 * (-0.0310907 * t61 * t70 + t48 - 0.0197516734986138 * t84) + 0.0197516734986138 * t59 * t84) / 2.0);
        let t95 = param_css_1;
        let t96 = t95 * param_gamma_ss;
        let t97 = t96 * sigma[ip];
        let t98 = rho[ip] * rho[ip];
        let t100 = 1.0 / t34 / t98;
        let t101 = t36 * t100;
        let t104 = param_gamma_ss * sigma[ip] * t101 + 1.0;
        let t105 = 1.0 / t104;
        let t106 = t101 * t105;
        let t108 = param_css_2;
        let t109 = param_gamma_ss * param_gamma_ss;
        let t110 = t108 * t109;
        let t111 = sigma[ip] * sigma[ip];
        let t112 = t110 * t111;
        let t113 = t98 * t98;
        let t114 = t113 * rho[ip];
        let t116 = 1.0 / t14 / t114;
        let t117 = t16 * t116;
        let t118 = t104 * t104;
        let t119 = 1.0 / t118;
        let t120 = t117 * t119;
        let t123 = param_css_3;
        let t124 = t109 * param_gamma_ss;
        let t125 = t123 * t124;
        let t126 = t111 * sigma[ip];
        let t127 = t113 * t113;
        let t128 = 1.0 / t127;
        let t129 = t126 * t128;
        let t130 = t118 * t104;
        let t131 = 1.0 / t130;
        let t135 = param_css_4;
        let t136 = t109 * t109;
        let t137 = t135 * t136;
        let t138 = t111 * t111;
        let t139 = t137 * t138;
        let t140 = t127 * t98;
        let t142 = 1.0 / t34 / t140;
        let t143 = t36 * t142;
        let t144 = t118 * t118;
        let t145 = 1.0 / t144;
        let t146 = t143 * t145;
        let t149 = 4.0 * t125 * t129 * t131 + t97 * t106 + 2.0 * t112 * t120 + 4.0 * t139 * t146 + param_css_0;
        let t150 = t93 * t149;
        let t151 = 1.0 / rho[ip];
        let t153 = 1.0 / tau[ip];
        let t156 = 1.0 - sigma[ip] * t151 * t153 / 8.0;
        let t157 = tau[ip] * tau[ip];
        let t159 = t98 * rho[ip];
        let t161 = 1.0 / t14 / t159;
        let t162 = param_Fermi_D_cnst * param_Fermi_D_cnst;
        let t163 = 1.0 / t162;
        let t167 = f64::exp(-8.0 * t157 * t16 * t161 * t163);
        let t168 = 1.0 - t167;
        let t169 = t156 * t168;
        let t171 = 2.0 * t150 * t169;
        let t173 = t10 * t12 * t15;
        let t175 = 1.0 + 0.053425 * t173;
        let t176 = f64::sqrt(t173);
        let t179 = pow_3_2(t173);
        let t182 = t32 * t11 * t35;
        let t184 = 3.79785 * t176 + 0.8969 * t173 + 0.204775 * t179 + 0.123235 * t182;
        let t187 = 1.0 + 16.081979498692537 / t184;
        let t188 = f64::ln(t187);
        let t191 = piecewise3(t4, t50, 1.0);
        let t194 = (2.0 * t191 - 2.0) * t58;
        let t196 = 1.0 + 0.0278125 * t173;
        let t201 = 5.1785 * t176 + 0.905775 * t173 + 0.1100325 * t179 + 0.1241775 * t182;
        let t204 = 1.0 + 29.608749977793437 / t201;
        let t205 = f64::ln(t204);
        let t210 = -0.0621814 * t175 * t188 + 0.0197516734986138 * t194 * t196 * t205 - 2.0 * t93;
        let t212 = param_cab_1;
        let t213 = t212 * param_gamma_ab;
        let t214 = t213 * sigma[ip];
        let t218 = 2.0 * param_gamma_ab * sigma[ip] * t101 + 1.0;
        let t219 = 1.0 / t218;
        let t220 = t101 * t219;
        let t223 = param_cab_2;
        let t224 = param_gamma_ab * param_gamma_ab;
        let t225 = t223 * t224;
        let t226 = t225 * t111;
        let t227 = t218 * t218;
        let t228 = 1.0 / t227;
        let t229 = t117 * t228;
        let t232 = param_cab_3;
        let t233 = t224 * param_gamma_ab;
        let t234 = t232 * t233;
        let t235 = t227 * t218;
        let t236 = 1.0 / t235;
        let t240 = param_cab_4;
        let t241 = t224 * t224;
        let t242 = t240 * t241;
        let t243 = t242 * t138;
        let t244 = t227 * t227;
        let t245 = 1.0 / t244;
        let t246 = t143 * t245;
        let t249 = 32.0 * t234 * t129 * t236 + 2.0 * t214 * t220 + 8.0 * t226 * t229 + 64.0 * t243 * t246 + param_cab_0;
        let t250 = t210 * t249;
        let tzk0 = t171 + t250;
        zk[ip] += tzk0;
        let t252 = 1.0 / t14 / rho[ip];
        let t253 = t252 * t16;
        let t254 = t20 * t46;
        let t257 = 0.0011073470983333333 * t13 * t253 * t254;
        let t258 = t42 * t42;
        let t259 = 1.0 / t258;
        let t260 = t24 * t259;
        let t263 = 1.0 / t25 * t7 * t9;
        let t264 = t12 * t252;
        let t265 = t16 * t20;
        let t266 = t264 * t265;
        let t267 = t263 * t266;
        let t269 = t253 * t20;
        let t270 = t13 * t269;
        let t272 = f64::sqrt(t22);
        let t274 = t272 * t7 * t9;
        let t275 = t274 * t266;
        let t278 = 1.0 / t34 / rho[ip];
        let t281 = t33 * t278 * t36 * t38;
        let t283 = -0.632975 * t267 - 0.29896666666666666 * t270 - 0.1023875 * t275 - 0.08215666666666667 * t281;
        let t284 = 1.0 / t45;
        let t285 = t283 * t284;
        let t287 = 1.0 * t260 * t285;
        let t288 = t20 * t70;
        let t292 = t66 * t66;
        let t293 = 1.0 / t292;
        let t294 = t61 * t293;
        let t299 = -1.176575 * t267 - 0.516475 * t270 - 0.2103875 * t275 - 0.104195 * t281;
        let t300 = 1.0 / t69;
        let t301 = t299 * t300;
        let t304 = t20 * t83;
        let t308 = t79 * t79;
        let t309 = 1.0 / t308;
        let t310 = t74 * t309;
        let t315 = -0.8630833333333333 * t267 - 0.301925 * t270 - 0.05501625 * t275 - 0.082785 * t281;
        let t316 = 1.0 / t82;
        let t317 = t315 * t316;
        let t322 = t59 * t10;
        let t323 = t265 * t83;
        let t327 = t59 * t74;
        let t329 = t309 * t315 * t316;
        let t335 = piecewise3(t5, 0.0, t6 * (t257 + t287 + t59 * (0.0005323764196666666 * t13 * t253 * t288 + 1.0 * t294 * t301 - t257 - t287 + 0.00018311447306006544 * t13 * t253 * t304 + 0.5848223622634646 * t310 * t317) - 0.00018311447306006544 * t322 * t264 * t323 - 0.5848223622634646 * t327 * t329) / 2.0);
        let t336 = t335 * t149;
        let t337 = t336 * t169;
        let t340 = 1.0 / t34 / t159;
        let t341 = t36 * t340;
        let t342 = t341 * t105;
        let t345 = t95 * t109;
        let t346 = t345 * t111;
        let t347 = t113 * t98;
        let t349 = 1.0 / t14 / t347;
        let t350 = t16 * t349;
        let t351 = t350 * t119;
        let t356 = t108 * t124;
        let t357 = t127 * rho[ip];
        let t358 = 1.0 / t357;
        let t359 = t126 * t358;
        let t360 = t359 * t131;
        let t365 = t123 * t136;
        let t366 = t365 * t138;
        let t367 = t127 * t159;
        let t369 = 1.0 / t34 / t367;
        let t371 = t369 * t145 * t36;
        let t376 = t136 * param_gamma_ss;
        let t377 = t135 * t376;
        let t378 = t138 * sigma[ip];
        let t379 = t377 * t378;
        let t380 = t127 * t347;
        let t382 = 1.0 / t14 / t380;
        let t383 = t16 * t382;
        let t385 = 1.0 / t144 / t104;
        let t386 = t383 * t385;
        let t389 = -8.0 / 3.0 * t97 * t342 + 16.0 / 3.0 * t346 * t351 - 32.0 / 3.0 * t112 * t351 + 64.0 / 3.0 * t356 * t360 - 32.0 * t125 * t360 + 32.0 * t366 * t371 - 128.0 / 3.0 * t139 * t371 + 256.0 / 3.0 * t379 * t386;
        let t390 = t93 * t389;
        let t391 = t390 * t169;
        let t393 = t150 * sigma[ip];
        let t394 = 1.0 / t98;
        let t396 = t394 * t153 * t168;
        let t397 = t393 * t396;
        let t399 = t156 * t157;
        let t400 = t150 * t399;
        let t402 = 1.0 / t14 / t113;
        let t403 = t16 * t402;
        let t404 = t163 * t167;
        let t405 = t403 * t404;
        let t406 = t400 * t405;
        let t411 = t184 * t184;
        let t412 = 1.0 / t411;
        let t413 = t175 * t412;
        let t415 = 1.0 / t176 * t7;
        let t416 = t9 * t12;
        let t417 = t416 * t252;
        let t418 = t415 * t417;
        let t420 = t10 * t264;
        let t422 = f64::sqrt(t173);
        let t423 = t422 * t7;
        let t424 = t423 * t417;
        let t427 = t32 * t11 * t278;
        let t429 = -0.632975 * t418 - 0.29896666666666666 * t420 - 0.1023875 * t424 - 0.08215666666666667 * t427;
        let t430 = 1.0 / t187;
        let t431 = t429 * t430;
        let t434 = t194 * t7;
        let t439 = t194 * t196;
        let t440 = t201 * t201;
        let t441 = 1.0 / t440;
        let t446 = -0.8630833333333333 * t418 - 0.301925 * t420 - 0.05501625 * t424 - 0.082785 * t427;
        let t448 = 1.0 / t204;
        let t449 = t441 * t446 * t448;
        let t453 = 0.0011073470983333333 * t10 * t264 * t188 + 1.0 * t413 * t431 - 0.00018311447306006544 * t434 * t416 * t252 * t205 - 0.5848223622634646 * t439 * t449 - 2.0 * t335;
        let t454 = t453 * t249;
        let t455 = t341 * t219;
        let t458 = t212 * t224;
        let t459 = t458 * t111;
        let t460 = t350 * t228;
        let t465 = t223 * t233;
        let t466 = t359 * t236;
        let t471 = t232 * t241;
        let t472 = t471 * t138;
        let t474 = t369 * t245 * t36;
        let t479 = t241 * param_gamma_ab;
        let t480 = t240 * t479;
        let t481 = t480 * t378;
        let t483 = 1.0 / t244 / t218;
        let t484 = t383 * t483;
        let t487 = -16.0 / 3.0 * t214 * t455 + 64.0 / 3.0 * t459 * t460 - 128.0 / 3.0 * t226 * t460 + 512.0 / 3.0 * t465 * t466 - 256.0 * t234 * t466 + 512.0 * t472 * t474 - 2048.0 / 3.0 * t243 * t474 + 8192.0 / 3.0 * t481 * t484;
        let t488 = t210 * t487;
        let tvrho0 = t171 + t250 + rho[ip] * (2.0 * t337 + 2.0 * t391 + t397 / 4.0 - 160.0 / 3.0 * t406 + t454 + t488);
        vrho[ip] += tvrho0;
        let t495 = t110 * sigma[ip];
        let t498 = t111 * t128;
        let t499 = t498 * t131;
        let t504 = t365 * t126;
        let t507 = t137 * t126;
        let t510 = t377 * t138;
        let t511 = t127 * t114;
        let t513 = 1.0 / t14 / t511;
        let t514 = t16 * t513;
        let t515 = t514 * t385;
        let t518 = -2.0 * t345 * sigma[ip] * t120 + t96 * t106 + 4.0 * t495 * t120 + 12.0 * t125 * t499 - 12.0 * t504 * t146 + 16.0 * t507 * t146 - 8.0 * t356 * t499 - 32.0 * t510 * t515;
        let t519 = t93 * t518;
        let t521 = 2.0 * t519 * t169;
        let t523 = t151 * t153 * t168;
        let t525 = t150 * t523 / 4.0;
        let t531 = t225 * sigma[ip];
        let t534 = t498 * t236;
        let t539 = t471 * t126;
        let t542 = t242 * t126;
        let t545 = t480 * t138;
        let t546 = t514 * t483;
        let t549 = -8.0 * t458 * sigma[ip] * t229 + 2.0 * t213 * t220 + 16.0 * t531 * t229 + 96.0 * t234 * t534 - 192.0 * t539 * t246 + 256.0 * t542 * t246 - 64.0 * t465 * t534 - 1024.0 * t545 * t546;
        let t550 = t210 * t549;
        let tvsigma0 = rho[ip] * (t521 - t525 + t550);
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t552 = 1.0 / t157;
        let t554 = t151 * t552 * t168;
        let t556 = t393 * t554 / 4.0;
        let t557 = t156 * tau[ip];
        let t558 = t150 * t557;
        let t559 = t16 * t161;
        let t560 = t559 * t404;
        let t562 = 32.0 * t558 * t560;
        let tvtau0 = rho[ip] * (t556 + t562);
        vtau[ip] += tvtau0;
    }
}
