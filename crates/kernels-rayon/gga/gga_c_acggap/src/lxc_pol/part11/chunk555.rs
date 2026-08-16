//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 555/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk555(t1004: f64, t996: f64, t390: f64, t1020: f64, t997: f64, t3055: f64, t383: f64, t1039: f64, t1029: f64, t1032: f64, t993: f64, t1205: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3770 = t1004 * t996;
    let t3772 = 0.60023625365297631762e-2_f64 * t3770 * t390;
    let t3773 = t997 * t1020;
    let t3775 = t3055 * t383;
    let t3777 = 0.12862205435420921092e-2_f64 * t3775 * t1039;
    let t3778 = t997 * t1029;
    let t3782 = 0.30011812682648815881e-2_f64 * t1032 * t993;
    let t3783 = t997 * t1205;
    (t3772, t3773, t3777, t3778, t3782, t3783)
}
