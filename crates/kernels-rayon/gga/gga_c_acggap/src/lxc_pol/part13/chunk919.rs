//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 919/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk919(t31038: f64, t409: f64, t7712: f64, t957: f64, t1181: f64, t30806: f64, t3491: f64, t599: f64, t1983: f64, t30127: f64, t7586: f64, t945: f64) -> (f64, f64, f64, f64) {
    let t31039 = t31038 * t409;
    let t31041 = t7712 * t957;
    let t31045 = t30806 * t1181 * t599 * t3491;
    let t31049 = t30127 * t7586 * t1983 * t945;
    (t31039, t31041, t31045, t31049)
}
