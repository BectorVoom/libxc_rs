//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1159/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1159(t161: f64, t20535: f64, t31379: f64, t4130: f64, t3176: f64, t493: f64, t6576: f64, t6578: f64, t20013: f64, t4782: f64, t883: f64, t9272: f64) -> (f64, f64, f64) {
    let t31386 = 0.23005755572352449806e1_f64 * t20535 * t4130 * t161 * t31379;
    let t31393 = t6576 * t493 * t3176 * t6578;
    let t31394 = 0.1533717038156829987e1_f64 * t31393;
    let t31412 = 0.11502877786176224903e1_f64 * t9272 * t4782 * t883 * t20013;
    (t31386, t31394, t31412)
}
