//! GGA_C_CS1 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_cs1.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(
    unused_imports,
    unused_variables,
    non_snake_case,
    clippy::excessive_precision,
    clippy::too_many_arguments,
    clippy::needless_return
)]

use libxc_rkernel_math::constants::M_CBRT2;
use libxc_rkernel_math::piecewise::piecewise3;
use libxc_rkernel_math::powers::pow_1_3;
use libxc_rkernel_math::rmath;

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_cs1_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = 1.0 + 0.349 * t2;
        let t5 = 1.0 / t4;
        let t6 = sigma[ip] * sigma[ip];
        let t7 = rho[ip] * rho[ip];
        let t8 = t7 * t7;
        let t9 = t8 * rho[ip];
        let t11 = 1.0 / t1 / t9;
        let t13 = t1 * t1;
        let t15 = 1.0 / t13 / t7;
        let t18 = 1.0 + 0.006 * sigma[ip] * t15;
        let t19 = t18 * t18;
        let t20 = 1.0 / t19;
        let t23 = -0.159068 + 2.86308e-07 * t6 * t11 * t20;
        let t25 = t5 * t23 / 4.0;
        let t27 = piecewise3(1.0 <= zeta_threshold, zeta_threshold, 1.0);
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = t27 * t29;
        let t33 = t29 * t1 / 2.0 + 0.349;
        let t34 = 1.0 / t33;
        let t35 = t1 * t34;
        let t36 = t6 * t28;
        let t37 = sigma[ip] * t29;
        let t40 = 1.0 + 0.006 * t37 * t15;
        let t41 = t40 * t40;
        let t42 = 1.0 / t41;
        let t43 = t11 * t42;
        let t46 = -0.018897 + 1.117728e-05 * t36 * t43;
        let t49 = t30 * t35 * t46 / 2.0;
        let tzk0 = t25 + t49;
        zk[ip] += tzk0;
    }
}
