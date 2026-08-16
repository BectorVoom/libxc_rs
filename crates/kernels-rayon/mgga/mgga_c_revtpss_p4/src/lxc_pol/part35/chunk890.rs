//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 890/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk890(t187: f64, t22789: f64, t13621: f64, t13630: f64, t13633: f64, t22764: f64, t22765: f64, t22766: f64, t22768: f64, t22791: f64, t9394: f64, t9396: f64, t9409: f64, t9412: f64) -> (f64, f64, f64, f64, f64) {
    let t22919 = 0.19751673498613801407e-1_f64 * t22789 * t187;
    let t22920 = 24.0_f64 * t13621;
    let t22921 = 0.35089341735807877242e1_f64 * t13630;
    let t22922 = 3.0_f64 * t13633;
    let t22923 = -t22764 - t22765 + t22766 - t22768 + t22791 + t22919 + t9394 - t22920 - t9396 + t22921 + t22922 + t9409 - t9412;
    (t22919, t22920, t22921, t22922, t22923)
}
