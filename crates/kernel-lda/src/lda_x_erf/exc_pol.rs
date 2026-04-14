//! LDA_X_ERF exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 93 shared lines across all orders.
//! Delta: 93 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::erf::{erf_approx};

/// LDA_X_ERF exc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_x_erf_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (93 lines) ---
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = t1 * t3 * t6;
        let t8 = M_CBRT2;
        let t9 = t8 * t8;
        let t10 = rho0 - rho1;
        let t11 = rho0 + rho1;
        let t12 = 1.0 / t11;
        let t13 = t10 * t12;
        let t14 = 1.0 + t13;
        let t15 = t14 <= zeta_threshold;
        let t16 = pow_1_3(zeta_threshold);
        let t17 = t16 * zeta_threshold;
        let t18 = pow_1_3(t14);
        let t20 = piecewise3(t15, t17, t18 * t14);
        let t21 = t9 * t20;
        let t22 = pow_1_3(t11);
        let t23 = pow_1_3(9.0);
        let t24 = t23 * t23;
        let t25 = t3 * t3;
        let t26 = t24 * t25;
        let t27 = t26 * param_hyb_omega_0;
        let t28 = 1.0 / t22;
        let t29 = t1 * t28;
        let t30 = piecewise3(t15, t16, t18);
        let t31 = 1.0 / t30;
        let t34 = t27 * t29 * t31 / 18.0;
        let t35 = 1.35 <= t34;
        let t36 = 1.35 < t34;
        let t37 = piecewise3(t36, t34, 1.35);
        let t38 = t37 * t37;
        let t41 = t38 * t38;
        let t42 = 1.0 / t41;
        let t44 = t41 * t38;
        let t45 = 1.0 / t44;
        let t47 = t41 * t41;
        let t48 = 1.0 / t47;
        let t51 = 1.0 / t47 / t38;
        let t54 = 1.0 / t47 / t41;
        let t57 = 1.0 / t47 / t44;
        let t59 = t47 * t47;
        let t60 = 1.0 / t59;
        let t63 = piecewise3(t36, 1.35, t34);
        let t64 = f64::sqrt(M_PI);
        let t65 = 1.0 / t63;
        let t67 = erf_approx(t65 / 2.0);
        let t69 = t63 * t63;
        let t70 = 1.0 / t69;
        let t72 = f64::exp(-t70 / 4.0);
        let t73 = t72 - 1.0;
        let t76 = t72 - 3.0 / 2.0 - 2.0 * t69 * t73;
        let t79 = 2.0 * t63 * t76 + t64 * t67;
        let t83 = piecewise3(t35, 1.0 / t38 / 36.0 - t42 / 960.0 + t45 / 26880.0 - t48 / 829440.0 + t51 / 28385280.0 - t54 / 1073479680.0 + t57 / 44590694400.0 - t60 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t63 * t79);
        let t84 = t22 * t83;
        let t86 = t7 * t21 * t84;
        let t87 = 1.0 - t13;
        let t88 = t87 <= zeta_threshold;
        let t89 = pow_1_3(t87);
        let t91 = piecewise3(t88, t17, t89 * t87);
        let t92 = t9 * t91;
        let t93 = piecewise3(t88, t16, t89);
        let t94 = 1.0 / t93;
        let t97 = t27 * t29 * t94 / 18.0;
        let t98 = 1.35 <= t97;
        let t99 = 1.35 < t97;
        let t100 = piecewise3(t99, t97, 1.35);
        let t101 = t100 * t100;
        let t104 = t101 * t101;
        let t105 = 1.0 / t104;
        let t107 = t104 * t101;
        let t108 = 1.0 / t107;
        let t110 = t104 * t104;
        let t111 = 1.0 / t110;
        let t114 = 1.0 / t110 / t101;
        let t117 = 1.0 / t110 / t104;
        let t120 = 1.0 / t110 / t107;
        let t122 = t110 * t110;
        let t123 = 1.0 / t122;
        let t126 = piecewise3(t99, 1.35, t97);
        let t127 = 1.0 / t126;
        let t129 = erf_approx(t127 / 2.0);
        let t131 = t126 * t126;
        let t132 = 1.0 / t131;
        let t134 = f64::exp(-t132 / 4.0);
        let t135 = t134 - 1.0;
        let t138 = t134 - 3.0 / 2.0 - 2.0 * t131 * t135;
        let t141 = 2.0 * t126 * t138 + t64 * t129;
        let t145 = piecewise3(t98, 1.0 / t101 / 36.0 - t105 / 960.0 + t108 / 26880.0 - t111 / 829440.0 + t114 / 28385280.0 - t117 / 1073479680.0 + t120 / 44590694400.0 - t123 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t126 * t141);
        let t146 = t22 * t145;
        let t148 = t7 * t92 * t146;
        let tzk0 = -3.0 / 32.0 * t86 - 3.0 / 32.0 * t148;
        zk[ip] += tzk0;
    }
}
