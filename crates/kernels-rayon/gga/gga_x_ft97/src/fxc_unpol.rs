//! GGA_X_FT97 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ft97.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ft97_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_beta1: f64,
    param_beta2: f64,
    param_beta0: f64,
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
        let t20 = param_beta1 * sigma[ip];
        let t21 = t18 * t18;
        let t22 = 1.0 / t21;
        let t23 = t20 * t22;
        let t24 = t11 * t11;
        let t25 = t11 * rho[ip];
        let t26 = pow_1_3(t25);
        let t27 = t26 * t26;
        let t28 = t24 * t27;
        let t29 = sigma[ip] * t22;
        let t32 = param_beta2 + t29 * t28 / 4.0;
        let t33 = 1.0 / t32;
        let t34 = t28 * t33;
        let t37 = param_beta0 + t23 * t34 / 4.0;
        let t38 = t37 * sigma[ip];
        let t39 = M_CBRT2;
        let t40 = t39 * t39;
        let t41 = rho[ip] * rho[ip];
        let t43 = 1.0 / t21 / t41;
        let t44 = t40 * t43;
        let t45 = t38 * t44;
        let t46 = t3 * t3;
        let t48 = pow_1_3(1.0 / M_PI);
        let t49 = 1.0 / t48;
        let t50 = t46 * t49;
        let t51 = M_CBRT4;
        let t52 = sigma[ip] * t40;
        let t53 = t37 * t37;
        let t55 = t52 * t43;
        let t56 = f64::ln(t55 + f64::sqrt(t55 * t55 + 1.0));
        let t57 = t56 * t56;
        let t61 = 9.0 * t52 * t43 * t53 * t57 + 1.0;
        let t62 = f64::sqrt(t61);
        let t65 = t50 * t51 / t62;
        let t68 = 1.0 + 2.0 / 9.0 * t45 * t65;
        let t72 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t68);
        let tzk0 = 2.0 * t72;
        zk[ip] += tzk0;
        let t73 = t17 * t22;
        let t78 = 1.0 / t21 / rho[ip];
        let t79 = t20 * t78;
        let t82 = t24 * t11;
        let t83 = 1.0 / t26;
        let t84 = t82 * t83;
        let t85 = t84 * t33;
        let t88 = t32 * t32;
        let t89 = 1.0 / t88;
        let t90 = sigma[ip] * t78;
        let t94 = -t90 * t28 / 6.0 + t29 * t84 / 6.0;
        let t95 = t89 * t94;
        let t96 = t28 * t95;
        let t99 = -t79 * t34 / 6.0 + t23 * t85 / 6.0 - t23 * t96 / 4.0;
        let t100 = t99 * sigma[ip];
        let t101 = t100 * t44;
        let t104 = t41 * rho[ip];
        let t106 = 1.0 / t21 / t104;
        let t107 = t40 * t106;
        let t108 = t38 * t107;
        let t112 = 1.0 / t62 / t61;
        let t113 = t51 * t112;
        let t118 = t37 * t57;
        let t119 = t118 * t99;
        let t122 = sigma[ip] * sigma[ip];
        let t123 = t122 * t39;
        let t124 = t41 * t41;
        let t125 = t124 * t41;
        let t127 = 1.0 / t18 / t125;
        let t128 = t123 * t127;
        let t129 = t53 * t56;
        let t130 = t124 * rho[ip];
        let t132 = 1.0 / t18 / t130;
        let t135 = 2.0 * t123 * t132 + 1.0;
        let t136 = f64::sqrt(t135);
        let t137 = 1.0 / t136;
        let t138 = t129 * t137;
        let t141 = -24.0 * t52 * t106 * t53 * t57 + 18.0 * t55 * t119 - 96.0 * t128 * t138;
        let t143 = t50 * t113 * t141;
        let t146 = 2.0 / 9.0 * t101 * t65 - 16.0 / 27.0 * t108 * t65 - t45 * t143 / 9.0;
        let t151 = piecewise3(t2, 0.0, -t6 * t73 * t68 / 8.0 - 3.0 / 8.0 * t6 * t19 * t146);
        let tvrho0 = 2.0 * rho[ip] * t151 + 2.0 * t72;
        vrho[ip] += tvrho0;
        let t154 = param_beta1 * t22;
        let t158 = 1.0 / t18 / rho[ip];
        let t159 = t20 * t158;
        let t160 = t24 * t24;
        let t161 = t26 * t25;
        let t162 = t160 * t161;
        let t163 = t162 * t89;
        let t166 = t154 * t34 / 4.0 - t159 * t163 / 16.0;
        let t167 = t166 * sigma[ip];
        let t168 = t167 * t44;
        let t171 = t37 * t40;
        let t175 = t53 * t57;
        let t178 = t118 * t166;
        let t181 = sigma[ip] * t39;
        let t182 = t181 * t132;
        let t185 = 36.0 * t182 * t138 + 9.0 * t44 * t175 + 18.0 * t55 * t178;
        let t187 = t50 * t113 * t185;
        let t190 = 2.0 / 9.0 * t168 * t65 + 2.0 / 9.0 * t171 * t43 * t65 - t45 * t187 / 9.0;
        let t194 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t190);
        let tvsigma0 = 2.0 * rho[ip] * t194;
        vsigma[ip] += tvsigma0;
        let t197 = t17 * t78;
        let t204 = t20 * t43;
        let t211 = 1.0 / t161;
        let t212 = t160 * t211;
        let t213 = t212 * t33;
        let t216 = t84 * t95;
        let t220 = 1.0 / t88 / t32;
        let t221 = t94 * t94;
        let t222 = t220 * t221;
        let t223 = t28 * t222;
        let t226 = sigma[ip] * t43;
        let t233 = 5.0 / 18.0 * t226 * t28 - 2.0 / 9.0 * t90 * t84 - t29 * t212 / 18.0;
        let t234 = t89 * t233;
        let t235 = t28 * t234;
        let t238 = 5.0 / 18.0 * t204 * t34 - 2.0 / 9.0 * t79 * t85 + t79 * t96 / 3.0 - t23 * t213 / 18.0 - t23 * t216 / 3.0 + t23 * t223 / 2.0 - t23 * t235 / 4.0;
        let t239 = t238 * sigma[ip];
        let t240 = t239 * t44;
        let t243 = t100 * t107;
        let t249 = 1.0 / t21 / t124;
        let t250 = t40 * t249;
        let t251 = t38 * t250;
        let t256 = t61 * t61;
        let t258 = 1.0 / t62 / t256;
        let t259 = t51 * t258;
        let t260 = t141 * t141;
        let t262 = t50 * t259 * t260;
        let t269 = t52 * t106;
        let t272 = t124 * t104;
        let t274 = 1.0 / t18 / t272;
        let t275 = t123 * t274;
        let t278 = t99 * t99;
        let t283 = t37 * t56;
        let t285 = t283 * t99 * t137;
        let t288 = t118 * t238;
        let t291 = t122 * sigma[ip];
        let t292 = t124 * t124;
        let t293 = t292 * t41;
        let t294 = 1.0 / t293;
        let t295 = t291 * t294;
        let t296 = 1.0 / t135;
        let t297 = t53 * t296;
        let t300 = t122 * t122;
        let t301 = t300 * t40;
        let t302 = t292 * t124;
        let t304 = 1.0 / t21 / t302;
        let t305 = t301 * t304;
        let t307 = 1.0 / t136 / t135;
        let t308 = t129 * t307;
        let t311 = 88.0 * t52 * t249 * t53 * t57 + 18.0 * t52 * t43 * t278 * t57 - 96.0 * t269 * t119 - 384.0 * t128 * t285 + 864.0 * t275 * t138 + 18.0 * t55 * t288 + 512.0 * t295 * t297 - 512.0 * t305 * t308;
        let t313 = t50 * t113 * t311;
        let t316 = 2.0 / 9.0 * t240 * t65 - 32.0 / 27.0 * t243 * t65 - 2.0 / 9.0 * t101 * t143 + 176.0 / 81.0 * t251 * t65 + 16.0 / 27.0 * t108 * t143 + t45 * t262 / 6.0 - t45 * t313 / 9.0;
        let t321 = piecewise3(t2, 0.0, t6 * t197 * t68 / 12.0 - t6 * t73 * t146 / 4.0 - 3.0 / 8.0 * t6 * t19 * t316);
        let tv2rho20 = 2.0 * rho[ip] * t321 + 4.0 * t151;
        v2rho2[ip] += tv2rho20;
        let t327 = param_beta1 * t78;
        let t332 = t154 * t24;
        let t333 = t27 * t89;
        let t334 = t333 * t94;
        let t338 = 1.0 / t18 / t41;
        let t339 = t20 * t338;
        let t342 = t160 * t11;
        let t343 = t342 * t26;
        let t344 = t343 * t89;
        let t347 = t220 * t94;
        let t348 = t162 * t347;
        let t351 = -t327 * t34 / 6.0 + t154 * t85 / 6.0 - t332 * t334 / 4.0 + t339 * t163 / 12.0 - t159 * t344 / 12.0 + t159 * t348 / 8.0;
        let t352 = t351 * sigma[ip];
        let t353 = t352 * t44;
        let t356 = t167 * t107;
        let t361 = t99 * t40;
        let t368 = t43 * t46;
        let t369 = t171 * t368;
        let t370 = t49 * t51;
        let t372 = t370 * t112 * t141;
        let t379 = t44 * t46;
        let t380 = t38 * t379;
        let t381 = t258 * t185;
        let t383 = t370 * t381 * t141;
        let t390 = t39 * t127;
        let t393 = t56 * sigma[ip] * t137;
        let t398 = t99 * t57;
        let t399 = t398 * t166;
        let t402 = t166 * t137;
        let t403 = t283 * t402;
        let t406 = t118 * t351;
        let t411 = t292 * rho[ip];
        let t412 = 1.0 / t411;
        let t413 = t122 * t412;
        let t416 = t291 * t40;
        let t417 = t292 * t104;
        let t419 = 1.0 / t21 / t417;
        let t420 = t416 * t419;
        let t423 = -288.0 * t390 * t53 * t393 - 24.0 * t107 * t175 + 18.0 * t44 * t119 - 192.0 * t128 * t403 - 48.0 * t269 * t178 + 72.0 * t182 * t285 - 192.0 * t413 * t297 + 192.0 * t420 * t308 + 18.0 * t55 * t399 + 18.0 * t55 * t406;
        let t425 = t50 * t113 * t423;
        let t428 = 2.0 / 9.0 * t353 * t65 - 16.0 / 27.0 * t356 * t65 - t168 * t143 / 9.0 + 2.0 / 9.0 * t361 * t43 * t65 - 16.0 / 27.0 * t171 * t106 * t65 - t369 * t372 / 9.0 - t101 * t187 / 9.0 + 8.0 / 27.0 * t108 * t187 + t380 * t383 / 6.0 - t45 * t425 / 9.0;
        let t433 = piecewise3(t2, 0.0, -t6 * t73 * t190 / 8.0 - 3.0 / 8.0 * t6 * t19 * t428);
        let tv2rhosigma0 = 2.0 * rho[ip] * t433 + 2.0 * t194;
        v2rhosigma[ip] += tv2rhosigma0;
        let t436 = param_beta1 * t158;
        let t439 = t160 * t160;
        let t443 = -t436 * t163 / 8.0 + t20 * t439 * t220 / 32.0;
        let t444 = t443 * sigma[ip];
        let t445 = t444 * t44;
        let t448 = t166 * t40;
        let t455 = t370 * t112 * t185;
        let t458 = t185 * t185;
        let t460 = t50 * t259 * t458;
        let t465 = t39 * t132;
        let t468 = t166 * t166;
        let t475 = t118 * t443;
        let t478 = 1.0 / t292;
        let t479 = sigma[ip] * t478;
        let t484 = 1.0 / t21 / t293;
        let t485 = t122 * t40 * t484;
        let t488 = 18.0 * t52 * t43 * t468 * t57 + 72.0 * t465 * t138 + 36.0 * t44 * t178 + 144.0 * t182 * t403 + 72.0 * t479 * t297 - 72.0 * t485 * t308 + 18.0 * t55 * t475;
        let t490 = t50 * t113 * t488;
        let t493 = 2.0 / 9.0 * t445 * t65 + 4.0 / 9.0 * t448 * t43 * t65 - 2.0 / 9.0 * t168 * t187 - 2.0 / 9.0 * t369 * t455 + t45 * t460 / 6.0 - t45 * t490 / 9.0;
        let t497 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t493);
        let tv2sigma20 = 2.0 * rho[ip] * t497;
        v2sigma2[ip] += tv2sigma20;
    }
}
