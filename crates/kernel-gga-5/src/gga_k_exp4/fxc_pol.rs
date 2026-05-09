//! GGA_K_EXP4 fxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 66 shared lines across all orders.
//! Delta: 110 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_exp4_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
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
        // --- shared preamble (66 lines) ---
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
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t36 = 1.0 / t35;
        let t37 = t32 * t36;
        let t38 = rho0 * rho0;
        let t39 = pow_1_3(rho0);
        let t40 = t39 * t39;
        let t42 = 1.0 / t40 / t38;
        let t46 = f64::exp(-0.83254166666666666664e1 * t37 * sigma0 * t42);
        let t48 = t32 * t32;
        let t51 = t48 / t34 / t33;
        let t52 = sigma0 * sigma0;
        let t53 = t38 * t38;
        let t54 = t53 * rho0;
        let t56 = 1.0 / t39 / t54;
        let t60 = f64::exp(-0.75479166666666666666e-2 * t51 * t52 * t56);
        let t62 = 0.20788e1 - 0.8524e0 * t46 - 0.12264e1 * t60;
        let t66 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t62);
        let t67 = rho1 <= dens_threshold;
        let t68 = -t17;
        let t70 = piecewise5(t15, t12, t11, t16, t68 * t8);
        let t71 = 1.0 + t70;
        let t72 = t71 <= zeta_threshold;
        let t73 = pow_1_3(t71);
        let t74 = t73 * t73;
        let t76 = piecewise3(t72, t24, t74 * t71);
        let t77 = t76 * t30;
        let t78 = rho1 * rho1;
        let t79 = pow_1_3(rho1);
        let t80 = t79 * t79;
        let t82 = 1.0 / t80 / t78;
        let t86 = f64::exp(-0.83254166666666666664e1 * t37 * sigma2 * t82);
        let t88 = sigma2 * sigma2;
        let t89 = t78 * t78;
        let t90 = t89 * rho1;
        let t92 = 1.0 / t79 / t90;
        let t96 = f64::exp(-0.75479166666666666666e-2 * t51 * t88 * t92);
        let t98 = 0.20788e1 - 0.8524e0 * t86 - 0.12264e1 * t96;
        let t102 = piecewise3(t67, 0.0, 3.0 / 20.0 * t6 * t77 * t98);
        let tzk0 = t66 + t102;
        zk[ip] += tzk0;
        // --- vxc delta (44 lines) ---
        let t103 = t7 * t7;
        let t104 = 1.0 / t103;
        let t105 = t17 * t104;
        let t107 = piecewise5(t11, 0.0, t15, 0.0, t8 - t105);
        let t110 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t107);
        let t111 = t110 * t30;
        let t115 = 1.0 / t29;
        let t116 = t28 * t115;
        let t119 = t6 * t116 * t62 / 10.0;
        let t120 = t38 * rho0;
        let t122 = 1.0 / t40 / t120;
        let t127 = t53 * t38;
        let t129 = 1.0 / t39 / t127;
        let t134 = -0.1892422711111111111e2 * t37 * sigma0 * t122 * t46 - 0.49369413333333333333e-1 * t51 * t52 * t129 * t60;
        let t139 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t111 * t62 + t119 + 3.0 / 20.0 * t6 * t31 * t134);
        let t140 = t68 * t104;
        let t142 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t140);
        let t145 = piecewise3(t72, 0.0, 5.0 / 3.0 * t74 * t142);
        let t146 = t145 * t30;
        let t150 = t76 * t115;
        let t153 = t6 * t150 * t98 / 10.0;
        let t155 = piecewise3(t67, 0.0, 3.0 / 20.0 * t6 * t146 * t98 + t153);
        let tvrho0 = t66 + t102 + t7 * (t139 + t155);
        vrho[ip * 2] += tvrho0;
        let t159 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t105);
        let t162 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t159);
        let t163 = t162 * t30;
        let t168 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t163 * t62 + t119);
        let t170 = piecewise5(t15, 0.0, t11, 0.0, t8 - t140);
        let t173 = piecewise3(t72, 0.0, 5.0 / 3.0 * t74 * t170);
        let t174 = t173 * t30;
        let t178 = t78 * rho1;
        let t180 = 1.0 / t80 / t178;
        let t185 = t89 * t78;
        let t187 = 1.0 / t79 / t185;
        let t192 = -0.1892422711111111111e2 * t37 * sigma2 * t180 * t86 - 0.49369413333333333333e-1 * t51 * t88 * t187 * t96;
        let t197 = piecewise3(t67, 0.0, 3.0 / 20.0 * t6 * t174 * t98 + t153 + 3.0 / 20.0 * t6 * t77 * t192);
        let tvrho1 = t66 + t102 + t7 * (t168 + t197);
        vrho[ip * 2 + 1] += tvrho1;
        let t207 = 0.70965851666666666664e1 * t37 * t42 * t46 + 0.1851353e-1 * t51 * sigma0 * t56 * t60;
        let t211 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t207);
        let tvsigma0 = t7 * t211;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t219 = 0.70965851666666666664e1 * t37 * t82 * t86 + 0.1851353e-1 * t51 * sigma2 * t92 * t96;
        let t223 = piecewise3(t67, 0.0, 3.0 / 20.0 * t6 * t77 * t219);
        let tvsigma2 = t7 * t223;
        vsigma[ip * 3 + 2] += tvsigma2;
        // --- fxc delta (this level) (110 lines) ---
        let t226 = 1.0 / t25;
        let t227 = t107 * t107;
        let t230 = t103 * t7;
        let t231 = 1.0 / t230;
        let t232 = t17 * t231;
        let t235 = piecewise5(t11, 0.0, t15, 0.0, -2.0 * t104 + 2.0 * t232);
        let t239 = piecewise3(t21, 0.0, 10.0 / 9.0 * t226 * t227 + 5.0 / 3.0 * t26 * t235);
        let t240 = t239 * t30;
        let t244 = t110 * t115;
        let t246 = t6 * t244 * t62;
        let t252 = 1.0 / t29 / t7;
        let t253 = t28 * t252;
        let t256 = t6 * t253 * t62 / 30.0;
        let t258 = t6 * t116 * t134;
        let t261 = 1.0 / t40 / t53;
        let t266 = t53 * t120;
        let t268 = 1.0 / t39 / t266;
        let t269 = t52 * t268;
        let t276 = t33 * t33;
        let t279 = t32 / t35 / t276;
        let t280 = t52 * t52;
        let t281 = t53 * t53;
        let t282 = t281 * t53;
        let t284 = 1.0 / t40 / t282;
        let t289 = 0.69388832740740740737e2 * t37 * sigma0 * t261 * t46 - 0.4201388687856790123e3 * t51 * t269 * t46 + 0.31267295111111111111e0 * t51 * t269 * t60 - 0.11924358967111111111e-1 * t279 * t280 * t284 * t60;
        let t294 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t240 * t62 + t246 / 5.0 + 3.0 / 10.0 * t6 * t111 * t134 - t256 + t258 / 5.0 + 3.0 / 20.0 * t6 * t31 * t289);
        let t295 = 1.0 / t73;
        let t296 = t142 * t142;
        let t299 = t68 * t231;
        let t302 = piecewise5(t15, 0.0, t11, 0.0, 2.0 * t104 + 2.0 * t299);
        let t306 = piecewise3(t72, 0.0, 10.0 / 9.0 * t295 * t296 + 5.0 / 3.0 * t74 * t302);
        let t307 = t306 * t30;
        let t311 = t145 * t115;
        let t313 = t6 * t311 * t98;
        let t315 = t76 * t252;
        let t318 = t6 * t315 * t98 / 30.0;
        let t320 = piecewise3(t67, 0.0, 3.0 / 20.0 * t6 * t307 * t98 + t313 / 5.0 - t318);
        let tv2rho20 = 2.0 * t139 + 2.0 * t155 + t7 * (t294 + t320);
        v2rho2[ip * 3] += tv2rho20;
        let t323 = t226 * t159;
        let t327 = piecewise5(t11, 0.0, t15, 0.0, 2.0 * t232);
        let t331 = piecewise3(t21, 0.0, 10.0 / 9.0 * t323 * t107 + 5.0 / 3.0 * t26 * t327);
        let t332 = t331 * t30;
        let t336 = t162 * t115;
        let t338 = t6 * t336 * t62;
        let t346 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t332 * t62 + t338 / 10.0 + 3.0 / 20.0 * t6 * t163 * t134 + t246 / 10.0 - t256 + t258 / 10.0);
        let t347 = t295 * t170;
        let t351 = piecewise5(t15, 0.0, t11, 0.0, 2.0 * t299);
        let t355 = piecewise3(t72, 0.0, 10.0 / 9.0 * t347 * t142 + 5.0 / 3.0 * t74 * t351);
        let t356 = t355 * t30;
        let t360 = t173 * t115;
        let t362 = t6 * t360 * t98;
        let t369 = t6 * t150 * t192;
        let t372 = piecewise3(t67, 0.0, 3.0 / 20.0 * t6 * t356 * t98 + t362 / 10.0 + t313 / 10.0 - t318 + 3.0 / 20.0 * t6 * t146 * t192 + t369 / 10.0);
        let tv2rho21 = t139 + t155 + t168 + t197 + t7 * (t346 + t372);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t377 = t159 * t159;
        let t382 = piecewise5(t11, 0.0, t15, 0.0, 2.0 * t104 + 2.0 * t232);
        let t386 = piecewise3(t21, 0.0, 10.0 / 9.0 * t226 * t377 + 5.0 / 3.0 * t26 * t382);
        let t387 = t386 * t30;
        let t393 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t387 * t62 + t338 / 5.0 - t256);
        let t394 = t170 * t170;
        let t399 = piecewise5(t15, 0.0, t11, 0.0, -2.0 * t104 + 2.0 * t299);
        let t403 = piecewise3(t72, 0.0, 10.0 / 9.0 * t295 * t394 + 5.0 / 3.0 * t74 * t399);
        let t404 = t403 * t30;
        let t414 = 1.0 / t80 / t89;
        let t419 = t89 * t178;
        let t421 = 1.0 / t79 / t419;
        let t422 = t88 * t421;
        let t429 = t88 * t88;
        let t430 = t89 * t89;
        let t431 = t430 * t89;
        let t433 = 1.0 / t80 / t431;
        let t438 = 0.69388832740740740737e2 * t37 * sigma2 * t414 * t86 - 0.4201388687856790123e3 * t51 * t422 * t86 + 0.31267295111111111111e0 * t51 * t422 * t96 - 0.11924358967111111111e-1 * t279 * t429 * t433 * t96;
        let t443 = piecewise3(t67, 0.0, 3.0 / 20.0 * t6 * t404 * t98 + t362 / 5.0 + 3.0 / 10.0 * t6 * t174 * t192 - t318 + t369 / 5.0 + 3.0 / 20.0 * t6 * t77 * t438);
        let tv2rho22 = 2.0 * t168 + 2.0 * t197 + t7 * (t393 + t443);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t451 = t6 * t116 * t207 / 10.0;
        let t455 = t129 * sigma0;
        let t462 = t52 * sigma0;
        let t463 = t281 * t120;
        let t465 = 1.0 / t40 / t463;
        let t470 = -0.1892422711111111111e2 * t37 * t122 * t46 + 0.15755207579462962962e3 * t51 * t455 * t46 - 0.98738826666666666667e-1 * t51 * t455 * t60 + 0.44716346126666666666e-2 * t279 * t462 * t465 * t60;
        let t475 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t111 * t207 + t451 + 3.0 / 20.0 * t6 * t31 * t470);
        let tv2rhosigma0 = t7 * t475 + t211;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t482 = t6 * t150 * t219 / 10.0;
        let t484 = piecewise3(t67, 0.0, 3.0 / 20.0 * t6 * t146 * t219 + t482);
        let tv2rhosigma2 = t7 * t484 + t223;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t490 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t163 * t207 + t451);
        let tv2rhosigma3 = t7 * t490 + t211;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t498 = t187 * sigma2;
        let t505 = t88 * sigma2;
        let t506 = t430 * t178;
        let t508 = 1.0 / t80 / t506;
        let t513 = -0.1892422711111111111e2 * t37 * t180 * t86 + 0.15755207579462962962e3 * t51 * t498 * t86 - 0.98738826666666666667e-1 * t51 * t498 * t96 + 0.44716346126666666666e-2 * t279 * t505 * t508 * t96;
        let t518 = piecewise3(t67, 0.0, 3.0 / 20.0 * t6 * t174 * t219 + t482 + 3.0 / 20.0 * t6 * t77 * t513);
        let tv2rhosigma5 = t7 * t518 + t223;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t526 = t281 * t38;
        let t528 = 1.0 / t40 / t526;
        let t533 = -0.59082028422986111107e2 * t51 * t56 * t46 + 0.1851353e-1 * t51 * t56 * t60 - 0.167686297975e-2 * t279 * t52 * t528 * t60;
        let t537 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t533);
        let tv2sigma20 = t7 * t537;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t544 = t430 * t78;
        let t546 = 1.0 / t80 / t544;
        let t551 = -0.59082028422986111107e2 * t51 * t92 * t86 + 0.1851353e-1 * t51 * t92 * t96 - 0.167686297975e-2 * t279 * t88 * t546 * t96;
        let t555 = piecewise3(t67, 0.0, 3.0 / 20.0 * t6 * t77 * t551);
        let tv2sigma25 = t7 * t555;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
