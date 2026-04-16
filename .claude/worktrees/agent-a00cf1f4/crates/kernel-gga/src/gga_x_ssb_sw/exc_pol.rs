//! GGA_X_SSB_SW exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ssb_sw.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ssb_sw_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
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
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_B * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = sigma0 * t39;
        let t41 = param_C * t28;
        let t42 = t33 * sigma0;
        let t46 = 1.0 + t41 * t42 * t39 / 24.0;
        let t47 = 1.0 / t46;
        let t51 = param_D * t28;
        let t52 = t51 * t33;
        let t53 = t28 * t28;
        let t54 = param_E * t53;
        let t56 = 1.0 / t31 / t30;
        let t57 = sigma0 * sigma0;
        let t59 = t35 * t35;
        let t60 = t59 * rho0;
        let t62 = 1.0 / t36 / t60;
        let t66 = 1.0 + t54 * t56 * t57 * t62 / 576.0;
        let t67 = 1.0 / t66;
        let t71 = param_A + t34 * t40 * t47 / 24.0 - t52 * t40 * t67 / 24.0;
        let t75 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t71);
        let t76 = rho1 <= dens_threshold;
        let t77 = -t16;
        let t79 = piecewise5(t14, t11, t10, t15, t77 * t7);
        let t80 = 1.0 + t79;
        let t81 = t80 <= zeta_threshold;
        let t82 = pow_1_3(t80);
        let t84 = piecewise3(t81, t22, t82 * t80);
        let t85 = t84 * t26;
        let t86 = rho1 * rho1;
        let t87 = pow_1_3(rho1);
        let t88 = t87 * t87;
        let t90 = 1.0 / t88 / t86;
        let t91 = sigma2 * t90;
        let t92 = t33 * sigma2;
        let t96 = 1.0 + t41 * t92 * t90 / 24.0;
        let t97 = 1.0 / t96;
        let t101 = sigma2 * sigma2;
        let t103 = t86 * t86;
        let t104 = t103 * rho1;
        let t106 = 1.0 / t87 / t104;
        let t110 = 1.0 + t54 * t56 * t101 * t106 / 576.0;
        let t111 = 1.0 / t110;
        let t115 = param_A + t34 * t91 * t97 / 24.0 - t52 * t91 * t111 / 24.0;
        let t119 = piecewise3(t76, 0.0, -3.0 / 8.0 * t5 * t85 * t115);
        let tzk0 = t75 + t119;
        zk[ip] += tzk0;
    }
}
