//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1852/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1852(t25375: f64, t95628: f64, t136: f64, t137: f64, t2061: f64, t10505: f64, t93377: f64, t7406: f64, t9288: f64, t7064: f64, t10073: f64, t25308: f64, t26554: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95722 = t25375 * t95628;
    let t95725 = t2061 * t136 * t137;
    let t95726 = t95725 * t10505;
    let t95727 = t93377 * t95726;
    let t95730 = t7406 * t9288;
    let t95732 = 0.39982213492741449076e-1_f64 * t7064 * t95730;
    let t95740 = t10073 * t25308 * t26554;
    (t95722, t95725, t95726, t95727, t95730, t95732, t95740)
}
