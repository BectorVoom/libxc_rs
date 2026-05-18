//! LDA_C_CHACHIYO fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_chachiyo.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_CHACHIYO fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_chachiyo_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
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
    }
}
