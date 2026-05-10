//! GGA_X_MPBE exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 84 shared lines across all orders.
//! Delta: 84 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_mpbe_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
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
        // --- shared preamble (84 lines) ---
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
        let t29 = param_c1 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = param_a * t28;
        let t42 = t33 * sigma0;
        let t46 = 1.0 + t41 * t42 * t39 / 24.0;
        let t47 = 1.0 / t46;
        let t51 = t28 * t28;
        let t52 = param_c2 * t51;
        let t54 = 1.0 / t31 / t30;
        let t55 = t52 * t54;
        let t56 = sigma0 * sigma0;
        let t57 = t35 * t35;
        let t58 = t57 * rho0;
        let t60 = 1.0 / t36 / t58;
        let t62 = t46 * t46;
        let t63 = 1.0 / t62;
        let t67 = t30 * t30;
        let t68 = 1.0 / t67;
        let t69 = param_c3 * t68;
        let t70 = t56 * sigma0;
        let t71 = t57 * t57;
        let t72 = 1.0 / t71;
        let t74 = t62 * t46;
        let t75 = 1.0 / t74;
        let t79 = 1.0 + t34 * sigma0 * t39 * t47 / 24.0 + t55 * t56 * t60 * t63 / 576.0 + t69 * t70 * t72 * t75 / 2304.0;
        let t83 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t79);
        let t84 = rho1 <= dens_threshold;
        let t85 = -t16;
        let t87 = piecewise5(t14, t11, t10, t15, t85 * t7);
        let t88 = 1.0 + t87;
        let t89 = t88 <= zeta_threshold;
        let t90 = pow_1_3(t88);
        let t92 = piecewise3(t89, t22, t90 * t88);
        let t93 = t92 * t26;
        let t94 = rho1 * rho1;
        let t95 = pow_1_3(rho1);
        let t96 = t95 * t95;
        let t98 = 1.0 / t96 / t94;
        let t100 = t33 * sigma2;
        let t104 = 1.0 + t41 * t100 * t98 / 24.0;
        let t105 = 1.0 / t104;
        let t109 = sigma2 * sigma2;
        let t110 = t94 * t94;
        let t111 = t110 * rho1;
        let t113 = 1.0 / t95 / t111;
        let t115 = t104 * t104;
        let t116 = 1.0 / t115;
        let t120 = t109 * sigma2;
        let t121 = t110 * t110;
        let t122 = 1.0 / t121;
        let t124 = t115 * t104;
        let t125 = 1.0 / t124;
        let t129 = 1.0 + t34 * sigma2 * t98 * t105 / 24.0 + t55 * t109 * t113 * t116 / 576.0 + t69 * t120 * t122 * t125 / 2304.0;
        let t133 = piecewise3(t84, 0.0, -3.0 / 8.0 * t5 * t93 * t129);
        let tzk0 = t83 + t133;
        zk[ip] += tzk0;
    }
}
