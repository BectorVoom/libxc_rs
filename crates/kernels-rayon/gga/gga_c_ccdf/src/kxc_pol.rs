//! GGA_C_CCDF kxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_ccdf.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(
    unused_imports,
    unused_variables,
    non_snake_case,
    clippy::excessive_precision,
    clippy::too_many_arguments,
    clippy::needless_return
)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT6, M_PI};
use libxc_rkernel_math::powers::pow_1_3;
use libxc_rkernel_math::rmath;

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_ccdf_kxc_pol(
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
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
    param_c4: f64,
    param_c5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t3 = 1.0 / t2;
        let t5 = param_c2 * t3 + 1.0;
        let t6 = 1.0 / t5;
        let t7 = param_c1 * t6;
        let t8 = M_CBRT2;
        let t9 = M_CBRT6;
        let t10 = t9 * t9;
        let t11 = t8 * t10;
        let t12 = M_PI * M_PI;
        let t13 = pow_1_3(t12);
        let t14 = 1.0 / t13;
        let t16 = sigma0 + 2.0 * sigma1 + sigma2;
        let t17 = rmath::sqrt(t16);
        let t18 = t14 * t17;
        let t20 = 1.0 / t2 / t1;
        let t26 = rmath::exp(-param_c4 * (t11 * t18 * t20 / 12.0 - param_c5));
        let t27 = 1.0 + t26;
        let t30 = 1.0 - param_c3 / t27;
        let tzk0 = t7 * t30;
        zk[ip] += tzk0;
        let t31 = t3 * param_c1;
        let t32 = t5 * t5;
        let t33 = 1.0 / t32;
        let t39 = t6 * param_c3;
        let t40 = t27 * t27;
        let t41 = 1.0 / t40;
        let t42 = t39 * t41;
        let t43 = t20 * param_c1 * t42;
        let t45 = param_c4 * t8 * t10;
        let tvrho0 = tzk0 + t31 * t33 * t30 * param_c2 / 3.0 + t43 * t45 * t18 * t26 / 9.0;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t50 = t31 * t42;
        let t51 = 1.0 / t17;
        let t54 = t45 * t14 * t51 * t26;
        let t55 = t50 * t54;
        let tvsigma0 = -t55 / 24.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = -t55 / 12.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t58 = param_c1 * t33;
        let t59 = t30 * param_c2;
        let t63 = param_c3 * t41;
        let t64 = t63 * param_c4;
        let t65 = t7 * t64;
        let t66 = t11 * t14;
        let t67 = t1 * t1;
        let t69 = 1.0 / t2 / t67;
        let t75 = t2 * t2;
        let t78 = 1.0 / t75 / t1 * param_c1;
        let t80 = 1.0 / t32 / t5;
        let t82 = param_c2 * param_c2;
        let t87 = 1.0 / t75 / t67;
        let t88 = t87 * param_c1;
        let t90 = t88 * t33 * t64;
        let t91 = t17 * t26;
        let t92 = t91 * param_c2;
        let t96 = t67 * t1;
        let t98 = 1.0 / t75 / t96;
        let t99 = t98 * param_c1;
        let t101 = 1.0 / t40 / t27;
        let t102 = t39 * t101;
        let t103 = t99 * t102;
        let t104 = param_c4 * param_c4;
        let t105 = t8 * t8;
        let t106 = t104 * t105;
        let t107 = t106 * t9;
        let t108 = t13 * t13;
        let t109 = 1.0 / t108;
        let t110 = t109 * t16;
        let t111 = t26 * t26;
        let t116 = t99 * t42;
        let tv2rho20 = 2.0 / 9.0 * t58 * t59 * t20 - t65 * t66 * t17 * t69 * t26 / 27.0
            + 2.0 / 9.0 * t78 * t80 * t30 * t82
            + 2.0 / 27.0 * t90 * t66 * t92
            - 4.0 / 27.0 * t103 * t107 * t110 * t111
            + 2.0 / 27.0 * t116 * t107 * t110 * t26;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t121 = t43 * t54;
        let t124 = t78 * t33 * t64;
        let t125 = t51 * t26;
        let t126 = t125 * param_c2;
        let t127 = t66 * t126;
        let t128 = t124 * t127;
        let t130 = t88 * t102;
        let t131 = t9 * t109;
        let t133 = t106 * t131 * t111;
        let t134 = t130 * t133;
        let t136 = t88 * t42;
        let t138 = t106 * t131 * t26;
        let t139 = t136 * t138;
        let tv2rhosigma0 = t121 / 72.0 - t128 / 72.0 + t134 / 18.0 - t139 / 36.0;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = t121 / 36.0 - t128 / 36.0 + t134 / 9.0 - t139 / 18.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let tv2rhosigma2 = tv2rhosigma0;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let tv2rhosigma3 = tv2rhosigma2;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = tv2rhosigma1;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let tv2rhosigma5 = tv2rhosigma3;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t145 = t78 * t102;
        let t146 = 1.0 / t16;
        let t147 = t109 * t146;
        let t149 = t107 * t147 * t111;
        let t150 = t145 * t149;
        let t152 = t17 * t16;
        let t153 = 1.0 / t152;
        let t156 = t45 * t14 * t153 * t26;
        let t157 = t50 * t156;
        let t159 = t78 * t42;
        let t161 = t107 * t147 * t26;
        let t162 = t159 * t161;
        let tv2sigma20 = -t150 / 48.0 + t157 / 48.0 + t162 / 96.0;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = -t150 / 24.0 + t157 / 24.0 + t162 / 48.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = tv2sigma20;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = -t150 / 12.0 + t157 / 12.0 + t162 / 24.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = tv2sigma21;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let tv2sigma25 = tv2sigma22;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
        let t170 = param_c1 * t80;
        let t171 = t30 * t82;
        let t175 = t58 * param_c3;
        let t177 = t41 * param_c4 * t8;
        let t178 = t175 * t177;
        let t179 = t10 * t14;
        let t180 = t179 * t17;
        let t189 = param_c3 * t101;
        let t190 = t189 * t104;
        let t191 = t7 * t190;
        let t193 = t105 * t9 * t109;
        let t194 = t67 * t67;
        let t196 = 1.0 / t75 / t194;
        let t197 = t16 * t196;
        let t203 = 1.0 / t2 / t96;
        let t209 = t63 * t104;
        let t210 = t7 * t209;
        let t216 = 1.0 / t96 * param_c1;
        let t217 = t32 * t32;
        let t218 = 1.0 / t217;
        let t220 = t82 * param_c2;
        let t224 = 1.0 / t194;
        let t225 = t224 * param_c1;
        let t227 = t225 * t80 * t64;
        let t232 = t194 * t1;
        let t233 = 1.0 / t232;
        let t234 = t233 * param_c1;
        let t235 = t234 * t33;
        let t237 = t16 * t111;
        let t243 = t16 * t26;
        let t248 = t194 * t67;
        let t249 = 1.0 / t248;
        let t251 = t249 * param_c1 * t39;
        let t252 = t40 * t40;
        let t253 = 1.0 / t252;
        let t254 = t104 * param_c4;
        let t255 = t253 * t254;
        let t256 = 1.0 / t12;
        let t257 = t256 * t152;
        let t258 = t111 * t26;
        let t263 = t101 * t254;
        let t268 = t41 * t254;
        let tv3rho30 = -2.0 / 9.0 * t170 * t171 * t87
            - 5.0 / 27.0 * t178 * t180 * t98 * t26 * param_c2
            - 8.0 / 27.0 * t58 * t59 * t69
            + 16.0 / 27.0 * t191 * t193 * t197 * t111
            + 7.0 / 81.0 * t65 * t66 * t17 * t203 * t26
            - 8.0 / 27.0 * t210 * t193 * t197 * t26
            + 2.0 / 9.0 * t216 * t218 * t30 * t220
            + 2.0 / 27.0 * t227 * t66 * t91 * t82
            - 4.0 / 27.0 * t235 * t190 * t193 * t237 * param_c2
            + 2.0 / 27.0 * t235 * t209 * t193 * t243 * param_c2
            + 16.0 / 27.0 * t251 * t255 * t257 * t258
            - 16.0 / 27.0 * t251 * t263 * t257 * t111
            + 8.0 / 81.0 * t251 * t268 * t257 * t26;
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
        let t274 = t69 * param_c1 * t42;
        let t275 = t274 * t54;
        let t277 = t90 * t127;
        let t279 = t103 * t133;
        let t281 = t116 * t138;
        let t284 = t216 * t80 * t64;
        let t286 = t66 * t125 * t82;
        let t287 = t284 * t286;
        let t289 = t33 * param_c3;
        let t290 = t289 * t101;
        let t292 = t109 * t111;
        let t294 = t107 * t292 * param_c2;
        let t295 = t225 * t290 * t294;
        let t297 = t289 * t41;
        let t299 = t109 * t26;
        let t301 = t107 * t299 * param_c2;
        let t302 = t225 * t297 * t301;
        let t304 = t234 * t39;
        let t305 = t256 * t258;
        let t307 = t255 * t305 * t17;
        let t308 = t304 * t307;
        let t310 = t256 * t111;
        let t312 = t263 * t310 * t17;
        let t313 = t304 * t312;
        let t317 = t268 * t256 * t17 * t26;
        let t318 = t304 * t317;
        let tv3rho2sigma0 = -t275 / 54.0 + t277 / 36.0 - t279 / 6.0 + t281 / 12.0 - t287 / 108.0
            + t295 / 27.0
            - t302 / 54.0
            - 2.0 / 9.0 * t308
            + 2.0 / 9.0 * t313
            - t318 / 27.0;
        v3rho2sigma[ip * 9] += tv3rho2sigma0;
        let tv3rho2sigma1 = -t275 / 27.0 + t277 / 18.0 - t279 / 3.0 + t281 / 6.0 - t287 / 54.0
            + 2.0 / 27.0 * t295
            - t302 / 27.0
            - 4.0 / 9.0 * t308
            + 4.0 / 9.0 * t313
            - 2.0 / 27.0 * t318;
        v3rho2sigma[ip * 9 + 1] += tv3rho2sigma1;
        let tv3rho2sigma2 = tv3rho2sigma0;
        v3rho2sigma[ip * 9 + 2] += tv3rho2sigma2;
        let tv3rho2sigma3 = tv3rho2sigma2;
        v3rho2sigma[ip * 9 + 3] += tv3rho2sigma3;
        let tv3rho2sigma4 = tv3rho2sigma1;
        v3rho2sigma[ip * 9 + 4] += tv3rho2sigma4;
        let tv3rho2sigma5 = tv3rho2sigma3;
        v3rho2sigma[ip * 9 + 5] += tv3rho2sigma5;
        let tv3rho2sigma6 = tv3rho2sigma5;
        v3rho2sigma[ip * 9 + 6] += tv3rho2sigma6;
        let tv3rho2sigma7 = tv3rho2sigma4;
        v3rho2sigma[ip * 9 + 7] += tv3rho2sigma7;
        let tv3rho2sigma8 = tv3rho2sigma6;
        v3rho2sigma[ip * 9 + 8] += tv3rho2sigma8;
        let t330 = t130 * t149;
        let t332 = t216 * t33;
        let t333 = t332 * t190;
        let t334 = t146 * t111;
        let t336 = t193 * t334 * param_c2;
        let t337 = t333 * t336;
        let t339 = t225 * t39;
        let t340 = t256 * t51;
        let t342 = t255 * t340 * t258;
        let t343 = t339 * t342;
        let t346 = t263 * t340 * t111;
        let t347 = t339 * t346;
        let t349 = t43 * t156;
        let t351 = t153 * t26;
        let t352 = t351 * param_c2;
        let t353 = t66 * t352;
        let t354 = t124 * t353;
        let t356 = t136 * t161;
        let t358 = t332 * t209;
        let t359 = t146 * t26;
        let t361 = t193 * t359 * param_c2;
        let t362 = t358 * t361;
        let t365 = t268 * t340 * t26;
        let t366 = t339 * t365;
        let tv3rhosigma20 = t330 / 144.0 - t337 / 144.0 + t343 / 12.0 - t347 / 12.0 - t349 / 144.0
            + t354 / 144.0
            - t356 / 288.0
            + t362 / 288.0
            + t366 / 72.0;
        v3rhosigma2[ip * 12] += tv3rhosigma20;
        let tv3rhosigma21 = t330 / 72.0 - t337 / 72.0 + t343 / 6.0 - t347 / 6.0 - t349 / 72.0
            + t354 / 72.0
            - t356 / 144.0
            + t362 / 144.0
            + t366 / 36.0;
        v3rhosigma2[ip * 12 + 1] += tv3rhosigma21;
        let tv3rhosigma22 = tv3rhosigma20;
        v3rhosigma2[ip * 12 + 2] += tv3rhosigma22;
        let tv3rhosigma23 = t330 / 36.0 - t337 / 36.0 + t343 / 3.0 - t347 / 3.0 - t349 / 36.0
            + t354 / 36.0
            - t356 / 72.0
            + t362 / 72.0
            + t366 / 18.0;
        v3rhosigma2[ip * 12 + 3] += tv3rhosigma23;
        let tv3rhosigma24 = tv3rhosigma21;
        v3rhosigma2[ip * 12 + 4] += tv3rhosigma24;
        let tv3rhosigma25 = tv3rhosigma22;
        v3rhosigma2[ip * 12 + 5] += tv3rhosigma25;
        let tv3rhosigma26 = tv3rhosigma25;
        v3rhosigma2[ip * 12 + 6] += tv3rhosigma26;
        let tv3rhosigma27 = tv3rhosigma24;
        v3rhosigma2[ip * 12 + 7] += tv3rhosigma27;
        let tv3rhosigma28 = tv3rhosigma26;
        v3rhosigma2[ip * 12 + 8] += tv3rhosigma28;
        let tv3rhosigma29 = tv3rhosigma23;
        v3rhosigma2[ip * 12 + 9] += tv3rhosigma29;
        let tv3rhosigma210 = tv3rhosigma27;
        v3rhosigma2[ip * 12 + 10] += tv3rhosigma210;
        let tv3rhosigma211 = tv3rhosigma28;
        v3rhosigma2[ip * 12 + 11] += tv3rhosigma211;
        let t386 = t216 * t39;
        let t387 = t256 * t153;
        let t389 = t255 * t387 * t258;
        let t390 = t386 * t389;
        let t392 = t16 * t16;
        let t393 = 1.0 / t392;
        let t394 = t109 * t393;
        let t396 = t107 * t394 * t111;
        let t397 = t145 * t396;
        let t400 = t263 * t387 * t111;
        let t401 = t386 * t400;
        let t404 = 1.0 / t17 / t392;
        let t407 = t45 * t14 * t404 * t26;
        let t408 = t50 * t407;
        let t411 = t107 * t394 * t26;
        let t412 = t159 * t411;
        let t415 = t268 * t387 * t26;
        let t416 = t386 * t415;
        let tv3sigma30 =
            -t390 / 32.0 + t397 / 32.0 + t401 / 32.0 - t408 / 32.0 - t412 / 64.0 - t416 / 192.0;
        v3sigma3[ip * 10] += tv3sigma30;
        let tv3sigma31 =
            -t390 / 16.0 + t397 / 16.0 + t401 / 16.0 - t408 / 16.0 - t412 / 32.0 - t416 / 96.0;
        v3sigma3[ip * 10 + 1] += tv3sigma31;
        let tv3sigma32 = tv3sigma30;
        v3sigma3[ip * 10 + 2] += tv3sigma32;
        let tv3sigma33 =
            -t390 / 8.0 + t397 / 8.0 + t401 / 8.0 - t408 / 8.0 - t412 / 16.0 - t416 / 48.0;
        v3sigma3[ip * 10 + 3] += tv3sigma33;
        let tv3sigma34 = tv3sigma31;
        v3sigma3[ip * 10 + 4] += tv3sigma34;
        let tv3sigma35 = tv3sigma32;
        v3sigma3[ip * 10 + 5] += tv3sigma35;
        let tv3sigma36 =
            -t390 / 4.0 + t397 / 4.0 + t401 / 4.0 - t408 / 4.0 - t412 / 8.0 - t416 / 24.0;
        v3sigma3[ip * 10 + 6] += tv3sigma36;
        let tv3sigma37 = tv3sigma33;
        v3sigma3[ip * 10 + 7] += tv3sigma37;
        let tv3sigma38 = tv3sigma34;
        v3sigma3[ip * 10 + 8] += tv3sigma38;
        let tv3sigma39 = tv3sigma35;
        v3sigma3[ip * 10 + 9] += tv3sigma39;
    }
}
