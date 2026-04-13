//! GGA_C_WI exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wi.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_wi_exc_unpol(
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
        let t1 = param_b * sigma[ip];
        let t2 = rho[ip] * rho[ip];
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / t2;
        let t7 = param_k * sigma[ip];
        let t9 = f64::exp(-t7 * t6);
        let t12 = t1 * t6 * t9 + param_a;
        let t13 = M_CBRT3;
        let t15 = pow_1_3(1.0 / M_PI);
        let t16 = t13 * t15;
        let t17 = M_CBRT4;
        let t18 = t17 * t17;
        let t22 = t13 * t13;
        let t23 = M_CBRTPI;
        let t25 = f64::sqrt(sigma[ip]);
        let t26 = t25 * sigma[ip];
        let t27 = t2 * t2;
        let t28 = 1.0 / t27;
        let t31 = 1.0 / t3 / rho[ip];
        let t32 = t25 * t31;
        let t33 = f64::sqrt(t32);
        let t38 = 1.0 + param_d * t17 * t22 * t23 * t33 * t26 * t28 / 3.0;
        let t42 = param_c + t16 * t18 / t3 * t38 / 4.0;
        let t43 = 1.0 / t42;
        let tzk0 = t12 * t43;
        zk[ip] += tzk0;
    }
}
