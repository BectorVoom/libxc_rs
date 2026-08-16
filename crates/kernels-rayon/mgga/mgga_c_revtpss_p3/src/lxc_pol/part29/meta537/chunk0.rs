//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1869/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1869(t10505: f64, t95725: f64, t93377: f64, t7406: f64, t9288: f64, t7064: f64, t25411: f64, t95593: f64, t10073: f64, t25308: f64, t26554: f64, t7399: f64, t786: f64, t867: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95726 = t95725 * t10505;
    let t95727 = t93377 * t95726;
    let t95730 = t7406 * t9288;
    let t95732 = 0.39982213492741449076e-1_f64 * t7064 * t95730;
    let t95733 = t25411 * t95593;
    let t95740 = t10073 * t25308 * t26554;
    let t95743 = t786 * t7399 * t867;
    (t95726, t95727, t95730, t95732, t95733, t95740, t95743)
}
