//! HYB_MGGA_X_M05 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_m05.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_mgga_x_m05_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_csi_HF: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_a_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
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
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = t6 * t26;
        let t28 = pow_1_3(t7);
        let t29 = t28 * param_csi_HF;
        let t30 = M_CBRT6;
        let t31 = M_PI * M_PI;
        let t32 = pow_1_3(t31);
        let t33 = t32 * t32;
        let t34 = 1.0 / t33;
        let t35 = t30 * t34;
        let t36 = rho0 * rho0;
        let t37 = pow_1_3(rho0);
        let t38 = t37 * t37;
        let t40 = 1.0 / t38 / t36;
        let t44 = 0.804 + 0.009146457198521547 * t35 * sigma0 * t40;
        let t47 = 1.804 - 0.646416 / t44;
        let t48 = param_a_0;
        let t49 = param_a_1;
        let t50 = t30 * t30;
        let t52 = 3.0 / 10.0 * t50 * t33;
        let t54 = 1.0 / t38 / rho0;
        let t55 = tau0 * t54;
        let t56 = t52 - t55;
        let t57 = t49 * t56;
        let t58 = t52 + t55;
        let t59 = 1.0 / t58;
        let t61 = param_a_2;
        let t62 = t56 * t56;
        let t63 = t61 * t62;
        let t64 = t58 * t58;
        let t65 = 1.0 / t64;
        let t67 = param_a_3;
        let t68 = t62 * t56;
        let t69 = t67 * t68;
        let t70 = t64 * t58;
        let t71 = 1.0 / t70;
        let t73 = param_a_4;
        let t74 = t62 * t62;
        let t75 = t73 * t74;
        let t76 = t64 * t64;
        let t77 = 1.0 / t76;
        let t79 = param_a_5;
        let t80 = t74 * t56;
        let t81 = t79 * t80;
        let t82 = t76 * t58;
        let t83 = 1.0 / t82;
        let t85 = param_a_6;
        let t86 = t74 * t62;
        let t87 = t85 * t86;
        let t88 = t76 * t64;
        let t89 = 1.0 / t88;
        let t91 = param_a_7;
        let t92 = t74 * t68;
        let t93 = t91 * t92;
        let t94 = t76 * t70;
        let t95 = 1.0 / t94;
        let t97 = param_a_8;
        let t98 = t74 * t74;
        let t99 = t97 * t98;
        let t100 = t76 * t76;
        let t101 = 1.0 / t100;
        let t103 = param_a_9;
        let t104 = t98 * t56;
        let t105 = t103 * t104;
        let t107 = 1.0 / t100 / t58;
        let t109 = param_a_10;
        let t110 = t98 * t62;
        let t111 = t109 * t110;
        let t113 = 1.0 / t100 / t64;
        let t115 = param_a_11;
        let t117 = t115 * t98 * t68;
        let t119 = 1.0 / t100 / t70;
        let t121 = t99 * t101 + t105 * t107 + t111 * t113 + t117 * t119 + t57 * t59 + t63 * t65 + t69 * t71 + t75 * t77 + t81 * t83 + t87 * t89 + t93 * t95 + t48;
        let t122 = t47 * t121;
        let t123 = t29 * t122;
        let t126 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t123);
        let t127 = rho1 <= dens_threshold;
        let t128 = -t17;
        let t130 = piecewise5(t15, t12, t11, t16, t128 * t8);
        let t131 = 1.0 + t130;
        let t132 = t131 <= zeta_threshold;
        let t133 = pow_1_3(t131);
        let t135 = piecewise3(t132, t23, t133 * t131);
        let t136 = t6 * t135;
        let t137 = rho1 * rho1;
        let t138 = pow_1_3(rho1);
        let t139 = t138 * t138;
        let t141 = 1.0 / t139 / t137;
        let t145 = 0.804 + 0.009146457198521547 * t35 * sigma2 * t141;
        let t148 = 1.804 - 0.646416 / t145;
        let t150 = 1.0 / t139 / rho1;
        let t151 = tau1 * t150;
        let t152 = t52 - t151;
        let t153 = t49 * t152;
        let t154 = t52 + t151;
        let t155 = 1.0 / t154;
        let t157 = t152 * t152;
        let t158 = t61 * t157;
        let t159 = t154 * t154;
        let t160 = 1.0 / t159;
        let t162 = t157 * t152;
        let t163 = t67 * t162;
        let t164 = t159 * t154;
        let t165 = 1.0 / t164;
        let t167 = t157 * t157;
        let t168 = t73 * t167;
        let t169 = t159 * t159;
        let t170 = 1.0 / t169;
        let t172 = t167 * t152;
        let t173 = t79 * t172;
        let t174 = t169 * t154;
        let t175 = 1.0 / t174;
        let t177 = t167 * t157;
        let t178 = t85 * t177;
        let t179 = t169 * t159;
        let t180 = 1.0 / t179;
        let t182 = t167 * t162;
        let t183 = t91 * t182;
        let t184 = t169 * t164;
        let t185 = 1.0 / t184;
        let t187 = t167 * t167;
        let t188 = t97 * t187;
        let t189 = t169 * t169;
        let t190 = 1.0 / t189;
        let t192 = t187 * t152;
        let t193 = t103 * t192;
        let t195 = 1.0 / t189 / t154;
        let t197 = t187 * t157;
        let t198 = t109 * t197;
        let t200 = 1.0 / t189 / t159;
        let t203 = t115 * t187 * t162;
        let t205 = 1.0 / t189 / t164;
        let t207 = t153 * t155 + t158 * t160 + t163 * t165 + t168 * t170 + t173 * t175 + t178 * t180 + t183 * t185 + t188 * t190 + t193 * t195 + t198 * t200 + t203 * t205 + t48;
        let t208 = t148 * t207;
        let t209 = t29 * t208;
        let t212 = piecewise3(t127, 0.0, -3.0 / 8.0 * t136 * t209);
        let tzk0 = t126 + t212;
        zk[ip] += tzk0;
        let t213 = t7 * t7;
        let t214 = 1.0 / t213;
        let t215 = t17 * t214;
        let t217 = piecewise5(t11, 0.0, t15, 0.0, t8 - t215);
        let t220 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t217);
        let t221 = t6 * t220;
        let t224 = t28 * t28;
        let t225 = 1.0 / t224;
        let t226 = t225 * param_csi_HF;
        let t227 = t226 * t122;
        let t229 = t27 * t227 / 8.0;
        let t230 = t3 * t26;
        let t231 = t44 * t44;
        let t232 = 1.0 / t231;
        let t233 = t29 * t232;
        let t234 = t230 * t233;
        let t235 = t36 * rho0;
        let t237 = 1.0 / t38 / t235;
        let t238 = sigma0 * t237;
        let t240 = t35 * t238 * t121;
        let t243 = t49 * tau0;
        let t247 = t65 * tau0;
        let t248 = t247 * t40;
        let t251 = t61 * t56;
        let t254 = t71 * tau0;
        let t255 = t254 * t40;
        let t258 = t67 * t62;
        let t261 = t77 * tau0;
        let t262 = t261 * t40;
        let t265 = t73 * t68;
        let t268 = t83 * tau0;
        let t269 = t268 * t40;
        let t272 = t79 * t74;
        let t275 = t89 * tau0;
        let t276 = t275 * t40;
        let t279 = t85 * t80;
        let t282 = 5.0 / 3.0 * t243 * t40 * t59 + 5.0 / 3.0 * t57 * t248 + 10.0 / 3.0 * t251 * t248 + 10.0 / 3.0 * t63 * t255 + 5.0 * t258 * t255 + 5.0 * t69 * t262 + 20.0 / 3.0 * t265 * t262 + 20.0 / 3.0 * t75 * t269 + 25.0 / 3.0 * t272 * t269 + 25.0 / 3.0 * t81 * t276 + 10.0 * t279 * t276;
        let t283 = t95 * tau0;
        let t284 = t283 * t40;
        let t287 = t91 * t86;
        let t290 = t101 * tau0;
        let t291 = t290 * t40;
        let t294 = t97 * t92;
        let t297 = t107 * tau0;
        let t298 = t297 * t40;
        let t301 = t103 * t98;
        let t304 = t113 * tau0;
        let t305 = t304 * t40;
        let t308 = t109 * t104;
        let t311 = t119 * tau0;
        let t312 = t311 * t40;
        let t315 = t115 * t110;
        let t319 = 1.0 / t100 / t76;
        let t320 = t319 * tau0;
        let t324 = 10.0 * t87 * t284 + 35.0 / 3.0 * t287 * t284 + 35.0 / 3.0 * t93 * t291 + 40.0 / 3.0 * t294 * t291 + 40.0 / 3.0 * t99 * t298 + 15.0 * t301 * t298 + 15.0 * t105 * t305 + 50.0 / 3.0 * t308 * t305 + 50.0 / 3.0 * t111 * t312 + 55.0 / 3.0 * t315 * t312 + 55.0 / 3.0 * t117 * t320 * t40;
        let t325 = t282 + t324;
        let t326 = t47 * t325;
        let t327 = t29 * t326;
        let t331 = piecewise3(t2, 0.0, -3.0 / 8.0 * t221 * t123 - t229 + 0.0040369036088841095 * t234 * t240 - 3.0 / 8.0 * t27 * t327);
        let t332 = t128 * t214;
        let t334 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t332);
        let t337 = piecewise3(t132, 0.0, 4.0 / 3.0 * t133 * t334);
        let t338 = t6 * t337;
        let t341 = t226 * t208;
        let t343 = t136 * t341 / 8.0;
        let t345 = piecewise3(t127, 0.0, -3.0 / 8.0 * t338 * t209 - t343);
        let tvrho0 = t126 + t212 + t7 * (t331 + t345);
        vrho[ip * 2] += tvrho0;
        let t349 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t215);
        let t352 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t349);
        let t353 = t6 * t352;
        let t357 = piecewise3(t2, 0.0, -3.0 / 8.0 * t353 * t123 - t229);
        let t359 = piecewise5(t15, 0.0, t11, 0.0, t8 - t332);
        let t362 = piecewise3(t132, 0.0, 4.0 / 3.0 * t133 * t359);
        let t363 = t6 * t362;
        let t366 = t3 * t135;
        let t367 = t145 * t145;
        let t368 = 1.0 / t367;
        let t369 = t29 * t368;
        let t370 = t366 * t369;
        let t371 = t137 * rho1;
        let t373 = 1.0 / t139 / t371;
        let t374 = sigma2 * t373;
        let t376 = t35 * t374 * t207;
        let t379 = t49 * tau1;
        let t383 = t160 * tau1;
        let t384 = t383 * t141;
        let t387 = t61 * t152;
        let t390 = t165 * tau1;
        let t391 = t390 * t141;
        let t394 = t67 * t157;
        let t397 = t170 * tau1;
        let t398 = t397 * t141;
        let t401 = t73 * t162;
        let t404 = t175 * tau1;
        let t405 = t404 * t141;
        let t408 = t79 * t167;
        let t411 = t180 * tau1;
        let t412 = t411 * t141;
        let t415 = t85 * t172;
        let t418 = 5.0 / 3.0 * t379 * t141 * t155 + 5.0 / 3.0 * t153 * t384 + 10.0 / 3.0 * t387 * t384 + 10.0 / 3.0 * t158 * t391 + 5.0 * t394 * t391 + 5.0 * t163 * t398 + 20.0 / 3.0 * t401 * t398 + 20.0 / 3.0 * t168 * t405 + 25.0 / 3.0 * t408 * t405 + 25.0 / 3.0 * t173 * t412 + 10.0 * t415 * t412;
        let t419 = t185 * tau1;
        let t420 = t419 * t141;
        let t423 = t91 * t177;
        let t426 = t190 * tau1;
        let t427 = t426 * t141;
        let t430 = t97 * t182;
        let t433 = t195 * tau1;
        let t434 = t433 * t141;
        let t437 = t103 * t187;
        let t440 = t200 * tau1;
        let t441 = t440 * t141;
        let t444 = t109 * t192;
        let t447 = t205 * tau1;
        let t448 = t447 * t141;
        let t451 = t115 * t197;
        let t455 = 1.0 / t189 / t169;
        let t456 = t455 * tau1;
        let t460 = 10.0 * t178 * t420 + 35.0 / 3.0 * t423 * t420 + 35.0 / 3.0 * t183 * t427 + 40.0 / 3.0 * t430 * t427 + 40.0 / 3.0 * t188 * t434 + 15.0 * t437 * t434 + 15.0 * t193 * t441 + 50.0 / 3.0 * t444 * t441 + 50.0 / 3.0 * t198 * t448 + 55.0 / 3.0 * t451 * t448 + 55.0 / 3.0 * t203 * t456 * t141;
        let t461 = t418 + t460;
        let t462 = t148 * t461;
        let t463 = t29 * t462;
        let t467 = piecewise3(t127, 0.0, -3.0 / 8.0 * t363 * t209 - t343 + 0.0040369036088841095 * t370 * t376 - 3.0 / 8.0 * t136 * t463);
        let tvrho1 = t126 + t212 + t7 * (t357 + t467);
        vrho[ip * 2 + 1] += tvrho1;
        let t470 = t230 * t29;
        let t471 = t232 * t30;
        let t472 = t34 * t40;
        let t474 = t471 * t472 * t121;
        let t477 = piecewise3(t2, 0.0, -0.0015138388533315413 * t470 * t474);
        let tvsigma0 = t7 * t477;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t478 = t366 * t29;
        let t479 = t368 * t30;
        let t480 = t34 * t141;
        let t482 = t479 * t480 * t207;
        let t485 = piecewise3(t127, 0.0, -0.0015138388533315413 * t478 * t482);
        let tvsigma2 = t7 * t485;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t488 = t65 * t54;
        let t492 = t71 * t54;
        let t497 = t77 * t54;
        let t502 = t83 * t54;
        let t507 = t89 * t54;
        let t512 = -t49 * t54 * t59 - 2.0 * t251 * t488 - 3.0 * t258 * t492 - 4.0 * t265 * t497 - 5.0 * t272 * t502 - 6.0 * t279 * t507 - t57 * t488 - 2.0 * t63 * t492 - 3.0 * t69 * t497 - 4.0 * t75 * t502 - 5.0 * t81 * t507;
        let t513 = t95 * t54;
        let t518 = t101 * t54;
        let t523 = t107 * t54;
        let t528 = t113 * t54;
        let t533 = t119 * t54;
        let t541 = -11.0 * t117 * t319 * t54 - 9.0 * t105 * t528 - 10.0 * t111 * t533 - 7.0 * t287 * t513 - 8.0 * t294 * t518 - 9.0 * t301 * t523 - 10.0 * t308 * t528 - 11.0 * t315 * t533 - 6.0 * t87 * t513 - 7.0 * t93 * t518 - 8.0 * t99 * t523;
        let t542 = t512 + t541;
        let t543 = t47 * t542;
        let t544 = t29 * t543;
        let t547 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t544);
        let tvtau0 = t7 * t547;
        vtau[ip * 2] += tvtau0;
        let t550 = t160 * t150;
        let t554 = t165 * t150;
        let t559 = t170 * t150;
        let t564 = t175 * t150;
        let t569 = t180 * t150;
        let t574 = -t49 * t150 * t155 - t153 * t550 - 2.0 * t158 * t554 - 3.0 * t163 * t559 - 4.0 * t168 * t564 - 5.0 * t173 * t569 - 2.0 * t387 * t550 - 3.0 * t394 * t554 - 4.0 * t401 * t559 - 5.0 * t408 * t564 - 6.0 * t415 * t569;
        let t575 = t185 * t150;
        let t580 = t190 * t150;
        let t585 = t195 * t150;
        let t590 = t200 * t150;
        let t595 = t205 * t150;
        let t603 = -11.0 * t203 * t455 * t150 - 6.0 * t178 * t575 - 7.0 * t183 * t580 - 8.0 * t188 * t585 - 9.0 * t193 * t590 - 10.0 * t198 * t595 - 7.0 * t423 * t575 - 8.0 * t430 * t580 - 9.0 * t437 * t585 - 10.0 * t444 * t590 - 11.0 * t451 * t595;
        let t604 = t574 + t603;
        let t605 = t148 * t604;
        let t606 = t29 * t605;
        let t609 = piecewise3(t127, 0.0, -3.0 / 8.0 * t136 * t606);
        let tvtau1 = t7 * t609;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
