//! GGA_K_LGAP_GE fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lgap_ge.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_lgap_ge_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_mu_0: f64,
    param_mu_1: f64,
    param_mu_2: f64,
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
        let t33 = M_CBRT6;
        let t34 = t33 * t33;
        let t35 = param_mu_0 * t34;
        let t36 = M_PI * M_PI;
        let t37 = pow_1_3(t36);
        let t38 = 1.0 / t37;
        let t39 = f64::sqrt(sigma0);
        let t40 = t38 * t39;
        let t41 = pow_1_3(rho0);
        let t43 = 1.0 / t41 / rho0;
        let t48 = param_mu_1 * t33;
        let t49 = t37 * t37;
        let t50 = 1.0 / t49;
        let t51 = t50 * sigma0;
        let t52 = rho0 * rho0;
        let t53 = t41 * t41;
        let t55 = 1.0 / t53 / t52;
        let t61 = param_mu_2 / t36;
        let t62 = t39 * sigma0;
        let t63 = t52 * t52;
        let t64 = 1.0 / t63;
        let t68 = 1.0 + t35 * t40 * t43 / 12.0 + t48 * t51 * t55 / 24.0 + t61 * t62 * t64 / 48.0;
        let t72 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t68);
        let t73 = rho1 <= dens_threshold;
        let t74 = -t17;
        let t76 = piecewise5(t15, t12, t11, t16, t74 * t8);
        let t77 = 1.0 + t76;
        let t78 = t77 <= zeta_threshold;
        let t79 = pow_1_3(t77);
        let t80 = t79 * t79;
        let t82 = piecewise3(t78, t24, t80 * t77);
        let t83 = t82 * t30;
        let t84 = f64::sqrt(sigma2);
        let t85 = t38 * t84;
        let t86 = pow_1_3(rho1);
        let t88 = 1.0 / t86 / rho1;
        let t92 = t50 * sigma2;
        let t93 = rho1 * rho1;
        let t94 = t86 * t86;
        let t96 = 1.0 / t94 / t93;
        let t100 = t84 * sigma2;
        let t101 = t93 * t93;
        let t102 = 1.0 / t101;
        let t106 = 1.0 + t35 * t85 * t88 / 12.0 + t48 * t92 * t96 / 24.0 + t61 * t100 * t102 / 48.0;
        let t110 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t83 * t106);
        let tzk0 = t72 + t110;
        zk[ip] += tzk0;
        let t111 = t7 * t7;
        let t112 = 1.0 / t111;
        let t113 = t17 * t112;
        let t115 = piecewise5(t11, 0.0, t15, 0.0, t8 - t113);
        let t118 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t115);
        let t119 = t118 * t30;
        let t123 = 1.0 / t29;
        let t124 = t28 * t123;
        let t127 = t6 * t124 * t68 / 10.0;
        let t129 = 1.0 / t41 / t52;
        let t133 = t52 * rho0;
        let t135 = 1.0 / t53 / t133;
        let t139 = t63 * rho0;
        let t140 = 1.0 / t139;
        let t144 = -t35 * t40 * t129 / 9.0 - t48 * t51 * t135 / 9.0 - t61 * t62 * t140 / 12.0;
        let t149 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t119 * t68 + t127 + 3.0 / 20.0 * t6 * t31 * t144);
        let t150 = t74 * t112;
        let t152 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t150);
        let t155 = piecewise3(t78, 0.0, 5.0 / 3.0 * t80 * t152);
        let t156 = t155 * t30;
        let t160 = t82 * t123;
        let t163 = t6 * t160 * t106 / 10.0;
        let t165 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t156 * t106 + t163);
        let tvrho0 = t72 + t110 + t7 * (t149 + t165);
        vrho[ip * 2] += tvrho0;
        let t169 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t113);
        let t172 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t169);
        let t173 = t172 * t30;
        let t178 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t173 * t68 + t127);
        let t180 = piecewise5(t15, 0.0, t11, 0.0, t8 - t150);
        let t183 = piecewise3(t78, 0.0, 5.0 / 3.0 * t80 * t180);
        let t184 = t183 * t30;
        let t189 = 1.0 / t86 / t93;
        let t193 = t93 * rho1;
        let t195 = 1.0 / t94 / t193;
        let t199 = t101 * rho1;
        let t200 = 1.0 / t199;
        let t204 = -t35 * t85 * t189 / 9.0 - t48 * t92 * t195 / 9.0 - t61 * t100 * t200 / 12.0;
        let t209 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t184 * t106 + t163 + 3.0 / 20.0 * t6 * t83 * t204);
        let tvrho1 = t72 + t110 + t7 * (t178 + t209);
        vrho[ip * 2 + 1] += tvrho1;
        let t212 = 1.0 / t39;
        let t213 = t38 * t212;
        let t223 = t35 * t213 * t43 / 24.0 + t48 * t50 * t55 / 24.0 + t61 * t39 * t64 / 32.0;
        let t227 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t223);
        let tvsigma0 = t7 * t227;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t228 = 1.0 / t84;
        let t229 = t38 * t228;
        let t239 = t35 * t229 * t88 / 24.0 + t48 * t50 * t96 / 24.0 + t61 * t84 * t102 / 32.0;
        let t243 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t83 * t239);
        let tvsigma2 = t7 * t243;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t246 = 1.0 / t25;
        let t247 = t115 * t115;
        let t250 = t111 * t7;
        let t251 = 1.0 / t250;
        let t252 = t17 * t251;
        let t255 = piecewise5(t11, 0.0, t15, 0.0, -2.0 * t112 + 2.0 * t252);
        let t259 = piecewise3(t21, 0.0, 10.0 / 9.0 * t246 * t247 + 5.0 / 3.0 * t26 * t255);
        let t260 = t259 * t30;
        let t264 = t118 * t123;
        let t266 = t6 * t264 * t68;
        let t272 = 1.0 / t29 / t7;
        let t273 = t28 * t272;
        let t276 = t6 * t273 * t68 / 30.0;
        let t278 = t6 * t124 * t144;
        let t281 = 1.0 / t41 / t133;
        let t286 = 1.0 / t53 / t63;
        let t290 = t63 * t52;
        let t291 = 1.0 / t290;
        let t295 = 7.0 / 27.0 * t35 * t40 * t281 + 11.0 / 27.0 * t48 * t51 * t286 + 5.0 / 12.0 * t61 * t62 * t291;
        let t300 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t260 * t68 + t266 / 5.0 + 3.0 / 10.0 * t6 * t119 * t144 - t276 + t278 / 5.0 + 3.0 / 20.0 * t6 * t31 * t295);
        let t301 = 1.0 / t79;
        let t302 = t152 * t152;
        let t305 = t74 * t251;
        let t308 = piecewise5(t15, 0.0, t11, 0.0, 2.0 * t112 + 2.0 * t305);
        let t312 = piecewise3(t78, 0.0, 10.0 / 9.0 * t301 * t302 + 5.0 / 3.0 * t80 * t308);
        let t313 = t312 * t30;
        let t317 = t155 * t123;
        let t319 = t6 * t317 * t106;
        let t321 = t82 * t272;
        let t324 = t6 * t321 * t106 / 30.0;
        let t326 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t313 * t106 + t319 / 5.0 - t324);
        let tv2rho20 = 2.0 * t149 + 2.0 * t165 + t7 * (t300 + t326);
        v2rho2[ip * 3] += tv2rho20;
        let t329 = t246 * t169;
        let t333 = piecewise5(t11, 0.0, t15, 0.0, 2.0 * t252);
        let t337 = piecewise3(t21, 0.0, 10.0 / 9.0 * t329 * t115 + 5.0 / 3.0 * t26 * t333);
        let t338 = t337 * t30;
        let t342 = t172 * t123;
        let t344 = t6 * t342 * t68;
        let t352 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t338 * t68 + t344 / 10.0 + 3.0 / 20.0 * t6 * t173 * t144 + t266 / 10.0 - t276 + t278 / 10.0);
        let t353 = t301 * t180;
        let t357 = piecewise5(t15, 0.0, t11, 0.0, 2.0 * t305);
        let t361 = piecewise3(t78, 0.0, 10.0 / 9.0 * t353 * t152 + 5.0 / 3.0 * t80 * t357);
        let t362 = t361 * t30;
        let t366 = t183 * t123;
        let t368 = t6 * t366 * t106;
        let t375 = t6 * t160 * t204;
        let t378 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t362 * t106 + t368 / 10.0 + t319 / 10.0 - t324 + 3.0 / 20.0 * t6 * t156 * t204 + t375 / 10.0);
        let tv2rho21 = t149 + t165 + t178 + t209 + t7 * (t352 + t378);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t383 = t169 * t169;
        let t388 = piecewise5(t11, 0.0, t15, 0.0, 2.0 * t112 + 2.0 * t252);
        let t392 = piecewise3(t21, 0.0, 10.0 / 9.0 * t246 * t383 + 5.0 / 3.0 * t26 * t388);
        let t393 = t392 * t30;
        let t399 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t393 * t68 + t344 / 5.0 - t276);
        let t400 = t180 * t180;
        let t405 = piecewise5(t15, 0.0, t11, 0.0, -2.0 * t112 + 2.0 * t305);
        let t409 = piecewise3(t78, 0.0, 10.0 / 9.0 * t301 * t400 + 5.0 / 3.0 * t80 * t405);
        let t410 = t409 * t30;
        let t420 = 1.0 / t86 / t193;
        let t425 = 1.0 / t94 / t101;
        let t429 = t101 * t93;
        let t430 = 1.0 / t429;
        let t434 = 7.0 / 27.0 * t35 * t85 * t420 + 11.0 / 27.0 * t48 * t92 * t425 + 5.0 / 12.0 * t61 * t100 * t430;
        let t439 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t410 * t106 + t368 / 5.0 + 3.0 / 10.0 * t6 * t184 * t204 - t324 + t375 / 5.0 + 3.0 / 20.0 * t6 * t83 * t434);
        let tv2rho22 = 2.0 * t178 + 2.0 * t209 + t7 * (t399 + t439);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t447 = t6 * t124 * t223 / 10.0;
        let t457 = -t35 * t213 * t129 / 18.0 - t48 * t50 * t135 / 9.0 - t61 * t39 * t140 / 8.0;
        let t462 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t119 * t223 + t447 + 3.0 / 20.0 * t6 * t31 * t457);
        let tv2rhosigma0 = t7 * t462 + t227;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t469 = t6 * t160 * t239 / 10.0;
        let t471 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t156 * t239 + t469);
        let tv2rhosigma2 = t7 * t471 + t243;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t477 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t173 * t223 + t447);
        let tv2rhosigma3 = t7 * t477 + t227;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t491 = -t35 * t229 * t189 / 18.0 - t48 * t50 * t195 / 9.0 - t61 * t84 * t200 / 8.0;
        let t496 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t184 * t239 + t469 + 3.0 / 20.0 * t6 * t83 * t491);
        let tv2rhosigma5 = t7 * t496 + t243;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t498 = 1.0 / t62;
        let t499 = t38 * t498;
        let t506 = -t35 * t499 * t43 / 48.0 + t61 * t212 * t64 / 64.0;
        let t510 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t506);
        let tv2sigma20 = t7 * t510;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t511 = 1.0 / t100;
        let t512 = t38 * t511;
        let t519 = -t35 * t512 * t88 / 48.0 + t61 * t228 * t102 / 64.0;
        let t523 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t83 * t519);
        let tv2sigma25 = t7 * t523;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
