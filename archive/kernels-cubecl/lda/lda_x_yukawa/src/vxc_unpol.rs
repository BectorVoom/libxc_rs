//! LDA_X_YUKAWA vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_yukawa.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_X_YUKAWA vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_x_yukawa_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t3 = pow_1_3::<f64>(1.0 / M_PI);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = t6 * t3 * t1;
        let t8 = M_CBRT2;
        let t9 = t8 * t8;
        let t10 = 1.0 <= zeta_threshold;
        let t11 = pow_1_3::<f64>(zeta_threshold);
        let t13 = piecewise3::<f64>(t10, t11 * zeta_threshold, 1.0);
        let t14 = t13 * t9;
        let t15 = pow_1_3::<f64>(rho[ip]);
        let t16 = pow_1_3::<f64>(9.0);
        let t17 = t16 * t16;
        let t18 = t3 * t3;
        let t20 = param_hyb_omega_0 * t18 * t17;
        let t23 = piecewise3::<f64>(t10, t11, 1.0);
        let t24 = 1.0 / t23;
        let t27 = t24 / t15 * t1 * t20 / 18.0;
        let t28 = 1.92 <= t27;
        let t29 = 1.92 < t27;
        let t30 = piecewise3::<f64>(t29, t27, 1.92);
        let t31 = t30 * t30;
        let t34 = t31 * t31;
        let t35 = 1.0 / t34;
        let t37 = t34 * t31;
        let t38 = 1.0 / t37;
        let t40 = t34 * t34;
        let t41 = 1.0 / t40;
        let t43 = t40 * t31;
        let t44 = 1.0 / t43;
        let t46 = t40 * t34;
        let t47 = 1.0 / t46;
        let t49 = t40 * t37;
        let t50 = 1.0 / t49;
        let t52 = t40 * t40;
        let t53 = 1.0 / t52;
        let t56 = 1.0 / t52 / t31;
        let t59 = 1.0 / t52 / t34;
        let t62 = 1.0 / t52 / t37;
        let t65 = 1.0 / t52 / t40;
        let t68 = 1.0 / t52 / t43;
        let t71 = 1.0 / t52 / t46;
        let t74 = 1.0 / t52 / t49;
        let t76 = t52 * t52;
        let t77 = 1.0 / t76;
        let t80 = 1.0 / t76 / t31;
        let t83 = 1.0 / t76 / t34;
        let t85 = 1.0 / t31 / 9.0 - t35 / 30.0 + t38 / 70.0 - t41 / 135.0 + t44 / 231.0 - t47 / 364.0 + t50 / 540.0 - t53 / 765.0 + t56 / 1045.0 - t59 / 1386.0 + t62 / 1794.0 - t65 / 2275.0 + t68 / 2835.0 - t71 / 3480.0 + t74 / 4216.0 - t77 / 5049.0 + t80 / 5985.0 - t83 / 7030.0;
        let t86 = piecewise3::<f64>(t29, 1.92, t27);
        let t87 = f64::atan2(1.0, t86);
        let t88 = t86 * t86;
        let t89 = t88 + 3.0;
        let t90 = 1.0 / t88;
        let t91 = 1.0 + t90;
        let t92 = f64::ln(t91);
        let t94 = -t92 * t89 + 1.0;
        let t97 = t87 + t94 * t86 / 4.0;
        let t101 = piecewise3::<f64>(t28, t85, 1.0 - 8.0 / 3.0 * t97 * t86);
        let t104 = t101 * t15 * t14 * t7;
        let tzk0 = -3.0 / 16.0 * t104;
        zk[ip] += tzk0;
        let t107 = t15 * rho[ip];
        let t109 = t3 * t1 * t107;
        let t110 = t9 * t6;
        let t111 = t31 * t30;
        let t112 = 1.0 / t111;
        let t117 = t24 / t107 * t1 * t20 / 54.0;
        let t118 = piecewise3::<f64>(t29, -t117, 0.0);
        let t121 = t34 * t30;
        let t122 = 1.0 / t121;
        let t125 = t34 * t111;
        let t126 = 1.0 / t125;
        let t129 = t40 * t30;
        let t130 = 1.0 / t129;
        let t133 = t40 * t111;
        let t134 = 1.0 / t133;
        let t137 = t40 * t121;
        let t138 = 1.0 / t137;
        let t141 = t40 * t125;
        let t142 = 1.0 / t141;
        let t146 = 1.0 / t52 / t30;
        let t150 = 1.0 / t52 / t111;
        let t154 = 1.0 / t52 / t121;
        let t158 = 1.0 / t52 / t125;
        let t162 = 1.0 / t52 / t129;
        let t166 = 1.0 / t52 / t133;
        let t170 = 1.0 / t52 / t137;
        let t174 = 1.0 / t52 / t141;
        let t178 = 1.0 / t76 / t30;
        let t182 = 1.0 / t76 / t111;
        let t186 = 1.0 / t76 / t121;
        let t189 = -2.0 / 9.0 * t118 * t112 + 2.0 / 15.0 * t118 * t122 - 3.0 / 35.0 * t118 * t126 + 8.0 / 135.0 * t118 * t130 - 10.0 / 231.0 * t118 * t134 + 3.0 / 91.0 * t118 * t138 - 7.0 / 270.0 * t118 * t142 + 16.0 / 765.0 * t118 * t146 - 18.0 / 1045.0 * t118 * t150 + 10.0 / 693.0 * t118 * t154 - 11.0 / 897.0 * t118 * t158 + 24.0 / 2275.0 * t118 * t162 - 26.0 / 2835.0 * t118 * t166 + 7.0 / 870.0 * t118 * t170 - 15.0 / 2108.0 * t118 * t174 + 32.0 / 5049.0 * t118 * t178 - 34.0 / 5985.0 * t118 * t182 + 18.0 / 3515.0 * t118 * t186;
        let t190 = piecewise3::<f64>(t29, 0.0, -t117);
        let t193 = 1.0 / t91;
        let t199 = t88 * t86;
        let t200 = 1.0 / t199;
        let t201 = t200 * t89;
        let t202 = t193 * t190;
        let t205 = -2.0 * t92 * t190 * t86 + 2.0 * t202 * t201;
        let t208 = -t193 * t90 * t190 + t94 * t190 / 4.0 + t205 * t86 / 4.0;
        let t212 = piecewise3::<f64>(t28, t189, -8.0 / 3.0 * t97 * t190 - 8.0 / 3.0 * t208 * t86);
        let tvrho0 = -t104 / 4.0 - 3.0 / 16.0 * t212 * t13 * t110 * t109;
        vrho[ip] += tvrho0;
    }
}
