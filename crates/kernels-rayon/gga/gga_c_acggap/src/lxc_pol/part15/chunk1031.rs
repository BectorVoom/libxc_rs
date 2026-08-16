//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1031/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1031(t30148: f64, t7585: f64, t7842: f64, t8906: f64, t1451: f64, t7614: f64, t2304: f64, t7630: f64, t2294: f64, t7610: f64, t1988: f64, t8497: f64) -> (f64, f64, f64, f64, f64) {
    let t36123 = t7585 * t7842 * t30148 * t8906;
    let t36125 = t7614 * t1451;
    let t36127 = t7630 * t2304;
    let t36129 = t7610 * t2294;
    let t36131 = t1988 * t8497;
    (t36123, t36125, t36127, t36129, t36131)
}
