//! GGA_C_PBE_VWN exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbe_vwn.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_pbe_vwn_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_BB: f64,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = f64::sqrt(t10);
        let t14 = t11 + 0.186372e1 * t12 + 0.129352e2;
        let t15 = 1.0 / t14;
        let t19 = f64::ln(t4 * t9 * t15 / 4.0);
        let t20 = 0.310907e-1 * t19;
        let t21 = t12 + 0.372744e1;
        let t24 = f64::atan(0.61519908197590802322e1 / t21);
        let t25 = 0.38783294878113014393e-1 * t24;
        let t26 = t12 / 2.0;
        let t27 = t26 + 0.10498e0;
        let t28 = t27 * t27;
        let t30 = f64::ln(t28 * t15);
        let t31 = 0.96902277115443742139e-3 * t30;
        let t32 = M_PI * M_PI;
        let t33 = 1.0 / t32;
        let t35 = t11 + 0.565535e0 * t12 + 0.130045e2;
        let t36 = 1.0 / t35;
        let t40 = f64::ln(t4 * t9 * t36 / 4.0);
        let t41 = t12 + 0.113107e1;
        let t44 = f64::atan(0.71231089178181179908e1 / t41);
        let t46 = t26 + 0.47584e-2;
        let t47 = t46 * t46;
        let t49 = f64::ln(t47 * t36);
        let t53 = 1.0 <= zeta_threshold;
        let t54 = pow_1_3(zeta_threshold);
        let t56 = piecewise3(t53, t54 * zeta_threshold, 1.0);
        let t59 = M_CBRT2;
        let t60 = t59 - 1.0;
        let t65 = 9.0 * t56 - 9.0;
        let t67 = t33 * (t40 + 0.317708004743941464e0 * t44 + 0.41403379428206274608e-3 * t49) * t65 / 24.0;
        let t68 = t54 * t54;
        let t69 = piecewise3(t53, t68, 1.0);
        let t70 = t69 * t69;
        let t71 = t70 * t69;
        let t72 = param_gamma * t71;
        let t73 = rho[ip] * rho[ip];
        let t75 = 1.0 / t7 / t73;
        let t78 = 1.0 / t70;
        let t79 = t1 * t1;
        let t81 = 1.0 / t3;
        let t82 = t81 * t5;
        let t83 = t78 * t79 * t82;
        let t86 = param_BB * param_beta;
        let t87 = 1.0 / param_gamma;
        let t90 = 1.0 / t71;
        let t92 = f64::exp(-(t20 + t25 + t31 - t67) * t87 * t90);
        let t93 = t92 - 1.0;
        let t94 = 1.0 / t93;
        let t95 = t87 * t94;
        let t96 = sigma[ip] * sigma[ip];
        let t98 = t86 * t95 * t96;
        let t99 = t73 * t73;
        let t100 = t7 * t7;
        let t102 = 1.0 / t100 / t99;
        let t103 = t59 * t59;
        let t104 = t102 * t103;
        let t105 = t70 * t70;
        let t106 = 1.0 / t105;
        let t107 = t104 * t106;
        let t108 = t3 * t3;
        let t109 = 1.0 / t108;
        let t110 = t1 * t109;
        let t111 = t110 * t6;
        let t112 = t107 * t111;
        let t115 = sigma[ip] * t75 * t59 * t83 / 96.0 + t98 * t112 / 3072.0;
        let t116 = param_beta * t115;
        let t117 = param_beta * t87;
        let t120 = t117 * t94 * t115 + 1.0;
        let t121 = 1.0 / t120;
        let t122 = t87 * t121;
        let t124 = t116 * t122 + 1.0;
        let t125 = f64::ln(t124);
        let t126 = t72 * t125;
        let tzk0 = t20 + t25 + t31 - t67 + t126;
        zk[ip] += tzk0;
    }
}
