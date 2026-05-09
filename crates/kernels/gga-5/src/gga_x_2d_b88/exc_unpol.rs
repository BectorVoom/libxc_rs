//! GGA_X_2D_B88 exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 27 shared lines across all orders.
//! Delta: 27 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_2d_b88_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (27 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = f64::sqrt(M_PI);
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = f64::sqrt(zeta_threshold);
        let t13 = f64::sqrt(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t16 = 1.0 / t3 * t15;
        let t17 = M_SQRT2;
        let t18 = f64::sqrt(rho[ip]);
        let t19 = t17 * t18;
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t22 = 1.0 / t21;
        let t23 = sigma[ip] * t22;
        let t24 = f64::sqrt(sigma[ip]);
        let t25 = t24 * t17;
        let t27 = 1.0 / t18 / rho[ip];
        let t29 = f64::ln(t25 * t27 + f64::sqrt(pow_2(t25 * t27) + 1.0));
        let t30 = t27 * t29;
        let t33 = 1.0 + 0.56e-1 * t25 * t30;
        let t34 = 1.0 / t33;
        let t37 = 1.0 + 0.93053827172539591434e-2 * t23 * t34;
        let t41 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t37);
        let tzk0 = 2.0 * t41;
        zk[ip] += tzk0;
    }
}
