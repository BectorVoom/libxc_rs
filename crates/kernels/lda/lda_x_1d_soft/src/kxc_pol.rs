//! LDA_X_1D_SOFT kxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_1d_soft.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::bessel::{xc_bessel_K0, xc_bessel_K1};
use libxc_kernel_math::integrate::{xc_integrate_lda_soft_func1, xc_integrate_lda_soft_func2};

/// LDA_X_1D_SOFT kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_x_1d_soft_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t7 = 1.0 + t5 <= zeta_threshold;
        let t8 = rho0 <= dens_threshold || t7;
        let t9 = zeta_threshold - 1.0;
        let t11 = 1.0 - t5 <= zeta_threshold;
        let t12 = -t9;
        let t13 = piecewise5::<f64>(t7, t9, t11, t12, t5);
        let t14 = 1.0 + t13;
        let t15 = t14 * M_PI;
        let t16 = param_beta * t3;
        let t17 = t15 * t16;
        let t18 = xc_integrate_lda_soft_func1::<f64>(t17);
        let t20 = xc_integrate_lda_soft_func2::<f64>(t17);
        let t21 = 1.0 / M_PI;
        let t22 = t20 * t21;
        let t23 = 1.0 / param_beta;
        let t24 = t23 * t4;
        let t29 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t14 * t18 - t22 * t24) * t23);
        let t31 = rho1 <= dens_threshold || t11;
        let t32 = piecewise5::<f64>(t11, t9, t7, t12, -t5);
        let t33 = 1.0 + t32;
        let t34 = t33 * M_PI;
        let t35 = t34 * t16;
        let t36 = xc_integrate_lda_soft_func1::<f64>(t35);
        let t38 = xc_integrate_lda_soft_func2::<f64>(t35);
        let t39 = t38 * t21;
        let t44 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (-t39 * t24 + t33 * t36) * t23);
        let tzk0 = t29 + t44;
        zk[ip] += tzk0;
        let t45 = t3 * t3;
        let t46 = 1.0 / t45;
        let t47 = t2 * t46;
        let t48 = t4 - t47;
        let t49 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t48);
        let t51 = t23 * t46;
        let t52 = t22 * t51;
        let t56 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t49 * t18 + t52) * t23);
        let t58 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t48);
        let t60 = t39 * t51;
        let t64 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (t58 * t36 + t60) * t23);
        let tvrho0 = t29 + t44 + t3 * (t56 + t64);
        vrho[ip * 2] += tvrho0;
        let t67 = -t4 - t47;
        let t68 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t67);
        let t73 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t68 * t18 + t52) * t23);
        let t75 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t67);
        let t80 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (t75 * t36 + t60) * t23);
        let tvrho1 = t29 + t44 + t3 * (t73 + t80);
        vrho[ip * 2 + 1] += tvrho1;
        let t86 = 1.0 / t45 / t3;
        let t87 = t2 * t86;
        let t89 = -2.0 * t46 + 2.0 * t87;
        let t90 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t89);
        let t92 = t49 * M_PI;
        let t94 = t15 * param_beta;
        let t95 = t92 * t16 + t94;
        let t96 = t49 * t95;
        let t97 = xc_bessel_K0::<f64>( t17);
        let t100 = t95 * t97;
        let t101 = t14 * t4;
        let t103 = 2.0 * t100 * t101;
        let t104 = t23 * t86;
        let t106 = 2.0 * t22 * t104;
        let t110 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t90 * t18 + 2.0 * t96 * t97 + t103 - t106) * t23);
        let t112 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t89);
        let t114 = t58 * M_PI;
        let t116 = t34 * param_beta;
        let t117 = t114 * t16 + t116;
        let t118 = t58 * t117;
        let t119 = xc_bessel_K0::<f64>( t35);
        let t122 = t117 * t119;
        let t123 = t33 * t4;
        let t125 = 2.0 * t122 * t123;
        let t127 = 2.0 * t39 * t104;
        let t131 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (t112 * t36 + 2.0 * t118 * t119 + t125 - t127) * t23);
        let tv2rho20 = 2.0 * t56 + 2.0 * t64 + t3 * (t110 + t131);
        v2rho2[ip * 3] += tv2rho20;
        let t134 = 2.0 * t87;
        let t135 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t134);
        let t137 = t68 * t95;
        let t143 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t135 * t18 + 2.0 * t137 * t97 + t103 - t106) * t23);
        let t144 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t134);
        let t146 = t75 * t117;
        let t152 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (t144 * t36 + 2.0 * t146 * t119 + t125 - t127) * t23);
        let tv2rho21 = t56 + t64 + t73 + t80 + t3 * (t143 + t152);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t158 = 2.0 * t46 + 2.0 * t87;
        let t159 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t158);
        let t161 = t68 * M_PI;
        let t163 = t161 * t16 + t94;
        let t164 = t68 * t163;
        let t167 = t163 * t97;
        let t173 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t159 * t18 + 2.0 * t164 * t97 + 2.0 * t167 * t101 - t106) * t23);
        let t175 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t158);
        let t177 = t75 * M_PI;
        let t179 = t177 * t16 + t116;
        let t180 = t75 * t179;
        let t183 = t179 * t119;
        let t189 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (t175 * t36 + 2.0 * t180 * t119 + 2.0 * t183 * t123 - t127) * t23);
        let tv2rho22 = 2.0 * t73 + 2.0 * t80 + t3 * (t173 + t189);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t194 = t45 * t45;
        let t195 = 1.0 / t194;
        let t196 = t2 * t195;
        let t198 = 6.0 * t86 - 6.0 * t196;
        let t199 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t198);
        let t204 = t90 * M_PI;
        let t206 = t92 * param_beta;
        let t208 = t204 * t16 + 2.0 * t206;
        let t209 = t49 * t208;
        let t212 = t95 * t95;
        let t214 = xc_bessel_K1::<f64>( t17);
        let t217 = t208 * t97;
        let t219 = 2.0 * t217 * t101;
        let t220 = t212 * t214;
        let t222 = 2.0 * t220 * t101;
        let t223 = t49 * t4;
        let t225 = 2.0 * t100 * t223;
        let t226 = t14 * t46;
        let t227 = t100 * t226;
        let t228 = 6.0 * t227;
        let t229 = t23 * t195;
        let t231 = 6.0 * t22 * t229;
        let t235 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t199 * t18 + 4.0 * t90 * t95 * t97 + 2.0 * t209 * t97 - 2.0 * t49 * t212 * t214 + t219 - t222 + t225 - t228 + t231) * t23);
        let t237 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t198);
        let t242 = t112 * M_PI;
        let t244 = t114 * param_beta;
        let t246 = t242 * t16 + 2.0 * t244;
        let t247 = t58 * t246;
        let t250 = t117 * t117;
        let t252 = xc_bessel_K1::<f64>( t35);
        let t255 = t246 * t119;
        let t257 = 2.0 * t255 * t123;
        let t258 = t250 * t252;
        let t260 = 2.0 * t258 * t123;
        let t261 = t58 * t4;
        let t263 = 2.0 * t122 * t261;
        let t264 = t33 * t46;
        let t265 = t122 * t264;
        let t266 = 6.0 * t265;
        let t268 = 6.0 * t39 * t229;
        let t272 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (t237 * t36 + 4.0 * t112 * t117 * t119 + 2.0 * t247 * t119 - 2.0 * t58 * t250 * t252 + t257 - t260 + t263 - t266 + t268) * t23);
        let tv3rho30 = 3.0 * t110 + 3.0 * t131 + t3 * (t235 + t272);
        v3rho3[ip * 4] += tv3rho30;
        let t275 = 2.0 * t143;
        let t276 = 2.0 * t152;
        let t277 = 2.0 * t86;
        let t278 = 6.0 * t196;
        let t279 = t277 - t278;
        let t280 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t279);
        let t285 = t68 * t208;
        let t294 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t280 * t18 + 4.0 * t135 * t95 * t97 + 2.0 * t285 * t97 - 2.0 * t68 * t212 * t214 + t219 - t222 + t225 - t228 + t231) * t23);
        let t296 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t279);
        let t301 = t75 * t246;
        let t310 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (t296 * t36 + 4.0 * t144 * t117 * t119 + 2.0 * t301 * t119 - 2.0 * t75 * t250 * t252 + t257 - t260 + t263 - t266 + t268) * t23);
        let tv3rho31 = t110 + t131 + t275 + t276 + t3 * (t294 + t310);
        v3rho3[ip * 4 + 1] += tv3rho31;
        let t313 = -t277 - t278;
        let t314 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t313);
        let t319 = t135 * t163;
        let t322 = t135 * M_PI;
        let t324 = t161 * param_beta;
        let t325 = t322 * t16 + t206 + t324;
        let t326 = t68 * t325;
        let t329 = t214 * t95;
        let t332 = t325 * t97;
        let t335 = t163 * t214;
        let t336 = t95 * t14;
        let t337 = t336 * t4;
        let t342 = t167 * t226;
        let t345 = t314 * t18 + 2.0 * t159 * t95 * t97 + 2.0 * t319 * t97 + 2.0 * t326 * t97 - 2.0 * t164 * t329 + 2.0 * t332 * t101 - 2.0 * t335 * t337 + 2.0 * t167 * t223 - 2.0 * t342 - 4.0 * t227 + t231;
        let t348 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * t345 * t23);
        let t350 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t313);
        let t355 = t144 * t179;
        let t358 = t144 * M_PI;
        let t360 = t177 * param_beta;
        let t361 = t358 * t16 + t244 + t360;
        let t362 = t75 * t361;
        let t365 = t252 * t117;
        let t368 = t361 * t119;
        let t371 = t179 * t252;
        let t372 = t117 * t33;
        let t373 = t372 * t4;
        let t378 = t183 * t264;
        let t381 = t350 * t36 + 2.0 * t175 * t117 * t119 + 2.0 * t355 * t119 + 2.0 * t362 * t119 - 2.0 * t180 * t365 + 2.0 * t368 * t123 - 2.0 * t371 * t373 + 2.0 * t183 * t261 - 2.0 * t378 - 4.0 * t265 + t268;
        let t384 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * t381 * t23);
        let tv3rho32 = t275 + t276 + t173 + t189 + t3 * (t348 + t384);
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t390 = -6.0 * t86 - 6.0 * t196;
        let t391 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t390);
        let t393 = t159 * t163;
        let t396 = t159 * M_PI;
        let t399 = t396 * t16 + 2.0 * t324;
        let t400 = t68 * t399;
        let t403 = t163 * t163;
        let t404 = t68 * t403;
        let t407 = t399 * t97;
        let t410 = t403 * t214;
        let t413 = t68 * t4;
        let t420 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t391 * t18 + 4.0 * t393 * t97 + 2.0 * t400 * t97 - 2.0 * t404 * t214 + 2.0 * t407 * t101 - 2.0 * t410 * t101 + 2.0 * t167 * t413 - 6.0 * t342 + t231) * t23);
        let t422 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t390);
        let t424 = t175 * t179;
        let t427 = t175 * M_PI;
        let t430 = t427 * t16 + 2.0 * t360;
        let t431 = t75 * t430;
        let t434 = t179 * t179;
        let t435 = t75 * t434;
        let t438 = t430 * t119;
        let t441 = t434 * t252;
        let t444 = t75 * t4;
        let t451 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (t422 * t36 + 4.0 * t424 * t119 + 2.0 * t431 * t119 - 2.0 * t435 * t252 + 2.0 * t438 * t123 - 2.0 * t441 * t123 + 2.0 * t183 * t444 - 6.0 * t378 + t268) * t23);
        let tv3rho33 = 3.0 * t173 + 3.0 * t189 + t3 * (t420 + t451);
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}
