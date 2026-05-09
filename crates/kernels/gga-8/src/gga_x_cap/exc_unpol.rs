//! GGA_X_CAP exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 33 shared lines across all orders.
//! Delta: 33 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_cap_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_alphaoAx: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (33 lines) ---
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
        let t21 = t20 * t20;
        let t22 = param_alphaoAx * t21;
        let t23 = M_PI * M_PI;
        let t24 = pow_1_3(t23);
        let t25 = 1.0 / t24;
        let t26 = f64::sqrt(sigma[ip]);
        let t28 = t22 * t25 * t26;
        let t29 = M_CBRT2;
        let t31 = 1.0 / t18 / rho[ip];
        let t33 = t21 * t25;
        let t38 = 1.0 + t33 * t26 * t29 * t31 / 12.0;
        let t39 = f64::ln(t38);
        let t41 = param_c * t39 + 1.0;
        let t42 = 1.0 / t41;
        let t43 = t39 * t42;
        let t44 = t29 * t31 * t43;
        let t47 = 1.0 - t28 * t44 / 12.0;
        let t51 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t47);
        let tzk0 = 2.0 * t51;
        zk[ip] += tzk0;
    }
}
