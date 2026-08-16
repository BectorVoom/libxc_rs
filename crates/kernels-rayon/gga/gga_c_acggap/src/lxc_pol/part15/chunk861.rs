//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 861/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk861(t1298: f64, t525: f64, t1854: f64, t301: f64, t1016: f64, t1742: f64, t1662: f64, t495: f64, t7884: f64, t7911: f64, t7930: f64, t862: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26995 = t525 * t1298;
    let t27011 = t1854 * t301;
    let t27338 = t1742 * t1016;
    let t28242 = t495 * t1662;
    let t29976 = t7884 * t7911;
    let t29979 = t862 * t7930;
    (t26995, t27011, t27338, t28242, t29976, t29979)
}
