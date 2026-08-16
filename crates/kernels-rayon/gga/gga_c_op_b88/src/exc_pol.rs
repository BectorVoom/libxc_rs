//! GGA_C_OP_B88 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_b88.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_op_b88_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t50 = t41 * t49;
        let t51 = rho0 * rho0;
        let t52 = pow_1_3(rho0);
        let t53 = t52 * t52;
        let t55 = 1.0 / t53 / t51;
        let t56 = sigma0 * t55;
        let t57 = f64::sqrt(sigma0);
        let t59 = 1.0 / t52 / rho0;
        let t60 = t57 * t59;
        let t61 = f64::ln(t60 + f64::sqrt(t60 * t60 + 1.0));
        let t64 = 1.0 + 0.252e-1 * t60 * t61;
        let t65 = 1.0 / t64;
        let t69 = 1.0 + 0.93333333333333333332e-3 * t40 * t56 * t65;
        let t70 = 1.0 / t69;
        let t74 = piecewise3(t32, 0.0, t40 * t50 * t70 / 9.0);
        let t78 = t43 * t2 / 2.0 <= dens_threshold;
        let t79 = piecewise5(t44, t14, t42, t17, -t28);
        let t80 = 1.0 + t79;
        let t81 = t80 * t2;
        let t82 = pow_1_3(t81);
        let t83 = 1.0 / t82;
        let t84 = t41 * t83;
        let t85 = rho1 * rho1;
        let t86 = pow_1_3(rho1);
        let t87 = t86 * t86;
        let t89 = 1.0 / t87 / t85;
        let t90 = sigma2 * t89;
        let t91 = f64::sqrt(sigma2);
        let t93 = 1.0 / t86 / rho1;
        let t94 = t91 * t93;
        let t95 = f64::ln(t94 + f64::sqrt(t94 * t94 + 1.0));
        let t98 = 1.0 + 0.252e-1 * t94 * t95;
        let t99 = 1.0 / t98;
        let t103 = 1.0 + 0.93333333333333333332e-3 * t40 * t90 * t99;
        let t104 = 1.0 / t103;
        let t108 = piecewise3(t78, 0.0, t40 * t84 * t104 / 9.0);
        let t109 = t74 + t108;
        let t110 = t109 == 0.0;
        let t111 = piecewise3(t110, f64::EPSILON, t109);
        let t114 = 0.36011538e1 / t111 + 0.5764e0;
        let t115 = t111 * t111;
        let t116 = t115 * t115;
        let t117 = 1.0 / t116;
        let t119 = t115 * t111;
        let t120 = 1.0 / t119;
        let t122 = 1.0 / t115;
        let t124 = 0.31390124030721e2 * t117 + 0.149643497914092e2 * t120 + 0.17833359087e1 * t122;
        let t125 = 1.0 / t124;
        let t126 = t114 * t125;
        let tzk0 = piecewise3(t11, 0.0, -0.25e0 * t21 * t126);
        zk[ip] += tzk0;
    }
}
