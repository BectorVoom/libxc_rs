//! GGA_X_LAG exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 51 shared lines across all orders.
//! Delta: 51 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_lag_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
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
        // --- shared preamble (51 lines) ---
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t7 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t11 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t12 = -t8;
        let t13 = rho0 - rho1;
        let t15 = piecewise5(t7, t8, t11, t12, t13 * t4);
        let t16 = 1.0 + t15;
        let t17 = t16 <= zeta_threshold;
        let t18 = pow_1_3(zeta_threshold);
        let t19 = t18 * zeta_threshold;
        let t20 = pow_1_3(t16);
        let t22 = piecewise3(t17, t19, t20 * t16);
        let t23 = t2 * t22;
        let t24 = pow_1_3(t3);
        let t25 = M_CBRT6;
        let t26 = t25 * t25;
        let t27 = M_PI * M_PI;
        let t28 = pow_1_3(t27);
        let t29 = 1.0 / t28;
        let t30 = t26 * t29;
        let t31 = f64::sqrt(sigma0);
        let t32 = pow_1_3(rho0);
        let t34 = 1.0 / t32 / rho0;
        let t36 = t30 * t31 * t34;
        let t37 = f64::powf(t36, 0.2626712e1);
        let t40 = 1.0 + 0.13471619689594796103e-3 * t37;
        let t41 = f64::powf(t40, -0.657946e0);
        let t42 = t24 * t37 * t41;
        let t45 = piecewise3(t1, 0.0, -0.15400028771927569605e-4 * t23 * t42);
        let t46 = rho1 <= dens_threshold;
        let t47 = -t13;
        let t49 = piecewise5(t11, t8, t7, t12, t47 * t4);
        let t50 = 1.0 + t49;
        let t51 = t50 <= zeta_threshold;
        let t52 = pow_1_3(t50);
        let t54 = piecewise3(t51, t19, t52 * t50);
        let t55 = t2 * t54;
        let t56 = f64::sqrt(sigma2);
        let t57 = pow_1_3(rho1);
        let t59 = 1.0 / t57 / rho1;
        let t61 = t30 * t56 * t59;
        let t62 = f64::powf(t61, 0.2626712e1);
        let t65 = 1.0 + 0.13471619689594796103e-3 * t62;
        let t66 = f64::powf(t65, -0.657946e0);
        let t67 = t24 * t62 * t66;
        let t70 = piecewise3(t46, 0.0, -0.15400028771927569605e-4 * t55 * t67);
        let tzk0 = t45 + t70;
        zk[ip] += tzk0;
    }
}
