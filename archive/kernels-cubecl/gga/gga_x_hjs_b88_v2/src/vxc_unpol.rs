//! GGA_X_HJS_B88_V2 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hjs_b88_v2.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_hjs_b88_v2_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t12 = t11 <= zeta_threshold;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t12, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3::<f64>(rho[ip]);
        let t19 = t17 * t18;
        let t20 = t3 * t3;
        let t21 = param_hyb_omega_0 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3::<f64>(t22);
        let t24 = 1.0 / t23;
        let t25 = t21 * t24;
        let t26 = piecewise3::<f64>(t12, t13, t15);
        let t27 = 1.0 / t26;
        let t28 = 1.0 / t18;
        let t29 = t27 * t28;
        let t30 = M_CBRT6;
        let t31 = t30 * t30;
        let t32 = t31 * t24;
        let t33 = f64::sqrt(sigma[ip]);
        let t34 = M_CBRT2;
        let t35 = t33 * t34;
        let t37 = 1.0 / t18 / rho[ip];
        let t41 = f64::exp(-t32 * t35 * t37 / 12.0);
        let t42 = f64::exp(20.0);
        let t44 = 1.0 / (t42 - 1.0);
        let t45 = t41 + t44;
        let t49 = f64::ln(t45 / (1.0 + t44));
        let t50 = t49 * t49;
        let t51 = param_a_0;
        let t53 = param_a_1;
        let t54 = t50 * t49;
        let t56 = param_a_2;
        let t57 = t50 * t50;
        let t59 = param_a_3;
        let t60 = t57 * t49;
        let t62 = param_a_4;
        let t63 = t57 * t50;
        let t65 = param_a_5;
        let t66 = t57 * t54;
        let t68 = t50 * t51 - t53 * t54 + t56 * t57 - t59 * t60 + t62 * t63 - t65 * t66;
        let t69 = t50 * t68;
        let t70 = param_b_0;
        let t72 = param_b_1;
        let t74 = param_b_2;
        let t76 = param_b_3;
        let t78 = param_b_4;
        let t80 = param_b_5;
        let t82 = param_b_6;
        let t84 = param_b_7;
        let t85 = t57 * t57;
        let t87 = param_b_8;
        let t90 = -t49 * t85 * t87 - t49 * t70 + t50 * t72 - t54 * t74 + t57 * t76 - t60 * t78 + t63 * t80 - t66 * t82 + t84 * t85 + 1.0;
        let t91 = 1.0 / t90;
        let t92 = t69 * t91;
        let t93 = 0.1e-9 < t92;
        let t94 = piecewise3::<f64>(t93, t92, 0.1e-9);
        let t95 = param_hyb_omega_0 * param_hyb_omega_0;
        let t96 = t95 * t3;
        let t97 = t23 * t23;
        let t98 = 1.0 / t97;
        let t99 = t26 * t26;
        let t101 = t98 / t99;
        let t102 = t18 * t18;
        let t103 = 1.0 / t102;
        let t105 = t96 * t101 * t103;
        let t107 = 0.60965e0 + t94 + t105 / 3.0;
        let t108 = f64::sqrt(t107);
        let t109 = 1.0 / t108;
        let t111 = t25 * t29 * t109;
        let t113 = 1.0 - t111 / 3.0;
        let t114 = 0.60965e0 + t94;
        let t115 = 1.0 / t114;
        let t119 = 1.0 + t50 / 4.0;
        let t120 = 1.0 / t119;
        let t124 = 1.0 + 0.31215633538451261314e0 * t50 * t120 + 0.42141105276909202774e1 * t94;
        let t126 = 1.0 / t22;
        let t127 = t95 * param_hyb_omega_0 * t126;
        let t129 = 1.0 / t99 / t26;
        let t130 = 1.0 / rho[ip];
        let t131 = t129 * t130;
        let t133 = 1.0 / t108 / t107;
        let t135 = t127 * t131 * t133;
        let t137 = 2.0 - t111 + t135 / 3.0;
        let t138 = t124 * t137;
        let t139 = t114 * t114;
        let t140 = 1.0 / t139;
        let t146 = t139 * t114;
        let t148 = f64::sqrt(t114);
        let t149 = t148 * t146;
        let t150 = f64::sqrt(M_PI);
        let t152 = f64::sqrt(t94);
        let t155 = 0.0 < 0.7572109999e0 + t94;
        let t157 = piecewise3::<f64>(t155, 0.757211e0 + t94, 0.1e-9);
        let t158 = f64::sqrt(t157);
        let t160 = 4.0 / 5.0 * t150 + 12.0 / 5.0 * t152 - 12.0 / 5.0 * t158;
        let t162 = 0.474596e-1 * t124 * t114 + 0.28363733333333333333e-1 * t139 - 0.9086532e0 * t146 - t149 * t160;
        let t165 = t95 * t95;
        let t167 = t165 * param_hyb_omega_0 * t3;
        let t169 = 1.0 / t97 / t22;
        let t170 = t167 * t169;
        let t171 = t99 * t99;
        let t173 = 1.0 / t171 / t26;
        let t175 = 1.0 / t102 / rho[ip];
        let t176 = t173 * t175;
        let t177 = t107 * t107;
        let t179 = 1.0 / t108 / t177;
        let t183 = 8.0 - 5.0 * t111 + 10.0 / 3.0 * t135 - t170 * t176 * t179 / 3.0;
        let t184 = t162 * t183;
        let t185 = 1.0 / t146;
        let t189 = 3.0 * t105;
        let t190 = 9.0 * t94 + t189;
        let t191 = f64::sqrt(t190);
        let t193 = 9.0 * t157 + t189;
        let t194 = f64::sqrt(t193);
        let t196 = t191 / 3.0 - t194 / 3.0;
        let t200 = t24 * t27;
        let t202 = t21 * t200 * t28;
        let t204 = t202 / 3.0 + t191 / 3.0;
        let t206 = t202 / 3.0 + t108;
        let t207 = 1.0 / t206;
        let t209 = f64::ln(t204 * t207);
        let t213 = t202 / 3.0 + t194 / 3.0;
        let t215 = f64::ln(t213 * t207);
        let t218 = 0.757211e0 + 0.47272888888888888889e-1 * t113 * t115 + 0.26366444444444444444e-1 * t138 * t140 - t184 * t185 / 9.0 + 2.0 / 3.0 * t25 * t29 * t196 + 2.0 * t94 * t209 - 2.0 * t157 * t215;
        let t222 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t218);
        let tzk0 = 2.0 * t222;
        zk[ip] += tzk0;
        let t223 = t17 * t103;
        let t227 = t27 * t37;
        let t229 = t25 * t227 * t109;
        let t231 = t49 * t68;
        let t233 = t91 * t31 * t24;
        let t234 = t231 * t233;
        let t235 = rho[ip] * rho[ip];
        let t237 = 1.0 / t18 / t235;
        let t239 = 1.0 / t45;
        let t240 = t237 * t41 * t239;
        let t241 = t35 * t240;
        let t244 = t51 * t49;
        let t245 = t244 * t32;
        let t248 = t53 * t50;
        let t249 = t248 * t32;
        let t252 = t56 * t54;
        let t253 = t252 * t32;
        let t256 = t59 * t57;
        let t257 = t256 * t32;
        let t260 = t62 * t60;
        let t261 = t260 * t32;
        let t264 = t65 * t63;
        let t265 = t264 * t32;
        let t268 = 2.0 / 9.0 * t245 * t241 - t249 * t241 / 3.0 + 4.0 / 9.0 * t253 * t241 - 5.0 / 9.0 * t257 * t241 + 2.0 / 3.0 * t261 * t241 - 7.0 / 9.0 * t265 * t241;
        let t269 = t50 * t268;
        let t271 = t90 * t90;
        let t272 = 1.0 / t271;
        let t273 = t70 * t31;
        let t274 = t24 * t33;
        let t275 = t273 * t274;
        let t276 = t34 * t237;
        let t277 = t41 * t239;
        let t278 = t276 * t277;
        let t281 = t72 * t49;
        let t282 = t281 * t32;
        let t285 = t74 * t50;
        let t286 = t285 * t32;
        let t289 = t76 * t54;
        let t290 = t289 * t32;
        let t293 = t78 * t57;
        let t294 = t293 * t32;
        let t297 = t80 * t60;
        let t298 = t297 * t32;
        let t301 = t82 * t63;
        let t302 = t301 * t32;
        let t305 = t84 * t66;
        let t306 = t305 * t32;
        let t309 = t87 * t85;
        let t310 = t309 * t32;
        let t312 = -t275 * t278 / 9.0 + 2.0 / 9.0 * t282 * t241 - t286 * t241 / 3.0 + 4.0 / 9.0 * t290 * t241 - 5.0 / 9.0 * t294 * t241 + 2.0 / 3.0 * t298 * t241 - 7.0 / 9.0 * t302 * t241 + 8.0 / 9.0 * t306 * t241 - t310 * t241;
        let t313 = t272 * t312;
        let t316 = piecewise3::<f64>(t93, 2.0 / 9.0 * t234 * t241 + t269 * t91 - t69 * t313, 0.0);
        let t318 = t96 * t101 * t175;
        let t320 = t316 - 2.0 / 9.0 * t318;
        let t321 = t133 * t320;
        let t323 = t25 * t29 * t321;
        let t325 = t229 / 9.0 + t323 / 6.0;
        let t328 = t113 * t140;
        let t331 = t49 * t120;
        let t332 = t331 * t32;
        let t335 = t119 * t119;
        let t336 = 1.0 / t335;
        let t337 = t54 * t336;
        let t338 = t337 * t32;
        let t342 = 0.69368074529891691809e-1 * t332 * t241 - 0.17342018632472922952e-1 * t338 * t241 + 0.42141105276909202774e1 * t316;
        let t343 = t342 * t137;
        let t348 = 1.0 / t235;
        let t351 = t127 * t129 * t348 * t133;
        let t353 = t127 * t129;
        let t354 = t130 * t179;
        let t356 = t353 * t354 * t320;
        let t358 = t229 / 3.0 + t323 / 2.0 - t351 / 3.0 - t356 / 2.0;
        let t359 = t124 * t358;
        let t362 = t185 * t316;
        let t369 = t114 * t316;
        let t373 = t148 * t139;
        let t374 = t373 * t160;
        let t377 = 1.0 / t152;
        let t379 = 1.0 / t158;
        let t380 = piecewise3::<f64>(t155, t316, 0.0);
        let t383 = 6.0 / 5.0 * t377 * t316 - 6.0 / 5.0 * t379 * t380;
        let t385 = 0.474596e-1 * t342 * t114 + 0.474596e-1 * t124 * t316 + 0.56727466666666666666e-1 * t369 - 0.27259596e1 * t139 * t316 - 7.0 / 2.0 * t374 * t316 - t149 * t383;
        let t386 = t385 * t183;
        let t394 = 1.0 / t102 / t235;
        let t395 = t173 * t394;
        let t399 = t177 * t107;
        let t401 = 1.0 / t108 / t399;
        let t402 = t401 * t320;
        let t406 = 5.0 / 3.0 * t229 + 5.0 / 2.0 * t323 - 10.0 / 3.0 * t351 - 5.0 * t356 + 5.0 / 9.0 * t170 * t395 * t179 + 5.0 / 6.0 * t170 * t176 * t402;
        let t407 = t162 * t406;
        let t410 = t139 * t139;
        let t411 = 1.0 / t410;
        let t412 = t411 * t316;
        let t418 = 1.0 / t191;
        let t420 = 2.0 * t318;
        let t421 = 9.0 * t316 - t420;
        let t422 = t418 * t421;
        let t423 = 1.0 / t194;
        let t425 = 9.0 * t380 - t420;
        let t426 = t423 * t425;
        let t428 = t422 / 6.0 - t426 / 6.0;
        let t435 = t21 * t200 * t37;
        let t436 = t435 / 9.0;
        let t438 = -t436 + t422 / 6.0;
        let t440 = t206 * t206;
        let t441 = 1.0 / t440;
        let t442 = t204 * t441;
        let t445 = -t436 + t109 * t320 / 2.0;
        let t447 = t207 * t438 - t442 * t445;
        let t448 = t94 * t447;
        let t449 = 1.0 / t204;
        let t450 = t449 * t206;
        let t456 = -t436 + t426 / 6.0;
        let t458 = t213 * t441;
        let t460 = t207 * t456 - t445 * t458;
        let t461 = t157 * t460;
        let t462 = 1.0 / t213;
        let t463 = t462 * t206;
        let t466 = 0.47272888888888888889e-1 * t325 * t115 - 0.47272888888888888889e-1 * t328 * t316 + 0.26366444444444444444e-1 * t343 * t140 + 0.26366444444444444444e-1 * t359 * t140 - 0.52732888888888888888e-1 * t138 * t362 - t386 * t185 / 9.0 - t407 * t185 / 9.0 + t184 * t412 / 3.0 - 2.0 / 9.0 * t25 * t227 * t196 + 2.0 / 3.0 * t25 * t29 * t428 + 2.0 * t316 * t209 + 2.0 * t448 * t450 - 2.0 * t380 * t215 - 2.0 * t461 * t463;
        let t471 = piecewise3::<f64>(t2, 0.0, -t6 * t223 * t218 / 8.0 - 3.0 / 8.0 * t6 * t19 * t466);
        let tvrho0 = 2.0 * rho[ip] * t471 + 2.0 * t222;
        vrho[ip] += tvrho0;
        let t474 = t21 * t200;
        let t475 = t28 * t133;
        let t476 = 1.0 / t33;
        let t477 = t476 * t34;
        let t479 = t37 * t41 * t239;
        let t480 = t477 * t479;
        let t495 = -t245 * t480 / 12.0 + t249 * t480 / 8.0 - t253 * t480 / 6.0 + 5.0 / 24.0 * t257 * t480 - t261 * t480 / 4.0 + 7.0 / 24.0 * t265 * t480;
        let t496 = t50 * t495;
        let t499 = t273 * t24 * t476;
        let t500 = t34 * t37;
        let t501 = t500 * t277;
        let t520 = t499 * t501 / 24.0 - t282 * t480 / 12.0 + t286 * t480 / 8.0 - t290 * t480 / 6.0 + 5.0 / 24.0 * t294 * t480 - t298 * t480 / 4.0 + 7.0 / 24.0 * t302 * t480 - t306 * t480 / 3.0 + 3.0 / 8.0 * t310 * t480;
        let t521 = t272 * t520;
        let t524 = piecewise3::<f64>(t93, -t234 * t480 / 12.0 + t496 * t91 - t69 * t521, 0.0);
        let t525 = t524 * t115;
        let t536 = -0.26013027948709384428e-1 * t332 * t480 + 0.65032569871773461071e-2 * t338 * t480 + 0.42141105276909202774e1 * t524;
        let t537 = t536 * t137;
        let t540 = t133 * t524;
        let t542 = t25 * t29 * t540;
        let t544 = t353 * t354 * t524;
        let t546 = t542 / 2.0 - t544 / 2.0;
        let t547 = t124 * t546;
        let t550 = t185 * t524;
        let t557 = t114 * t524;
        let t564 = piecewise3::<f64>(t155, t524, 0.0);
        let t567 = 6.0 / 5.0 * t377 * t524 - 6.0 / 5.0 * t379 * t564;
        let t569 = 0.474596e-1 * t536 * t114 + 0.474596e-1 * t124 * t524 + 0.56727466666666666666e-1 * t557 - 0.27259596e1 * t139 * t524 - 7.0 / 2.0 * t374 * t524 - t149 * t567;
        let t570 = t569 * t183;
        let t575 = t401 * t524;
        let t579 = 5.0 / 2.0 * t542 - 5.0 * t544 + 5.0 / 6.0 * t170 * t176 * t575;
        let t580 = t162 * t579;
        let t583 = t411 * t524;
        let t586 = t418 * t524;
        let t587 = t423 * t564;
        let t589 = 3.0 / 2.0 * t586 - 3.0 / 2.0 * t587;
        let t597 = t109 * t524;
        let t600 = 3.0 / 2.0 * t586 * t207 - t442 * t597 / 2.0;
        let t601 = t94 * t600;
        let t610 = 3.0 / 2.0 * t587 * t207 - t458 * t597 / 2.0;
        let t611 = t157 * t610;
        let t614 = 0.78788148148148148148e-2 * t474 * t475 * t525 - 0.47272888888888888889e-1 * t328 * t524 + 0.26366444444444444444e-1 * t537 * t140 + 0.26366444444444444444e-1 * t547 * t140 - 0.52732888888888888888e-1 * t138 * t550 - t570 * t185 / 9.0 - t580 * t185 / 9.0 + t184 * t583 / 3.0 + 2.0 / 3.0 * t25 * t29 * t589 + 2.0 * t524 * t209 + 2.0 * t601 * t450 - 2.0 * t564 * t215 - 2.0 * t611 * t463;
        let t618 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t614);
        let tvsigma0 = 2.0 * rho[ip] * t618;
        vsigma[ip] += tvsigma0;
    }
}
