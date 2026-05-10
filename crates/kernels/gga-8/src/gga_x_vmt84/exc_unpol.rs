//! GGA_X_VMT84 exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 52 shared lines across all orders.
//! Delta: 52 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_vmt84_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_alpha: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (52 lines) ---
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
        let t21 = param_mu * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t25 * sigma[ip];
        let t27 = t21 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t32 = t31 * t30;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t36 = param_alpha * t20 * t25;
        let t37 = sigma[ip] * t29;
        let t38 = t37 * t33;
        let t41 = f64::exp(-t36 * t38 / 24.0);
        let t42 = t21 * t25;
        let t45 = 1.0 + t42 * t38 / 24.0;
        let t46 = 1.0 / t45;
        let t47 = t41 * t46;
        let t48 = t34 * t47;
        let t51 = t20 * t20;
        let t54 = 1.0 / t23 / t22;
        let t55 = param_alpha * t51 * t54;
        let t56 = sigma[ip] * sigma[ip];
        let t57 = t56 * t28;
        let t58 = t30 * t30;
        let t59 = t58 * rho[ip];
        let t61 = 1.0 / t18 / t59;
        let t65 = f64::exp(-t55 * t57 * t61 / 288.0);
        let t68 = (1.0 - t65) * t51 * t24;
        let t69 = 1.0 / sigma[ip];
        let t70 = t69 * t28;
        let t74 = t27 * t48 / 24.0 + 2.0 * t68 * t70 * t32 + t65;
        let t78 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
    }
}
