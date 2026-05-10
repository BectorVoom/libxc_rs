//! MGGA_X_M08 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 195 shared lines across all orders.
//! Delta: 205 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_m08_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_a_0: f64,
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
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_b_9: f64,
    param_b_10: f64,
    param_b_11: f64,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        // --- shared preamble (195 lines) ---
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
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = M_CBRT6;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = t34 * sigma0 * t39;
        let t43 = 0.804e0 + 0.914625e-2 * t41;
        let t46 = 0.1804e1 - 0.646416e0 / t43;
        let t47 = param_a_0;
        let t48 = param_a_1;
        let t49 = t29 * t29;
        let t51 = 3.0 / 10.0 * t49 * t32;
        let t53 = 1.0 / t37 / rho0;
        let t54 = tau0 * t53;
        let t55 = t51 - t54;
        let t56 = t48 * t55;
        let t57 = t51 + t54;
        let t58 = 1.0 / t57;
        let t60 = param_a_2;
        let t61 = t55 * t55;
        let t62 = t60 * t61;
        let t63 = t57 * t57;
        let t64 = 1.0 / t63;
        let t66 = param_a_3;
        let t67 = t61 * t55;
        let t68 = t66 * t67;
        let t69 = t63 * t57;
        let t70 = 1.0 / t69;
        let t72 = param_a_4;
        let t73 = t61 * t61;
        let t74 = t72 * t73;
        let t75 = t63 * t63;
        let t76 = 1.0 / t75;
        let t78 = param_a_5;
        let t79 = t73 * t55;
        let t80 = t78 * t79;
        let t81 = t75 * t57;
        let t82 = 1.0 / t81;
        let t84 = param_a_6;
        let t85 = t73 * t61;
        let t86 = t84 * t85;
        let t87 = t75 * t63;
        let t88 = 1.0 / t87;
        let t90 = param_a_7;
        let t91 = t73 * t67;
        let t92 = t90 * t91;
        let t93 = t75 * t69;
        let t94 = 1.0 / t93;
        let t96 = param_a_8;
        let t97 = t73 * t73;
        let t98 = t96 * t97;
        let t99 = t75 * t75;
        let t100 = 1.0 / t99;
        let t102 = param_a_9;
        let t103 = t97 * t55;
        let t104 = t102 * t103;
        let t106 = 1.0 / t99 / t57;
        let t108 = param_a_10;
        let t109 = t97 * t61;
        let t110 = t108 * t109;
        let t112 = 1.0 / t99 / t63;
        let t114 = param_a_11;
        let t115 = t97 * t67;
        let t116 = t114 * t115;
        let t118 = 1.0 / t99 / t69;
        let t120 = t98 * t100 + t104 * t106 + t110 * t112 + t116 * t118 + t56 * t58 + t62 * t64 + t68 * t70 + t74 * t76 + t80 * t82 + t86 * t88 + t92 * t94 + t47;
        let t123 = f64::exp(-0.93189002206715572255e-2 * t41);
        let t125 = 0.1552e1 - 0.552e0 * t123;
        let t126 = param_b_0;
        let t127 = param_b_1;
        let t128 = t127 * t55;
        let t130 = param_b_2;
        let t131 = t130 * t61;
        let t133 = param_b_3;
        let t134 = t133 * t67;
        let t136 = param_b_4;
        let t137 = t136 * t73;
        let t139 = param_b_5;
        let t140 = t139 * t79;
        let t142 = param_b_6;
        let t143 = t142 * t85;
        let t145 = param_b_7;
        let t146 = t145 * t91;
        let t148 = param_b_8;
        let t149 = t148 * t97;
        let t151 = param_b_9;
        let t152 = t151 * t103;
        let t154 = param_b_10;
        let t155 = t154 * t109;
        let t157 = param_b_11;
        let t158 = t157 * t115;
        let t160 = t149 * t100 + t152 * t106 + t155 * t112 + t158 * t118 + t128 * t58 + t131 * t64 + t134 * t70 + t137 * t76 + t140 * t82 + t143 * t88 + t146 * t94 + t126;
        let t162 = t46 * t120 + t125 * t160;
        let t166 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t162);
        let t167 = rho1 <= dens_threshold;
        let t168 = -t17;
        let t170 = piecewise5(t15, t12, t11, t16, t168 * t8);
        let t171 = 1.0 + t170;
        let t172 = t171 <= zeta_threshold;
        let t173 = pow_1_3(t171);
        let t175 = piecewise3(t172, t23, t173 * t171);
        let t176 = t175 * t27;
        let t177 = rho1 * rho1;
        let t178 = pow_1_3(rho1);
        let t179 = t178 * t178;
        let t181 = 1.0 / t179 / t177;
        let t183 = t34 * sigma2 * t181;
        let t185 = 0.804e0 + 0.914625e-2 * t183;
        let t188 = 0.1804e1 - 0.646416e0 / t185;
        let t190 = 1.0 / t179 / rho1;
        let t191 = tau1 * t190;
        let t192 = t51 - t191;
        let t193 = t48 * t192;
        let t194 = t51 + t191;
        let t195 = 1.0 / t194;
        let t197 = t192 * t192;
        let t198 = t60 * t197;
        let t199 = t194 * t194;
        let t200 = 1.0 / t199;
        let t202 = t197 * t192;
        let t203 = t66 * t202;
        let t204 = t199 * t194;
        let t205 = 1.0 / t204;
        let t207 = t197 * t197;
        let t208 = t72 * t207;
        let t209 = t199 * t199;
        let t210 = 1.0 / t209;
        let t212 = t207 * t192;
        let t213 = t78 * t212;
        let t214 = t209 * t194;
        let t215 = 1.0 / t214;
        let t217 = t207 * t197;
        let t218 = t84 * t217;
        let t219 = t209 * t199;
        let t220 = 1.0 / t219;
        let t222 = t207 * t202;
        let t223 = t90 * t222;
        let t224 = t209 * t204;
        let t225 = 1.0 / t224;
        let t227 = t207 * t207;
        let t228 = t96 * t227;
        let t229 = t209 * t209;
        let t230 = 1.0 / t229;
        let t232 = t227 * t192;
        let t233 = t102 * t232;
        let t235 = 1.0 / t229 / t194;
        let t237 = t227 * t197;
        let t238 = t108 * t237;
        let t240 = 1.0 / t229 / t199;
        let t242 = t227 * t202;
        let t243 = t114 * t242;
        let t245 = 1.0 / t229 / t204;
        let t247 = t193 * t195 + t198 * t200 + t203 * t205 + t208 * t210 + t213 * t215 + t218 * t220 + t223 * t225 + t228 * t230 + t233 * t235 + t238 * t240 + t243 * t245 + t47;
        let t250 = f64::exp(-0.93189002206715572255e-2 * t183);
        let t252 = 0.1552e1 - 0.552e0 * t250;
        let t253 = t127 * t192;
        let t255 = t130 * t197;
        let t257 = t133 * t202;
        let t259 = t136 * t207;
        let t261 = t139 * t212;
        let t263 = t142 * t217;
        let t265 = t145 * t222;
        let t267 = t148 * t227;
        let t269 = t151 * t232;
        let t271 = t154 * t237;
        let t273 = t157 * t242;
        let t275 = t253 * t195 + t255 * t200 + t257 * t205 + t259 * t210 + t261 * t215 + t263 * t220 + t265 * t225 + t267 * t230 + t269 * t235 + t271 * t240 + t273 * t245 + t126;
        let t277 = t188 * t247 + t252 * t275;
        let t281 = piecewise3(t167, 0.0, -3.0 / 8.0 * t6 * t176 * t277);
        let tzk0 = t166 + t281;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (205 lines) ---
        let t282 = t7 * t7;
        let t283 = 1.0 / t282;
        let t284 = t17 * t283;
        let t286 = piecewise5(t11, 0.0, t15, 0.0, t8 - t284);
        let t289 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t286);
        let t290 = t289 * t27;
        let t294 = t27 * t27;
        let t295 = 1.0 / t294;
        let t296 = t26 * t295;
        let t299 = t6 * t296 * t162 / 8.0;
        let t300 = t43 * t43;
        let t302 = 1.0 / t300 * t29;
        let t303 = t302 * t33;
        let t304 = t35 * rho0;
        let t306 = 1.0 / t37 / t304;
        let t307 = sigma0 * t306;
        let t311 = t48 * tau0;
        let t312 = t39 * t58;
        let t315 = t64 * tau0;
        let t316 = t315 * t39;
        let t319 = t60 * t55;
        let t322 = t70 * tau0;
        let t323 = t322 * t39;
        let t326 = t66 * t61;
        let t329 = t76 * tau0;
        let t330 = t329 * t39;
        let t333 = t72 * t67;
        let t336 = t82 * tau0;
        let t337 = t336 * t39;
        let t340 = t78 * t73;
        let t343 = t88 * tau0;
        let t344 = t343 * t39;
        let t347 = t84 * t79;
        let t350 = 5.0 / 3.0 * t311 * t312 + 5.0 / 3.0 * t56 * t316 + 10.0 / 3.0 * t319 * t316 + 10.0 / 3.0 * t62 * t323 + 5.0 * t326 * t323 + 5.0 * t68 * t330 + 20.0 / 3.0 * t333 * t330 + 20.0 / 3.0 * t74 * t337 + 25.0 / 3.0 * t340 * t337 + 25.0 / 3.0 * t80 * t344 + 10.0 * t347 * t344;
        let t351 = t94 * tau0;
        let t352 = t351 * t39;
        let t355 = t90 * t85;
        let t358 = t100 * tau0;
        let t359 = t358 * t39;
        let t362 = t96 * t91;
        let t365 = t106 * tau0;
        let t366 = t365 * t39;
        let t369 = t102 * t97;
        let t372 = t112 * tau0;
        let t373 = t372 * t39;
        let t376 = t108 * t103;
        let t379 = t118 * tau0;
        let t380 = t379 * t39;
        let t383 = t114 * t109;
        let t387 = 1.0 / t99 / t75;
        let t388 = t387 * tau0;
        let t389 = t388 * t39;
        let t392 = 10.0 * t86 * t352 + 35.0 / 3.0 * t355 * t352 + 35.0 / 3.0 * t92 * t359 + 40.0 / 3.0 * t362 * t359 + 40.0 / 3.0 * t98 * t366 + 15.0 * t369 * t366 + 15.0 * t104 * t373 + 50.0 / 3.0 * t376 * t373 + 50.0 / 3.0 * t110 * t380 + 55.0 / 3.0 * t383 * t380 + 55.0 / 3.0 * t116 * t389;
        let t393 = t350 + t392;
        let t395 = t34 * sigma0;
        let t396 = t306 * t123;
        let t397 = t396 * t160;
        let t400 = t127 * tau0;
        let t405 = t130 * t55;
        let t410 = t133 * t61;
        let t415 = t136 * t67;
        let t420 = t139 * t73;
        let t425 = t142 * t79;
        let t428 = 5.0 / 3.0 * t400 * t312 + 5.0 / 3.0 * t128 * t316 + 10.0 / 3.0 * t405 * t316 + 10.0 / 3.0 * t131 * t323 + 5.0 * t410 * t323 + 5.0 * t134 * t330 + 20.0 / 3.0 * t415 * t330 + 20.0 / 3.0 * t137 * t337 + 25.0 / 3.0 * t420 * t337 + 25.0 / 3.0 * t140 * t344 + 10.0 * t425 * t344;
        let t431 = t145 * t85;
        let t436 = t148 * t91;
        let t441 = t151 * t97;
        let t446 = t154 * t103;
        let t451 = t157 * t109;
        let t456 = 10.0 * t143 * t352 + 35.0 / 3.0 * t431 * t352 + 35.0 / 3.0 * t146 * t359 + 40.0 / 3.0 * t436 * t359 + 40.0 / 3.0 * t149 * t366 + 15.0 * t441 * t366 + 15.0 * t152 * t373 + 50.0 / 3.0 * t446 * t373 + 50.0 / 3.0 * t155 * t380 + 55.0 / 3.0 * t451 * t380 + 55.0 / 3.0 * t158 * t389;
        let t457 = t428 + t456;
        let t459 = -0.1576608624e-1 * t303 * t307 * t120 + t46 * t393 - 0.13717421124828532236e-1 * t395 * t397 + t125 * t457;
        let t464 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t290 * t162 - t299 - 3.0 / 8.0 * t6 * t28 * t459);
        let t465 = t168 * t283;
        let t467 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t465);
        let t470 = piecewise3(t172, 0.0, 4.0 / 3.0 * t173 * t467);
        let t471 = t470 * t27;
        let t475 = t175 * t295;
        let t478 = t6 * t475 * t277 / 8.0;
        let t480 = piecewise3(t167, 0.0, -3.0 / 8.0 * t6 * t471 * t277 - t478);
        let tvrho0 = t166 + t281 + t7 * (t464 + t480);
        vrho[ip * 2] += tvrho0;
        let t484 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t284);
        let t487 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t484);
        let t488 = t487 * t27;
        let t493 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t488 * t162 - t299);
        let t495 = piecewise5(t15, 0.0, t11, 0.0, t8 - t465);
        let t498 = piecewise3(t172, 0.0, 4.0 / 3.0 * t173 * t495);
        let t499 = t498 * t27;
        let t503 = t185 * t185;
        let t505 = 1.0 / t503 * t29;
        let t506 = t505 * t33;
        let t507 = t177 * rho1;
        let t509 = 1.0 / t179 / t507;
        let t510 = sigma2 * t509;
        let t514 = t48 * tau1;
        let t515 = t181 * t195;
        let t518 = t200 * tau1;
        let t519 = t518 * t181;
        let t522 = t60 * t192;
        let t525 = t205 * tau1;
        let t526 = t525 * t181;
        let t529 = t66 * t197;
        let t532 = t210 * tau1;
        let t533 = t532 * t181;
        let t536 = t72 * t202;
        let t539 = t215 * tau1;
        let t540 = t539 * t181;
        let t543 = t78 * t207;
        let t546 = t220 * tau1;
        let t547 = t546 * t181;
        let t550 = t84 * t212;
        let t553 = 5.0 / 3.0 * t514 * t515 + 5.0 / 3.0 * t193 * t519 + 10.0 / 3.0 * t522 * t519 + 10.0 / 3.0 * t198 * t526 + 5.0 * t529 * t526 + 5.0 * t203 * t533 + 20.0 / 3.0 * t536 * t533 + 20.0 / 3.0 * t208 * t540 + 25.0 / 3.0 * t543 * t540 + 25.0 / 3.0 * t213 * t547 + 10.0 * t550 * t547;
        let t554 = t225 * tau1;
        let t555 = t554 * t181;
        let t558 = t90 * t217;
        let t561 = t230 * tau1;
        let t562 = t561 * t181;
        let t565 = t96 * t222;
        let t568 = t235 * tau1;
        let t569 = t568 * t181;
        let t572 = t102 * t227;
        let t575 = t240 * tau1;
        let t576 = t575 * t181;
        let t579 = t108 * t232;
        let t582 = t245 * tau1;
        let t583 = t582 * t181;
        let t586 = t114 * t237;
        let t590 = 1.0 / t229 / t209;
        let t591 = t590 * tau1;
        let t592 = t591 * t181;
        let t595 = 10.0 * t218 * t555 + 35.0 / 3.0 * t558 * t555 + 35.0 / 3.0 * t223 * t562 + 40.0 / 3.0 * t565 * t562 + 40.0 / 3.0 * t228 * t569 + 15.0 * t572 * t569 + 15.0 * t233 * t576 + 50.0 / 3.0 * t579 * t576 + 50.0 / 3.0 * t238 * t583 + 55.0 / 3.0 * t586 * t583 + 55.0 / 3.0 * t243 * t592;
        let t596 = t553 + t595;
        let t598 = t34 * sigma2;
        let t599 = t509 * t250;
        let t600 = t599 * t275;
        let t603 = t127 * tau1;
        let t608 = t130 * t192;
        let t613 = t133 * t197;
        let t618 = t136 * t202;
        let t623 = t139 * t207;
        let t628 = t142 * t212;
        let t631 = 5.0 / 3.0 * t603 * t515 + 5.0 / 3.0 * t253 * t519 + 10.0 / 3.0 * t608 * t519 + 10.0 / 3.0 * t255 * t526 + 5.0 * t613 * t526 + 5.0 * t257 * t533 + 20.0 / 3.0 * t618 * t533 + 20.0 / 3.0 * t259 * t540 + 25.0 / 3.0 * t623 * t540 + 25.0 / 3.0 * t261 * t547 + 10.0 * t628 * t547;
        let t634 = t145 * t217;
        let t639 = t148 * t222;
        let t644 = t151 * t227;
        let t649 = t154 * t232;
        let t654 = t157 * t237;
        let t659 = 10.0 * t263 * t555 + 35.0 / 3.0 * t634 * t555 + 35.0 / 3.0 * t265 * t562 + 40.0 / 3.0 * t639 * t562 + 40.0 / 3.0 * t267 * t569 + 15.0 * t644 * t569 + 15.0 * t269 * t576 + 50.0 / 3.0 * t649 * t576 + 50.0 / 3.0 * t271 * t583 + 55.0 / 3.0 * t654 * t583 + 55.0 / 3.0 * t273 * t592;
        let t660 = t631 + t659;
        let t662 = -0.1576608624e-1 * t506 * t510 * t247 + t188 * t596 - 0.13717421124828532236e-1 * t598 * t600 + t252 * t660;
        let t667 = piecewise3(t167, 0.0, -3.0 / 8.0 * t6 * t499 * t277 - t478 - 3.0 / 8.0 * t6 * t176 * t662);
        let tvrho1 = t166 + t281 + t7 * (t493 + t667);
        vrho[ip * 2 + 1] += tvrho1;
        let t670 = t33 * t39;
        let t674 = t39 * t123;
        let t678 = 0.591228234e-2 * t302 * t670 * t120 + 0.51440329218106995885e-2 * t34 * t674 * t160;
        let t682 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t678);
        let tvsigma0 = t7 * t682;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t683 = t33 * t181;
        let t687 = t181 * t250;
        let t691 = 0.591228234e-2 * t505 * t683 * t247 + 0.51440329218106995885e-2 * t34 * t687 * t275;
        let t695 = piecewise3(t167, 0.0, -3.0 / 8.0 * t6 * t176 * t691);
        let tvsigma2 = t7 * t695;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t698 = t64 * t53;
        let t702 = t70 * t53;
        let t707 = t76 * t53;
        let t712 = t82 * t53;
        let t717 = t88 * t53;
        let t722 = -t48 * t53 * t58 - 2.0 * t319 * t698 - 3.0 * t326 * t702 - 4.0 * t333 * t707 - 5.0 * t340 * t712 - 6.0 * t347 * t717 - t56 * t698 - 2.0 * t62 * t702 - 3.0 * t68 * t707 - 4.0 * t74 * t712 - 5.0 * t80 * t717;
        let t723 = t94 * t53;
        let t728 = t100 * t53;
        let t733 = t106 * t53;
        let t738 = t112 * t53;
        let t743 = t118 * t53;
        let t748 = t387 * t53;
        let t751 = -9.0 * t104 * t738 - 10.0 * t110 * t743 - 11.0 * t116 * t748 - 7.0 * t355 * t723 - 8.0 * t362 * t728 - 9.0 * t369 * t733 - 10.0 * t376 * t738 - 11.0 * t383 * t743 - 6.0 * t86 * t723 - 7.0 * t92 * t728 - 8.0 * t98 * t733;
        let t752 = t722 + t751;
        let t775 = -t127 * t53 * t58 - t128 * t698 - 2.0 * t131 * t702 - 3.0 * t134 * t707 - 4.0 * t137 * t712 - 5.0 * t140 * t717 - 2.0 * t405 * t698 - 3.0 * t410 * t702 - 4.0 * t415 * t707 - 5.0 * t420 * t712 - 6.0 * t425 * t717;
        let t798 = -6.0 * t143 * t723 - 7.0 * t146 * t728 - 8.0 * t149 * t733 - 9.0 * t152 * t738 - 10.0 * t155 * t743 - 11.0 * t158 * t748 - 7.0 * t431 * t723 - 8.0 * t436 * t728 - 9.0 * t441 * t733 - 10.0 * t446 * t738 - 11.0 * t451 * t743;
        let t799 = t775 + t798;
        let t801 = t125 * t799 + t46 * t752;
        let t805 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t801);
        let tvtau0 = t7 * t805;
        vtau[ip * 2] += tvtau0;
        let t808 = t200 * t190;
        let t812 = t205 * t190;
        let t817 = t210 * t190;
        let t822 = t215 * t190;
        let t827 = t220 * t190;
        let t832 = -t48 * t190 * t195 - t193 * t808 - 2.0 * t198 * t812 - 3.0 * t203 * t817 - 4.0 * t208 * t822 - 5.0 * t213 * t827 - 2.0 * t522 * t808 - 3.0 * t529 * t812 - 4.0 * t536 * t817 - 5.0 * t543 * t822 - 6.0 * t550 * t827;
        let t833 = t225 * t190;
        let t838 = t230 * t190;
        let t843 = t235 * t190;
        let t848 = t240 * t190;
        let t853 = t245 * t190;
        let t858 = t590 * t190;
        let t861 = -6.0 * t218 * t833 - 7.0 * t223 * t838 - 8.0 * t228 * t843 - 9.0 * t233 * t848 - 10.0 * t238 * t853 - 11.0 * t243 * t858 - 7.0 * t558 * t833 - 8.0 * t565 * t838 - 9.0 * t572 * t843 - 10.0 * t579 * t848 - 11.0 * t586 * t853;
        let t862 = t832 + t861;
        let t885 = -t127 * t190 * t195 - t253 * t808 - 2.0 * t255 * t812 - 3.0 * t257 * t817 - 4.0 * t259 * t822 - 5.0 * t261 * t827 - 2.0 * t608 * t808 - 3.0 * t613 * t812 - 4.0 * t618 * t817 - 5.0 * t623 * t822 - 6.0 * t628 * t827;
        let t908 = -6.0 * t263 * t833 - 7.0 * t265 * t838 - 8.0 * t267 * t843 - 9.0 * t269 * t848 - 10.0 * t271 * t853 - 11.0 * t273 * t858 - 7.0 * t634 * t833 - 8.0 * t639 * t838 - 9.0 * t644 * t843 - 10.0 * t649 * t848 - 11.0 * t654 * t853;
        let t909 = t885 + t908;
        let t911 = t188 * t862 + t252 * t909;
        let t915 = piecewise3(t167, 0.0, -3.0 / 8.0 * t6 * t176 * t911);
        let tvtau1 = t7 * t915;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
