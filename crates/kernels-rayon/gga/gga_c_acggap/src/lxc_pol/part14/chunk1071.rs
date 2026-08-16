//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1071/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1071(t1844: f64, t1992: f64, t7585: f64, t7842: f64, t1165: f64, t6218: f64, t7351: f64, t7575: f64, t301: f64, t1181: f64, t599: f64, t7337: f64) -> (f64, f64, f64, f64) {
    let t38875 = t7585 * t7842 * t1992 * t1844;
    let t38879 = t7575 * t1165 * t7351 * t6218;
    let t38883 = t1844 * t301;
    let t38886 = t7337 * t1181 * t599 * t38883;
    (t38875, t38879, t38883, t38886)
}
