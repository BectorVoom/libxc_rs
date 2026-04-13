//! MGGA_XC_ZLP exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_zlp.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_xc_zlp_exc_unpol(
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
        let t2 = M_CBRT3;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t10 = rho[ip] * rho[ip];
        let t11 = pow_1_3(rho[ip]);
        let t12 = t11 * t11;
        let t14 = 1.0 / t12 / t10;
        let t17 = 1.0 / t12 / rho[ip];
        let t24 = 0.207108e0 * t5 * t7 + 0.5387725e-2 * t5 * t7 * (-lapl[ip] * t17 / 8.0 + sigma[ip] * t14 / 8.0);
        let t25 = 1.0 / t11;
        let t27 = 1.0 + 0.48849425066691677572e3 * t25;
        let t28 = f64::ln(t27);
        let t31 = 1.0 - 0.2047107e-2 * t28 * t11;
        let t33 = t2 * t2;
        let t34 = t24 * t31 * t33;
        let t35 = 1.0 / t4;
        let t36 = t35 * t6;
        let t37 = t36 * t11;
        let t38 = t34 * t37;
        let tzk0 = -t38 / 3.0;
        zk[ip] += tzk0;
    }
}
