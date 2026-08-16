//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 800/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk800(t1023: f64, t9348: f64, t3088: f64, t3091: f64, t1018: f64, t1932: f64, t3097: f64, t197: f64, t4962: f64, t1022: f64, t1928: f64, t3096: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9349 = t9348 * t1023;
    let t9351 = t3088 * t3091;
    let t9353 = t1932 * t1018;
    let t9354 = t9353 * t3097;
    let t9356 = t197 * t4962;
    let t9357 = t1022 * t9356;
    let t9359 = t3096 * t1928;
    (t9349, t9351, t9354, t9356, t9357, t9359)
}
