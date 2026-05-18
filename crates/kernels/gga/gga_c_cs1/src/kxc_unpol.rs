//! GGA_C_CS1 kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_cs1.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_cs1_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3::<f64>(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = 1.0 + 0.349e0 * t2;
        let t5 = 1.0 / t4;
        let t6 = sigma[ip] * sigma[ip];
        let t7 = rho[ip] * rho[ip];
        let t8 = t7 * t7;
        let t9 = t8 * rho[ip];
        let t11 = 1.0 / t1 / t9;
        let t13 = t1 * t1;
        let t15 = 1.0 / t13 / t7;
        let t18 = 1.0 + 0.6e-2 * sigma[ip] * t15;
        let t19 = t18 * t18;
        let t20 = 1.0 / t19;
        let t23 = -0.159068e0 + 0.286308e-6 * t6 * t11 * t20;
        let t25 = t5 * t23 / 4.0;
        let t27 = piecewise3::<f64>(1.0 <= zeta_threshold, zeta_threshold, 1.0);
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = t27 * t29;
        let t33 = t29 * t1 / 2.0 + 0.349e0;
        let t34 = 1.0 / t33;
        let t35 = t1 * t34;
        let t36 = t6 * t28;
        let t37 = sigma[ip] * t29;
        let t40 = 1.0 + 0.6e-2 * t37 * t15;
        let t41 = t40 * t40;
        let t42 = 1.0 / t41;
        let t43 = t11 * t42;
        let t46 = -0.18897e-1 + 0.1117728e-4 * t36 * t43;
        let t49 = t30 * t35 * t46 / 2.0;
        let tzk0 = t25 + t49;
        zk[ip] += tzk0;
        let t50 = t4 * t4;
        let t51 = 1.0 / t50;
        let t52 = t51 * t23;
        let t54 = 1.0 / t1 / rho[ip];
        let t55 = t52 * t54;
        let t57 = t8 * t7;
        let t59 = 1.0 / t1 / t57;
        let t63 = t6 * sigma[ip];
        let t64 = t8 * t8;
        let t65 = t64 * rho[ip];
        let t66 = 1.0 / t65;
        let t67 = t63 * t66;
        let t69 = 1.0 / t19 / t18;
        let t72 = -0.1526976e-5 * t6 * t59 * t20 + 0.9161856e-8 * t67 * t69;
        let t73 = t5 * t72;
        let t76 = 1.0 / t13 * t34;
        let t78 = t30 * t76 * t46;
        let t80 = t27 * t28;
        let t81 = t33 * t33;
        let t82 = 1.0 / t81;
        let t83 = t2 * t82;
        let t85 = t80 * t83 * t46;
        let t87 = t59 * t42;
        let t91 = 1.0 / t41 / t40;
        let t94 = -0.5961216e-4 * t36 * t87 + 0.71534592e-6 * t67 * t91;
        let t96 = t30 * t35 * t94;
        let tvrho0 = t25 + t49 + rho[ip] * (0.29083333333333333332e-1 * t55 + t73 / 4.0 + t78 / 6.0 - t85 / 6.0 + t96 / 2.0);
        vrho[ip] += tvrho0;
        let t103 = 1.0 / t64;
        let t104 = t6 * t103;
        let t107 = 0.572616e-6 * sigma[ip] * t11 * t20 - 0.3435696e-8 * t104 * t69;
        let t109 = t5 * t107 / 4.0;
        let t110 = sigma[ip] * t28;
        let t115 = 0.2235456e-4 * t110 * t43 - 0.26825472e-6 * t104 * t91;
        let t118 = t30 * t35 * t115 / 2.0;
        let tvsigma0 = rho[ip] * (t109 + t118);
        vsigma[ip] += tvsigma0;
        let t125 = 1.0 / t50 / t4;
        let t126 = t125 * t23;
        let t127 = t126 * t15;
        let t129 = t51 * t72;
        let t130 = t129 * t54;
        let t133 = 1.0 / t1 / t7;
        let t134 = t52 * t133;
        let t136 = t7 * rho[ip];
        let t137 = t8 * t136;
        let t139 = 1.0 / t1 / t137;
        let t143 = t64 * t7;
        let t144 = 1.0 / t143;
        let t145 = t63 * t144;
        let t148 = t6 * t6;
        let t149 = t64 * t8;
        let t151 = 1.0 / t13 / t149;
        let t152 = t148 * t151;
        let t153 = t19 * t19;
        let t154 = 1.0 / t153;
        let t157 = 0.9670848e-5 * t6 * t139 * t20 - 0.131319936e-6 * t145 * t69 + 0.439769088e-9 * t152 * t154;
        let t158 = t5 * t157;
        let t161 = 1.0 / t13 / rho[ip];
        let t162 = t161 * t34;
        let t164 = t30 * t162 * t46;
        let t167 = t30 * t76 * t94;
        let t170 = t27 / rho[ip];
        let t172 = 1.0 / t81 / t33;
        let t173 = t172 * t46;
        let t174 = t170 * t173;
        let t177 = t80 * t83 * t94;
        let t179 = t139 * t42;
        let t184 = t41 * t41;
        let t185 = 1.0 / t184;
        let t186 = t185 * t29;
        let t189 = 0.37754368e-3 * t36 * t179 - 0.1025329152e-4 * t145 * t91 + 0.3433660416e-7 * t152 * t186;
        let t191 = t30 * t35 * t189;
        let tv2rho20 = 0.58166666666666666664e-1 * t55 + t73 / 2.0 + t78 / 3.0 - t85 / 3.0 + t96 + rho[ip] * (0.67667222222222222217e-2 * t127 + 0.58166666666666666664e-1 * t130 - 0.38777777777777777776e-1 * t134 + t158 / 4.0 - t164 / 9.0 + t167 / 3.0 + t174 / 9.0 - t177 / 3.0 + t191 / 2.0);
        v2rho2[ip] += tv2rho20;
        let t195 = t51 * t107;
        let t196 = t195 * t54;
        let t201 = t6 * t66;
        let t204 = t64 * t136;
        let t206 = 1.0 / t13 / t204;
        let t207 = t63 * t206;
        let t210 = -0.3053952e-5 * sigma[ip] * t59 * t20 + 0.4580928e-7 * t201 * t69 - 0.164913408e-9 * t207 * t154;
        let t211 = t5 * t210;
        let t214 = t30 * t76 * t115;
        let t217 = t80 * t83 * t115;
        let t225 = -0.11922432e-3 * t110 * t87 + 0.35767296e-5 * t201 * t91 - 0.1287622656e-7 * t207 * t186;
        let t227 = t30 * t35 * t225;
        let tv2rhosigma0 = t109 + t118 + rho[ip] * (0.29083333333333333332e-1 * t196 + t211 / 4.0 + t214 / 6.0 - t217 / 6.0 + t227 / 2.0);
        v2rhosigma[ip] += tv2rhosigma0;
        let t233 = sigma[ip] * t103;
        let t237 = 1.0 / t13 / t143;
        let t238 = t6 * t237;
        let t241 = 0.572616e-6 * t11 * t20 - 0.13742784e-7 * t233 * t69 + 0.61842528e-10 * t238 * t154;
        let t243 = t5 * t241 / 4.0;
        let t251 = 0.2235456e-4 * t28 * t11 * t42 - 0.107301888e-5 * t233 * t91 + 0.482858496e-8 * t238 * t186;
        let t254 = t30 * t35 * t251 / 2.0;
        let tv2sigma20 = rho[ip] * (t243 + t254);
        v2sigma2[ip] += tv2sigma20;
        let t264 = 1.0 / t13 / t136;
        let t265 = t126 * t264;
        let t267 = t129 * t133;
        let t270 = 1.0 / t1 / t136;
        let t271 = t52 * t270;
        let t273 = t172 * t94;
        let t274 = t170 * t273;
        let t276 = t125 * t72;
        let t277 = t276 * t15;
        let t279 = t50 * t50;
        let t280 = 1.0 / t279;
        let t281 = t280 * t23;
        let t282 = 1.0 / t8;
        let t283 = t281 * t282;
        let t285 = t51 * t157;
        let t286 = t285 * t54;
        let t289 = 1.0 / t1 / t64;
        let t293 = 1.0 / t204;
        let t294 = t63 * t293;
        let t297 = t64 * t9;
        let t299 = 1.0 / t13 / t297;
        let t300 = t148 * t299;
        let t303 = t148 * sigma[ip];
        let t304 = t64 * t64;
        let t306 = 1.0 / t1 / t304;
        let t307 = t303 * t306;
        let t309 = 1.0 / t153 / t18;
        let t312 = -0.70919552e-4 * t6 * t289 * t20 + 0.1622666496e-5 * t294 * t69 - 0.11873765376e-7 * t300 * t154 + 0.28145221632e-10 * t307 * t309;
        let t313 = t5 * t312;
        let t315 = t15 * t34;
        let t317 = t30 * t315 * t46;
        let t320 = t30 * t162 * t94;
        let t322 = t133 * t82;
        let t324 = t80 * t322 * t46;
        let t327 = t30 * t76 * t189;
        let t330 = t27 / t7;
        let t331 = t330 * t173;
        let t333 = t27 * t161;
        let t334 = t81 * t81;
        let t335 = 1.0 / t334;
        let t337 = t335 * t46 * t29;
        let t338 = t333 * t337;
        let t341 = t80 * t83 * t189;
        let t343 = t289 * t42;
        let t352 = 1.0 / t184 / t40 * t28;
        let t355 = -0.27686536533333333333e-2 * t36 * t343 + 0.12669571072e-3 * t294 * t91 - 0.92708831232e-6 * t300 * t186 + 0.439508533248e-8 * t307 * t352;
        let t357 = t30 * t35 * t355;
        let t359 = -0.27066888888888888887e-1 * t265 - 0.11633333333333333333e0 * t267 + 0.90481481481481481477e-1 * t271 + t274 / 3.0 + 0.20300166666666666665e-1 * t277 + 0.23615860555555555553e-2 * t283 + 0.87249999999999999996e-1 * t286 + t313 / 4.0 + 5.0 / 27.0 * t317 - t320 / 3.0 + t324 / 27.0 + t327 / 2.0 - t331 / 9.0 - t338 / 18.0 - t341 / 2.0 + t357 / 2.0;
        let tv3rho30 = 0.20300166666666666665e-1 * t127 + 0.1745e0 * t130 - 0.11633333333333333333e0 * t134 + 3.0 / 4.0 * t158 - t164 / 3.0 + t167 + t174 / 3.0 - t177 + 3.0 / 2.0 * t191 + rho[ip] * t359;
        v3rho3[ip] += tv3rho30;
        let t365 = t125 * t107;
        let t366 = t365 * t15;
        let t368 = t51 * t210;
        let t369 = t368 * t54;
        let t371 = t195 * t133;
        let t376 = t6 * t144;
        let t379 = t63 * t151;
        let t384 = 1.0 / t1 / t64 / t137;
        let t385 = t148 * t384;
        let t388 = 0.19341696e-4 * sigma[ip] * t139 * t20 - 0.510009984e-6 * t376 * t69 + 0.41228352e-8 * t379 * t154 - 0.10554458112e-10 * t385 * t309;
        let t389 = t5 * t388;
        let t392 = t30 * t162 * t115;
        let t395 = t30 * t76 * t225;
        let t397 = t172 * t115;
        let t398 = t170 * t397;
        let t401 = t80 * t83 * t225;
        let t411 = 0.75508736e-3 * t110 * t179 - 0.3982092288e-4 * t376 * t91 + 0.321905664e-6 * t379 * t186 - 0.164815699968e-8 * t385 * t352;
        let t413 = t30 * t35 * t411;
        let tv3rho2sigma0 = 0.58166666666666666664e-1 * t196 + t211 / 2.0 + t214 / 3.0 - t217 / 3.0 + t227 + rho[ip] * (0.67667222222222222217e-2 * t366 + 0.58166666666666666664e-1 * t369 - 0.38777777777777777776e-1 * t371 + t389 / 4.0 - t392 / 9.0 + t395 / 3.0 + t398 / 9.0 - t401 / 3.0 + t413 / 2.0);
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t417 = t51 * t241;
        let t418 = t417 * t54;
        let t422 = t66 * t69;
        let t425 = t6 * t206;
        let t428 = t64 * t57;
        let t430 = 1.0 / t1 / t428;
        let t431 = t63 * t430;
        let t434 = -0.3053952e-5 * t59 * t20 + 0.128265984e-6 * t422 * sigma[ip] - 0.1319307264e-8 * t425 * t154 + 0.3957921792e-11 * t431 * t309;
        let t435 = t5 * t434;
        let t438 = t30 * t76 * t251;
        let t441 = t80 * t83 * t251;
        let t446 = t66 * t91;
        let t453 = -0.11922432e-3 * t28 * t59 * t42 + 0.1001484288e-4 * t446 * sigma[ip] - 0.10300981248e-6 * t425 * t186 + 0.61805887488e-9 * t431 * t352;
        let t455 = t30 * t35 * t453;
        let tv3rhosigma20 = t243 + t254 + rho[ip] * (0.29083333333333333332e-1 * t418 + t435 / 4.0 + t438 / 6.0 - t441 / 6.0 + t455 / 2.0);
        v3rhosigma2[ip] += tv3rhosigma20;
        let t461 = sigma[ip] * t237;
        let t465 = 1.0 / t1 / t297;
        let t466 = t6 * t465;
        let t469 = -0.20614176e-7 * t103 * t69 + 0.371055168e-9 * t461 * t154 - 0.1484220672e-11 * t466 * t309;
        let t471 = t5 * t469 / 4.0;
        let t478 = -0.160952832e-5 * t103 * t91 + 0.2897150976e-7 * t461 * t186 - 0.23177207808e-9 * t466 * t352;
        let t481 = t30 * t35 * t478 / 2.0;
        let tv3sigma30 = rho[ip] * (t471 + t481);
        v3sigma3[ip] += tv3sigma30;
    }
}
