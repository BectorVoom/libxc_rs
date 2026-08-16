//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 245/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk245(t425: f64, t997: f64, t431: f64, t438: f64, t150: f64, t851: f64) -> (f64, f64, f64, f64) {
    let t998 = t997 * t425;
    let t1000 = t997 * t431;
    let t1002 = t997 * t438;
    let t1004 = t851 * t150;
    (t998, t1000, t1002, t1004)
}
