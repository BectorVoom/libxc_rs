//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1099/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1099(t46: f64, t5601: f64, t752: f64, t754: f64, t2100: f64, t5945: f64, t2099: f64, t5725: f64, t5730: f64, t5915: f64, t757: f64, t5722: f64, t768: f64) -> (f64, f64, f64, f64, f64) {
    let t18139 = t752 * t5601 * t754 * t46;
    let t18142 = t5945 * t2100;
    let t18145 = t5725 * t2099 * t5730;
    let t18150 = t757 * t2099 * t5915;
    let t18152 = t768 * t5722;
    (t18139, t18142, t18145, t18150, t18152)
}
