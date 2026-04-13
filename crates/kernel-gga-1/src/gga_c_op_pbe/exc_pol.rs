//! GGA_C_OP_PBE exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_pbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_op_pbe_exc_pol(
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
        let t38 = t34 / t36;
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
        let t51 = M_CBRT6;
        let t52 = M_PI * M_PI;
        let t53 = pow_1_3(t52);
        let t54 = t53 * t53;
        let t55 = 1.0 / t54;
        let t56 = t51 * t55;
        let t57 = rho0 * rho0;
        let t58 = pow_1_3(rho0);
        let t59 = t58 * t58;
        let t61 = 1.0 / t59 / t57;
        let t65 = 0.804e0 + 0.91464571985215458336e-2 * t56 * sigma0 * t61;
        let t68 = 0.1804e1 - 0.646416e0 / t65;
        let t69 = 1.0 / t68;
        let t73 = piecewise3(t32, 0.0, t40 * t41 * t49 * t69 / 9.0);
        let t77 = t43 * t2 / 2.0 <= dens_threshold;
        let t78 = piecewise5(t44, t14, t42, t17, -t28);
        let t79 = 1.0 + t78;
        let t80 = t79 * t2;
        let t81 = pow_1_3(t80);
        let t82 = 1.0 / t81;
        let t84 = rho1 * rho1;
        let t85 = pow_1_3(rho1);
        let t86 = t85 * t85;
        let t88 = 1.0 / t86 / t84;
        let t92 = 0.804e0 + 0.91464571985215458336e-2 * t56 * sigma2 * t88;
        let t95 = 0.1804e1 - 0.646416e0 / t92;
        let t96 = 1.0 / t95;
        let t100 = piecewise3(t77, 0.0, t40 * t41 * t82 * t96 / 9.0);
        let t101 = t73 + t100;
        let t102 = t101 == 0.0;
        let t103 = piecewise3(t102, f64::EPSILON, t101);
        let t106 = 0.361925846e1 / t103 + 0.5764e0;
        let t107 = t103 * t103;
        let t108 = t107 * t107;
        let t109 = 1.0 / t108;
        let t111 = t107 * t103;
        let t112 = 1.0 / t111;
        let t114 = 1.0 / t107;
        let t116 = 0.320261508740743441e2 * t109 + 0.151911844324290596e2 * t112 + 0.1801312286343e1 * t114;
        let t117 = 1.0 / t116;
        let t118 = t106 * t117;
        let tzk0 = piecewise3(t11, 0.0, -0.25e0 * t21 * t118);
        zk[ip] += tzk0;
    }
}
