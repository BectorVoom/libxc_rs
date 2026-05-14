//! GGA_C_LYP exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lyp.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_lyp_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
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
        let t2 = t1 * t1;
        let t3 = rho0 + rho1;
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = -t2 * t5 + 1.0;
        let t8 = pow_1_3(t3);
        let t9 = 1.0 / t8;
        let t11 = param_d * t9 + 1.0;
        let t12 = 1.0 / t11;
        let t15 = f64::exp(-param_c * t9);
        let t16 = param_b * t15;
        let t18 = sigma0 + 2.0 * sigma1 + sigma2;
        let t19 = t8 * t8;
        let t21 = 1.0 / t19 / t4;
        let t22 = t18 * t21;
        let t24 = param_d * t12 + param_c;
        let t25 = t24 * t9;
        let t27 = 47.0 - 7.0 * t25;
        let t30 = t7 * t27 / 72.0 - 2.0 / 3.0;
        let t32 = M_CBRT3;
        let t33 = t32 * t32;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = t33 * t36;
        let t38 = 1.0 / t3;
        let t39 = t1 * t38;
        let t40 = 1.0 + t39;
        let t41 = t40 <= zeta_threshold;
        let t42 = zeta_threshold * zeta_threshold;
        let t43 = pow_1_3(zeta_threshold);
        let t44 = t43 * t43;
        let t45 = t44 * t42;
        let t46 = t40 * t40;
        let t47 = pow_1_3(t40);
        let t48 = t47 * t47;
        let t49 = t48 * t46;
        let t50 = piecewise3(t41, t45, t49);
        let t51 = 1.0 - t39;
        let t52 = t51 <= zeta_threshold;
        let t53 = t51 * t51;
        let t54 = pow_1_3(t51);
        let t55 = t54 * t54;
        let t56 = t55 * t53;
        let t57 = piecewise3(t52, t45, t56);
        let t58 = t50 + t57;
        let t62 = M_CBRT2;
        let t63 = t62 * t7;
        let t65 = 5.0 / 2.0 - t25 / 18.0;
        let t66 = rho0 * rho0;
        let t67 = pow_1_3(rho0);
        let t68 = t67 * t67;
        let t70 = 1.0 / t68 / t66;
        let t71 = sigma0 * t70;
        let t72 = t71 * t50;
        let t73 = rho1 * rho1;
        let t74 = pow_1_3(rho1);
        let t75 = t74 * t74;
        let t77 = 1.0 / t75 / t73;
        let t78 = sigma2 * t77;
        let t79 = t78 * t57;
        let t80 = t72 + t79;
        let t81 = t65 * t80;
        let t84 = t25 - 11.0;
        let t86 = t44 * t42 * zeta_threshold;
        let t89 = piecewise3(t41, t86, t48 * t46 * t40);
        let t93 = piecewise3(t52, t86, t55 * t53 * t51);
        let t95 = t71 * t89 + t78 * t93;
        let t96 = t84 * t95;
        let t101 = piecewise3(t41, t42, t46);
        let t102 = t101 * sigma2;
        let t103 = t77 * t57;
        let t106 = piecewise3(t52, t42, t53);
        let t107 = t106 * sigma0;
        let t108 = t70 * t50;
        let t114 = -t22 * t30 - 3.0 / 20.0 * t37 * t7 * t58 + t63 * t81 / 32.0 + t63 * t96 / 576.0 - t62 * (2.0 / 3.0 * t72 + 2.0 / 3.0 * t79 - t102 * t103 / 4.0 - t107 * t108 / 4.0) / 8.0;
        let tzk0 = param_a * (t16 * t12 * t114 - t7 * t12);
        zk[ip] += tzk0;
    }
}
