//! GGA_C_CCDF lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_ccdf.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT6, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_ccdf_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
    param_c4: f64,
    param_c5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = param_c2 * t2 + 1.0;
        let t5 = 1.0 / t4;
        let t6 = param_c1 * t5;
        let t7 = M_CBRT2;
        let t8 = M_CBRT6;
        let t9 = t8 * t8;
        let t10 = t7 * t9;
        let t11 = M_PI * M_PI;
        let t12 = pow_1_3(t11);
        let t13 = 1.0 / t12;
        let t14 = f64::sqrt(sigma[ip]);
        let t15 = t13 * t14;
        let t17 = 1.0 / t1 / rho[ip];
        let t23 = f64::exp(-param_c4 * (t10 * t15 * t17 / 12.0 - param_c5));
        let t24 = 1.0 + t23;
        let t27 = 1.0 - param_c3 / t24;
        let tzk0 = t6 * t27;
        zk[ip] += tzk0;
        let t28 = t2 * param_c1;
        let t29 = t4 * t4;
        let t30 = 1.0 / t29;
        let t36 = t5 * param_c3;
        let t37 = t24 * t24;
        let t38 = 1.0 / t37;
        let t39 = t36 * t38;
        let t40 = t17 * param_c1 * t39;
        let t42 = param_c4 * t7 * t9;
        let tvrho0 = tzk0 + t28 * t30 * t27 * param_c2 / 3.0 + t40 * t42 * t15 * t23 / 9.0;
        vrho[ip] += tvrho0;
        let t47 = t28 * t39;
        let t48 = 1.0 / t14;
        let t51 = t42 * t13 * t48 * t23;
        let tvsigma0 = -t47 * t51 / 24.0;
        vsigma[ip] += tvsigma0;
        let t54 = param_c1 * t30;
        let t55 = t27 * param_c2;
        let t59 = param_c3 * t38;
        let t60 = t59 * param_c4;
        let t61 = t6 * t60;
        let t62 = t10 * t13;
        let t63 = rho[ip] * rho[ip];
        let t65 = 1.0 / t1 / t63;
        let t71 = t1 * t1;
        let t74 = 1.0 / t71 / rho[ip] * param_c1;
        let t76 = 1.0 / t29 / t4;
        let t78 = param_c2 * param_c2;
        let t83 = 1.0 / t71 / t63;
        let t84 = t83 * param_c1;
        let t86 = t84 * t30 * t60;
        let t87 = t14 * t23;
        let t88 = t87 * param_c2;
        let t92 = t63 * rho[ip];
        let t94 = 1.0 / t71 / t92;
        let t95 = t94 * param_c1;
        let t97 = 1.0 / t37 / t24;
        let t98 = t36 * t97;
        let t99 = t95 * t98;
        let t100 = param_c4 * param_c4;
        let t101 = t7 * t7;
        let t102 = t100 * t101;
        let t103 = t102 * t8;
        let t104 = t12 * t12;
        let t105 = 1.0 / t104;
        let t106 = t105 * sigma[ip];
        let t107 = t23 * t23;
        let t112 = t95 * t39;
        let tv2rho20 = 2.0 / 9.0 * t54 * t55 * t17 - t61 * t62 * t14 * t65 * t23 / 27.0 + 2.0 / 9.0 * t74 * t76 * t27 * t78 + 2.0 / 27.0 * t86 * t62 * t88 - 4.0 / 27.0 * t99 * t103 * t106 * t107 + 2.0 / 27.0 * t112 * t103 * t106 * t23;
        v2rho2[ip] += tv2rho20;
        let t120 = t74 * t30 * t60;
        let t121 = t48 * t23;
        let t122 = t121 * param_c2;
        let t123 = t62 * t122;
        let t126 = t84 * t98;
        let t127 = t8 * t105;
        let t129 = t102 * t127 * t107;
        let t132 = t84 * t39;
        let t134 = t102 * t127 * t23;
        let tv2rhosigma0 = t40 * t51 / 72.0 - t120 * t123 / 72.0 + t126 * t129 / 18.0 - t132 * t134 / 36.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t137 = t74 * t98;
        let t138 = 1.0 / sigma[ip];
        let t139 = t105 * t138;
        let t141 = t103 * t139 * t107;
        let t144 = t14 * sigma[ip];
        let t145 = 1.0 / t144;
        let t148 = t42 * t13 * t145 * t23;
        let t151 = t74 * t39;
        let t153 = t103 * t139 * t23;
        let tv2sigma20 = -t137 * t141 / 48.0 + t47 * t148 / 48.0 + t151 * t153 / 96.0;
        v2sigma2[ip] += tv2sigma20;
        let t156 = param_c1 * t76;
        let t157 = t27 * t78;
        let t161 = t54 * param_c3;
        let t163 = t38 * param_c4 * t7;
        let t164 = t161 * t163;
        let t165 = t9 * t13;
        let t166 = t165 * t14;
        let t175 = param_c3 * t97;
        let t176 = t175 * t100;
        let t177 = t6 * t176;
        let t179 = t101 * t8 * t105;
        let t180 = t63 * t63;
        let t182 = 1.0 / t71 / t180;
        let t183 = sigma[ip] * t182;
        let t189 = 1.0 / t1 / t92;
        let t195 = t59 * t100;
        let t196 = t6 * t195;
        let t202 = 1.0 / t92 * param_c1;
        let t203 = t29 * t29;
        let t204 = 1.0 / t203;
        let t206 = t78 * param_c2;
        let t210 = 1.0 / t180;
        let t211 = t210 * param_c1;
        let t213 = t211 * t76 * t60;
        let t218 = t180 * rho[ip];
        let t219 = 1.0 / t218;
        let t220 = t219 * param_c1;
        let t221 = t220 * t30;
        let t223 = sigma[ip] * t107;
        let t229 = sigma[ip] * t23;
        let t234 = t180 * t63;
        let t235 = 1.0 / t234;
        let t237 = t235 * param_c1 * t36;
        let t238 = t37 * t37;
        let t239 = 1.0 / t238;
        let t240 = t100 * param_c4;
        let t241 = t239 * t240;
        let t242 = 1.0 / t11;
        let t243 = t242 * t144;
        let t244 = t107 * t23;
        let t249 = t97 * t240;
        let t254 = t38 * t240;
        let tv3rho30 = -2.0 / 9.0 * t156 * t157 * t83 - 5.0 / 27.0 * t164 * t166 * t94 * t23 * param_c2 - 8.0 / 27.0 * t54 * t55 * t65 + 16.0 / 27.0 * t177 * t179 * t183 * t107 + 7.0 / 81.0 * t61 * t62 * t14 * t189 * t23 - 8.0 / 27.0 * t196 * t179 * t183 * t23 + 2.0 / 9.0 * t202 * t204 * t27 * t206 + 2.0 / 27.0 * t213 * t62 * t87 * t78 - 4.0 / 27.0 * t221 * t176 * t179 * t223 * param_c2 + 2.0 / 27.0 * t221 * t195 * t179 * t229 * param_c2 + 16.0 / 27.0 * t237 * t241 * t243 * t244 - 16.0 / 27.0 * t237 * t249 * t243 * t107 + 8.0 / 81.0 * t237 * t254 * t243 * t23;
        v3rho3[ip] += tv3rho30;
        let t260 = t65 * param_c1 * t39;
        let t270 = t202 * t76 * t60;
        let t272 = t62 * t121 * t78;
        let t275 = t30 * param_c3;
        let t276 = t275 * t97;
        let t278 = t105 * t107;
        let t280 = t103 * t278 * param_c2;
        let t283 = t275 * t38;
        let t285 = t105 * t23;
        let t287 = t103 * t285 * param_c2;
        let t290 = t220 * t36;
        let t291 = t242 * t244;
        let t293 = t241 * t291 * t14;
        let t296 = t242 * t107;
        let t298 = t249 * t296 * t14;
        let t303 = t254 * t242 * t14 * t23;
        let tv3rho2sigma0 = -t260 * t51 / 54.0 + t86 * t123 / 36.0 - t99 * t129 / 6.0 + t112 * t134 / 12.0 - t270 * t272 / 108.0 + t211 * t276 * t280 / 27.0 - t211 * t283 * t287 / 54.0 - 2.0 / 9.0 * t290 * t293 + 2.0 / 9.0 * t290 * t298 - t290 * t303 / 27.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t308 = t202 * t30;
        let t309 = t308 * t176;
        let t310 = t138 * t107;
        let t312 = t179 * t310 * param_c2;
        let t315 = t211 * t36;
        let t316 = t242 * t48;
        let t318 = t241 * t316 * t244;
        let t322 = t249 * t316 * t107;
        let t327 = t145 * t23;
        let t328 = t327 * param_c2;
        let t329 = t62 * t328;
        let t334 = t308 * t195;
        let t335 = t138 * t23;
        let t337 = t179 * t335 * param_c2;
        let t341 = t254 * t316 * t23;
        let tv3rhosigma20 = t126 * t141 / 144.0 - t309 * t312 / 144.0 + t315 * t318 / 12.0 - t315 * t322 / 12.0 - t40 * t148 / 144.0 + t120 * t329 / 144.0 - t132 * t153 / 288.0 + t334 * t337 / 288.0 + t315 * t341 / 72.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t344 = t202 * t36;
        let t345 = t242 * t145;
        let t347 = t241 * t345 * t244;
        let t350 = sigma[ip] * sigma[ip];
        let t351 = 1.0 / t350;
        let t352 = t105 * t351;
        let t354 = t103 * t352 * t107;
        let t358 = t249 * t345 * t107;
        let t362 = 1.0 / t14 / t350;
        let t365 = t42 * t13 * t362 * t23;
        let t369 = t103 * t352 * t23;
        let t373 = t254 * t345 * t23;
        let tv3sigma30 = -t344 * t347 / 32.0 + t137 * t354 / 32.0 + t344 * t358 / 32.0 - t47 * t365 / 32.0 - t151 * t369 / 64.0 - t344 * t373 / 192.0;
        v3sigma3[ip] += tv3sigma30;
        let t376 = param_c3 * t239;
        let t378 = t240 * t242;
        let t379 = t180 * t92;
        let t381 = t144 / t379;
        let t401 = t127 * sigma[ip];
        let t417 = sigma[ip] / t71 / t218;
        let t423 = 1.0 / t1 / t180;
        let t431 = 1.0 / t1 / t218 * param_c1;
        let t440 = 1.0 / t1 / t234 * param_c1;
        let t441 = t440 * t76;
        let t452 = t180 * t180;
        let t456 = 1.0 / t1 / t452 * param_c1 * t5;
        let t458 = 1.0 / t238 / t24;
        let t460 = t100 * t100;
        let t461 = param_c3 * t458 * t460;
        let t463 = t242 * t350;
        let t464 = t107 * t107;
        let t469 = t376 * t460;
        let t475 = -160.0 / 27.0 * t6 * t376 * t378 * t381 * t244 + 160.0 / 27.0 * t6 * t175 * t378 * t381 * t107 - 4.0 / 9.0 * t156 * param_c3 * t163 * t166 * t219 * t23 * t78 + 32.0 / 27.0 * t161 * t97 * t100 * t101 * t401 * t235 * t107 * param_c2 - 16.0 / 27.0 * t161 * t38 * t100 * t101 * t401 * t235 * t23 * param_c2 - 700.0 / 243.0 * t177 * t179 * t417 * t107 - 70.0 / 243.0 * t61 * t62 * t14 * t423 * t23 + 8.0 / 81.0 * t431 * t204 * t60 * t62 * t87 * t206 - 16.0 / 81.0 * t441 * t176 * t179 * t223 * t78 + 8.0 / 81.0 * t441 * t195 * t179 * t229 * t78 - 64.0 / 243.0 * t456 * t461 * t463 * t464 * t62 + 32.0 / 81.0 * t456 * t469 * t463 * t244 * t62;
        let t476 = t175 * t460;
        let t482 = t59 * t460;
        let t485 = t165 * t23;
        let t506 = 1.0 / t1 / t379 * param_c1;
        let t507 = t275 * t239;
        let t531 = t423 * param_c1;
        let t535 = t78 * t78;
        let t544 = -112.0 / 729.0 * t456 * t476 * t463 * t107 * t62 + 8.0 / 729.0 * t456 * t482 * t463 * t7 * t485 + 32.0 / 81.0 * t156 * t157 * t94 + 56.0 / 81.0 * t54 * t55 * t189 + 350.0 / 243.0 * t196 * t179 * t417 * t23 + 164.0 / 243.0 * t164 * t166 * t182 * t23 * param_c2 + 64.0 / 81.0 * t506 * t507 * t378 * t144 * t244 * param_c2 - 64.0 / 81.0 * t506 * t276 * t378 * t144 * t107 * param_c2 + 32.0 / 243.0 * t506 * t283 * t378 * t144 * t23 * param_c2 - 8.0 / 9.0 * param_c1 * t204 * t27 * t206 * t210 + 8.0 / 27.0 * t531 / t203 / t4 * t27 * t535 - 80.0 / 81.0 * t6 * t59 * t378 * t381 * t23;
        let tv4rho40 = t475 + t544;
        v4rho4[ip] += tv4rho40;
        let t561 = t506 * t5;
        let t590 = t76 * param_c3;
        let t603 = t182 * param_c1;
        let tv4rho3sigma0 = 7.0 / 162.0 * t189 * param_c1 * t39 * t51 - 13.0 / 162.0 * t95 * t30 * t60 * t123 + 5.0 / 108.0 * t213 * t272 - t531 * t204 * t60 * t62 * t121 * t206 / 108.0 + 8.0 / 81.0 * t561 * t461 * t242 * t464 * sigma[ip] * t62 - 4.0 / 27.0 * t561 * t469 * t291 * sigma[ip] * t62 + 14.0 / 243.0 * t561 * t476 * t296 * sigma[ip] * t62 - t561 * t482 * t242 * sigma[ip] * t7 * t485 / 243.0 - 13.0 / 54.0 * t220 * t276 * t280 + 13.0 / 108.0 * t220 * t283 * t287 + t431 * t590 * t97 * t103 * t278 * t78 / 27.0 - t431 * t590 * t38 * t103 * t285 * t78 / 54.0 + 103.0 / 162.0 * t603 * t98 * t129 - 103.0 / 324.0 * t603 * t39 * t134 - 2.0 / 9.0 * t440 * t507 * t378 * t244 * param_c2 * t14 + 2.0 / 9.0 * t440 * t276 * t378 * t107 * param_c2 * t14 - t440 * t283 * t378 * t88 / 27.0 + 16.0 / 9.0 * t237 * t293 - 16.0 / 9.0 * t237 * t298 + 8.0 / 27.0 * t237 * t303;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t634 = t211 * t30;
        let t638 = t531 * t76;
        let t662 = t460 * t242;
        let tv4rho2sigma20 = -t99 * t141 / 108.0 + t634 * t176 * t312 / 72.0 - t638 * t176 * t179 * t310 * t78 / 216.0 + t260 * t148 / 108.0 - t86 * t329 / 72.0 + t270 * t62 * t327 * t78 / 216.0 - t634 * t195 * t337 / 144.0 + t638 * t195 * t179 * t335 * t78 / 432.0 - t440 * t36 * t458 * t662 * t464 * t62 / 27.0 + t440 * t36 * t239 * t662 * t244 * t62 / 18.0 - 7.0 / 324.0 * t440 * t98 * t662 * t107 * t62 + t112 * t153 / 216.0 + t440 * t39 * t662 * t7 * t485 / 648.0 + t431 * t507 * t378 * t48 * t244 * param_c2 / 18.0 - t431 * t276 * t378 * t48 * t107 * param_c2 / 18.0 + t431 * t283 * t378 * t122 / 108.0 - 13.0 / 36.0 * t290 * t318 + 13.0 / 36.0 * t290 * t322 - 13.0 / 216.0 * t290 * t341;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t715 = t431 * t5;
        let t717 = t242 * t138;
        let tv4rhosigma30 = -t315 * t347 / 32.0 - t531 * t507 * t378 * t145 * t244 * param_c2 / 96.0 + t715 * t461 * t717 * t464 * t62 / 72.0 - t715 * t469 * t717 * t244 * t62 / 48.0 - t126 * t354 / 96.0 + t309 * t179 * t351 * t107 * param_c2 / 96.0 + t315 * t358 / 32.0 + t531 * t276 * t378 * t145 * t107 * param_c2 / 96.0 + 7.0 / 864.0 * t715 * t476 * t717 * t107 * t62 + t40 * t365 / 96.0 - t120 * t62 * t362 * t23 * param_c2 / 96.0 + t132 * t369 / 192.0 - t334 * t179 * t351 * t23 * param_c2 / 192.0 - t315 * t373 / 192.0 - t531 * t283 * t378 * t328 / 576.0 - t715 * t482 * t717 * t7 * t485 / 1728.0;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t772 = t531 * t5;
        let t774 = t242 * t351;
        let t779 = t242 * t362;
        let t789 = t350 * sigma[ip];
        let t791 = t105 / t789;
        let tv4sigma40 = -t772 * t461 * t774 * t464 * t62 / 192.0 + 3.0 / 32.0 * t344 * t241 * t779 * t244 + t772 * t469 * t774 * t244 * t62 / 128.0 - 5.0 / 64.0 * t137 * t103 * t791 * t107 - 3.0 / 32.0 * t344 * t249 * t779 * t107 - 7.0 / 2304.0 * t772 * t476 * t774 * t107 * t62 + 5.0 / 64.0 * t47 * t42 * t13 / t14 / t789 * t23 + 5.0 / 128.0 * t151 * t103 * t791 * t23 + t344 * t254 * t779 * t23 / 64.0 + t772 * t482 * t774 * t7 * t485 / 4608.0;
        v4sigma4[ip] += tv4sigma40;
    }
}
