//! GGA_C_LYPR exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lypr.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_PI};
use libxc_kernel_math::erf::{erfc_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_lypr_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_m1: f64,
    param_m2: f64,
    param_omega: f64,
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
        let t1 = param_m1 * param_omega;
        let t2 = rho0 + rho1;
        let t3 = pow_1_3(t2);
        let t4 = 1.0 / t3;
        let t6 = erfc_approx(t1 * t4);
        let t7 = rho0 - rho1;
        let t8 = t7 * t7;
        let t9 = t2 * t2;
        let t10 = 1.0 / t9;
        let t12 = -t8 * t10 + 1.0;
        let t13 = t6 * t12;
        let t15 = param_d * t4 + 1.0;
        let t16 = 1.0 / t15;
        let t18 = param_m2 * param_omega;
        let t20 = erfc_approx(t18 * t4);
        let t21 = t20 * param_b;
        let t23 = f64::exp(-param_c * t4);
        let t24 = t23 * t16;
        let t26 = sigma0 + 2.0 * sigma1 + sigma2;
        let t27 = t3 * t3;
        let t29 = 1.0 / t27 / t9;
        let t30 = t26 * t29;
        let t32 = param_d * t16 + param_c;
        let t33 = t32 * t4;
        let t35 = 47.0 - 7.0 * t33;
        let t38 = t12 * t35 / 72.0 - 2.0 / 3.0;
        let t40 = M_CBRT3;
        let t41 = t40 * t40;
        let t42 = M_PI * M_PI;
        let t43 = pow_1_3(t42);
        let t44 = t43 * t43;
        let t45 = t41 * t44;
        let t46 = 1.0 / t2;
        let t47 = t7 * t46;
        let t48 = 1.0 + t47;
        let t49 = t48 <= zeta_threshold;
        let t50 = zeta_threshold * zeta_threshold;
        let t51 = pow_1_3(zeta_threshold);
        let t52 = t51 * t51;
        let t53 = t52 * t50;
        let t54 = t48 * t48;
        let t55 = pow_1_3(t48);
        let t56 = t55 * t55;
        let t57 = t56 * t54;
        let t58 = piecewise3(t49, t53, t57);
        let t59 = 1.0 - t47;
        let t60 = t59 <= zeta_threshold;
        let t61 = t59 * t59;
        let t62 = pow_1_3(t59);
        let t63 = t62 * t62;
        let t64 = t63 * t61;
        let t65 = piecewise3(t60, t53, t64);
        let t66 = t58 + t65;
        let t70 = M_CBRT2;
        let t71 = t70 * t12;
        let t73 = 5.0 / 2.0 - t33 / 18.0;
        let t74 = rho0 * rho0;
        let t75 = pow_1_3(rho0);
        let t76 = t75 * t75;
        let t78 = 1.0 / t76 / t74;
        let t79 = sigma0 * t78;
        let t80 = t79 * t58;
        let t81 = rho1 * rho1;
        let t82 = pow_1_3(rho1);
        let t83 = t82 * t82;
        let t85 = 1.0 / t83 / t81;
        let t86 = sigma2 * t85;
        let t87 = t86 * t65;
        let t88 = t80 + t87;
        let t89 = t73 * t88;
        let t92 = t33 - 11.0;
        let t94 = t52 * t50 * zeta_threshold;
        let t97 = piecewise3(t49, t94, t56 * t54 * t48);
        let t101 = piecewise3(t60, t94, t63 * t61 * t59);
        let t103 = t86 * t101 + t79 * t97;
        let t104 = t92 * t103;
        let t109 = piecewise3(t49, t50, t54);
        let t110 = t109 * sigma2;
        let t111 = t85 * t65;
        let t114 = piecewise3(t60, t50, t61);
        let t115 = t114 * sigma0;
        let t116 = t78 * t58;
        let t122 = -t30 * t38 - 3.0 / 20.0 * t45 * t12 * t66 + t71 * t89 / 32.0 + t71 * t104 / 576.0 - t70 * (2.0 / 3.0 * t80 + 2.0 / 3.0 * t87 - t110 * t111 / 4.0 - t115 * t116 / 4.0) / 8.0;
        let t123 = t24 * t122;
        let t125 = param_b * t23;
        let t126 = f64::sqrt(M_PI);
        let t127 = 1.0 / t126;
        let t128 = t16 * t127;
        let t130 = t125 * t128 * param_m2;
        let t131 = param_m2 * param_m2;
        let t132 = param_omega * param_omega;
        let t134 = 1.0 / t27;
        let t136 = f64::exp(-t131 * t132 * t134);
        let t137 = param_omega * t136;
        let t138 = t4 * t12;
        let t142 = t47 / 6.0;
        let t143 = 7.0 / 6.0 + t142;
        let t144 = t143 * sigma0;
        let t145 = t70 * t78;
        let t146 = t145 * t58;
        let t149 = 7.0 / 6.0 - t142;
        let t150 = t149 * sigma2;
        let t151 = t70 * t85;
        let t152 = t151 * t65;
        let t155 = 7.0 / 6.0 * t30 - 7.0 / 48.0 * t70 * t88 + t144 * t146 / 8.0 + t150 * t152 / 8.0;
        let tzk0 = param_a * (-t13 * t16 + t21 * t123 + t130 * t137 * t138 * t155 / 6.0);
        zk[ip] += tzk0;
    }
}
