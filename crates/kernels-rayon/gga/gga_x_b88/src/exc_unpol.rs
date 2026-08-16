//! GGA_X_B88 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_b88.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_b88_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
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
        let t20 = t3 * t3;
        let t21 = param_beta * t20;
        let t23 = pow_1_3(1.0 / M_PI);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t26 = t24 * t25;
        let t27 = t21 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = sigma[ip] * t29;
        let t31 = rho[ip] * rho[ip];
        let t32 = t18 * t18;
        let t34 = 1.0 / t32 / t31;
        let t35 = param_gamma * param_beta;
        let t36 = f64::sqrt(sigma[ip]);
        let t37 = t35 * t36;
        let t39 = 1.0 / t18 / rho[ip];
        let t43 = f64::ln(t36 * t28 * t39 + f64::sqrt(pow_2(t36 * t28 * t39) + 1.0));
        let t44 = t28 * t39 * t43;
        let t46 = t37 * t44 + 1.0;
        let t47 = 1.0 / t46;
        let t48 = t34 * t47;
        let t52 = 1.0 + 2.0 / 9.0 * t27 * t30 * t48;
        let t56 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t52);
        let tzk0 = 2.0 * t56;
        zk[ip] += tzk0;
    }
}
