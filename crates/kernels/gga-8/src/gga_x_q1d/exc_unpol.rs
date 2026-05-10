//! GGA_X_Q1D exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 54 shared lines across all orders.
//! Delta: 54 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_q1d_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (54 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t33 = t28 * t32;
        let t34 = t25 * t33;
        let t36 = 0.804e0 + 5.0 / 972.0 * t34;
        let t38 = 0.646416e0 / t36;
        let t40 = t20 * t20;
        let t42 = 1.0 / t22 / t21;
        let t43 = t40 * t42;
        let t44 = sigma[ip] * sigma[ip];
        let t45 = t44 * t26;
        let t46 = t29 * t29;
        let t47 = t46 * rho[ip];
        let t49 = 1.0 / t18 / t47;
        let t52 = t43 * t45 * t49 / 288.0;
        let t53 = t34 / 24.0 + t52;
        let t54 = t21 * t21;
        let t55 = 1.0 / t54;
        let t56 = t44 * sigma[ip];
        let t57 = t55 * t56;
        let t58 = t46 * t46;
        let t59 = 1.0 / t58;
        let t62 = 1.0 + t52 + t57 * t59 / 576.0;
        let t63 = 1.0 / t62;
        let t64 = t53 * t63;
        let t66 = (0.1804e1 - t38) * t20;
        let t67 = t66 * t24;
        let t70 = -t67 * t33 / 24.0 + 0.6525e-1;
        let t72 = 0.1804e1 - t38 + t64 * t70;
        let t76 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t72);
        let tzk0 = 2.0 * t76;
        zk[ip] += tzk0;
    }
}
