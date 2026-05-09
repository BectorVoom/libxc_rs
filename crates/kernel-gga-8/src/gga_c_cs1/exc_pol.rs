//! GGA_C_CS1 exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 66 shared lines across all orders.
//! Delta: 66 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_cs1_exc_pol(
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
        // --- shared preamble (66 lines) ---
        let t1 = rho0 - rho1;
        let t2 = t1 * t1;
        let t3 = rho0 + rho1;
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = -t2 * t5 + 1.0;
        let t8 = pow_1_3(t3);
        let t9 = 1.0 / t8;
        let t11 = 1.0 + 0.349e0 * t9;
        let t12 = 1.0 / t11;
        let t13 = t7 * t12;
        let t15 = sigma0 + 2.0 * sigma1 + sigma2;
        let t16 = t15 * t15;
        let t17 = t4 * t4;
        let t18 = t17 * t3;
        let t20 = 1.0 / t8 / t18;
        let t22 = t8 * t8;
        let t24 = 1.0 / t22 / t4;
        let t27 = 1.0 + 0.6e-2 * t15 * t24;
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t32 = -0.159068e0 + 0.286308e-6 * t16 * t20 * t29;
        let t34 = t13 * t32 / 4.0;
        let t35 = 1.0 / t3;
        let t36 = t1 * t35;
        let t37 = 1.0 + t36;
        let t38 = t37 <= zeta_threshold;
        let t39 = piecewise3(t38, zeta_threshold, t37);
        let t40 = pow_1_3(rho0);
        let t41 = t39 * t40;
        let t42 = t40 + 0.349e0;
        let t43 = 1.0 / t42;
        let t44 = sigma0 * sigma0;
        let t45 = rho0 * rho0;
        let t46 = t45 * t45;
        let t47 = t46 * rho0;
        let t49 = 1.0 / t40 / t47;
        let t51 = t40 * t40;
        let t53 = 1.0 / t51 / t45;
        let t56 = 1.0 + 0.6e-2 * sigma0 * t53;
        let t57 = t56 * t56;
        let t58 = 1.0 / t57;
        let t61 = -0.18897e-1 + 0.558864e-5 * t44 * t49 * t58;
        let t62 = t43 * t61;
        let t64 = t41 * t62 / 2.0;
        let t65 = 1.0 - t36;
        let t66 = t65 <= zeta_threshold;
        let t67 = piecewise3(t66, zeta_threshold, t65);
        let t68 = pow_1_3(rho1);
        let t69 = t67 * t68;
        let t70 = t68 + 0.349e0;
        let t71 = 1.0 / t70;
        let t72 = sigma2 * sigma2;
        let t73 = rho1 * rho1;
        let t74 = t73 * t73;
        let t75 = t74 * rho1;
        let t77 = 1.0 / t68 / t75;
        let t79 = t68 * t68;
        let t81 = 1.0 / t79 / t73;
        let t84 = 1.0 + 0.6e-2 * sigma2 * t81;
        let t85 = t84 * t84;
        let t86 = 1.0 / t85;
        let t89 = -0.18897e-1 + 0.558864e-5 * t72 * t77 * t86;
        let t90 = t71 * t89;
        let t92 = t69 * t90 / 2.0;
        let tzk0 = t34 + t64 + t92;
        zk[ip] += tzk0;
    }
}
