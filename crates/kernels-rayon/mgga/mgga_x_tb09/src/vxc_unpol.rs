//! MGGA_X_TB09 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tb09.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRTPI, M_PI, M_SQRT2};
use libxc_rkernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_tb09_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c: f64,
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..vrho.len() {
        let t2 = M_CBRTPI;
        let t3 = param_c * t2;
        let t4 = M_CBRT2;
        let t5 = t4 * t4;
        let t6 = pow_1_3(rho[ip]);
        let t7 = t6 * t6;
        let t9 = 1.0 / t7 / rho[ip];
        let t14 = rho[ip] * rho[ip];
        let t16 = 1.0 / t7 / t14;
        let t20 = rmath::abs(lapl[ip] * t9 / 6.0 - 0.5333333333333333 * tau[ip] * t9 + 0.06666666666666667 * sigma[ip] * t16);
        let t22 = t5 * t20 < 5e-13;
        let t23 = lapl[ip] * t5;
        let t26 = tau[ip] * t5;
        let t27 = t26 * t9;
        let t29 = sigma[ip] * t5;
        let t32 = t23 * t9 / 6.0 - 0.5333333333333333 * t27 + 0.06666666666666667 * t29 * t16;
        let t33 = 0.0 < t32;
        let t34 = piecewise3(t33, 5e-13, -5e-13);
        let t35 = piecewise3(t22, t34, t32);
        let t36 = xc_mgga_x_br89_get_x(t35);
        let t38 = rmath::exp(t36 / 3.0);
        let t39 = rmath::exp(-t36);
        let t41 = 1.0 + t36 / 2.0;
        let t42 = t39 * t41;
        let t43 = 1.0 - t42;
        let t44 = t38 * t43;
        let t45 = 1.0 / t36;
        let t46 = t44 * t45;
        let t51 = rmath::sqrt(15.0);
        let t52 = (3.0 * param_c - 2.0) * t51;
        let t53 = 1.0 / M_PI;
        let t54 = M_SQRT2;
        let t55 = t53 * t54;
        let t56 = param_alpha * sigma[ip];
        let t57 = t5 * t16;
        let t60 = t27 - t56 * t57 / 8.0;
        let t61 = 1e-10 < t60;
        let t62 = piecewise3(t61, t60, 1e-10);
        let t63 = rmath::sqrt(t62);
        let t68 = (-2.0 * t3 * t46 + t52 * t55 * t63 / 6.0) * t5;
        let tvrho0 = t68 * t6 / 2.0;
        vrho[ip] += tvrho0;
    }
}
