//! MGGA_XC_LP90 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_lp90.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_lp90_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] * rho[ip];
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / t2;
        let t10 = 1.0 / t4 / rho[ip];
        let t13 = 0.80569 + 0.00037655 * sigma[ip] * t6 - 0.00037655 * lapl[ip] * t10;
        let t14 = 1.0 / t3;
        let t15 = t14 + 0.0040743;
        let t16 = 1.0 / t15;
        let tzk0 = -t13 * t16;
        zk[ip] += tzk0;
    }
}
