//! MGGA_X_JK exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 84 shared lines across all orders.
//! Delta: 84 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_jk_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_beta: f64,
    param_gamma: f64,
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
        // --- shared preamble (84 lines) ---
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t5 = 1.0 / t4;
        let t6 = t3 * t5;
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
        let t29 = t3 * t3;
        let t30 = param_beta * t29;
        let t32 = pow_1_3(1.0 / M_PI);
        let t33 = 1.0 / t32;
        let t34 = M_CBRT4;
        let t35 = t33 * t34;
        let t36 = t30 * t35;
        let t37 = rho0 * rho0;
        let t38 = pow_1_3(rho0);
        let t39 = t38 * t38;
        let t40 = t39 * t37;
        let t41 = 1.0 / t40;
        let t42 = sigma0 * t41;
        let t43 = param_gamma * param_beta;
        let t44 = f64::sqrt(sigma0);
        let t45 = t38 * rho0;
        let t46 = 1.0 / t45;
        let t47 = t44 * t46;
        let t48 = f64::ln(t47 + f64::sqrt(t47 * t47 + 1.0));
        let t51 = t43 * t47 * t48 + 1.0;
        let t52 = 1.0 / t51;
        let t53 = t39 * rho0;
        let t54 = 1.0 / t53;
        let t56 = -lapl0 * t54 + t42;
        let t57 = 1.0 / sigma0;
        let t58 = t56 * t57;
        let t61 = 2.0 * t40 * t58 + 1.0;
        let t62 = 1.0 / t61;
        let t63 = t52 * t62;
        let t67 = 1.0 + 2.0 / 9.0 * t36 * t42 * t63;
        let t71 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t67);
        let t72 = rho1 <= dens_threshold;
        let t73 = -t17;
        let t75 = piecewise5(t15, t12, t11, t16, t73 * t8);
        let t76 = 1.0 + t75;
        let t77 = t76 <= zeta_threshold;
        let t78 = pow_1_3(t76);
        let t80 = piecewise3(t77, t23, t78 * t76);
        let t81 = t80 * t27;
        let t82 = rho1 * rho1;
        let t83 = pow_1_3(rho1);
        let t84 = t83 * t83;
        let t85 = t84 * t82;
        let t86 = 1.0 / t85;
        let t87 = sigma2 * t86;
        let t88 = f64::sqrt(sigma2);
        let t89 = t83 * rho1;
        let t90 = 1.0 / t89;
        let t91 = t88 * t90;
        let t92 = f64::ln(t91 + f64::sqrt(t91 * t91 + 1.0));
        let t95 = t43 * t91 * t92 + 1.0;
        let t96 = 1.0 / t95;
        let t97 = t84 * rho1;
        let t98 = 1.0 / t97;
        let t100 = -lapl1 * t98 + t87;
        let t101 = 1.0 / sigma2;
        let t102 = t100 * t101;
        let t105 = 2.0 * t102 * t85 + 1.0;
        let t106 = 1.0 / t105;
        let t107 = t96 * t106;
        let t111 = 1.0 + 2.0 / 9.0 * t36 * t87 * t107;
        let t115 = piecewise3(t72, 0.0, -3.0 / 8.0 * t6 * t81 * t111);
        let tzk0 = t71 + t115;
        zk[ip] += tzk0;
    }
}
