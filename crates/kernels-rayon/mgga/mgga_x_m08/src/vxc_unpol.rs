//! MGGA_X_M08 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_m08.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_m08_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t35 = t26 * t29 * t33;
        let t37 = 0.804e0 + 0.914625e-2 * t35;
        let t40 = 0.1804e1 - 0.646416e0 / t37;
        let t42 = param_a_1;
        let t43 = t21 * t21;
        let t45 = 3.0 / 10.0 * t43 * t24;
        let t46 = tau[ip] * t28;
        let t48 = 1.0 / t31 / rho[ip];
        let t49 = t46 * t48;
        let t50 = t45 - t49;
        let t51 = t42 * t50;
        let t52 = t45 + t49;
        let t53 = 1.0 / t52;
        let t55 = param_a_2;
        let t56 = t50 * t50;
        let t57 = t55 * t56;
        let t58 = t52 * t52;
        let t59 = 1.0 / t58;
        let t61 = param_a_3;
        let t62 = t56 * t50;
        let t63 = t61 * t62;
        let t64 = t58 * t52;
        let t65 = 1.0 / t64;
        let t67 = param_a_4;
        let t68 = t56 * t56;
        let t69 = t67 * t68;
        let t70 = t58 * t58;
        let t71 = 1.0 / t70;
        let t73 = param_a_5;
        let t74 = t68 * t50;
        let t75 = t73 * t74;
        let t76 = t70 * t52;
        let t77 = 1.0 / t76;
        let t79 = param_a_6;
        let t80 = t68 * t56;
        let t81 = t79 * t80;
        let t82 = t70 * t58;
        let t83 = 1.0 / t82;
        let t85 = param_a_7;
        let t86 = t68 * t62;
        let t87 = t85 * t86;
        let t88 = t70 * t64;
        let t89 = 1.0 / t88;
        let t91 = param_a_8;
        let t92 = t68 * t68;
        let t93 = t91 * t92;
        let t94 = t70 * t70;
        let t95 = 1.0 / t94;
        let t97 = param_a_9;
        let t98 = t92 * t50;
        let t99 = t97 * t98;
        let t101 = 1.0 / t94 / t52;
        let t103 = param_a_10;
        let t104 = t92 * t56;
        let t105 = t103 * t104;
        let t107 = 1.0 / t94 / t58;
        let t109 = param_a_11;
        let t110 = t92 * t62;
        let t111 = t109 * t110;
        let t113 = 1.0 / t94 / t64;
        let t115 = t99 * t101 + t105 * t107 + t111 * t113 + t51 * t53 + t57 * t59 + t63 * t65 + t69 * t71 + t75 * t77 + t81 * t83 + t87 * t89 + t93 * t95 + param_a_0;
        let t118 = f64::exp(-0.93189002206715572255e-2 * t35);
        let t120 = 0.1552e1 - 0.552e0 * t118;
        let t122 = param_b_1;
        let t123 = t122 * t50;
        let t125 = param_b_2;
        let t126 = t125 * t56;
        let t128 = param_b_3;
        let t129 = t128 * t62;
        let t131 = param_b_4;
        let t132 = t131 * t68;
        let t134 = param_b_5;
        let t135 = t134 * t74;
        let t137 = param_b_6;
        let t138 = t137 * t80;
        let t140 = param_b_7;
        let t141 = t140 * t86;
        let t143 = param_b_8;
        let t144 = t143 * t92;
        let t146 = param_b_9;
        let t147 = t146 * t98;
        let t149 = param_b_10;
        let t150 = t149 * t104;
        let t152 = param_b_11;
        let t153 = t152 * t110;
        let t155 = t147 * t101 + t150 * t107 + t153 * t113 + t123 * t53 + t126 * t59 + t129 * t65 + t132 * t71 + t135 * t77 + t138 * t83 + t141 * t89 + t144 * t95 + param_b_0;
        let t157 = t40 * t115 + t120 * t155;
        let t161 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t157);
        let tzk0 = 2.0 * t161;
        zk[ip] += tzk0;
        let t163 = t18 / t31;
        let t167 = t37 * t37;
        let t170 = 1.0 / t167 * t21 * t25;
        let t171 = t30 * rho[ip];
        let t173 = 1.0 / t31 / t171;
        let t178 = t42 * tau[ip];
        let t179 = t28 * t33;
        let t180 = t179 * t53;
        let t183 = t51 * t59;
        let t184 = t46 * t33;
        let t187 = t55 * t50;
        let t188 = t187 * t59;
        let t191 = t57 * t65;
        let t194 = t61 * t56;
        let t195 = t194 * t65;
        let t198 = t63 * t71;
        let t201 = t67 * t62;
        let t202 = t201 * t71;
        let t205 = t69 * t77;
        let t208 = t73 * t68;
        let t209 = t208 * t77;
        let t212 = t75 * t83;
        let t215 = t79 * t74;
        let t216 = t215 * t83;
        let t219 = 5.0 / 3.0 * t178 * t180 + 5.0 / 3.0 * t183 * t184 + 10.0 / 3.0 * t188 * t184 + 10.0 / 3.0 * t191 * t184 + 5.0 * t195 * t184 + 5.0 * t198 * t184 + 20.0 / 3.0 * t202 * t184 + 20.0 / 3.0 * t205 * t184 + 25.0 / 3.0 * t209 * t184 + 25.0 / 3.0 * t212 * t184 + 10.0 * t216 * t184;
        let t220 = t81 * t89;
        let t223 = t85 * t80;
        let t224 = t223 * t89;
        let t227 = t87 * t95;
        let t230 = t91 * t86;
        let t231 = t230 * t95;
        let t234 = t93 * t101;
        let t237 = t97 * t92;
        let t238 = t237 * t101;
        let t241 = t99 * t107;
        let t244 = t103 * t98;
        let t245 = t244 * t107;
        let t248 = t105 * t113;
        let t251 = t109 * t104;
        let t252 = t251 * t113;
        let t256 = 1.0 / t94 / t70;
        let t257 = t111 * t256;
        let t260 = 10.0 * t220 * t184 + 35.0 / 3.0 * t224 * t184 + 35.0 / 3.0 * t227 * t184 + 40.0 / 3.0 * t231 * t184 + 40.0 / 3.0 * t234 * t184 + 15.0 * t238 * t184 + 15.0 * t241 * t184 + 50.0 / 3.0 * t245 * t184 + 50.0 / 3.0 * t248 * t184 + 55.0 / 3.0 * t252 * t184 + 55.0 / 3.0 * t257 * t184;
        let t261 = t219 + t260;
        let t263 = t26 * sigma[ip];
        let t264 = t28 * t173;
        let t265 = t118 * t155;
        let t269 = t122 * tau[ip];
        let t272 = t123 * t59;
        let t275 = t125 * t50;
        let t276 = t275 * t59;
        let t279 = t126 * t65;
        let t282 = t128 * t56;
        let t283 = t282 * t65;
        let t286 = t129 * t71;
        let t289 = t131 * t62;
        let t290 = t289 * t71;
        let t293 = t132 * t77;
        let t296 = t134 * t68;
        let t297 = t296 * t77;
        let t300 = t135 * t83;
        let t303 = t137 * t74;
        let t304 = t303 * t83;
        let t307 = 5.0 / 3.0 * t269 * t180 + 5.0 / 3.0 * t272 * t184 + 10.0 / 3.0 * t276 * t184 + 10.0 / 3.0 * t279 * t184 + 5.0 * t283 * t184 + 5.0 * t286 * t184 + 20.0 / 3.0 * t290 * t184 + 20.0 / 3.0 * t293 * t184 + 25.0 / 3.0 * t297 * t184 + 25.0 / 3.0 * t300 * t184 + 10.0 * t304 * t184;
        let t308 = t138 * t89;
        let t311 = t140 * t80;
        let t312 = t311 * t89;
        let t315 = t141 * t95;
        let t318 = t143 * t86;
        let t319 = t318 * t95;
        let t322 = t144 * t101;
        let t325 = t146 * t92;
        let t326 = t325 * t101;
        let t329 = t147 * t107;
        let t332 = t149 * t98;
        let t333 = t332 * t107;
        let t336 = t150 * t113;
        let t339 = t152 * t104;
        let t340 = t339 * t113;
        let t343 = t153 * t256;
        let t346 = 10.0 * t308 * t184 + 35.0 / 3.0 * t312 * t184 + 35.0 / 3.0 * t315 * t184 + 40.0 / 3.0 * t319 * t184 + 40.0 / 3.0 * t322 * t184 + 15.0 * t326 * t184 + 15.0 * t329 * t184 + 50.0 / 3.0 * t333 * t184 + 50.0 / 3.0 * t336 * t184 + 55.0 / 3.0 * t340 * t184 + 55.0 / 3.0 * t343 * t184;
        let t347 = t307 + t346;
        let t349 = -0.1576608624e-1 * t170 * t29 * t173 * t115 + t40 * t261 - 0.13717421124828532236e-1 * t263 * t264 * t265 + t120 * t347;
        let t354 = piecewise3(t3, 0.0, -t7 * t163 * t157 / 8.0 - 3.0 / 8.0 * t7 * t20 * t349);
        let tvrho0 = 2.0 * rho[ip] * t354 + 2.0 * t161;
        vrho[ip] += tvrho0;
        let t360 = t26 * t28;
        let t361 = t33 * t118;
        let t365 = 0.591228234e-2 * t170 * t179 * t115 + 0.51440329218106995885e-2 * t360 * t361 * t155;
        let t369 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t365);
        let tvsigma0 = 2.0 * rho[ip] * t369;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t371 = t42 * t28;
        let t372 = t48 * t53;
        let t374 = t59 * t28;
        let t375 = t374 * t48;
        let t379 = t65 * t28;
        let t380 = t379 * t48;
        let t385 = t71 * t28;
        let t386 = t385 * t48;
        let t391 = t77 * t28;
        let t392 = t391 * t48;
        let t397 = t83 * t28;
        let t398 = t397 * t48;
        let t403 = -2.0 * t187 * t375 - 3.0 * t194 * t380 - 4.0 * t201 * t386 - 5.0 * t208 * t392 - 6.0 * t215 * t398 - t371 * t372 - t51 * t375 - 2.0 * t57 * t380 - 3.0 * t63 * t386 - 4.0 * t69 * t392 - 5.0 * t75 * t398;
        let t404 = t89 * t28;
        let t405 = t404 * t48;
        let t410 = t95 * t28;
        let t411 = t410 * t48;
        let t416 = t101 * t28;
        let t417 = t416 * t48;
        let t422 = t107 * t28;
        let t423 = t422 * t48;
        let t428 = t113 * t28;
        let t429 = t428 * t48;
        let t434 = t256 * t28;
        let t435 = t434 * t48;
        let t438 = -10.0 * t105 * t429 - 11.0 * t111 * t435 - 7.0 * t223 * t405 - 8.0 * t230 * t411 - 9.0 * t237 * t417 - 10.0 * t244 * t423 - 11.0 * t251 * t429 - 6.0 * t81 * t405 - 7.0 * t87 * t411 - 8.0 * t93 * t417 - 9.0 * t99 * t423;
        let t439 = t403 + t438;
        let t441 = t122 * t28;
        let t462 = -t123 * t375 - 2.0 * t126 * t380 - 3.0 * t129 * t386 - 4.0 * t132 * t392 - 5.0 * t135 * t398 - 2.0 * t275 * t375 - 3.0 * t282 * t380 - 4.0 * t289 * t386 - 5.0 * t296 * t392 - 6.0 * t303 * t398 - t441 * t372;
        let t485 = -6.0 * t138 * t405 - 7.0 * t141 * t411 - 8.0 * t144 * t417 - 9.0 * t147 * t423 - 10.0 * t150 * t429 - 11.0 * t153 * t435 - 7.0 * t311 * t405 - 8.0 * t318 * t411 - 9.0 * t325 * t417 - 10.0 * t332 * t423 - 11.0 * t339 * t429;
        let t486 = t462 + t485;
        let t488 = t120 * t486 + t40 * t439;
        let t492 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t488);
        let tvtau0 = 2.0 * rho[ip] * t492;
        vtau[ip] += tvtau0;
    }
}
