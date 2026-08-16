//! LDA_C_RC04 fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_rc04.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_RC04 fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rc04_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = 1.0 + t4;
        let t6 = t5 <= zeta_threshold;
        let t7 = pow_1_3::<f64>(zeta_threshold);
        let t8 = t7 * t7;
        let t9 = pow_1_3::<f64>(t5);
        let t10 = t9 * t9;
        let t11 = piecewise3::<f64>(t6, t8, t10);
        let t12 = 1.0 - t4;
        let t13 = t12 <= zeta_threshold;
        let t14 = pow_1_3::<f64>(t12);
        let t15 = t14 * t14;
        let t16 = piecewise3::<f64>(t13, t8, t15);
        let t18 = t11 / 2.0 + t16 / 2.0;
        let t19 = t18 * t18;
        let t20 = t19 * t18;
        let t21 = M_CBRT3;
        let t23 = pow_1_3::<f64>(1.0 / M_PI);
        let t24 = t21 * t23;
        let t25 = M_CBRT4;
        let t26 = t25 * t25;
        let t27 = pow_1_3::<f64>(t2);
        let t32 = 4.88827 + 0.79425925 * t24 * t26 / t27;
        let t33 = f64::atan(t32);
        let t35 = -0.655868 * t33 + 0.897889;
        let t37 = t21 * t21;
        let t38 = t20 * t35 * t37;
        let t39 = 1.0 / t23;
        let t40 = t39 * t25;
        let t41 = t40 * t27;
        let t42 = t38 * t41;
        let tzk0 = t42 / 3.0;
        zk[ip] += tzk0;
        let t43 = 4.0 / 9.0 * t42;
        let t44 = t27 * t2;
        let t46 = t44 * t19 * t35;
        let t47 = t37 * t39;
        let t48 = 1.0 / t9;
        let t49 = t2 * t2;
        let t50 = 1.0 / t49;
        let t51 = t1 * t50;
        let t52 = t3 - t51;
        let t55 = piecewise3::<f64>(t6, 0.0, 2.0 / 3.0 * t48 * t52);
        let t56 = 1.0 / t14;
        let t57 = -t52;
        let t60 = piecewise3::<f64>(t13, 0.0, 2.0 / 3.0 * t56 * t57);
        let t62 = t55 / 2.0 + t60 / 2.0;
        let t66 = t32 * t32;
        let t67 = t66 + 1.0;
        let t68 = 1.0 / t67;
        let t70 = 0.6945723010386666 * t20 * t68;
        let tvrho0 = t46 * t47 * t25 * t62 + t43 + t70;
        vrho[ip * 2] += tvrho0;
        let t71 = -t3 - t51;
        let t74 = piecewise3::<f64>(t6, 0.0, 2.0 / 3.0 * t48 * t71);
        let t75 = -t71;
        let t78 = piecewise3::<f64>(t13, 0.0, 2.0 / 3.0 * t56 * t75);
        let t80 = t74 / 2.0 + t78 / 2.0;
        let t82 = t47 * t25 * t80;
        let tvrho1 = t46 * t82 + t43 + t70;
        vrho[ip * 2 + 1] += tvrho1;
        let t85 = t19 * t35 * t37;
        let t86 = t27 * t62;
        let t88 = t85 * t40 * t86;
        let t92 = 0.9260964013848889 * t20 * t3 * t68;
        let t93 = t27 * t27;
        let t94 = 1.0 / t93;
        let t97 = 4.0 / 27.0 * t38 * t40 * t94;
        let t98 = t44 * t18;
        let t99 = t98 * t35;
        let t100 = t62 * t62;
        let t105 = t19 * t68;
        let t106 = t105 * t62;
        let t109 = 1.0 / t9 / t5;
        let t110 = t52 * t52;
        let t113 = t49 * t2;
        let t114 = 1.0 / t113;
        let t115 = t1 * t114;
        let t117 = -2.0 * t50 + 2.0 * t115;
        let t121 = piecewise3::<f64>(t6, 0.0, -2.0 / 9.0 * t109 * t110 + 2.0 / 3.0 * t48 * t117);
        let t123 = 1.0 / t14 / t12;
        let t124 = t57 * t57;
        let t127 = -t117;
        let t131 = piecewise3::<f64>(t13, 0.0, -2.0 / 9.0 * t123 * t124 + 2.0 / 3.0 * t56 * t127);
        let t133 = t121 / 2.0 + t131 / 2.0;
        let t135 = t47 * t25 * t133;
        let t137 = t67 * t67;
        let t138 = 1.0 / t137;
        let t139 = t20 * t138;
        let t141 = 1.0 / t44;
        let t143 = t24 * t26 * t141;
        let t145 = 0.3677803165958304 * t139 * t32 * t143;
        let tv2rho20 = 8.0 / 3.0 * t88 + t92 + t97 + 2.0 * t99 * t47 * t25 * t100 + 4.167433806232 * t106 + t46 * t135 + t145;
        v2rho2[ip * 3] += tv2rho20;
        let t148 = t27 * t19 * t35;
        let t149 = t148 * t82;
        let t151 = t35 * t37;
        let t152 = t98 * t151;
        let t153 = t80 * t62;
        let t154 = t40 * t153;
        let t157 = t105 * t80;
        let t159 = t109 * t71;
        let t162 = t48 * t1;
        let t166 = piecewise3::<f64>(t6, 0.0, -2.0 / 9.0 * t159 * t52 + 4.0 / 3.0 * t162 * t114);
        let t167 = t123 * t75;
        let t170 = t56 * t1;
        let t174 = piecewise3::<f64>(t13, 0.0, -2.0 / 9.0 * t167 * t57 - 4.0 / 3.0 * t170 * t114);
        let t176 = t166 / 2.0 + t174 / 2.0;
        let t178 = t47 * t25 * t176;
        let tv2rho21 = 4.0 / 3.0 * t88 + t92 + t97 + 4.0 / 3.0 * t149 + 2.0 * t152 * t154 + 2.083716903116 * t157 + t46 * t178 + 2.083716903116 * t106 + t145;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t182 = t80 * t80;
        let t184 = t47 * t25 * t182;
        let t188 = t71 * t71;
        let t192 = 2.0 * t50 + 2.0 * t115;
        let t196 = piecewise3::<f64>(t6, 0.0, -2.0 / 9.0 * t109 * t188 + 2.0 / 3.0 * t48 * t192);
        let t197 = t75 * t75;
        let t200 = -t192;
        let t204 = piecewise3::<f64>(t13, 0.0, -2.0 / 9.0 * t123 * t197 + 2.0 / 3.0 * t56 * t200);
        let t206 = t196 / 2.0 + t204 / 2.0;
        let t208 = t47 * t25 * t206;
        let tv2rho22 = 8.0 / 3.0 * t149 + t92 + t97 + 2.0 * t99 * t184 + 4.167433806232 * t157 + t46 * t208 + t145;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
