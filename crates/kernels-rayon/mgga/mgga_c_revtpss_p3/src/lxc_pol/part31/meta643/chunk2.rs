//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2103/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2103(t114: f64, t105885: f64, t508: f64, t651: f64, t28166: f64, t7897: f64, t28168: f64, t22287: f64, t28167: f64, t8996: f64, t5824: f64, t775: f64, t5966: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t105886 = piecewise3(t115, 0.0_f64, t105885);
    let t105889 = 2.0_f64 * t651 * t508 * t105886;
    let t105892 = t7897 * t28166;
    let t105894 = 12.0_f64 * t105892 * t28168;
    let t105897 = 6.0_f64 * t28167 * t8996 * t22287;
    let t105898 = t5824 * t775;
    let t105902 = t605 * t5966;
    (t105886, t105889, t105894, t105897, t105898, t105902)
}
