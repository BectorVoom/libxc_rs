//! MGGA_X_M08 exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 114 shared lines across all orders.
//! Delta: 114 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_m08_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
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
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (114 lines) ---
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
    }
}
