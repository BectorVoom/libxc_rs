//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 782/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk782(t1013: f64, t1924: f64, t1016: f64, t1019: f64, t1386: f64, t3160: f64, t605: f64, t1717: f64, t8999: f64, t633: f64, t8769: f64, t1700: f64) -> (f64, f64, f64, f64, f64) {
    let t9135 = t1013 * t1924;
    let t9138 = t1386 * t1016 * t1019;
    let t9140 = t3160 * t605;
    let t9142 = t8999 * t1717;
    let t9144 = t633 * t8769;
    let t9145 = t9144 * t1700;
    (t9135, t9138, t9140, t9142, t9145)
}
