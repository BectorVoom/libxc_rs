//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2213/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2213(t1916: f64, t28277: f64, t1518: f64, t572: f64, t670: f64, t7741: f64, t28280: f64, t1459: f64, t30191: f64, t28264: f64, t5920: f64, t105886: f64, t117: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t109295 = 12.0_f64 * t1916 * t28277;
    let t109299 = 12.0_f64 * t572 * t670 * t7741 * t1518;
    let t109305 = 6.0_f64 * t1916 * t28280;
    let t109307 = 6.0_f64 * t1459 * t30191;
    let t109310 = 6.0_f64 * t572 * t28264 * t5920;
    let t109315 = 3.0_f64 * t572 * t117 * t105886;
    (t109295, t109299, t109305, t109307, t109310, t109315)
}
