//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2828/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2828(t3191: f64, t3201: f64, t1021: f64, t11970: f64, t11874: f64, t15688: f64, t11817: f64, t3224: f64, t3042: f64, t3056: f64, t225: f64, t366: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42324 = t3191 * t3201;
    let t42326 = t1021 * t11970;
    let t42328 = t11874 * t15688;
    let t42346 = t3224 * t11817;
    let t42358 = t3042 * t3056;
    let t42359 = t42358 * t225;
    let t42360 = t42359 * t366;
    (t42324, t42326, t42328, t42346, t42358, t42359, t42360)
}
