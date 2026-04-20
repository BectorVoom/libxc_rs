//! MGGA_X_TB09 vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 41 shared lines across all orders.
//! Delta: 41 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRTPI, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::br89::xc_mgga_x_br89_get_x;

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_tb09_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_alpha: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        // --- shared preamble (41 lines) ---
        let t2 = M_CBRTPI;
        let t3 = param_c * t2;
        let t4 = M_CBRT2;
        let t5 = t4 * t4;
        let t6 = pow_1_3(rho[ip]);
        let t7 = t6 * t6;
        let t9 = 1.0 / t7 / rho[ip];
        let t14 = rho[ip] * rho[ip];
        let t16 = 1.0 / t7 / t14;
        let t20 = f64::abs(lapl[ip] * t9 / 6.0 - 0.53333333333333333332e0 * tau[ip] * t9 + 0.66666666666666666668e-1 * sigma[ip] * t16);
        let t22 = t5 * t20 < 0.5e-12;
        let t23 = lapl[ip] * t5;
        let t26 = tau[ip] * t5;
        let t27 = t26 * t9;
        let t29 = sigma[ip] * t5;
        let t32 = t23 * t9 / 6.0 - 0.53333333333333333333e0 * t27 + 0.66666666666666666667e-1 * t29 * t16;
        let t33 = 0.0 < t32;
        let t34 = piecewise3(t33, 0.5e-12, -0.5e-12);
        let t35 = piecewise3(t22, t34, t32);
        let t36 = xc_mgga_x_br89_get_x(t35);
        let t38 = f64::exp(t36 / 3.0);
        let t39 = f64::exp(-t36);
        let t41 = 1.0 + t36 / 2.0;
        let t42 = t39 * t41;
        let t43 = 1.0 - t42;
        let t44 = t38 * t43;
        let t45 = 1.0 / t36;
        let t46 = t44 * t45;
        let t51 = f64::sqrt(15.0);
        let t52 = (3.0 * param_c - 2.0) * t51;
        let t53 = 1.0 / M_PI;
        let t54 = M_SQRT2;
        let t55 = t53 * t54;
        let t56 = param_alpha * sigma[ip];
        let t57 = t5 * t16;
        let t60 = t27 - t56 * t57 / 8.0;
        let t61 = 0.1e-9 < t60;
        let t62 = piecewise3(t61, t60, 0.1e-9);
        let t63 = f64::sqrt(t62);
        let t68 = (-2.0 * t3 * t46 + t52 * t55 * t63 / 6.0) * t5;
        let tvrho0 = t68 * t6 / 2.0;
        vrho[ip] += tvrho0;
    }
}
