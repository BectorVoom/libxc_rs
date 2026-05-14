//! LDA_X_1D_EXPONENTIAL fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_1d_exponential.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

/// LDA_X_1D_EXPONENTIAL fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_x_1d_exponential_fxc_pol(
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
        let t13 = piecewise5(t7, t9, t11, t12, t5);
        let t14 = 1.0 + t13;
        let t15 = t14 * M_PI;
        let t16 = param_beta * t3;
        let t17 = t15 * t16;
        let t18 = xc_integrate(func1, NULL, 1e-20, t17);
        let t20 = xc_integrate(func2, NULL, 1e-20, t17);
        let t21 = 1.0 / M_PI;
        let t22 = t20 * t21;
        let t23 = 1.0 / param_beta;
        let t24 = t23 * t4;
        let t29 = piecewise3(t8, 0.0, -0.07957747154594767 * (t14 * t18 - t22 * t24) * t23);
        let t31 = rho1 <= dens_threshold || t11;
        let t32 = piecewise5(t11, t9, t7, t12, -t5);
        let t33 = 1.0 + t32;
        let t34 = t33 * M_PI;
        let t35 = t34 * t16;
        let t36 = xc_integrate(func1, NULL, 1e-20, t35);
        let t38 = xc_integrate(func2, NULL, 1e-20, t35);
        let t39 = t38 * t21;
        let t44 = piecewise3(t31, 0.0, -0.07957747154594767 * (-t39 * t24 + t33 * t36) * t23);
        let tzk0 = t29 + t44;
        zk[ip] += tzk0;
        let t45 = t3 * t3;
        let t46 = 1.0 / t45;
        let t47 = t2 * t46;
        let t48 = t4 - t47;
        let t49 = piecewise5(t7, 0.0, t11, 0.0, t48);
        let t51 = t23 * t46;
        let t52 = t22 * t51;
        let t56 = piecewise3(t8, 0.0, -0.07957747154594767 * (t49 * t18 + t52) * t23);
        let t58 = piecewise5(t11, 0.0, t7, 0.0, -t48);
        let t60 = t39 * t51;
        let t64 = piecewise3(t31, 0.0, -0.07957747154594767 * (t58 * t36 + t60) * t23);
        let tvrho0 = t29 + t44 + t3 * (t56 + t64);
        vrho[ip * 2] += tvrho0;
        let t67 = -t4 - t47;
        let t68 = piecewise5(t7, 0.0, t11, 0.0, t67);
        let t73 = piecewise3(t8, 0.0, -0.07957747154594767 * (t68 * t18 + t52) * t23);
        let t75 = piecewise5(t11, 0.0, t7, 0.0, -t67);
        let t80 = piecewise3(t31, 0.0, -0.07957747154594767 * (t75 * t36 + t60) * t23);
        let tvrho1 = t29 + t44 + t3 * (t73 + t80);
        vrho[ip * 2 + 1] += tvrho1;
        let t86 = 1.0 / t45 / t3;
        let t87 = t2 * t86;
        let t89 = -2.0 * t46 + 2.0 * t87;
        let t90 = piecewise5(t7, 0.0, t11, 0.0, t89);
        let t92 = t49 * M_PI;
        let t94 = t15 * param_beta;
        let t95 = t92 * t16 + t94;
        let t96 = t49 * t95;
        let t97 = t14 * t14;
        let t98 = M_PI * M_PI;
        let t99 = t97 * t98;
        let t100 = param_beta * param_beta;
        let t101 = t100 * t45;
        let t103 = xc_E1_scaled(t99 * t101);
        let t105 = t95 * t103;
        let t106 = t14 * t4;
        let t107 = t105 * t106;
        let t108 = t23 * t86;
        let t110 = 2.0 * t22 * t108;
        let t114 = piecewise3(t8, 0.0, -0.07957747154594767 * (t96 * t103 + t90 * t18 + t107 - t110) * t23);
        let t116 = piecewise5(t11, 0.0, t7, 0.0, -t89);
        let t118 = t58 * M_PI;
        let t120 = t34 * param_beta;
        let t121 = t118 * t16 + t120;
        let t122 = t58 * t121;
        let t123 = t33 * t33;
        let t124 = t123 * t98;
        let t126 = xc_E1_scaled(t124 * t101);
        let t128 = t121 * t126;
        let t129 = t33 * t4;
        let t130 = t128 * t129;
        let t132 = 2.0 * t39 * t108;
        let t136 = piecewise3(t31, 0.0, -0.07957747154594767 * (t116 * t36 + t122 * t126 + t130 - t132) * t23);
        let tv2rho20 = 2.0 * t56 + 2.0 * t64 + t3 * (t114 + t136);
        v2rho2[ip * 3] += tv2rho20;
        let t139 = 2.0 * t87;
        let t140 = piecewise5(t7, 0.0, t11, 0.0, t139);
        let t142 = t68 * t95;
        let t147 = piecewise3(t8, 0.0, -0.07957747154594767 * (t142 * t103 + t140 * t18 + t107 - t110) * t23);
        let t148 = piecewise5(t11, 0.0, t7, 0.0, -t139);
        let t150 = t75 * t121;
        let t155 = piecewise3(t31, 0.0, -0.07957747154594767 * (t150 * t126 + t148 * t36 + t130 - t132) * t23);
        let tv2rho21 = t56 + t64 + t73 + t80 + t3 * (t147 + t155);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t161 = 2.0 * t46 + 2.0 * t87;
        let t162 = piecewise5(t7, 0.0, t11, 0.0, t161);
        let t164 = t68 * M_PI;
        let t166 = t164 * t16 + t94;
        let t167 = t68 * t166;
        let t169 = t166 * t103;
        let t174 = piecewise3(t8, 0.0, -0.07957747154594767 * (t167 * t103 + t169 * t106 + t162 * t18 - t110) * t23);
        let t176 = piecewise5(t11, 0.0, t7, 0.0, -t161);
        let t178 = t75 * M_PI;
        let t180 = t178 * t16 + t120;
        let t181 = t75 * t180;
        let t183 = t180 * t126;
        let t188 = piecewise3(t31, 0.0, -0.07957747154594767 * (t181 * t126 + t183 * t129 + t176 * t36 - t132) * t23);
        let tv2rho22 = 2.0 * t73 + 2.0 * t80 + t3 * (t174 + t188);
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
