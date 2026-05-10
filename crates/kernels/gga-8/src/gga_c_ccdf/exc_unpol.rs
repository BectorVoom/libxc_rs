//! GGA_C_CCDF exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 19 shared lines across all orders.
//! Delta: 19 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT6, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_ccdf_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
    param_c4: f64,
    param_c5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (19 lines) ---
        let t1 = pow_1_3(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = param_c2 * t2 + 1.0;
        let t5 = 1.0 / t4;
        let t6 = param_c1 * t5;
        let t7 = M_CBRT2;
        let t8 = M_CBRT6;
        let t9 = t8 * t8;
        let t10 = t7 * t9;
        let t11 = M_PI * M_PI;
        let t12 = pow_1_3(t11);
        let t13 = 1.0 / t12;
        let t14 = f64::sqrt(sigma[ip]);
        let t15 = t13 * t14;
        let t17 = 1.0 / t1 / rho[ip];
        let t23 = f64::exp(-param_c4 * (t10 * t15 * t17 / 12.0 - param_c5));
        let t24 = 1.0 + t23;
        let t27 = 1.0 - param_c3 / t24;
        let tzk0 = t6 * t27;
        zk[ip] += tzk0;
    }
}
