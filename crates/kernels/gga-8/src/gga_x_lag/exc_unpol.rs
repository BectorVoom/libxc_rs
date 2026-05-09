//! GGA_X_LAG exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 25 shared lines across all orders.
//! Delta: 25 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_lag_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (25 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = pow_1_3(zeta_threshold);
        let t12 = pow_1_3(t8);
        let t14 = piecewise3(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = t3 * t14;
        let t16 = pow_1_3(rho[ip]);
        let t17 = M_CBRT6;
        let t18 = t17 * t17;
        let t19 = M_PI * M_PI;
        let t20 = pow_1_3(t19);
        let t21 = 1.0 / t20;
        let t22 = t18 * t21;
        let t23 = f64::sqrt(sigma[ip]);
        let t24 = M_CBRT2;
        let t29 = t22 * t23 * t24 / t16 / rho[ip];
        let t30 = f64::powf(t29, 0.2626712e1);
        let t33 = 1.0 + 0.13471619689594796103e-3 * t30;
        let t34 = f64::powf(t33, -0.657946e0);
        let t38 = piecewise3(t2, 0.0, -0.15400028771927569605e-4 * t15 * t16 * t30 * t34);
        let tzk0 = 2.0 * t38;
        zk[ip] += tzk0;
    }
}
