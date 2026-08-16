//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 766/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk766(t611: f64, t9128: f64, t3085: f64, t3160: f64, t608: f64, t1013: f64, t1924: f64, t1016: f64, t1019: f64, t1386: f64, t605: f64, t1717: f64, t8999: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9129 = t611 * t9128;
    let t9130 = t9129 * t3085;
    let t9132 = t3160 * t608;
    let t9135 = t1013 * t1924;
    let t9138 = t1386 * t1016 * t1019;
    let t9140 = t3160 * t605;
    let t9142 = t8999 * t1717;
    (t9130, t9132, t9135, t9138, t9140, t9142)
}
