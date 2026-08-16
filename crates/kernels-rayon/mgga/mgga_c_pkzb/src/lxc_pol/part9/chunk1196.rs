//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1196/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1196(t20743: f64, t208: f64, t218: f64, t219: f64, t20716: f64, t17351: f64, t17354: f64, t17357: f64, t17455: f64, t20705: f64, t20719: f64, t20745: f64) -> (f64, f64) {
    let t20781 = t218 * t219 * t208 * t20743;
    let t20787 = 4.0_f64 / 3.0_f64 * t20716;
    let t20788 = t17455 - 28.0_f64 / 9.0_f64 * t17351 + 4.0_f64 / 3.0_f64 * t17354 - t17357 / 3.0_f64 - 28.0_f64 / 27.0_f64 * t20705 + t20787 - t20719 + t20745;
    (t20781, t20788)
}
