//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 908/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk908(t9810: f64, t9836: f64, t852: f64, t833: f64, t3769: f64, t6137: f64, t3038: f64, t8009: f64, t3074: f64, t8219: f64, t3740: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9837 = t9810 + t9836;
    let t9838 = t9837 * t852;
    let t9840 = 1.0_f64 * t833 * t9838;
    let t9842 = 0.16081979498692535067e2_f64 * t6137 * t3769;
    let t9844 = 4.0_f64 * t8009 * t3038;
    let t9846 = 0.32163958997385070134e2_f64 * t8219 * t3074;
    let t9847 = t3740 * t851;
    (t9837, t9838, t9840, t9842, t9844, t9846, t9847)
}
