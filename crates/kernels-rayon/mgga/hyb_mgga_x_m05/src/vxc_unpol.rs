//! HYB_MGGA_X_M05 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_m05.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_mgga_x_m05_vxc_unpol(
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
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t4 / t5 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t21 = t20 * param_csi_HF;
        let t22 = M_CBRT6;
        let t23 = M_PI * M_PI;
        let t24 = pow_1_3(t23);
        let t25 = t24 * t24;
        let t26 = 1.0 / t25;
        let t27 = t22 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = sigma[ip] * t29;
        let t31 = rho[ip] * rho[ip];
        let t32 = t20 * t20;
        let t34 = 1.0 / t32 / t31;
        let t38 = 0.804 + 0.009146457198521547 * t27 * t30 * t34;
        let t41 = 1.804 - 0.646416 / t38;
        let t43 = param_a_1;
        let t44 = t22 * t22;
        let t46 = 3.0 / 10.0 * t44 * t25;
        let t47 = tau[ip] * t29;
        let t49 = 1.0 / t32 / rho[ip];
        let t50 = t47 * t49;
        let t51 = t46 - t50;
        let t52 = t43 * t51;
        let t53 = t46 + t50;
        let t54 = 1.0 / t53;
        let t56 = param_a_2;
        let t57 = t51 * t51;
        let t58 = t56 * t57;
        let t59 = t53 * t53;
        let t60 = 1.0 / t59;
        let t62 = param_a_3;
        let t63 = t57 * t51;
        let t64 = t62 * t63;
        let t65 = t59 * t53;
        let t66 = 1.0 / t65;
        let t68 = param_a_4;
        let t69 = t57 * t57;
        let t70 = t68 * t69;
        let t71 = t59 * t59;
        let t72 = 1.0 / t71;
        let t74 = param_a_5;
        let t75 = t69 * t51;
        let t76 = t74 * t75;
        let t77 = t71 * t53;
        let t78 = 1.0 / t77;
        let t80 = param_a_6;
        let t81 = t69 * t57;
        let t82 = t80 * t81;
        let t83 = t71 * t59;
        let t84 = 1.0 / t83;
        let t86 = param_a_7;
        let t87 = t69 * t63;
        let t88 = t86 * t87;
        let t89 = t71 * t65;
        let t90 = 1.0 / t89;
        let t92 = param_a_8;
        let t93 = t69 * t69;
        let t94 = t92 * t93;
        let t95 = t71 * t71;
        let t96 = 1.0 / t95;
        let t98 = param_a_9;
        let t99 = t93 * t51;
        let t100 = t98 * t99;
        let t102 = 1.0 / t95 / t53;
        let t104 = param_a_10;
        let t105 = t93 * t57;
        let t106 = t104 * t105;
        let t108 = 1.0 / t95 / t59;
        let t110 = param_a_11;
        let t112 = t110 * t93 * t63;
        let t114 = 1.0 / t95 / t65;
        let t116 = t100 * t102 + t106 * t108 + t112 * t114 + t52 * t54 + t58 * t60 + t64 * t66 + t70 * t72 + t76 * t78 + t82 * t84 + t88 * t90 + t94 * t96 + param_a_0;
        let t117 = t41 * t116;
        let t121 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t21 * t117);
        let tzk0 = 2.0 * t121;
        zk[ip] += tzk0;
        let t123 = 1.0 / t32 * param_csi_HF;
        let t127 = t4 * t18;
        let t128 = t31 * rho[ip];
        let t130 = 1.0 / t20 / t128;
        let t131 = t130 * param_csi_HF;
        let t132 = t38 * t38;
        let t133 = 1.0 / t132;
        let t135 = t127 * t131 * t133;
        let t137 = t27 * t30 * t116;
        let t140 = t43 * tau[ip];
        let t145 = t52 * t60;
        let t146 = t47 * t34;
        let t149 = t56 * t51;
        let t150 = t149 * t60;
        let t153 = t58 * t66;
        let t156 = t62 * t57;
        let t157 = t156 * t66;
        let t160 = t64 * t72;
        let t163 = t68 * t63;
        let t164 = t163 * t72;
        let t167 = t70 * t78;
        let t170 = t74 * t69;
        let t171 = t170 * t78;
        let t174 = t76 * t84;
        let t177 = t80 * t75;
        let t178 = t177 * t84;
        let t181 = 5.0 / 3.0 * t140 * t29 * t34 * t54 + 5.0 / 3.0 * t145 * t146 + 10.0 / 3.0 * t150 * t146 + 10.0 / 3.0 * t153 * t146 + 5.0 * t157 * t146 + 5.0 * t160 * t146 + 20.0 / 3.0 * t164 * t146 + 20.0 / 3.0 * t167 * t146 + 25.0 / 3.0 * t171 * t146 + 25.0 / 3.0 * t174 * t146 + 10.0 * t178 * t146;
        let t182 = t82 * t90;
        let t185 = t86 * t81;
        let t186 = t185 * t90;
        let t189 = t88 * t96;
        let t192 = t92 * t87;
        let t193 = t192 * t96;
        let t196 = t94 * t102;
        let t199 = t98 * t93;
        let t200 = t199 * t102;
        let t203 = t100 * t108;
        let t206 = t104 * t99;
        let t207 = t206 * t108;
        let t210 = t106 * t114;
        let t213 = t110 * t105;
        let t214 = t213 * t114;
        let t218 = 1.0 / t95 / t71;
        let t219 = t112 * t218;
        let t222 = 10.0 * t182 * t146 + 35.0 / 3.0 * t186 * t146 + 35.0 / 3.0 * t189 * t146 + 40.0 / 3.0 * t193 * t146 + 40.0 / 3.0 * t196 * t146 + 15.0 * t200 * t146 + 15.0 * t203 * t146 + 50.0 / 3.0 * t207 * t146 + 50.0 / 3.0 * t210 * t146 + 55.0 / 3.0 * t214 * t146 + 55.0 / 3.0 * t219 * t146;
        let t223 = t181 + t222;
        let t224 = t41 * t223;
        let t229 = piecewise3(t3, 0.0, -t19 * t123 * t117 / 8.0 + 0.0040369036088841095 * t135 * t137 - 3.0 / 8.0 * t19 * t21 * t224);
        let tvrho0 = 2.0 * rho[ip] * t229 + 2.0 * t121;
        vrho[ip] += tvrho0;
        let t235 = t127 / t20 / t31 * param_csi_HF;
        let t236 = t133 * t22;
        let t237 = t26 * t29;
        let t239 = t236 * t237 * t116;
        let t242 = piecewise3(t3, 0.0, -0.0015138388533315413 * t235 * t239);
        let tvsigma0 = 2.0 * rho[ip] * t242;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t244 = t43 * t29;
        let t247 = t60 * t29;
        let t248 = t247 * t49;
        let t252 = t66 * t29;
        let t253 = t252 * t49;
        let t258 = t72 * t29;
        let t259 = t258 * t49;
        let t264 = t78 * t29;
        let t265 = t264 * t49;
        let t270 = t84 * t29;
        let t271 = t270 * t49;
        let t276 = -t244 * t49 * t54 - 2.0 * t149 * t248 - 3.0 * t156 * t253 - 4.0 * t163 * t259 - 5.0 * t170 * t265 - 6.0 * t177 * t271 - t52 * t248 - 2.0 * t58 * t253 - 3.0 * t64 * t259 - 4.0 * t70 * t265 - 5.0 * t76 * t271;
        let t277 = t90 * t29;
        let t278 = t277 * t49;
        let t283 = t96 * t29;
        let t284 = t283 * t49;
        let t289 = t102 * t29;
        let t290 = t289 * t49;
        let t295 = t108 * t29;
        let t296 = t295 * t49;
        let t301 = t114 * t29;
        let t302 = t301 * t49;
        let t307 = t218 * t29;
        let t311 = -11.0 * t112 * t307 * t49 - 9.0 * t100 * t296 - 10.0 * t106 * t302 - 7.0 * t185 * t278 - 8.0 * t192 * t284 - 9.0 * t199 * t290 - 10.0 * t206 * t296 - 11.0 * t213 * t302 - 6.0 * t82 * t278 - 7.0 * t88 * t284 - 8.0 * t94 * t290;
        let t312 = t276 + t311;
        let t313 = t41 * t312;
        let t317 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t21 * t313);
        let tvtau0 = 2.0 * rho[ip] * t317;
        vtau[ip] += tvtau0;
    }
}
