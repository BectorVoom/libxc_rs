//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1014/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1014(t11119: f64, t384: f64, t373: f64, t675: f64, t828: f64, t11238: f64, t196: f64, t10308: f64, t599: f64, t90: f64, t29: f64, t560: f64, t9655: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42066 = 1.0_f64 / t11119 / t384;
    let t42792 = t675 * t373;
    let t42793 = t828 * t42792;
    let t42859 = 1.0_f64 / t11238 / t196;
    let t45963 = t599 * t10308;
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46361 = 1.0_f64 / t9655 / t560;
    (t42066, t42792, t42793, t42859, t45963, t45972, t46361)
}
