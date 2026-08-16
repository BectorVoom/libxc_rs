//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1180/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1180(t14495: f64, t7076: f64, t2769: f64, t34074: f64, t7063: f64, t31801: f64, t10770: f64, t31756: f64, t31767: f64, t4433: f64, t119852: f64, t4364: f64, t4486: f64) -> (f64, f64, f64, f64, f64) {
    let t126246 = t7076 * t14495;
    let t126250 = t34074 * t2769;
    let t126251 = t7063 * t126250;
    let t126252 = t126251 * t31801;
    let t126256 = t31767 * t10770 * t31756 * t4433;
    let t126260 = t31767 * t4364 * t119852 * t4486;
    (t126246, t126250, t126252, t126256, t126260)
}
