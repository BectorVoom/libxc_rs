//! GGA_C_OP_G96 exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 73 shared lines across all orders.
//! Delta: 73 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_op_g96_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
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
        // --- shared preamble (73 lines) ---
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = f64::abs(t4);
        let t11 = 1.0 - t5 <= zeta_threshold || rho0 <= dens_threshold && rho1 <= dens_threshold;
        let t13 = 1.0 + t4 <= zeta_threshold;
        let t14 = zeta_threshold - 1.0;
        let t16 = 1.0 - t4 <= zeta_threshold;
        let t17 = -t14;
        let t18 = piecewise5(t13, t14, t16, t17, t4);
        let t19 = t18 * t18;
        let t20 = 1.0 - t19;
        let t21 = t20 * t2;
        let t24 = 2.0 * rho0 * t3 <= zeta_threshold;
        let t27 = 2.0 * rho1 * t3 <= zeta_threshold;
        let t28 = piecewise5(t24, t14, t27, t17, t4);
        let t29 = 1.0 + t28;
        let t32 = t29 * t2 / 2.0 <= dens_threshold;
        let t33 = M_CBRT3;
        let t34 = t33 * t33;
        let t36 = pow_1_3(1.0 / M_PI);
        let t37 = 1.0 / t36;
        let t38 = t34 * t37;
        let t39 = M_CBRT4;
        let t40 = t38 * t39;
        let t41 = M_CBRT2;
        let t42 = t29 <= zeta_threshold;
        let t43 = 1.0 - t28;
        let t44 = t43 <= zeta_threshold;
        let t45 = piecewise5(t42, t14, t44, t17, t28);
        let t46 = 1.0 + t45;
        let t47 = t46 * t2;
        let t48 = pow_1_3(t47);
        let t49 = 1.0 / t48;
        let t51 = f64::sqrt(sigma0);
        let t52 = pow_1_3(rho0);
        let t54 = 1.0 / t52 / rho0;
        let t55 = t51 * t54;
        let t56 = f64::sqrt(t55);
        let t57 = t56 * t55;
        let t61 = 1.0 + 2.0 / 1233.0 * t38 * t39 * t57;
        let t62 = 1.0 / t61;
        let t66 = piecewise3(t32, 0.0, t40 * t41 * t49 * t62 / 9.0);
        let t70 = t43 * t2 / 2.0 <= dens_threshold;
        let t71 = piecewise5(t44, t14, t42, t17, -t28);
        let t72 = 1.0 + t71;
        let t73 = t72 * t2;
        let t74 = pow_1_3(t73);
        let t75 = 1.0 / t74;
        let t77 = f64::sqrt(sigma2);
        let t78 = pow_1_3(rho1);
        let t80 = 1.0 / t78 / rho1;
        let t81 = t77 * t80;
        let t82 = f64::sqrt(t81);
        let t83 = t82 * t81;
        let t87 = 1.0 + 2.0 / 1233.0 * t38 * t39 * t83;
        let t88 = 1.0 / t87;
        let t92 = piecewise3(t70, 0.0, t40 * t41 * t75 * t88 / 9.0);
        let t93 = t66 + t92;
        let t94 = t93 == 0.0;
        let t95 = piecewise3(t94, f64::EPSILON, t93);
        let t98 = 0.359628532e1 / t95 + 0.5764e0;
        let t99 = t95 * t95;
        let t100 = t99 * t99;
        let t101 = 1.0 / t100;
        let t103 = t99 * t95;
        let t104 = 1.0 / t103;
        let t106 = 1.0 / t99;
        let t108 = 0.312207199195441936e2 * t101 + 0.149037398922132448e2 * t104 + 0.1778517305052e1 * t106;
        let t109 = 1.0 / t108;
        let t110 = t98 * t109;
        let tzk0 = piecewise3(t11, 0.0, -0.25e0 * t21 * t110);
        zk[ip] += tzk0;
    }
}
