//! MGGA_XC_LP90 exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_lp90.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_xc_lp90_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] * rho[ip];
        let t3 = pow_1_3::<f64>(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / t2;
        let t10 = 1.0 / t4 / rho[ip];
        let t13 = 0.80569e0 + 0.37655e-3 * sigma[ip] * t6 - 0.37655e-3 * lapl[ip] * t10;
        let t14 = 1.0 / t3;
        let t15 = t14 + 0.40743e-2;
        let t16 = 1.0 / t15;
        let tzk0 = -t13 * t16;
        zk[ip] += tzk0;
    }
}
