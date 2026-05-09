//! GGA_K_APBE fxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 56 shared lines across all orders.
//! Delta: 114 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_apbe_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        // --- shared preamble (56 lines) ---
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_CBRT6;
        let t33 = param_mu * t32;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t37 * sigma0;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t47 = param_kappa + t33 * t38 * t43 / 24.0;
        let t52 = 1.0 + param_kappa * (1.0 - param_kappa / t47);
        let t56 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t52);
        let t57 = rho1 <= dens_threshold;
        let t58 = -t17;
        let t60 = piecewise5(t15, t12, t11, t16, t58 * t8);
        let t61 = 1.0 + t60;
        let t62 = t61 <= zeta_threshold;
        let t63 = pow_1_3(t61);
        let t64 = t63 * t63;
        let t66 = piecewise3(t62, t24, t64 * t61);
        let t67 = t66 * t30;
        let t68 = t37 * sigma2;
        let t69 = rho1 * rho1;
        let t70 = pow_1_3(rho1);
        let t71 = t70 * t70;
        let t73 = 1.0 / t71 / t69;
        let t77 = param_kappa + t33 * t68 * t73 / 24.0;
        let t82 = 1.0 + param_kappa * (1.0 - param_kappa / t77);
        let t86 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t67 * t82);
        let tzk0 = t56 + t86;
        zk[ip] += tzk0;
        // --- vxc delta (52 lines) ---
        let t87 = t7 * t7;
        let t88 = 1.0 / t87;
        let t89 = t17 * t88;
        let t91 = piecewise5(t11, 0.0, t15, 0.0, t8 - t89);
        let t94 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t91);
        let t95 = t94 * t30;
        let t99 = 1.0 / t29;
        let t100 = t28 * t99;
        let t103 = t6 * t100 * t52 / 10.0;
        let t104 = param_kappa * param_kappa;
        let t105 = t31 * t104;
        let t106 = t6 * t105;
        let t107 = t47 * t47;
        let t109 = 1.0 / t107 * param_mu;
        let t110 = t109 * t32;
        let t111 = t39 * rho0;
        let t113 = 1.0 / t41 / t111;
        let t115 = t110 * t38 * t113;
        let t119 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t95 * t52 + t103 - t106 * t115 / 60.0);
        let t120 = t58 * t88;
        let t122 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t120);
        let t125 = piecewise3(t62, 0.0, 5.0 / 3.0 * t64 * t122);
        let t126 = t125 * t30;
        let t130 = t66 * t99;
        let t133 = t6 * t130 * t82 / 10.0;
        let t135 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t126 * t82 + t133);
        let tvrho0 = t56 + t86 + t7 * (t119 + t135);
        vrho[ip * 2] += tvrho0;
        let t139 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t89);
        let t142 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t139);
        let t143 = t142 * t30;
        let t148 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t143 * t52 + t103);
        let t150 = piecewise5(t15, 0.0, t11, 0.0, t8 - t120);
        let t153 = piecewise3(t62, 0.0, 5.0 / 3.0 * t64 * t150);
        let t154 = t153 * t30;
        let t158 = t67 * t104;
        let t159 = t6 * t158;
        let t160 = t77 * t77;
        let t162 = 1.0 / t160 * param_mu;
        let t163 = t162 * t32;
        let t164 = t69 * rho1;
        let t166 = 1.0 / t71 / t164;
        let t168 = t163 * t68 * t166;
        let t172 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t154 * t82 + t133 - t159 * t168 / 60.0);
        let tvrho1 = t56 + t86 + t7 * (t148 + t172);
        vrho[ip * 2 + 1] += tvrho1;
        let t175 = t32 * t37;
        let t177 = t109 * t175 * t43;
        let t180 = piecewise3(t1, 0.0, t106 * t177 / 160.0);
        let tvsigma0 = t7 * t180;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t182 = t162 * t175 * t73;
        let t185 = piecewise3(t57, 0.0, t159 * t182 / 160.0);
        let tvsigma2 = t7 * t185;
        vsigma[ip * 3 + 2] += tvsigma2;
        // --- fxc delta (this level) (114 lines) ---
        let t188 = 1.0 / t25;
        let t189 = t91 * t91;
        let t192 = t87 * t7;
        let t193 = 1.0 / t192;
        let t194 = t17 * t193;
        let t197 = piecewise5(t11, 0.0, t15, 0.0, -2.0 * t88 + 2.0 * t194);
        let t201 = piecewise3(t21, 0.0, 10.0 / 9.0 * t188 * t189 + 5.0 / 3.0 * t26 * t197);
        let t202 = t201 * t30;
        let t206 = t94 * t99;
        let t208 = t6 * t206 * t52;
        let t211 = t6 * t95 * t104;
        let t215 = 1.0 / t29 / t7;
        let t216 = t28 * t215;
        let t219 = t6 * t216 * t52 / 30.0;
        let t221 = t6 * t100 * t104;
        let t222 = t221 * t115;
        let t226 = param_mu * param_mu;
        let t227 = 1.0 / t107 / t47 * t226;
        let t228 = t32 * t32;
        let t229 = t227 * t228;
        let t231 = 1.0 / t35 / t34;
        let t232 = sigma0 * sigma0;
        let t233 = t231 * t232;
        let t234 = t39 * t39;
        let t237 = 1.0 / t40 / t234 / t111;
        let t239 = t229 * t233 * t237;
        let t243 = 1.0 / t41 / t234;
        let t245 = t110 * t38 * t243;
        let t249 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t202 * t52 + t208 / 5.0 - t211 * t115 / 30.0 - t219 - t222 / 45.0 - t106 * t239 / 270.0 + 11.0 / 180.0 * t106 * t245);
        let t250 = 1.0 / t63;
        let t251 = t122 * t122;
        let t254 = t58 * t193;
        let t257 = piecewise5(t15, 0.0, t11, 0.0, 2.0 * t88 + 2.0 * t254);
        let t261 = piecewise3(t62, 0.0, 10.0 / 9.0 * t250 * t251 + 5.0 / 3.0 * t64 * t257);
        let t262 = t261 * t30;
        let t266 = t125 * t99;
        let t268 = t6 * t266 * t82;
        let t270 = t66 * t215;
        let t273 = t6 * t270 * t82 / 30.0;
        let t275 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t262 * t82 + t268 / 5.0 - t273);
        let tv2rho20 = 2.0 * t119 + 2.0 * t135 + t7 * (t249 + t275);
        v2rho2[ip * 3] += tv2rho20;
        let t278 = t188 * t139;
        let t282 = piecewise5(t11, 0.0, t15, 0.0, 2.0 * t194);
        let t286 = piecewise3(t21, 0.0, 10.0 / 9.0 * t278 * t91 + 5.0 / 3.0 * t26 * t282);
        let t287 = t286 * t30;
        let t291 = t142 * t99;
        let t293 = t6 * t291 * t52;
        let t296 = t6 * t143 * t104;
        let t302 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t287 * t52 + t293 / 10.0 - t296 * t115 / 60.0 + t208 / 10.0 - t219 - t222 / 90.0);
        let t303 = t250 * t150;
        let t307 = piecewise5(t15, 0.0, t11, 0.0, 2.0 * t254);
        let t311 = piecewise3(t62, 0.0, 10.0 / 9.0 * t303 * t122 + 5.0 / 3.0 * t64 * t307);
        let t312 = t311 * t30;
        let t316 = t153 * t99;
        let t318 = t6 * t316 * t82;
        let t322 = t6 * t126 * t104;
        let t326 = t6 * t130 * t104;
        let t327 = t326 * t168;
        let t330 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t312 * t82 + t318 / 10.0 + t268 / 10.0 - t273 - t322 * t168 / 60.0 - t327 / 90.0);
        let tv2rho21 = t119 + t135 + t148 + t172 + t7 * (t302 + t330);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t335 = t139 * t139;
        let t340 = piecewise5(t11, 0.0, t15, 0.0, 2.0 * t88 + 2.0 * t194);
        let t344 = piecewise3(t21, 0.0, 10.0 / 9.0 * t188 * t335 + 5.0 / 3.0 * t26 * t340);
        let t345 = t344 * t30;
        let t351 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t345 * t52 + t293 / 5.0 - t219);
        let t352 = t150 * t150;
        let t357 = piecewise5(t15, 0.0, t11, 0.0, -2.0 * t88 + 2.0 * t254);
        let t361 = piecewise3(t62, 0.0, 10.0 / 9.0 * t250 * t352 + 5.0 / 3.0 * t64 * t357);
        let t362 = t361 * t30;
        let t368 = t6 * t154 * t104;
        let t374 = 1.0 / t160 / t77 * t226;
        let t375 = t374 * t228;
        let t376 = sigma2 * sigma2;
        let t377 = t231 * t376;
        let t378 = t69 * t69;
        let t381 = 1.0 / t70 / t378 / t164;
        let t383 = t375 * t377 * t381;
        let t387 = 1.0 / t71 / t378;
        let t389 = t163 * t68 * t387;
        let t393 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t362 * t82 + t318 / 5.0 - t368 * t168 / 30.0 - t273 - t327 / 45.0 - t159 * t383 / 270.0 + 11.0 / 180.0 * t159 * t389);
        let tv2rho22 = 2.0 * t148 + 2.0 * t172 + t7 * (t351 + t393);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t399 = t221 * t177 / 240.0;
        let t400 = t234 * t39;
        let t402 = 1.0 / t40 / t400;
        let t405 = t229 * t231 * t402 * sigma0;
        let t409 = t109 * t175 * t113;
        let t413 = piecewise3(t1, 0.0, t211 * t177 / 160.0 + t399 + t106 * t405 / 720.0 - t106 * t409 / 60.0);
        let tv2rhosigma0 = t7 * t413 + t180;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t418 = t326 * t182 / 240.0;
        let t420 = piecewise3(t57, 0.0, t322 * t182 / 160.0 + t418);
        let tv2rhosigma2 = t7 * t420 + t185;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t425 = piecewise3(t1, 0.0, t296 * t177 / 160.0 + t399);
        let tv2rhosigma3 = t7 * t425 + t180;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t429 = t378 * t69;
        let t431 = 1.0 / t70 / t429;
        let t434 = t375 * t231 * t431 * sigma2;
        let t438 = t162 * t175 * t166;
        let t442 = piecewise3(t57, 0.0, t368 * t182 / 160.0 + t418 + t159 * t434 / 720.0 - t159 * t438 / 60.0);
        let tv2rhosigma5 = t7 * t442 + t185;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t444 = t228 * t231;
        let t445 = t234 * rho0;
        let t449 = t227 * t444 / t40 / t445;
        let t452 = piecewise3(t1, 0.0, -t106 * t449 / 1920.0);
        let tv2sigma20 = t7 * t452;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t453 = t378 * rho1;
        let t457 = t374 * t444 / t70 / t453;
        let t460 = piecewise3(t57, 0.0, -t159 * t457 / 1920.0);
        let tv2sigma25 = t7 * t460;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
