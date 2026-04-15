//! MGGA_X_GX exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 88 shared lines across all orders.
//! Delta: 88 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5, Heaviside};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_gx_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_alphainf: f64,
    param_c0: f64,
    param_c1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        // --- shared preamble (88 lines) ---
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = M_CBRT2;
        let t30 = t3 * t3;
        let t32 = M_CBRT4;
        let t34 = 8.0 / 27.0 * t29 * t30 * t32;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / rho0;
        let t40 = rho0 * rho0;
        let t42 = 1.0 / t36 / t40;
        let t45 = tau0 * t38 - sigma0 * t42 / 8.0;
        let t46 = M_CBRT6;
        let t48 = M_PI * M_PI;
        let t49 = pow_1_3(t48);
        let t50 = t49 * t49;
        let t51 = 1.0 / t50;
        let t52 = t45 * t46 * t51;
        let t54 = t46 * t51;
        let t57 = param_c0 + 5.0 / 9.0 * param_c1 * t45 * t54;
        let t58 = param_c0 + param_c1 - 1.0;
        let t62 = 1.0 + 5.0 / 9.0 * t58 * t45 * t54;
        let t63 = 1.0 / t62;
        let t65 = 1.0 - t34;
        let t66 = t57 * t63 * t65;
        let t69 = t34 + 5.0 / 9.0 * t52 * t66;
        let t70 = 5.0 / 9.0 * t52;
        let t71 = 1.0 - t70;
        let t72 = Heaviside(t71);
        let t74 = 1.0 - param_alphainf;
        let t75 = t74 * t71;
        let t76 = 1.0 + t70;
        let t77 = 1.0 / t76;
        let t79 = t75 * t77 + 1.0;
        let t80 = -t71;
        let t81 = Heaviside(t80);
        let t83 = t69 * t72 + t79 * t81;
        let t87 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t83);
        let t88 = rho1 <= dens_threshold;
        let t89 = -t17;
        let t91 = piecewise5(t15, t12, t11, t16, t89 * t8);
        let t92 = 1.0 + t91;
        let t93 = t92 <= zeta_threshold;
        let t94 = pow_1_3(t92);
        let t96 = piecewise3(t93, t23, t94 * t92);
        let t97 = t96 * t27;
        let t98 = pow_1_3(rho1);
        let t99 = t98 * t98;
        let t101 = 1.0 / t99 / rho1;
        let t103 = rho1 * rho1;
        let t105 = 1.0 / t99 / t103;
        let t108 = tau1 * t101 - sigma2 * t105 / 8.0;
        let t110 = t108 * t46 * t51;
        let t114 = param_c0 + 5.0 / 9.0 * param_c1 * t108 * t54;
        let t118 = 1.0 + 5.0 / 9.0 * t58 * t108 * t54;
        let t119 = 1.0 / t118;
        let t121 = t114 * t119 * t65;
        let t124 = t34 + 5.0 / 9.0 * t110 * t121;
        let t125 = 5.0 / 9.0 * t110;
        let t126 = 1.0 - t125;
        let t127 = Heaviside(t126);
        let t129 = t74 * t126;
        let t130 = 1.0 + t125;
        let t131 = 1.0 / t130;
        let t133 = t129 * t131 + 1.0;
        let t134 = -t126;
        let t135 = Heaviside(t134);
        let t137 = t124 * t127 + t133 * t135;
        let t141 = piecewise3(t88, 0.0, -3.0 / 8.0 * t6 * t97 * t137);
        let tzk0 = t87 + t141;
        zk[ip] += tzk0;
    }
}
