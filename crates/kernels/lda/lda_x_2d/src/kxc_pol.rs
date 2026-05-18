//! LDA_X_2D kxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_2d.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_X_2D kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_x_2d_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_SQRT2;
        let t2 = f64::sqrt(M_PI);
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = rho0 - rho1;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t8 = t5 * t7;
        let t9 = 1.0 + t8;
        let t10 = t9 <= zeta_threshold;
        let t11 = f64::sqrt(zeta_threshold);
        let t12 = t11 * zeta_threshold;
        let t13 = f64::sqrt(t9);
        let t14 = t13 * t9;
        let t15 = piecewise3::<f64>(t10, t12, t14);
        let t16 = 1.0 - t8;
        let t17 = t16 <= zeta_threshold;
        let t18 = f64::sqrt(t16);
        let t19 = t18 * t16;
        let t20 = piecewise3::<f64>(t17, t12, t19);
        let t22 = t15 / 2.0 + t20 / 2.0;
        let t23 = f64::sqrt(t6);
        let t25 = t4 * t22 * t23;
        let tzk0 = -4.0 / 3.0 * t25;
        zk[ip] += tzk0;
        let t27 = 2.0 * t25;
        let t28 = t23 * t6;
        let t29 = t28 * t1;
        let t30 = t6 * t6;
        let t31 = 1.0 / t30;
        let t32 = t5 * t31;
        let t33 = t7 - t32;
        let t36 = piecewise3::<f64>(t10, 0.0, 3.0 / 2.0 * t13 * t33);
        let t37 = -t33;
        let t40 = piecewise3::<f64>(t17, 0.0, 3.0 / 2.0 * t18 * t37);
        let t42 = t36 / 2.0 + t40 / 2.0;
        let tvrho0 = -t27 - 4.0 / 3.0 * t29 * t3 * t42;
        vrho[ip * 2] += tvrho0;
        let t46 = -t7 - t32;
        let t49 = piecewise3::<f64>(t10, 0.0, 3.0 / 2.0 * t13 * t46);
        let t50 = -t46;
        let t53 = piecewise3::<f64>(t17, 0.0, 3.0 / 2.0 * t18 * t50);
        let t56 = t3 * (t49 / 2.0 + t53 / 2.0);
        let tvrho1 = -t27 - 4.0 / 3.0 * t29 * t56;
        vrho[ip * 2 + 1] += tvrho1;
        let t60 = t4 * t42 * t23;
        let t62 = 1.0 / t23;
        let t64 = t4 * t22 * t62;
        let t65 = 1.0 / t13;
        let t66 = t33 * t33;
        let t70 = 1.0 / t30 / t6;
        let t71 = t5 * t70;
        let t73 = -2.0 * t31 + 2.0 * t71;
        let t77 = piecewise3::<f64>(t10, 0.0, 3.0 / 4.0 * t65 * t66 + 3.0 / 2.0 * t13 * t73);
        let t78 = 1.0 / t18;
        let t79 = t37 * t37;
        let t82 = -t73;
        let t86 = piecewise3::<f64>(t17, 0.0, 3.0 / 4.0 * t78 * t79 + 3.0 / 2.0 * t18 * t82);
        let t88 = t77 / 2.0 + t86 / 2.0;
        let tv2rho20 = -4.0 * t60 - t64 - 4.0 / 3.0 * t29 * t3 * t88;
        v2rho2[ip * 3] += tv2rho20;
        let t93 = t23 * t1;
        let t94 = t93 * t56;
        let t96 = t65 * t46;
        let t99 = t13 * t5;
        let t103 = piecewise3::<f64>(t10, 0.0, 3.0 / 4.0 * t96 * t33 + 3.0 * t99 * t70);
        let t104 = t78 * t50;
        let t107 = t18 * t5;
        let t111 = piecewise3::<f64>(t17, 0.0, 3.0 / 4.0 * t104 * t37 - 3.0 * t107 * t70);
        let t114 = t3 * (t103 / 2.0 + t111 / 2.0);
        let tv2rho21 = -2.0 * t60 - t64 - 2.0 * t94 - 4.0 / 3.0 * t29 * t114;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t118 = t46 * t46;
        let t122 = 2.0 * t31 + 2.0 * t71;
        let t126 = piecewise3::<f64>(t10, 0.0, 3.0 / 4.0 * t65 * t118 + 3.0 / 2.0 * t13 * t122);
        let t127 = t50 * t50;
        let t130 = -t122;
        let t134 = piecewise3::<f64>(t17, 0.0, 3.0 / 4.0 * t78 * t127 + 3.0 / 2.0 * t18 * t130);
        let t137 = t3 * (t126 / 2.0 + t134 / 2.0);
        let tv2rho22 = -4.0 * t94 - t64 - 4.0 / 3.0 * t29 * t137;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t141 = t4 * t88 * t23;
        let t144 = t4 * t42 * t62;
        let t146 = 1.0 / t28;
        let t149 = t4 * t22 * t146 / 2.0;
        let t150 = 1.0 / t14;
        let t151 = t66 * t33;
        let t154 = t65 * t33;
        let t157 = t30 * t30;
        let t158 = 1.0 / t157;
        let t159 = t5 * t158;
        let t161 = 6.0 * t70 - 6.0 * t159;
        let t165 = piecewise3::<f64>(t10, 0.0, -3.0 / 8.0 * t150 * t151 + 9.0 / 4.0 * t154 * t73 + 3.0 / 2.0 * t13 * t161);
        let t166 = 1.0 / t19;
        let t167 = t79 * t37;
        let t170 = t78 * t37;
        let t173 = -t161;
        let t177 = piecewise3::<f64>(t17, 0.0, -3.0 / 8.0 * t166 * t167 + 9.0 / 4.0 * t170 * t82 + 3.0 / 2.0 * t18 * t173);
        let t179 = t165 / 2.0 + t177 / 2.0;
        let tv3rho30 = -6.0 * t141 - 3.0 * t144 + t149 - 4.0 / 3.0 * t29 * t3 * t179;
        v3rho3[ip * 4] += tv3rho30;
        let t185 = t62 * t1;
        let t186 = t185 * t56;
        let t188 = 4.0 * t93 * t114;
        let t189 = t150 * t46;
        let t192 = t65 * t5;
        let t203 = piecewise3::<f64>(t10, 0.0, -3.0 / 8.0 * t189 * t66 + 3.0 * t192 * t70 * t33 + 3.0 / 4.0 * t96 * t73 + 3.0 * t13 * t70 - 9.0 * t99 * t158);
        let t204 = t166 * t50;
        let t207 = t78 * t5;
        let t218 = piecewise3::<f64>(t17, 0.0, -3.0 / 8.0 * t204 * t79 - 3.0 * t207 * t70 * t37 + 3.0 / 4.0 * t104 * t82 - 3.0 * t18 * t70 + 9.0 * t107 * t158);
        let t221 = t3 * (t203 / 2.0 + t218 / 2.0);
        let tv3rho31 = -2.0 * t141 - 2.0 * t144 + t149 - t186 - t188 - 4.0 / 3.0 * t29 * t221;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let t225 = t93 * t137;
        let t227 = t150 * t118;
        let t232 = t65 * t122;
        let t237 = -2.0 * t70 - 6.0 * t159;
        let t241 = piecewise3::<f64>(t10, 0.0, -3.0 / 8.0 * t227 * t33 + 3.0 * t96 * t71 + 3.0 / 4.0 * t232 * t33 + 3.0 / 2.0 * t13 * t237);
        let t242 = t166 * t127;
        let t247 = t78 * t130;
        let t250 = -t237;
        let t254 = piecewise3::<f64>(t17, 0.0, -3.0 / 8.0 * t242 * t37 - 3.0 * t104 * t71 + 3.0 / 4.0 * t247 * t37 + 3.0 / 2.0 * t18 * t250);
        let t257 = t3 * (t241 / 2.0 + t254 / 2.0);
        let tv3rho32 = -2.0 * t186 - t188 - t144 + t149 - 2.0 * t225 - 4.0 / 3.0 * t29 * t257;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t262 = t118 * t46;
        let t268 = -6.0 * t70 - 6.0 * t159;
        let t272 = piecewise3::<f64>(t10, 0.0, -3.0 / 8.0 * t150 * t262 + 9.0 / 4.0 * t96 * t122 + 3.0 / 2.0 * t13 * t268);
        let t273 = t127 * t50;
        let t278 = -t268;
        let t282 = piecewise3::<f64>(t17, 0.0, -3.0 / 8.0 * t166 * t273 + 9.0 / 4.0 * t104 * t130 + 3.0 / 2.0 * t18 * t278);
        let t285 = t3 * (t272 / 2.0 + t282 / 2.0);
        let tv3rho33 = -3.0 * t186 - 6.0 * t225 + t149 - 4.0 / 3.0 * t29 * t285;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}
