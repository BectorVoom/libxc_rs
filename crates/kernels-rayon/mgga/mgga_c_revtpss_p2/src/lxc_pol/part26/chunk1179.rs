//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1179/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1179(t7406: f64, t9288: f64, t7064: f64, t25411: f64, t95593: f64, t10073: f64, t25308: f64, t26554: f64, t7399: f64, t786: f64, t867: f64, t2467: f64) -> (f64, f64, f64, f64, f64) {
    let t95730 = t7406 * t9288;
    let t95732 = 0.39982213492741449076e-1_f64 * t7064 * t95730;
    let t95733 = t25411 * t95593;
    let t95740 = t10073 * t25308 * t26554;
    let t95743 = t786 * t7399 * t867;
    let t95744 = t95743 * t2467;
    (t95730, t95732, t95733, t95740, t95744)
}
