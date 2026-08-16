//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 936/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk936(t7239: f64, t7898: f64, t197: f64, t530: f64, t2013: f64, t5627: f64, t8996: f64, t531: f64, t7933: f64, t7238: f64, t2014: f64, t1450: f64, t5591: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28165 = 3.0_f64 * t7898 * t7239;
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    let t28168 = t8996 * t5627;
    let t28170 = 6.0_f64 * t28167 * t28168;
    let t28172 = t531 * t7933;
    let t28173 = t28172 * t7238;
    let t28175 = 3.0_f64 * t2014 * t28173;
    let t28176 = t1450 * t5591;
    (t28165, t28166, t28167, t28168, t28170, t28173, t28175, t28176)
}
