//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2197/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2197(t28019: f64, t531: f64, t2014: f64, t7238: f64, t25866: f64, t7898: f64, t13867: f64, t28167: f64, t8996: f64, t13872: f64, t13517: f64, t196: f64, t197: f64) -> (f64, f64, f64, f64, f64) {
    let t101417 = t531 * t28019;
    let t101420 = 6.0_f64 * t2014 * t101417 * t7238;
    let t101422 = 6.0_f64 * t7898 * t25866;
    let t101428 = 12.0_f64 * t28167 * t8996 * t13867;
    let t101431 = 6.0_f64 * t28167 * t8996 * t13872;
    let t101435 = t13517 * t196 * t197;
    (t101420, t101422, t101428, t101431, t101435)
}
