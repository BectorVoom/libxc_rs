//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2228/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2228(t101211: f64, t101215: f64, t101342: f64, t18281: f64, t1923: f64, t1927: f64, t19661: f64, t19666: f64, t19680: f64, t25129: f64, t25132: f64, t28077: f64, t28081: f64, t28086: f64, t28090: f64, t28093: f64, t28147: f64, t28154: f64, t29525: f64, t29526: f64, t29529: f64, t5819: f64, t5825: f64, t6954: f64, t6968: f64, t6977: f64, t72: f64, t7702: f64, t7719: f64, t7720: f64, t92600: f64, t92605: f64, t92612: f64) -> f64 {
    let t108931 = -10.0_f64 / 3.0_f64 * t28154 * t101211 - 10.0_f64 / 3.0_f64 * t28154 * t101215 - 10.0_f64 * t101342 * t28147 - t7702 * t28081 / 3.0_f64 - t28093 * t7720 / 3.0_f64 - t7702 * t28086 / 3.0_f64 - t7702 * t28090 / 3.0_f64 - t6954 * t29526 / 6.0_f64 - t1923 * (-20.0_f64 / 27.0_f64 * t92600 * t5819 - 5.0_f64 / 108.0_f64 * t92605 * t19661 + 5.0_f64 / 9.0_f64 * t25132 * t19666 - 20.0_f64 / 9.0_f64 * t25129 * t5825 + 5.0_f64 / 18.0_f64 * t25132 * t19680 + 5.0_f64 / 6.0_f64 * t6968 * t18281 + t92612) * t72 * t1927 / 6.0_f64 - t1923 * t29525 * t6977 / 6.0_f64 - t6954 * t29529 / 3.0_f64 - t1923 * t28077 * t7719 / 3.0_f64;
    t108931
}
