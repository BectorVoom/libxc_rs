//! GGA_X_PBETRANS exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbetrans.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbetrans_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
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
        let t20 = M_PI * M_PI;
        let t21 = pow_1_3(t20);
        let t23 = M_CBRT6;
        let t24 = t23 * t23;
        let t27 = rmath::sqrt(sigma[ip]);
        let t28 = M_CBRT2;
        let t29 = t27 * t28;
        let t31 = 1.0 / t18 / rho[ip];
        let t38 = rmath::exp(-2.0 * t3 * t21 * (t24 / t21 * t29 * t31 / 12.0 - 3.0));
        let t39 = 1.0 + t38;
        let t41 = 0.413 / t39;
        let t42 = 1.227 - t41;
        let t43 = t21 * t21;
        let t45 = t23 / t43;
        let t46 = t28 * t28;
        let t47 = sigma[ip] * t46;
        let t48 = rho[ip] * rho[ip];
        let t49 = t18 * t18;
        let t51 = 1.0 / t49 / t48;
        let t55 = 1.227 - t41 + 0.009125 * t45 * t47 * t51;
        let t56 = 1.0 / t55;
        let t58 = -t42 * t56 + 1.0;
        let t60 = t42 * t58 + 1.0;
        let t64 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t60);
        let tzk0 = 2.0 * t64;
        zk[ip] += tzk0;
    }
}
