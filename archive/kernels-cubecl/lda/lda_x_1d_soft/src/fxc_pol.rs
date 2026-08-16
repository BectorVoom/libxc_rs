//! LDA_X_1D_SOFT fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_1d_soft.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::bessel::{xc_bessel_K0, xc_bessel_K1};
use libxc_kernel_math::integrate::{xc_integrate_lda_soft_func1, xc_integrate_lda_soft_func2};

/// LDA_X_1D_SOFT fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_x_1d_soft_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t7 = 1.0 + t5 <= zeta_threshold;
        let t8 = rho0 <= dens_threshold || t7;
        let t9 = zeta_threshold - 1.0;
        let t11 = 1.0 - t5 <= zeta_threshold;
        let t12 = -t9;
        let t13 = piecewise5::<f64>(t7, t9, t11, t12, t5);
        let t14 = 1.0 + t13;
        let t15 = t14 * M_PI;
        let t16 = param_beta * t3;
        let t17 = t15 * t16;
        let t18 = xc_integrate_lda_soft_func1::<f64>(t17);
        let t20 = xc_integrate_lda_soft_func2::<f64>(t17);
        let t21 = 1.0 / M_PI;
        let t22 = t20 * t21;
        let t23 = 1.0 / param_beta;
        let t24 = t23 * t4;
        let t29 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t14 * t18 - t22 * t24) * t23);
        let t31 = rho1 <= dens_threshold || t11;
        let t32 = piecewise5::<f64>(t11, t9, t7, t12, -t5);
        let t33 = 1.0 + t32;
        let t34 = t33 * M_PI;
        let t35 = t34 * t16;
        let t36 = xc_integrate_lda_soft_func1::<f64>(t35);
        let t38 = xc_integrate_lda_soft_func2::<f64>(t35);
        let t39 = t38 * t21;
        let t44 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (-t39 * t24 + t33 * t36) * t23);
        let tzk0 = t29 + t44;
        zk[ip] += tzk0;
        let t45 = t3 * t3;
        let t46 = 1.0 / t45;
        let t47 = t2 * t46;
        let t48 = t4 - t47;
        let t49 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t48);
        let t51 = t23 * t46;
        let t52 = t22 * t51;
        let t56 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t49 * t18 + t52) * t23);
        let t58 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t48);
        let t60 = t39 * t51;
        let t64 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (t58 * t36 + t60) * t23);
        let tvrho0 = t29 + t44 + t3 * (t56 + t64);
        vrho[ip * 2] += tvrho0;
        let t67 = -t4 - t47;
        let t68 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t67);
        let t73 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t68 * t18 + t52) * t23);
        let t75 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t67);
        let t80 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (t75 * t36 + t60) * t23);
        let tvrho1 = t29 + t44 + t3 * (t73 + t80);
        vrho[ip * 2 + 1] += tvrho1;
        let t86 = 1.0 / t45 / t3;
        let t87 = t2 * t86;
        let t89 = -2.0 * t46 + 2.0 * t87;
        let t90 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t89);
        let t92 = t49 * M_PI;
        let t94 = t15 * param_beta;
        let t95 = t92 * t16 + t94;
        let t96 = t49 * t95;
        let t97 = xc_bessel_K0::<f64>( t17);
        let t100 = t95 * t97;
        let t101 = t14 * t4;
        let t103 = 2.0 * t100 * t101;
        let t104 = t23 * t86;
        let t106 = 2.0 * t22 * t104;
        let t110 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t90 * t18 + 2.0 * t96 * t97 + t103 - t106) * t23);
        let t112 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t89);
        let t114 = t58 * M_PI;
        let t116 = t34 * param_beta;
        let t117 = t114 * t16 + t116;
        let t118 = t58 * t117;
        let t119 = xc_bessel_K0::<f64>( t35);
        let t122 = t117 * t119;
        let t123 = t33 * t4;
        let t125 = 2.0 * t122 * t123;
        let t127 = 2.0 * t39 * t104;
        let t131 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (t112 * t36 + 2.0 * t118 * t119 + t125 - t127) * t23);
        let tv2rho20 = 2.0 * t56 + 2.0 * t64 + t3 * (t110 + t131);
        v2rho2[ip * 3] += tv2rho20;
        let t134 = 2.0 * t87;
        let t135 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t134);
        let t137 = t68 * t95;
        let t143 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t135 * t18 + 2.0 * t137 * t97 + t103 - t106) * t23);
        let t144 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t134);
        let t146 = t75 * t117;
        let t152 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (t144 * t36 + 2.0 * t146 * t119 + t125 - t127) * t23);
        let tv2rho21 = t56 + t64 + t73 + t80 + t3 * (t143 + t152);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t158 = 2.0 * t46 + 2.0 * t87;
        let t159 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t158);
        let t161 = t68 * M_PI;
        let t163 = t161 * t16 + t94;
        let t164 = t68 * t163;
        let t167 = t163 * t97;
        let t173 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t159 * t18 + 2.0 * t164 * t97 + 2.0 * t167 * t101 - t106) * t23);
        let t175 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t158);
        let t177 = t75 * M_PI;
        let t179 = t177 * t16 + t116;
        let t180 = t75 * t179;
        let t183 = t179 * t119;
        let t189 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (t175 * t36 + 2.0 * t180 * t119 + 2.0 * t183 * t123 - t127) * t23);
        let tv2rho22 = 2.0 * t73 + 2.0 * t80 + t3 * (t173 + t189);
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
