//! GGA_X_DK87 kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_dk87.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_dk87_kxc_unpol(
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
    param_alpha: f64,
    param_a1: f64,
    param_b1: f64,
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
        let t20 = 1.0 / M_PI;
        let t21 = M_CBRT6;
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_PI * M_PI;
        let t25 = pow_1_3(t24);
        let t26 = 1.0 / t25;
        let t27 = t3 * t3;
        let t29 = pow_1_3(t20);
        let t30 = 1.0 / t29;
        let t32 = t23 * t26 * t27 * t30;
        let t33 = M_CBRT4;
        let t34 = t33 * sigma[ip];
        let t35 = M_CBRT2;
        let t36 = t35 * t35;
        let t37 = t34 * t36;
        let t38 = rho[ip] * rho[ip];
        let t39 = t18 * t18;
        let t41 = 1.0 / t39 / t38;
        let t42 = rmath::sqrt(sigma[ip]);
        let t47 = rmath::pow(t42 * t35 / t18 / rho[ip], param_alpha);
        let t48 = param_a1 * t47;
        let t49 = 1.0 + t48;
        let t51 = param_b1 * sigma[ip];
        let t52 = t36 * t41;
        let t54 = t51 * t52 + 1.0;
        let t55 = 1.0 / t54;
        let t56 = t41 * t49 * t55;
        let t60 = 1.0 + 7.0 / 11664.0 * t32 * t37 * t56;
        let t64 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t60);
        let tzk0 = 2.0 * t64;
        zk[ip] += tzk0;
        let t66 = t17 / t39;
        let t70 = t38 * rho[ip];
        let t72 = 1.0 / t39 / t70;
        let t74 = t72 * t49 * t55;
        let t78 = t23 * t26;
        let t79 = t27 * t30;
        let t81 = t78 * t79 * t33;
        let t82 = sigma[ip] * t36;
        let t85 = t48 * param_alpha * t55;
        let t89 = sigma[ip] * sigma[ip];
        let t90 = t89 * t35;
        let t91 = t38 * t38;
        let t92 = t91 * t38;
        let t94 = 1.0 / t18 / t92;
        let t96 = t54 * t54;
        let t97 = 1.0 / t96;
        let t99 = t49 * t97 * param_b1;
        let t103 = -7.0 / 4374.0 * t32 * t37 * t74 - 7.0 / 8748.0 * t81 * t82 * t72 * t85 + 7.0 / 2187.0 * t81 * t90 * t94 * t99;
        let t108 = piecewise3(t2, 0.0, -t6 * t66 * t60 / 8.0 - 3.0 / 8.0 * t6 * t19 * t103);
        let tvrho0 = 2.0 * rho[ip] * t108 + 2.0 * t64;
        vrho[ip] += tvrho0;
        let t111 = t33 * t36;
        let t115 = t52 * param_a1;
        let t116 = t47 * param_alpha;
        let t117 = t116 * t55;
        let t122 = t91 * rho[ip];
        let t124 = 1.0 / t18 / t122;
        let t129 = 7.0 / 11664.0 * t32 * t111 * t56 + 7.0 / 23328.0 * t81 * t115 * t117 - 7.0 / 5832.0 * t81 * sigma[ip] * t35 * t124 * t99;
        let t133 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t129);
        let tvsigma0 = 2.0 * rho[ip] * t133;
        vsigma[ip] += tvsigma0;
        let t138 = t17 / t39 / rho[ip];
        let t146 = 1.0 / t39 / t91;
        let t148 = t146 * t49 * t55;
        let t152 = t82 * t146;
        let t156 = t91 * t70;
        let t158 = 1.0 / t18 / t156;
        let t163 = param_alpha * param_alpha;
        let t165 = t48 * t163 * t55;
        let t169 = t33 * t89;
        let t171 = t78 * t79 * t169;
        let t172 = t35 * t158;
        let t173 = t172 * param_a1;
        let t174 = t97 * param_b1;
        let t175 = t116 * t174;
        let t176 = t173 * t175;
        let t179 = t89 * sigma[ip];
        let t180 = t33 * t179;
        let t181 = t91 * t91;
        let t182 = t181 * t38;
        let t183 = 1.0 / t182;
        let t186 = 1.0 / t96 / t54;
        let t188 = param_b1 * param_b1;
        let t189 = t49 * t186 * t188;
        let t193 = 77.0 / 13122.0 * t32 * t37 * t148 + 133.0 / 26244.0 * t81 * t152 * t85 - 7.0 / 243.0 * t81 * t90 * t158 * t99 + 7.0 / 6561.0 * t81 * t152 * t165 - 56.0 / 6561.0 * t171 * t176 + 224.0 / 6561.0 * t32 * t180 * t183 * t189;
        let t198 = piecewise3(t2, 0.0, t6 * t138 * t60 / 12.0 - t6 * t66 * t103 / 4.0 - 3.0 / 8.0 * t6 * t19 * t193);
        let tv2rho20 = 2.0 * rho[ip] * t198 + 4.0 * t108;
        v2rho2[ip] += tv2rho20;
        let t208 = t36 * t72 * param_a1;
        let t212 = t35 * t94;
        let t214 = t174 * sigma[ip];
        let t218 = t47 * t163;
        let t219 = t218 * t55;
        let t223 = t33 * t35;
        let t225 = t78 * t79 * t223;
        let t227 = t94 * param_a1 * t47;
        let t229 = param_alpha * t97 * t51;
        let t233 = t181 * rho[ip];
        let t234 = 1.0 / t233;
        let t239 = -7.0 / 4374.0 * t32 * t111 * t74 - 7.0 / 4374.0 * t81 * t208 * t117 + 7.0 / 729.0 * t81 * t212 * t49 * t214 - 7.0 / 17496.0 * t81 * t208 * t219 + 7.0 / 2187.0 * t225 * t227 * t229 - 28.0 / 2187.0 * t32 * t169 * t234 * t189;
        let t244 = piecewise3(t2, 0.0, -t6 * t66 * t129 / 8.0 - 3.0 / 8.0 * t6 * t19 * t239);
        let tv2rhosigma0 = 2.0 * rho[ip] * t244 + 2.0 * t133;
        v2rhosigma[ip] += tv2rhosigma0;
        let t247 = 1.0 / sigma[ip];
        let t248 = t247 * t55;
        let t249 = t116 * t248;
        let t257 = t218 * t248;
        let t266 = 1.0 / t181;
        let t271 = 7.0 / 23328.0 * t81 * t115 * t249 - 7.0 / 2916.0 * t32 * t223 * t124 * t99 + 7.0 / 46656.0 * t81 * t115 * t257 - 7.0 / 5832.0 * t81 * t35 * t124 * param_a1 * t175 + 7.0 / 1458.0 * t32 * t34 * t266 * t189;
        let t275 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t271);
        let tv2sigma20 = 2.0 * rho[ip] * t275;
        v2sigma2[ip] += tv2sigma20;
        let t278 = t17 * t41;
        let t289 = 1.0 / t39 / t122;
        let t291 = t289 * t49 * t55;
        let t295 = t82 * t289;
        let t300 = 1.0 / t18 / t181;
        let t308 = t35 * t300;
        let t309 = t308 * param_a1;
        let t313 = t181 * t70;
        let t314 = 1.0 / t313;
        let t319 = t163 * param_alpha;
        let t321 = t48 * t319 * t55;
        let t325 = t218 * t174;
        let t331 = t186 * t188;
        let t332 = t116 * t331;
        let t336 = t89 * t89;
        let t337 = t181 * t122;
        let t339 = 1.0 / t39 / t337;
        let t342 = t96 * t96;
        let t343 = 1.0 / t342;
        let t344 = t188 * param_b1;
        let t345 = t343 * t344;
        let t346 = t345 * t36;
        let t350 = -539.0 / 19683.0 * t32 * t37 * t291 - 413.0 / 13122.0 * t81 * t295 * t85 + 4774.0 / 19683.0 * t81 * t90 * t300 * t99 - 77.0 / 6561.0 * t81 * t295 * t165 + 280.0 / 2187.0 * t171 * t309 * t175 - 4256.0 / 6561.0 * t32 * t180 * t314 * t189 - 28.0 / 19683.0 * t81 * t295 * t321 + 112.0 / 6561.0 * t171 * t309 * t325 - 896.0 / 6561.0 * t81 * t179 * t314 * param_a1 * t332 + 1792.0 / 6561.0 * t81 * t336 * t339 * t49 * t346;
        let t355 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t278 * t60 + t6 * t138 * t103 / 4.0 - 3.0 / 8.0 * t6 * t66 * t193 - 3.0 / 8.0 * t6 * t19 * t350);
        let tv3rho30 = 2.0 * rho[ip] * t355 + 6.0 * t198;
        v3rho3[ip] += tv3rho30;
        let t369 = t36 * t146 * param_a1;
        let t381 = t158 * param_a1 * t47;
        let t386 = t33 * t183 * t49;
        let t387 = t331 * t89;
        let t391 = t47 * t319;
        let t392 = t391 * t55;
        let t397 = t163 * t97 * t51;
        let t402 = t183 * param_a1 * t47;
        let t403 = param_alpha * t186;
        let t404 = t188 * t89;
        let t405 = t403 * t404;
        let t409 = t181 * t91;
        let t411 = 1.0 / t39 / t409;
        let t417 = 77.0 / 13122.0 * t32 * t111 * t148 + 35.0 / 4374.0 * t81 * t369 * t117 - 455.0 / 6561.0 * t81 * t172 * t49 * t214 + 7.0 / 1944.0 * t81 * t369 * t219 - 91.0 / 2187.0 * t225 * t381 * t229 + 476.0 / 2187.0 * t32 * t386 * t387 + 7.0 / 13122.0 * t81 * t369 * t392 - 14.0 / 2187.0 * t225 * t381 * t397 + 112.0 / 2187.0 * t81 * t402 * t405 - 224.0 / 2187.0 * t81 * t179 * t411 * t49 * t346;
        let t422 = piecewise3(t2, 0.0, t6 * t138 * t129 / 12.0 - t6 * t66 * t239 / 4.0 - 3.0 / 8.0 * t6 * t19 * t417);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t422 + 4.0 * t244;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t434 = t212 * param_a1;
        let t442 = t33 * t234;
        let t444 = t331 * sigma[ip];
        let t448 = t391 * t248;
        let t456 = t234 * param_a1 * t47;
        let t457 = t188 * sigma[ip];
        let t458 = t403 * t457;
        let t463 = 1.0 / t39 / t313;
        let t469 = -7.0 / 8748.0 * t81 * t208 * t249 - 7.0 / 8748.0 * t81 * t208 * t257 + 49.0 / 4374.0 * t81 * t434 * t175 + 28.0 / 2187.0 * t32 * t223 * t94 * t99 - 140.0 / 2187.0 * t32 * t442 * t49 * t444 - 7.0 / 34992.0 * t81 * t208 * t448 + 7.0 / 2916.0 * t81 * t434 * t325 - 14.0 / 729.0 * t81 * t456 * t458 + 28.0 / 729.0 * t81 * t89 * t463 * t49 * t346;
        let t474 = piecewise3(t2, 0.0, -t6 * t66 * t271 / 8.0 - 3.0 / 8.0 * t6 * t19 * t469);
        let tv3rhosigma20 = 2.0 * rho[ip] * t474 + 2.0 * t275;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t477 = 1.0 / t89;
        let t478 = t477 * t55;
        let t479 = t116 * t478;
        let t484 = t124 * param_a1 * t47;
        let t485 = param_alpha * t247;
        let t486 = t485 * t174;
        let t494 = t391 * t478;
        let t498 = t163 * t247;
        let t499 = t498 * t174;
        let t504 = t266 * param_a1 * t47;
        let t505 = t403 * t188;
        let t510 = 1.0 / t39 / t182;
        let t516 = -7.0 / 23328.0 * t81 * t115 * t479 - 7.0 / 3888.0 * t225 * t484 * t486 + 7.0 / 486.0 * t32 * t33 * t266 * t189 + 7.0 / 93312.0 * t81 * t115 * t494 - 7.0 / 7776.0 * t225 * t484 * t499 + 7.0 / 972.0 * t81 * t504 * t505 - 7.0 / 486.0 * t81 * sigma[ip] * t510 * t49 * t346;
        let t520 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t516);
        let tv3sigma30 = 2.0 * rho[ip] * t520;
        v3sigma3[ip] += tv3sigma30;
    }
}
