//! GGA_C_W94 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_w94.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_w94_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = 0.0 < t4;
        let t6 = piecewise3(t5, t4, -t4);
        let t7 = 1e-10 < t6;
        let t8 = piecewise3(t7, t6, 1e-10);
        let t9 = pow_1_3(t8);
        let t10 = t9 * t9;
        let t12 = -t10 * t8 + 1.0;
        let t13 = rmath::sqrt(t12);
        let t15 = sigma0 + 2.0 * sigma1 + sigma2;
        let t16 = rmath::sqrt(t15);
        let t17 = t16 * t15;
        let t18 = t2 * t2;
        let t19 = t18 * t18;
        let t20 = 1.0 / t19;
        let t22 = pow_1_3(t2);
        let t24 = 1.0 / t22 / t2;
        let t25 = t16 * t24;
        let t26 = rmath::pow(t25, 1.0 / 16.0);
        let t27 = t26 * t26;
        let t28 = t27 * t26;
        let t31 = t18 * t2;
        let t32 = 1.0 / t31;
        let t35 = M_CBRT3;
        let t37 = pow_1_3(1.0 / M_PI);
        let t38 = t35 * t37;
        let t39 = M_CBRT4;
        let t40 = t39 * t39;
        let t45 = 11.8 + 0.15067 * t28 * t17 * t20 + 0.01102 * t15 * t32 + t38 * t40 / t22 / 4.0;
        let t46 = 1.0 / t45;
        let tzk0 = -t13 * t46;
        zk[ip] += tzk0;
    }
}
