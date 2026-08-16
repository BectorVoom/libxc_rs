//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1131/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1131(t2453: f64, t26264: f64, t7496: f64, t9692: f64, t26249: f64, t9664: f64, t94701: f64, t96204: f64, t26359: f64, t9303: f64, t1892: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96515 = t2453 * t26264;
    let t96549 = 0.30356481678079769392e-1_f64 * t7496 * t9692;
    let t96564 = 0.46263278077393568556e-2_f64 * t26249 * t9664;
    let t96584 = 0.51727911450665971904e-3_f64 * t94701 * t96204;
    let t96591 = 0.26019841438354088051e-2_f64 * t9303 * t26359;
    let t97699 = t786 * t1892;
    (t96515, t96549, t96564, t96584, t96591, t97699)
}
