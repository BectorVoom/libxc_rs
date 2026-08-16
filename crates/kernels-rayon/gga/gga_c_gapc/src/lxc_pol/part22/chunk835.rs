//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 835/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk835(t1736: f64, t3292: f64, t311: f64, t314: f64, t329: f64, t6: f64, t103: f64, t3278: f64, t962: f64, t191: f64, t2153: f64, t1093: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9551 = t3292 * t1736;
    let t9552 = t311 * t9551;
    let t9554 = t6 * t329 * t314;
    let t9555 = t103 * t9554;
    let t9556 = t9552 * t9555;
    let t9558 = t3278 * t962;
    let t9560 = t2153 * t191;
    let t9561 = t9560 * t1093;
    (t9552, t9554, t9555, t9556, t9558, t9561)
}
