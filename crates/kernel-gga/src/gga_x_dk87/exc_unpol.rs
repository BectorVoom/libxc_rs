//! GGA_X_DK87 exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 44 shared lines across all orders.
//! Delta: 44 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_dk87_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_a1: f64,
    param_alpha: f64,
    param_b1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (44 lines) ---
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
        let t20 = 1.0 / M_PI;
        let t21 = M_CBRT6;
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_PI * M_PI;
        let t25 = pow_1_3(t24);
        let t26 = 1.0 / t25;
        let t27 = t3 * t3;
        let t29 = pow_1_3(t20);
        let t30 = 1.0 / t29;
        let t32 = t23 * t26 * t27 * t30;
        let t33 = M_CBRT4;
        let t34 = t33 * sigma[ip];
        let t35 = M_CBRT2;
        let t36 = t35 * t35;
        let t37 = t34 * t36;
        let t38 = rho[ip] * rho[ip];
        let t39 = t18 * t18;
        let t41 = 1.0 / t39 / t38;
        let t42 = f64::sqrt(sigma[ip]);
        let t47 = f64::powf(t42 * t35 / t18 / rho[ip], param_alpha);
        let t48 = param_a1 * t47;
        let t49 = 1.0 + t48;
        let t51 = param_b1 * sigma[ip];
        let t52 = t36 * t41;
        let t54 = t51 * t52 + 1.0;
        let t55 = 1.0 / t54;
        let t56 = t41 * t49 * t55;
        let t60 = 1.0 + 7.0 / 11664.0 * t32 * t37 * t56;
        let t64 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t60);
        let tzk0 = 2.0 * t64;
        zk[ip] += tzk0;
    }
}
