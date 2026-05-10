//! GGA_X_S12 fxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 66 shared lines across all orders.
//! Delta: 131 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_s12_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    param_bx: f64,
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
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = t5 * t25;
        let t27 = pow_1_3(t6);
        let t28 = t27 * param_bx;
        let t29 = param_C * sigma0;
        let t30 = rho0 * rho0;
        let t31 = pow_1_3(rho0);
        let t32 = t31 * t31;
        let t34 = 1.0 / t32 / t30;
        let t36 = sigma0 * sigma0;
        let t37 = param_D * t36;
        let t38 = t30 * t30;
        let t39 = t38 * rho0;
        let t41 = 1.0 / t31 / t39;
        let t43 = t29 * t34 + t37 * t41 + 1.0;
        let t46 = param_B * (1.0 - 1.0 / t43);
        let t47 = param_E * sigma0;
        let t49 = t47 * t34 + 1.0;
        let t51 = 1.0 - 1.0 / t49;
        let t53 = t46 * t51 + param_A;
        let t54 = t28 * t53;
        let t57 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t54);
        let t58 = rho1 <= dens_threshold;
        let t59 = -t16;
        let t61 = piecewise5(t14, t11, t10, t15, t59 * t7);
        let t62 = 1.0 + t61;
        let t63 = t62 <= zeta_threshold;
        let t64 = pow_1_3(t62);
        let t66 = piecewise3(t63, t22, t64 * t62);
        let t67 = t5 * t66;
        let t68 = param_C * sigma2;
        let t69 = rho1 * rho1;
        let t70 = pow_1_3(rho1);
        let t71 = t70 * t70;
        let t73 = 1.0 / t71 / t69;
        let t75 = sigma2 * sigma2;
        let t76 = param_D * t75;
        let t77 = t69 * t69;
        let t78 = t77 * rho1;
        let t80 = 1.0 / t70 / t78;
        let t82 = t68 * t73 + t76 * t80 + 1.0;
        let t85 = param_B * (1.0 - 1.0 / t82);
        let t86 = param_E * sigma2;
        let t88 = t86 * t73 + 1.0;
        let t90 = 1.0 - 1.0 / t88;
        let t92 = t85 * t90 + param_A;
        let t93 = t28 * t92;
        let t96 = piecewise3(t58, 0.0, -3.0 / 8.0 * t67 * t93);
        let tzk0 = t57 + t96;
        zk[ip] += tzk0;
        // --- vxc delta (73 lines) ---
        let t97 = t6 * t6;
        let t98 = 1.0 / t97;
        let t99 = t16 * t98;
        let t101 = piecewise5(t10, 0.0, t14, 0.0, t7 - t99);
        let t104 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t101);
        let t105 = t5 * t104;
        let t108 = t27 * t27;
        let t110 = 1.0 / t108 * param_bx;
        let t111 = t110 * t53;
        let t113 = t26 * t111 / 8.0;
        let t114 = t43 * t43;
        let t116 = param_B / t114;
        let t117 = t30 * rho0;
        let t119 = 1.0 / t32 / t117;
        let t122 = t38 * t30;
        let t124 = 1.0 / t31 / t122;
        let t127 = -8.0 / 3.0 * t29 * t119 - 16.0 / 3.0 * t37 * t124;
        let t128 = t127 * t51;
        let t130 = t49 * t49;
        let t131 = 1.0 / t130;
        let t132 = t46 * t131;
        let t136 = t116 * t128 - 8.0 / 3.0 * t132 * t47 * t119;
        let t137 = t28 * t136;
        let t141 = piecewise3(t1, 0.0, -3.0 / 8.0 * t105 * t54 - t113 - 3.0 / 8.0 * t26 * t137);
        let t142 = t59 * t98;
        let t144 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t142);
        let t147 = piecewise3(t63, 0.0, 4.0 / 3.0 * t64 * t144);
        let t148 = t5 * t147;
        let t151 = t110 * t92;
        let t153 = t67 * t151 / 8.0;
        let t155 = piecewise3(t58, 0.0, -3.0 / 8.0 * t148 * t93 - t153);
        let tvrho0 = t57 + t96 + t6 * (t141 + t155);
        vrho[ip * 2] += tvrho0;
        let t159 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t99);
        let t162 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t159);
        let t163 = t5 * t162;
        let t167 = piecewise3(t1, 0.0, -3.0 / 8.0 * t163 * t54 - t113);
        let t169 = piecewise5(t14, 0.0, t10, 0.0, t7 - t142);
        let t172 = piecewise3(t63, 0.0, 4.0 / 3.0 * t64 * t169);
        let t173 = t5 * t172;
        let t176 = t82 * t82;
        let t178 = param_B / t176;
        let t179 = t69 * rho1;
        let t181 = 1.0 / t71 / t179;
        let t184 = t77 * t69;
        let t186 = 1.0 / t70 / t184;
        let t189 = -8.0 / 3.0 * t68 * t181 - 16.0 / 3.0 * t76 * t186;
        let t190 = t189 * t90;
        let t192 = t88 * t88;
        let t193 = 1.0 / t192;
        let t194 = t85 * t193;
        let t198 = t178 * t190 - 8.0 / 3.0 * t194 * t86 * t181;
        let t199 = t28 * t198;
        let t203 = piecewise3(t58, 0.0, -3.0 / 8.0 * t173 * t93 - t153 - 3.0 / 8.0 * t67 * t199);
        let tvrho1 = t57 + t96 + t6 * (t167 + t203);
        vrho[ip * 2 + 1] += tvrho1;
        let t207 = param_D * sigma0;
        let t210 = 2.0 * t207 * t41 + param_C * t34;
        let t211 = t210 * t51;
        let t213 = t131 * param_E;
        let t214 = t213 * t34;
        let t216 = t116 * t211 + t46 * t214;
        let t217 = t28 * t216;
        let t220 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t217);
        let tvsigma0 = t6 * t220;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t222 = param_D * sigma2;
        let t225 = 2.0 * t222 * t80 + param_C * t73;
        let t226 = t225 * t90;
        let t228 = t193 * param_E;
        let t229 = t228 * t73;
        let t231 = t178 * t226 + t85 * t229;
        let t232 = t28 * t231;
        let t235 = piecewise3(t58, 0.0, -3.0 / 8.0 * t67 * t232);
        let tvsigma2 = t6 * t235;
        vsigma[ip * 3 + 2] += tvsigma2;
        // --- fxc delta (this level) (131 lines) ---
        let t238 = t23 * t23;
        let t239 = 1.0 / t238;
        let t240 = t101 * t101;
        let t243 = t97 * t6;
        let t244 = 1.0 / t243;
        let t245 = t16 * t244;
        let t248 = piecewise5(t10, 0.0, t14, 0.0, -2.0 * t98 + 2.0 * t245);
        let t252 = piecewise3(t20, 0.0, 4.0 / 9.0 * t239 * t240 + 4.0 / 3.0 * t23 * t248);
        let t253 = t5 * t252;
        let t256 = t105 * t111;
        let t262 = 1.0 / t108 / t6 * param_bx;
        let t263 = t262 * t53;
        let t265 = t26 * t263 / 12.0;
        let t266 = t110 * t136;
        let t267 = t26 * t266;
        let t271 = param_B / t114 / t43;
        let t272 = t127 * t127;
        let t273 = t272 * t51;
        let t277 = 1.0 / t32 / t38;
        let t282 = 1.0 / t31 / t38 / t117;
        let t285 = 88.0 / 9.0 * t29 * t277 + 304.0 / 9.0 * t37 * t282;
        let t288 = t116 * t127;
        let t290 = t213 * sigma0 * t119;
        let t294 = 1.0 / t130 / t49;
        let t295 = t46 * t294;
        let t296 = param_E * param_E;
        let t297 = t296 * t36;
        let t304 = -2.0 * t271 * t273 + t116 * t285 * t51 - 16.0 / 3.0 * t288 * t290 - 128.0 / 9.0 * t295 * t297 * t282 + 88.0 / 9.0 * t132 * t47 * t277;
        let t305 = t28 * t304;
        let t309 = piecewise3(t1, 0.0, -3.0 / 8.0 * t253 * t54 - t256 / 4.0 - 3.0 / 4.0 * t105 * t137 + t265 - t267 / 4.0 - 3.0 / 8.0 * t26 * t305);
        let t310 = t64 * t64;
        let t311 = 1.0 / t310;
        let t312 = t144 * t144;
        let t315 = t59 * t244;
        let t318 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t98 + 2.0 * t315);
        let t322 = piecewise3(t63, 0.0, 4.0 / 9.0 * t311 * t312 + 4.0 / 3.0 * t64 * t318);
        let t323 = t5 * t322;
        let t326 = t148 * t151;
        let t328 = t262 * t92;
        let t330 = t67 * t328 / 12.0;
        let t332 = piecewise3(t58, 0.0, -3.0 / 8.0 * t323 * t93 - t326 / 4.0 + t330);
        let tv2rho20 = 2.0 * t141 + 2.0 * t155 + t6 * (t309 + t332);
        v2rho2[ip * 3] += tv2rho20;
        let t335 = t239 * t159;
        let t339 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t245);
        let t343 = piecewise3(t20, 0.0, 4.0 / 9.0 * t335 * t101 + 4.0 / 3.0 * t23 * t339);
        let t344 = t5 * t343;
        let t347 = t163 * t111;
        let t354 = piecewise3(t1, 0.0, -3.0 / 8.0 * t344 * t54 - t347 / 8.0 - 3.0 / 8.0 * t163 * t137 - t256 / 8.0 + t265 - t267 / 8.0);
        let t355 = t311 * t169;
        let t359 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t315);
        let t363 = piecewise3(t63, 0.0, 4.0 / 9.0 * t355 * t144 + 4.0 / 3.0 * t64 * t359);
        let t364 = t5 * t363;
        let t367 = t173 * t151;
        let t372 = t110 * t198;
        let t373 = t67 * t372;
        let t376 = piecewise3(t58, 0.0, -3.0 / 8.0 * t364 * t93 - t367 / 8.0 - t326 / 8.0 + t330 - 3.0 / 8.0 * t148 * t199 - t373 / 8.0);
        let tv2rho21 = t141 + t155 + t167 + t203 + t6 * (t354 + t376);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t381 = t159 * t159;
        let t386 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t98 + 2.0 * t245);
        let t390 = piecewise3(t20, 0.0, 4.0 / 9.0 * t239 * t381 + 4.0 / 3.0 * t23 * t386);
        let t391 = t5 * t390;
        let t396 = piecewise3(t1, 0.0, -3.0 / 8.0 * t391 * t54 - t347 / 4.0 + t265);
        let t397 = t169 * t169;
        let t402 = piecewise5(t14, 0.0, t10, 0.0, -2.0 * t98 + 2.0 * t315);
        let t406 = piecewise3(t63, 0.0, 4.0 / 9.0 * t311 * t397 + 4.0 / 3.0 * t64 * t402);
        let t407 = t5 * t406;
        let t416 = param_B / t176 / t82;
        let t417 = t189 * t189;
        let t418 = t417 * t90;
        let t422 = 1.0 / t71 / t77;
        let t427 = 1.0 / t70 / t77 / t179;
        let t430 = 88.0 / 9.0 * t68 * t422 + 304.0 / 9.0 * t76 * t427;
        let t433 = t178 * t189;
        let t435 = t228 * sigma2 * t181;
        let t439 = 1.0 / t192 / t88;
        let t440 = t85 * t439;
        let t441 = t296 * t75;
        let t448 = -2.0 * t416 * t418 + t178 * t430 * t90 - 16.0 / 3.0 * t433 * t435 - 128.0 / 9.0 * t440 * t441 * t427 + 88.0 / 9.0 * t194 * t86 * t422;
        let t449 = t28 * t448;
        let t453 = piecewise3(t58, 0.0, -3.0 / 8.0 * t407 * t93 - t367 / 4.0 - 3.0 / 4.0 * t173 * t199 + t330 - t373 / 4.0 - 3.0 / 8.0 * t67 * t449);
        let tv2rho22 = 2.0 * t167 + 2.0 * t203 + t6 * (t396 + t453);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t458 = t110 * t216;
        let t460 = t26 * t458 / 8.0;
        let t468 = -8.0 / 3.0 * param_C * t119 - 32.0 / 3.0 * t207 * t124;
        let t469 = t468 * t51;
        let t471 = t116 * t210;
        let t475 = t296 * t124;
        let t479 = t213 * t119;
        let t482 = -2.0 * t271 * t211 * t127 + t116 * t469 - 8.0 / 3.0 * t471 * t290 + t288 * t214 + 16.0 / 3.0 * t295 * t475 * sigma0 - 8.0 / 3.0 * t46 * t479;
        let t483 = t28 * t482;
        let t487 = piecewise3(t1, 0.0, -3.0 / 8.0 * t105 * t217 - t460 - 3.0 / 8.0 * t26 * t483);
        let tv2rhosigma0 = t6 * t487 + t220;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t491 = t110 * t231;
        let t493 = t67 * t491 / 8.0;
        let t495 = piecewise3(t58, 0.0, -3.0 / 8.0 * t148 * t232 - t493);
        let tv2rhosigma2 = t6 * t495 + t235;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t500 = piecewise3(t1, 0.0, -3.0 / 8.0 * t163 * t217 - t460);
        let tv2rhosigma3 = t6 * t500 + t220;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t511 = -8.0 / 3.0 * param_C * t181 - 32.0 / 3.0 * t222 * t186;
        let t512 = t511 * t90;
        let t514 = t178 * t225;
        let t518 = t296 * t186;
        let t522 = t228 * t181;
        let t525 = -2.0 * t416 * t226 * t189 + t178 * t512 - 8.0 / 3.0 * t514 * t435 + t433 * t229 + 16.0 / 3.0 * t440 * t518 * sigma2 - 8.0 / 3.0 * t85 * t522;
        let t526 = t28 * t525;
        let t530 = piecewise3(t58, 0.0, -3.0 / 8.0 * t173 * t232 - t493 - 3.0 / 8.0 * t67 * t526);
        let tv2rhosigma5 = t6 * t530 + t235;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t532 = t210 * t210;
        let t533 = t532 * t51;
        let t536 = param_D * t41 * t51;
        let t539 = t294 * t296;
        let t540 = t539 * t41;
        let t543 = 2.0 * t116 * t536 + 2.0 * t471 * t214 - 2.0 * t271 * t533 - 2.0 * t46 * t540;
        let t544 = t28 * t543;
        let t547 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t544);
        let tv2sigma20 = t6 * t547;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t548 = t225 * t225;
        let t549 = t548 * t90;
        let t552 = param_D * t80 * t90;
        let t555 = t439 * t296;
        let t556 = t555 * t80;
        let t559 = 2.0 * t178 * t552 + 2.0 * t514 * t229 - 2.0 * t416 * t549 - 2.0 * t85 * t556;
        let t560 = t28 * t559;
        let t563 = piecewise3(t58, 0.0, -3.0 / 8.0 * t67 * t560);
        let tv2sigma25 = t6 * t563;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
