//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 950/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk950(t2087: f64, t7630: f64, t1160: f64, t30539: f64, t1167: f64, t30268: f64, t7339: f64, t1165: f64, t12816: f64, t7351: f64, t7493: f64, t1998: f64, t3493: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31629 = t7630 * t2087;
    let t31631 = t1160 * t30539;
    let t31632 = t31631 * t1167;
    let t31634 = t30268 * t7339;
    let t31638 = t7493 * t1165 * t7351 * t12816;
    let t31640 = t1998 * t3493;
    (t31629, t31631, t31632, t31634, t31638, t31640)
}
