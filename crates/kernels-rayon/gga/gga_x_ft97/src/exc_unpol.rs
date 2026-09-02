//! GGA_X_FT97 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ft97.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ft97_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_beta1: f64,
    param_beta2: f64,
    param_beta0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t20 = param_beta1 * sigma[ip];
        let t21 = t18 * t18;
        let t22 = 1.0 / t21;
        let t23 = t20 * t22;
        let t24 = t11 * t11;
        let t25 = t11 * rho[ip];
        let t26 = pow_1_3(t25);
        let t27 = t26 * t26;
        let t28 = t24 * t27;
        let t29 = sigma[ip] * t22;
        let t32 = param_beta2 + t29 * t28 / 4.0;
        let t33 = 1.0 / t32;
        let t34 = t28 * t33;
        let t37 = param_beta0 + t23 * t34 / 4.0;
        let t38 = t37 * sigma[ip];
        let t39 = M_CBRT2;
        let t40 = t39 * t39;
        let t41 = rho[ip] * rho[ip];
        let t43 = 1.0 / t21 / t41;
        let t44 = t40 * t43;
        let t45 = t38 * t44;
        let t46 = t3 * t3;
        let t48 = pow_1_3(1.0 / M_PI);
        let t49 = 1.0 / t48;
        let t50 = t46 * t49;
        let t51 = M_CBRT4;
        let t52 = sigma[ip] * t40;
        let t53 = t37 * t37;
        let t55 = t52 * t43;
        let t56 = rmath::ln(t55 + rmath::sqrt(t55 * t55 + 1.0));
        let t57 = t56 * t56;
        let t61 = 9.0 * t52 * t43 * t53 * t57 + 1.0;
        let t62 = rmath::sqrt(t61);
        let t65 = t50 * t51 / t62;
        let t68 = 1.0 + 2.0 / 9.0 * t45 * t65;
        let t72 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t68);
        let tzk0 = 2.0 * t72;
        zk[ip] += tzk0;
    }
}
