//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1003/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1003(t1153: f64, t2417: f64, t6851: f64, t869: f64, t291: f64, t3707: f64, t1180: f64, t7451: f64, t2579: f64, t891: f64, t2232: f64, t2546: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16404 = t2417 * t1153;
    let t16408 = t869 * t6851;
    let t16471 = t3707 * t291;
    let t16676 = t7451 * t1180;
    let t16677 = t2579 * t891;
    let t16720 = t2546 * t2232;
    (t16404, t16408, t16471, t16676, t16677, t16720)
}
