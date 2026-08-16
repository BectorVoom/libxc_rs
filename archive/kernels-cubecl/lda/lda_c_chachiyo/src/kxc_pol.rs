//! LDA_C_CHACHIYO kxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_chachiyo.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_CHACHIYO kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_chachiyo_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t3 = param_bp * t2;
        let t5 = pow_1_3::<f64>(1.0 / M_PI);
        let t7 = M_CBRT4;
        let t8 = 1.0 / t5 * t7;
        let t9 = rho0 + rho1;
        let t10 = pow_1_3::<f64>(t9);
        let t11 = t8 * t10;
        let t14 = param_cp * t1;
        let t15 = t5 * t5;
        let t17 = t7 * t7;
        let t18 = 1.0 / t15 * t17;
        let t19 = t10 * t10;
        let t20 = t18 * t19;
        let t23 = 1.0 + t3 * t11 / 3.0 + t14 * t20 / 3.0;
        let t24 = f64::ln(t23);
        let t25 = param_ap * t24;
        let t26 = param_bf * t2;
        let t29 = param_cf * t1;
        let t32 = 1.0 + t26 * t11 / 3.0 + t29 * t20 / 3.0;
        let t33 = f64::ln(t32);
        let t35 = param_af * t33 - t25;
        let t36 = rho0 - rho1;
        let t37 = 1.0 / t9;
        let t38 = t36 * t37;
        let t39 = 1.0 + t38;
        let t40 = t39 <= zeta_threshold;
        let t41 = pow_1_3::<f64>(zeta_threshold);
        let t42 = t41 * zeta_threshold;
        let t43 = pow_1_3::<f64>(t39);
        let t45 = piecewise3::<f64>(t40, t42, t43 * t39);
        let t46 = 1.0 - t38;
        let t47 = t46 <= zeta_threshold;
        let t48 = pow_1_3::<f64>(t46);
        let t50 = piecewise3::<f64>(t47, t42, t48 * t46);
        let t51 = t45 + t50 - 2.0;
        let t53 = M_CBRT2;
        let t56 = 1.0 / (2.0 * t53 - 2.0);
        let t57 = t35 * t51 * t56;
        let tzk0 = t25 + t57;
        zk[ip] += tzk0;
        let t59 = t8 / t19;
        let t63 = t18 / t10;
        let t66 = t3 * t59 / 9.0 + 2.0 / 9.0 * t14 * t63;
        let t68 = 1.0 / t23;
        let t69 = param_ap * t66 * t68;
        let t74 = t26 * t59 / 9.0 + 2.0 / 9.0 * t29 * t63;
        let t76 = 1.0 / t32;
        let t78 = param_af * t74 * t76 - t69;
        let t80 = t78 * t51 * t56;
        let t81 = t9 * t9;
        let t82 = 1.0 / t81;
        let t83 = t36 * t82;
        let t84 = t37 - t83;
        let t87 = piecewise3::<f64>(t40, 0.0, 4.0 / 3.0 * t43 * t84);
        let t88 = -t84;
        let t91 = piecewise3::<f64>(t47, 0.0, 4.0 / 3.0 * t48 * t88);
        let t92 = t87 + t91;
        let t94 = t35 * t92 * t56;
        let tvrho0 = t25 + t57 + t9 * (t69 + t80 + t94);
        vrho[ip * 2] += tvrho0;
        let t97 = -t37 - t83;
        let t100 = piecewise3::<f64>(t40, 0.0, 4.0 / 3.0 * t43 * t97);
        let t101 = -t97;
        let t104 = piecewise3::<f64>(t47, 0.0, 4.0 / 3.0 * t48 * t101);
        let t105 = t100 + t104;
        let t107 = t35 * t105 * t56;
        let tvrho1 = t25 + t57 + t9 * (t69 + t80 + t107);
        vrho[ip * 2 + 1] += tvrho1;
        let t110 = 2.0 * t69;
        let t111 = 2.0 * t80;
        let t115 = t8 / t19 / t9;
        let t119 = t18 / t10 / t9;
        let t122 = -2.0 / 27.0 * t3 * t115 - 2.0 / 27.0 * t14 * t119;
        let t123 = param_ap * t122;
        let t124 = t123 * t68;
        let t125 = t66 * t66;
        let t127 = t23 * t23;
        let t128 = 1.0 / t127;
        let t129 = param_ap * t125 * t128;
        let t133 = -2.0 / 27.0 * t26 * t115 - 2.0 / 27.0 * t29 * t119;
        let t134 = param_af * t133;
        let t136 = t74 * t74;
        let t138 = t32 * t32;
        let t139 = 1.0 / t138;
        let t141 = -param_af * t136 * t139 + t134 * t76 - t124 + t129;
        let t143 = t141 * t51 * t56;
        let t145 = t78 * t92 * t56;
        let t146 = 2.0 * t145;
        let t147 = t43 * t43;
        let t148 = 1.0 / t147;
        let t149 = t84 * t84;
        let t152 = t81 * t9;
        let t153 = 1.0 / t152;
        let t154 = t36 * t153;
        let t156 = -2.0 * t82 + 2.0 * t154;
        let t160 = piecewise3::<f64>(t40, 0.0, 4.0 / 9.0 * t148 * t149 + 4.0 / 3.0 * t43 * t156);
        let t161 = t48 * t48;
        let t162 = 1.0 / t161;
        let t163 = t88 * t88;
        let t166 = -t156;
        let t170 = piecewise3::<f64>(t47, 0.0, 4.0 / 9.0 * t162 * t163 + 4.0 / 3.0 * t48 * t166);
        let t171 = t160 + t170;
        let t173 = t35 * t171 * t56;
        let tv2rho20 = t110 + t111 + 2.0 * t94 + t9 * (t124 - t129 + t143 + t146 + t173);
        v2rho2[ip * 3] += tv2rho20;
        let t177 = t78 * t105 * t56;
        let t178 = t148 * t97;
        let t181 = t43 * t36;
        let t185 = piecewise3::<f64>(t40, 0.0, 4.0 / 9.0 * t178 * t84 + 8.0 / 3.0 * t181 * t153);
        let t186 = t162 * t101;
        let t189 = t48 * t36;
        let t193 = piecewise3::<f64>(t47, 0.0, 4.0 / 9.0 * t186 * t88 - 8.0 / 3.0 * t189 * t153);
        let t194 = t185 + t193;
        let t196 = t35 * t194 * t56;
        let tv2rho21 = t110 + t111 + t94 + t107 + t9 * (t124 - t129 + t143 + t145 + t177 + t196);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t200 = 2.0 * t177;
        let t201 = t97 * t97;
        let t205 = 2.0 * t82 + 2.0 * t154;
        let t209 = piecewise3::<f64>(t40, 0.0, 4.0 / 9.0 * t148 * t201 + 4.0 / 3.0 * t43 * t205);
        let t210 = t101 * t101;
        let t213 = -t205;
        let t217 = piecewise3::<f64>(t47, 0.0, 4.0 / 9.0 * t162 * t210 + 4.0 / 3.0 * t48 * t213);
        let t218 = t209 + t217;
        let t220 = t35 * t218 * t56;
        let tv2rho22 = t110 + t111 + 2.0 * t107 + t9 * (t124 - t129 + t143 + t200 + t220);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t223 = 3.0 * t124;
        let t224 = 3.0 * t129;
        let t225 = 3.0 * t143;
        let t230 = t8 / t19 / t81;
        let t235 = t18 / t10 / t81;
        let t239 = param_ap * (10.0 / 81.0 * t3 * t230 + 8.0 / 81.0 * t14 * t235);
        let t240 = t239 * t68;
        let t241 = t128 * t66;
        let t242 = t123 * t241;
        let t243 = 3.0 * t242;
        let t247 = 1.0 / t127 / t23;
        let t248 = param_ap * t125 * t66 * t247;
        let t249 = 2.0 * t248;
        let t255 = param_af * (10.0 / 81.0 * t26 * t230 + 8.0 / 81.0 * t29 * t235);
        let t257 = t139 * t74;
        let t263 = 1.0 / t138 / t32;
        let t266 = 2.0 * param_af * t136 * t74 * t263 - 3.0 * t134 * t257 + t255 * t76 - t240 + t243 - t249;
        let t268 = t266 * t51 * t56;
        let t270 = t141 * t92 * t56;
        let t271 = 3.0 * t270;
        let t273 = t78 * t171 * t56;
        let t276 = 1.0 / t147 / t39;
        let t277 = t149 * t84;
        let t280 = t148 * t84;
        let t283 = t81 * t81;
        let t284 = 1.0 / t283;
        let t285 = t36 * t284;
        let t287 = 6.0 * t153 - 6.0 * t285;
        let t291 = piecewise3::<f64>(t40, 0.0, -8.0 / 27.0 * t276 * t277 + 4.0 / 3.0 * t280 * t156 + 4.0 / 3.0 * t43 * t287);
        let t293 = 1.0 / t161 / t46;
        let t294 = t163 * t88;
        let t297 = t162 * t88;
        let t300 = -t287;
        let t304 = piecewise3::<f64>(t47, 0.0, -8.0 / 27.0 * t293 * t294 + 4.0 / 3.0 * t297 * t166 + 4.0 / 3.0 * t48 * t300);
        let t305 = t291 + t304;
        let t307 = t35 * t305 * t56;
        let tv3rho30 = t223 - t224 + t225 + 6.0 * t145 + 3.0 * t173 + t9 * (t240 - t243 + t249 + t268 + t271 + 3.0 * t273 + t307);
        v3rho3[ip * 4] += tv3rho30;
        let t311 = 2.0 * t196;
        let t314 = t141 * t105 * t56;
        let t316 = t78 * t194 * t56;
        let t317 = 2.0 * t316;
        let t318 = t276 * t97;
        let t321 = t148 * t36;
        let t332 = piecewise3::<f64>(t40, 0.0, -8.0 / 27.0 * t318 * t149 + 16.0 / 9.0 * t321 * t153 * t84 + 4.0 / 9.0 * t178 * t156 + 8.0 / 3.0 * t43 * t153 - 8.0 * t181 * t284);
        let t333 = t293 * t101;
        let t336 = t162 * t36;
        let t347 = piecewise3::<f64>(t47, 0.0, -8.0 / 27.0 * t333 * t163 - 16.0 / 9.0 * t336 * t153 * t88 + 4.0 / 9.0 * t186 * t166 - 8.0 / 3.0 * t48 * t153 + 8.0 * t189 * t284);
        let t348 = t332 + t347;
        let t350 = t35 * t348 * t56;
        let tv3rho31 = t223 - t224 + t225 + 4.0 * t145 + t173 + t200 + t311 + t9 * (t240 - t243 + t249 + t268 + 2.0 * t270 + t273 + t314 + t317 + t350);
        v3rho3[ip * 4 + 1] += tv3rho31;
        let t356 = t78 * t218 * t56;
        let t357 = t276 * t201;
        let t362 = t148 * t205;
        let t367 = -2.0 * t153 - 6.0 * t285;
        let t371 = piecewise3::<f64>(t40, 0.0, -8.0 / 27.0 * t357 * t84 + 16.0 / 9.0 * t178 * t154 + 4.0 / 9.0 * t362 * t84 + 4.0 / 3.0 * t43 * t367);
        let t372 = t293 * t210;
        let t377 = t162 * t213;
        let t380 = -t367;
        let t384 = piecewise3::<f64>(t47, 0.0, -8.0 / 27.0 * t372 * t88 - 16.0 / 9.0 * t186 * t154 + 4.0 / 9.0 * t377 * t88 + 4.0 / 3.0 * t48 * t380);
        let t385 = t371 + t384;
        let t387 = t35 * t385 * t56;
        let tv3rho32 = t223 - t224 + t225 + t146 + 4.0 * t177 + t311 + t220 + t9 * (t240 - t243 + t249 + t268 + t270 + 2.0 * t314 + t317 + t356 + t387);
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t392 = 3.0 * t314;
        let t394 = t201 * t97;
        let t400 = -6.0 * t153 - 6.0 * t285;
        let t404 = piecewise3::<f64>(t40, 0.0, -8.0 / 27.0 * t276 * t394 + 4.0 / 3.0 * t178 * t205 + 4.0 / 3.0 * t43 * t400);
        let t405 = t210 * t101;
        let t410 = -t400;
        let t414 = piecewise3::<f64>(t47, 0.0, -8.0 / 27.0 * t293 * t405 + 4.0 / 3.0 * t186 * t213 + 4.0 / 3.0 * t48 * t410);
        let t415 = t404 + t414;
        let t417 = t35 * t415 * t56;
        let tv3rho33 = t223 - t224 + t225 + 6.0 * t177 + 3.0 * t220 + t9 * (t240 - t243 + t249 + t268 + t392 + 3.0 * t356 + t417);
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}
