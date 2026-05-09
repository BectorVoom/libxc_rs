//! GGA_K_DK lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 55 shared lines across all orders.
//! Delta: 32 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_dk_lxc_unpol(
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
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    param_aa_0: f64,
    param_aa_1: f64,
    param_aa_2: f64,
    param_aa_3: f64,
    param_aa_4: f64,
    param_bb_0: f64,
    param_bb_1: f64,
    param_bb_2: f64,
    param_bb_3: f64,
    param_bb_4: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (55 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = t7 * t20;
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t25 = param_aa_1;
        let t26 = t25 * sigma[ip];
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t23 / t29;
        let t32 = t28 * t31;
        let t34 = param_aa_2;
        let t35 = sigma[ip] * sigma[ip];
        let t36 = t34 * t35;
        let t37 = t29 * t29;
        let t38 = t37 * rho[ip];
        let t40 = 1.0 / t22 / t38;
        let t41 = t27 * t40;
        let t44 = param_aa_3;
        let t45 = t35 * sigma[ip];
        let t46 = t44 * t45;
        let t47 = t37 * t37;
        let t48 = 1.0 / t47;
        let t51 = param_aa_4;
        let t52 = t35 * t35;
        let t53 = t51 * t52;
        let t54 = t47 * t29;
        let t57 = t28 / t23 / t54;
        let t60 = t26 * t32 + 2.0 * t36 * t41 + 4.0 * t46 * t48 + 4.0 * t53 * t57 + param_aa_0;
        let t61 = t23 * t60;
        let t63 = param_bb_1;
        let t64 = t63 * sigma[ip];
        let t66 = param_bb_2;
        let t67 = t66 * t35;
        let t70 = param_bb_3;
        let t71 = t70 * t45;
        let t74 = param_bb_4;
        let t75 = t74 * t52;
        let t78 = t64 * t32 + 2.0 * t67 * t41 + 4.0 * t71 * t48 + 4.0 * t75 * t57 + param_bb_0;
        let t79 = 1.0 / t78;
        let t83 = piecewise3(t2, 0.0, 3.0 / 20.0 * t21 * t61 * t79);
        let tzk0 = 2.0 * t83;
        zk[ip] += tzk0;
        // --- vxc delta (34 lines) ---
        let t84 = 1.0 / t22;
        let t85 = t84 * t60;
        let t89 = t29 * rho[ip];
        let t91 = 1.0 / t23 / t89;
        let t92 = t28 * t91;
        let t95 = t37 * t29;
        let t97 = 1.0 / t22 / t95;
        let t98 = t27 * t97;
        let t101 = t47 * rho[ip];
        let t102 = 1.0 / t101;
        let t105 = t47 * t89;
        let t108 = t28 / t23 / t105;
        let t111 = -8.0 / 3.0 * t26 * t92 - 32.0 / 3.0 * t36 * t98 - 32.0 * t46 * t102 - 128.0 / 3.0 * t53 * t108;
        let t112 = t23 * t111;
        let t116 = t78 * t78;
        let t117 = 1.0 / t116;
        let t126 = -8.0 / 3.0 * t64 * t92 - 32.0 / 3.0 * t67 * t98 - 32.0 * t71 * t102 - 128.0 / 3.0 * t75 * t108;
        let t127 = t117 * t126;
        let t132 = piecewise3(t2, 0.0, t21 * t85 * t79 / 10.0 + 3.0 / 20.0 * t21 * t112 * t79 - 3.0 / 20.0 * t21 * t61 * t127);
        let tvrho0 = 2.0 * rho[ip] * t132 + 2.0 * t83;
        vrho[ip] += tvrho0;
        let t135 = t25 * t28;
        let t137 = t34 * sigma[ip];
        let t140 = t44 * t35;
        let t143 = t51 * t45;
        let t146 = t135 * t31 + 4.0 * t137 * t41 + 12.0 * t140 * t48 + 16.0 * t143 * t57;
        let t147 = t23 * t146;
        let t150 = t63 * t28;
        let t152 = t66 * sigma[ip];
        let t155 = t70 * t35;
        let t158 = t74 * t45;
        let t161 = t150 * t31 + 4.0 * t152 * t41 + 12.0 * t155 * t48 + 16.0 * t158 * t57;
        let t162 = t117 * t161;
        let t167 = piecewise3(t2, 0.0, 3.0 / 20.0 * t21 * t147 * t79 - 3.0 / 20.0 * t21 * t61 * t162);
        let tvsigma0 = 2.0 * rho[ip] * t167;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (44 lines) ---
        let t171 = 1.0 / t22 / rho[ip];
        let t172 = t171 * t60;
        let t176 = t84 * t111;
        let t184 = 1.0 / t23 / t37;
        let t185 = t28 * t184;
        let t190 = 1.0 / t22 / t37 / t89;
        let t191 = t27 * t190;
        let t194 = 1.0 / t54;
        let t197 = t47 * t37;
        let t200 = t28 / t23 / t197;
        let t203 = 88.0 / 9.0 * t26 * t185 + 608.0 / 9.0 * t36 * t191 + 288.0 * t46 * t194 + 4480.0 / 9.0 * t53 * t200;
        let t204 = t23 * t203;
        let t212 = 1.0 / t116 / t78;
        let t213 = t126 * t126;
        let t214 = t212 * t213;
        let t226 = 88.0 / 9.0 * t64 * t185 + 608.0 / 9.0 * t67 * t191 + 288.0 * t71 * t194 + 4480.0 / 9.0 * t75 * t200;
        let t227 = t117 * t226;
        let t232 = piecewise3(t2, 0.0, -t21 * t172 * t79 / 30.0 + t21 * t176 * t79 / 5.0 - t21 * t85 * t127 / 5.0 + 3.0 / 20.0 * t21 * t204 * t79 - 3.0 / 10.0 * t21 * t112 * t127 + 3.0 / 10.0 * t21 * t61 * t214 - 3.0 / 20.0 * t21 * t61 * t227);
        let tv2rho20 = 2.0 * rho[ip] * t232 + 4.0 * t132;
        v2rho2[ip] += tv2rho20;
        let t235 = t84 * t146;
        let t247 = -8.0 / 3.0 * t135 * t91 - 64.0 / 3.0 * t137 * t98 - 96.0 * t140 * t102 - 512.0 / 3.0 * t143 * t108;
        let t248 = t23 * t247;
        let t262 = t7 * t20 * t23;
        let t263 = t60 * t212;
        let t264 = t161 * t126;
        let t265 = t263 * t264;
        let t276 = -8.0 / 3.0 * t150 * t91 - 64.0 / 3.0 * t152 * t98 - 96.0 * t155 * t102 - 512.0 / 3.0 * t158 * t108;
        let t277 = t117 * t276;
        let t282 = piecewise3(t2, 0.0, t21 * t235 * t79 / 10.0 + 3.0 / 20.0 * t21 * t248 * t79 - 3.0 / 20.0 * t21 * t147 * t127 - t21 * t85 * t162 / 10.0 - 3.0 / 20.0 * t21 * t112 * t162 + 3.0 / 10.0 * t262 * t265 - 3.0 / 20.0 * t21 * t61 * t277);
        let tv2rhosigma0 = 2.0 * rho[ip] * t282 + 2.0 * t167;
        v2rhosigma[ip] += tv2rhosigma0;
        let t285 = t34 * t27;
        let t288 = t44 * sigma[ip];
        let t291 = t51 * t35;
        let t294 = 4.0 * t285 * t40 + 24.0 * t288 * t48 + 48.0 * t291 * t57;
        let t295 = t23 * t294;
        let t302 = t161 * t161;
        let t303 = t212 * t302;
        let t307 = t66 * t27;
        let t310 = t70 * sigma[ip];
        let t313 = t74 * t35;
        let t316 = 4.0 * t307 * t40 + 24.0 * t310 * t48 + 48.0 * t313 * t57;
        let t317 = t117 * t316;
        let t322 = piecewise3(t2, 0.0, 3.0 / 20.0 * t21 * t295 * t79 - 3.0 / 10.0 * t21 * t147 * t162 + 3.0 / 10.0 * t21 * t61 * t303 - 3.0 / 20.0 * t21 * t61 * t317);
        let tv2sigma20 = 2.0 * rho[ip] * t322;
        v2sigma2[ip] += tv2sigma20;
        // --- kxc delta (69 lines) ---
        let t335 = 1.0 / t23 / t38;
        let t336 = t28 * t335;
        let t341 = t27 / t22 / t47;
        let t344 = 1.0 / t105;
        let t350 = t28 / t23 / t47 / t38;
        let t353 = -1232.0 / 27.0 * t64 * t336 - 13376.0 / 27.0 * t67 * t341 - 2880.0 * t71 * t344 - 170240.0 / 27.0 * t75 * t350;
        let t354 = t117 * t353;
        let t359 = 1.0 / t22 / t29;
        let t360 = t359 * t60;
        let t376 = t116 * t116;
        let t377 = 1.0 / t376;
        let t378 = t213 * t126;
        let t379 = t377 * t378;
        let t383 = t126 * t226;
        let t384 = t263 * t383;
        let t387 = t171 * t111;
        let t391 = t84 * t203;
        let t403 = -1232.0 / 27.0 * t26 * t336 - 13376.0 / 27.0 * t36 * t341 - 2880.0 * t46 * t344 - 170240.0 / 27.0 * t53 * t350;
        let t404 = t23 * t403;
        let t408 = -3.0 / 10.0 * t21 * t85 * t227 - 9.0 / 20.0 * t21 * t204 * t127 - 9.0 / 20.0 * t21 * t112 * t227 - 3.0 / 20.0 * t21 * t61 * t354 + 2.0 / 45.0 * t21 * t360 * t79 + t21 * t172 * t127 / 10.0 - 3.0 / 5.0 * t21 * t176 * t127 + 3.0 / 5.0 * t21 * t85 * t214 + 9.0 / 10.0 * t21 * t112 * t214 - 9.0 / 10.0 * t21 * t61 * t379 + 9.0 / 10.0 * t262 * t384 - t21 * t387 * t79 / 10.0 + 3.0 / 10.0 * t21 * t391 * t79 + 3.0 / 20.0 * t21 * t404 * t79;
        let t409 = piecewise3(t2, 0.0, t408);
        let tv3rho30 = 2.0 * rho[ip] * t409 + 6.0 * t232;
        v3rho3[ip] += tv3rho30;
        let t413 = t171 * t146;
        let t446 = 88.0 / 9.0 * t150 * t184 + 1216.0 / 9.0 * t152 * t191 + 864.0 * t155 * t194 + 17920.0 / 9.0 * t158 * t200;
        let t447 = t117 * t446;
        let t458 = t7 * t20 * t84;
        let t461 = t111 * t212;
        let t462 = t461 * t264;
        let t465 = t276 * t126;
        let t466 = t263 * t465;
        let t469 = t161 * t226;
        let t470 = t263 * t469;
        let t473 = t84 * t247;
        let t485 = 88.0 / 9.0 * t135 * t184 + 1216.0 / 9.0 * t137 * t191 + 864.0 * t140 * t194 + 17920.0 / 9.0 * t143 * t200;
        let t486 = t23 * t485;
        let t490 = t60 * t377;
        let t491 = t161 * t213;
        let t492 = t490 * t491;
        let t495 = -t21 * t413 * t79 / 30.0 - t21 * t235 * t127 / 5.0 - 3.0 / 10.0 * t21 * t248 * t127 - 3.0 / 20.0 * t21 * t147 * t227 - t21 * t176 * t162 / 5.0 - t21 * t85 * t277 / 5.0 - 3.0 / 20.0 * t21 * t204 * t162 - 3.0 / 10.0 * t21 * t112 * t277 - 3.0 / 20.0 * t21 * t61 * t447 + 3.0 / 10.0 * t21 * t147 * t214 + t21 * t172 * t162 / 30.0 + 2.0 / 5.0 * t458 * t265 + 3.0 / 5.0 * t262 * t462 + 3.0 / 5.0 * t262 * t466 + 3.0 / 10.0 * t262 * t470 + t21 * t473 * t79 / 5.0 + 3.0 / 20.0 * t21 * t486 * t79 - 9.0 / 10.0 * t262 * t492;
        let t496 = piecewise3(t2, 0.0, t495);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t496 + 4.0 * t282;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t499 = t84 * t294;
        let t509 = -64.0 / 3.0 * t285 * t97 - 192.0 * t288 * t102 - 512.0 * t291 * t108;
        let t510 = t23 * t509;
        let t523 = t146 * t212;
        let t524 = t523 * t264;
        let t536 = t302 * t126;
        let t537 = t490 * t536;
        let t540 = t161 * t276;
        let t541 = t263 * t540;
        let t550 = t316 * t126;
        let t551 = t263 * t550;
        let t560 = -64.0 / 3.0 * t307 * t97 - 192.0 * t310 * t102 - 512.0 * t313 * t108;
        let t561 = t117 * t560;
        let t565 = t21 * t499 * t79 / 10.0 + 3.0 / 20.0 * t21 * t510 * t79 - 3.0 / 20.0 * t21 * t295 * t127 - t21 * t235 * t162 / 5.0 - 3.0 / 10.0 * t21 * t248 * t162 + 3.0 / 5.0 * t262 * t524 - 3.0 / 10.0 * t21 * t147 * t277 + t21 * t85 * t303 / 5.0 + 3.0 / 10.0 * t21 * t112 * t303 - 9.0 / 10.0 * t262 * t537 + 3.0 / 5.0 * t262 * t541 - t21 * t85 * t317 / 10.0 - 3.0 / 20.0 * t21 * t112 * t317 + 3.0 / 10.0 * t262 * t551 - 3.0 / 20.0 * t21 * t61 * t561;
        let t566 = piecewise3(t2, 0.0, t565);
        let tv3rhosigma20 = 2.0 * rho[ip] * t566 + 2.0 * t322;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t571 = t51 * sigma[ip];
        let t574 = 24.0 * t44 * t48 + 96.0 * t571 * t57;
        let t575 = t23 * t574;
        let t588 = t302 * t161;
        let t589 = t377 * t588;
        let t593 = t161 * t316;
        let t594 = t263 * t593;
        let t599 = t74 * sigma[ip];
        let t602 = 24.0 * t70 * t48 + 96.0 * t599 * t57;
        let t603 = t117 * t602;
        let t608 = piecewise3(t2, 0.0, 3.0 / 20.0 * t21 * t575 * t79 - 9.0 / 20.0 * t21 * t295 * t162 + 9.0 / 10.0 * t21 * t147 * t303 - 9.0 / 20.0 * t21 * t147 * t317 - 9.0 / 10.0 * t21 * t61 * t589 + 9.0 / 10.0 * t262 * t594 - 3.0 / 20.0 * t21 * t61 * t603);
        let tv3sigma30 = 2.0 * rho[ip] * t608;
        v3sigma3[ip] += tv3sigma30;
        // --- lxc delta (this level) (32 lines) ---
        let t625 = t28 / t23 / t95;
        let t630 = t27 / t22 / t101;
        let t633 = 1.0 / t197;
        let t639 = t28 / t23 / t47 / t95;
        let t666 = t226 * t226;
        let t680 = 8.0 / 45.0 * t21 * t359 * t111 * t79 - t21 * t171 * t203 * t79 / 5.0 + 2.0 / 5.0 * t21 * t84 * t403 * t79 + 3.0 / 20.0 * t21 * t23 * (20944.0 / 81.0 * t26 * t625 + 334400.0 / 81.0 * t36 * t630 + 31680.0 * t46 * t633 + 6979840.0 / 81.0 * t53 * t639) * t79 - 14.0 / 135.0 * t21 / t22 / t89 * t60 * t79 + 18.0 / 5.0 * t262 * t461 * t383 + 6.0 / 5.0 * t262 * t263 * t353 * t126 - 27.0 / 5.0 * t262 * t490 * t213 * t226 + 12.0 / 5.0 * t458 * t384 + 9.0 / 10.0 * t21 * t61 * t212 * t666 - 2.0 / 5.0 * t21 * t172 * t214 + 12.0 / 5.0 * t21 * t176 * t214 - 12.0 / 5.0 * t21 * t85 * t379;
        let t685 = 1.0 / t376 / t78;
        let t686 = t213 * t213;
        let t734 = -18.0 / 5.0 * t21 * t112 * t379 + 18.0 / 5.0 * t21 * t61 * t685 * t686 + t21 * t172 * t227 / 5.0 - 6.0 / 5.0 * t21 * t391 * t127 + 9.0 / 5.0 * t21 * t204 * t214 - 6.0 / 5.0 * t21 * t176 * t227 - 2.0 / 5.0 * t21 * t85 * t354 - 3.0 / 5.0 * t21 * t404 * t127 - 9.0 / 10.0 * t21 * t204 * t227 - 3.0 / 5.0 * t21 * t112 * t354 - 3.0 / 20.0 * t21 * t61 * t117 * (20944.0 / 81.0 * t64 * t625 + 334400.0 / 81.0 * t67 * t630 + 31680.0 * t71 * t633 + 6979840.0 / 81.0 * t75 * t639) - 8.0 / 45.0 * t21 * t360 * t127 + 2.0 / 5.0 * t21 * t387 * t127;
        let t736 = piecewise3(t2, 0.0, t680 + t734);
        let tv4rho40 = 2.0 * rho[ip] * t736 + 8.0 * t409;
        v4rho4[ip] += tv4rho40;
        let t742 = t111 * t377;
        let t750 = t60 * t685;
        let t815 = -9.0 / 5.0 * t458 * t492 - 27.0 / 10.0 * t262 * t742 * t491 - 27.0 / 10.0 * t262 * t490 * t276 * t213 + 18.0 / 5.0 * t262 * t750 * t161 * t378 + 3.0 / 10.0 * t262 * t263 * t161 * t353 + 9.0 / 10.0 * t262 * t263 * t446 * t126 - t7 * t20 * t171 * t265 / 5.0 + 3.0 / 5.0 * t458 * t470 + 9.0 / 10.0 * t262 * t461 * t469 + 9.0 / 10.0 * t262 * t263 * t276 * t226 + 9.0 / 10.0 * t262 * t523 * t383 + 6.0 / 5.0 * t458 * t462 + 6.0 / 5.0 * t458 * t466 + 9.0 / 10.0 * t262 * t203 * t212 * t264 + 9.0 / 5.0 * t262 * t461 * t465 - t21 * t171 * t247 * t79 / 10.0 + 3.0 / 10.0 * t21 * t84 * t485 * t79 + 3.0 / 20.0 * t21 * t23 * (-1232.0 / 27.0 * t135 * t335 - 26752.0 / 27.0 * t137 * t341 - 8640.0 * t140 * t344 - 680960.0 / 27.0 * t143 * t350) * t79 + 2.0 / 45.0 * t21 * t359 * t146 * t79;
        let t887 = -9.0 / 10.0 * t21 * t147 * t379 - 2.0 / 45.0 * t21 * t360 * t162 + 3.0 / 5.0 * t21 * t235 * t214 + 9.0 / 10.0 * t21 * t248 * t214 + t21 * t387 * t162 / 10.0 + t21 * t172 * t277 / 10.0 - 3.0 / 20.0 * t21 * t404 * t162 - 9.0 / 20.0 * t21 * t204 * t277 - 9.0 / 20.0 * t21 * t112 * t447 - 3.0 / 20.0 * t21 * t61 * t117 * (-1232.0 / 27.0 * t150 * t335 - 26752.0 / 27.0 * t152 * t341 - 8640.0 * t155 * t344 - 680960.0 / 27.0 * t158 * t350) + t21 * t413 * t127 / 10.0 - 3.0 / 5.0 * t21 * t473 * t127 - 3.0 / 10.0 * t21 * t235 * t227 - 9.0 / 20.0 * t21 * t486 * t127 - 9.0 / 20.0 * t21 * t248 * t227 - 3.0 / 20.0 * t21 * t147 * t354 - 3.0 / 10.0 * t21 * t391 * t162 - 3.0 / 5.0 * t21 * t176 * t277 - 3.0 / 10.0 * t21 * t85 * t447 - 27.0 / 10.0 * t262 * t490 * t469 * t126;
        let t889 = piecewise3(t2, 0.0, t815 + t887);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t889 + 6.0 * t496;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t895 = t146 * t377;
        let t967 = 4.0 / 5.0 * t458 * t541 - 9.0 / 5.0 * t262 * t895 * t491 + 18.0 / 5.0 * t262 * t750 * t302 * t213 - 9.0 / 10.0 * t262 * t490 * t316 * t213 - 9.0 / 5.0 * t262 * t742 * t536 + 6.0 / 5.0 * t262 * t461 * t540 - 9.0 / 10.0 * t262 * t490 * t302 * t226 + 3.0 / 5.0 * t262 * t263 * t161 * t446 + 2.0 / 5.0 * t458 * t551 + 3.0 / 5.0 * t262 * t461 * t550 + 3.0 / 5.0 * t262 * t263 * t560 * t126 + 3.0 / 10.0 * t262 * t263 * t316 * t226 + 4.0 / 5.0 * t458 * t524 + 6.0 / 5.0 * t262 * t247 * t212 * t264 + 6.0 / 5.0 * t262 * t523 * t465 + 3.0 / 5.0 * t262 * t523 * t469 - 6.0 / 5.0 * t458 * t537 + t21 * t84 * t509 * t79 / 5.0 + 3.0 / 20.0 * t21 * t23 * (1216.0 / 9.0 * t285 * t190 + 1728.0 * t288 * t194 + 17920.0 / 3.0 * t291 * t200) * t79 - t21 * t171 * t294 * t79 / 30.0;
        let t1025 = t276 * t276;
        let t1043 = 3.0 / 10.0 * t21 * t204 * t303 - t21 * t176 * t317 / 5.0 - t21 * t85 * t561 / 5.0 - 3.0 / 20.0 * t21 * t204 * t317 - 3.0 / 10.0 * t21 * t112 * t561 - 3.0 / 20.0 * t21 * t61 * t117 * (1216.0 / 9.0 * t307 * t190 + 1728.0 * t310 * t194 + 17920.0 / 3.0 * t313 * t200) + 3.0 / 5.0 * t21 * t61 * t212 * t1025 + t21 * t172 * t317 / 30.0 + 3.0 / 10.0 * t21 * t295 * t214 + t21 * t413 * t162 / 15.0 - 18.0 / 5.0 * t262 * t490 * t264 * t276;
        let t1046 = piecewise3(t2, 0.0, t967 - t21 * t172 * t303 / 15.0 - t21 * t499 * t127 / 5.0 - 3.0 / 10.0 * t21 * t510 * t127 - 3.0 / 20.0 * t21 * t295 * t227 - 2.0 / 5.0 * t21 * t473 * t162 - 2.0 / 5.0 * t21 * t235 * t277 - 3.0 / 10.0 * t21 * t486 * t162 - 3.0 / 5.0 * t21 * t248 * t277 - 3.0 / 10.0 * t21 * t147 * t447 + 2.0 / 5.0 * t21 * t176 * t303 + t1043);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t1046 + 4.0 * t566;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t1103 = 3.0 / 10.0 * t262 * t263 * t602 * t126 + 9.0 / 10.0 * t262 * t294 * t212 * t264 - 27.0 / 10.0 * t262 * t895 * t536 + 9.0 / 5.0 * t262 * t523 * t540 + 3.0 / 5.0 * t458 * t594 + 9.0 / 10.0 * t262 * t523 * t550 + 18.0 / 5.0 * t262 * t750 * t588 * t126 - 27.0 / 10.0 * t262 * t490 * t302 * t276 + 9.0 / 10.0 * t262 * t461 * t593 + 9.0 / 10.0 * t262 * t263 * t276 * t316 + 9.0 / 10.0 * t262 * t263 * t161 * t560 + 3.0 / 20.0 * t21 * t23 * (-192.0 * t44 * t102 - 1024.0 * t571 * t108) * t79 + t21 * t84 * t574 * t79 / 10.0 - 9.0 / 20.0 * t21 * t147 * t561;
        let t1153 = -9.0 / 10.0 * t21 * t112 * t589 - 3.0 / 20.0 * t21 * t112 * t603 - 3.0 / 20.0 * t21 * t61 * t117 * (-192.0 * t70 * t102 - 1024.0 * t599 * t108) - 3.0 / 20.0 * t21 * t575 * t127 - 9.0 / 20.0 * t21 * t510 * t162 - 9.0 / 20.0 * t21 * t295 * t277 + 9.0 / 10.0 * t21 * t248 * t303 - 9.0 / 20.0 * t21 * t248 * t317 - 3.0 / 10.0 * t21 * t235 * t317 - 3.0 / 5.0 * t21 * t85 * t589 - t21 * t85 * t603 / 10.0 - 3.0 / 10.0 * t21 * t499 * t162 + 3.0 / 5.0 * t21 * t235 * t303 - 27.0 / 10.0 * t262 * t490 * t593 * t126;
        let t1155 = piecewise3(t2, 0.0, t1103 + t1153);
        let tv4rhosigma30 = 2.0 * rho[ip] * t1155 + 2.0 * t608;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t1181 = t302 * t302;
        let t1190 = t316 * t316;
        let t1206 = 72.0 / 5.0 * t21 * t194 * t51 * t28 * t79 - 3.0 / 5.0 * t21 * t575 * t162 + 9.0 / 5.0 * t21 * t295 * t303 - 9.0 / 10.0 * t21 * t295 * t317 - 18.0 / 5.0 * t21 * t147 * t589 + 18.0 / 5.0 * t262 * t523 * t593 - 3.0 / 5.0 * t21 * t147 * t603 + 18.0 / 5.0 * t21 * t61 * t685 * t1181 - 27.0 / 5.0 * t262 * t490 * t302 * t316 + 9.0 / 10.0 * t21 * t61 * t212 * t1190 + 6.0 / 5.0 * t262 * t263 * t161 * t602 - 72.0 / 5.0 * t7 * t20 * t194 * t60 * t117 * t74 * t28;
        let t1207 = piecewise3(t2, 0.0, t1206);
        let tv4sigma40 = 2.0 * rho[ip] * t1207;
        v4sigma4[ip] += tv4sigma40;
    }
}
