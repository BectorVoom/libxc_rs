//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 761/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk761(t10985: f64, t2454: f64, t252: f64, t2769: f64, t786: f64, t866: f64, t225: f64, t788: f64, t9288: f64, t787: f64, t781: f64, t9292: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10987 = 0.46263278077393568556e-2_f64 * t2454 * t10985;
    let t10994 = t252 * t2769;
    let t10995 = t786 * t10994;
    let t11006 = t866 * t866;
    let t11007 = 1.0_f64 / t11006;
    let t11008 = t225 * t11007;
    let t11015 = t788 * t9288;
    let t11017 = 0.30356481678079769392e-1_f64 * t787 * t11015;
    let t11040 = 0.17073386770573548589e-1_f64 * t9292 * t781;
    (t10987, t10995, t11006, t11007, t11008, t11015, t11017, t11040)
}
