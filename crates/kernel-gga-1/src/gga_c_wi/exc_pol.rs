//! GGA_C_WI exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 28 shared lines across all orders.
//! Delta: 28 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_wi_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_k: f64,
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
        // --- shared preamble (28 lines) ---
        let t2 = sigma0 + 2.0 * sigma1 + sigma2;
        let t3 = param_b * t2;
        let t4 = rho0 + rho1;
        let t5 = t4 * t4;
        let t6 = pow_1_3(t4);
        let t7 = t6 * t6;
        let t9 = 1.0 / t7 / t5;
        let t10 = param_k * t2;
        let t12 = f64::exp(-t10 * t9);
        let t15 = t3 * t9 * t12 + param_a;
        let t16 = M_CBRT3;
        let t18 = pow_1_3(1.0 / M_PI);
        let t19 = t16 * t18;
        let t20 = M_CBRT4;
        let t21 = t20 * t20;
        let t25 = t16 * t16;
        let t26 = M_CBRTPI;
        let t28 = f64::sqrt(t2);
        let t29 = t28 * t2;
        let t30 = t5 * t5;
        let t31 = 1.0 / t30;
        let t34 = 1.0 / t6 / t4;
        let t35 = t28 * t34;
        let t36 = f64::sqrt(t35);
        let t41 = 1.0 + param_d * t20 * t25 * t26 * t36 * t29 * t31 / 3.0;
        let t45 = param_c + t19 * t21 / t6 * t41 / 4.0;
        let t46 = 1.0 / t45;
        let tzk0 = t15 * t46;
        zk[ip] += tzk0;
    }
}
