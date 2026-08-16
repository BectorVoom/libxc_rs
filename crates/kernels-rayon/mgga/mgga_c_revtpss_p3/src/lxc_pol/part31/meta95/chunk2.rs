//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 613/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk613(t2042: f64, t572: f64, t2040: f64, t573: f64, t10: f64, t17: f64, t576: f64, t580: f64, t15: f64, t22: f64, t11: f64, t14: f64) -> (f64, f64, f64, f64, f64) {
    let t2044 = 3.0_f64 * t572 * t2042;
    let t2045 = t2040 * t573 + t2044;
    let t2219 = 2.0_f64 * t10 * t17;
    let t2221 = 8.0_f64 * t576 * t580;
    let t2223 = 6.0_f64 * t15 * t22;
    let t2224 = t11 * t14;
    (t2045, t2219, t2221, t2223, t2224)
}
