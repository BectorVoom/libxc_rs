//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2255/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2255(t13388: f64, t76: f64, t13312: f64, t13392: f64, t13396: f64, t1469: f64, t15936: f64, t1923: f64, t1926: f64, t1927: f64, t25129: f64, t25132: f64, t25139: f64, t25146: f64, t25150: f64, t28077: f64, t28078: f64, t28081: f64, t28086: f64, t28089: f64, t28090: f64, t4181: f64, t4186: f64, t6954: f64, t6963: f64, t6968: f64, t6973: f64, t6977: f64, t72: f64, t7715: f64, t7719: f64, t7720: f64, t92597: f64, t92600: f64, t92605: f64, t92612: f64) -> f64 {
    let t101303 = t76 * t13388;
    let t101309 = -t6954 * t28078 / 3.0_f64 - t6954 * t28081 / 3.0_f64 - t1923 * (220.0_f64 / 27.0_f64 * t92597 * t1469 - 40.0_f64 / 27.0_f64 * t92600 * t4181 - 40.0_f64 / 9.0_f64 * t25129 * t4186 - 5.0_f64 / 108.0_f64 * t92605 * t15936 + 5.0_f64 / 9.0_f64 * t25132 * t13396 + 5.0_f64 / 18.0_f64 * t25132 * t13392 + 5.0_f64 / 6.0_f64 * t6968 * t13312 + t92612) * t72 * t1927 / 6.0_f64 - t1923 * t28077 * t6977 / 3.0_f64 - t1923 * t7715 * t25146 / 6.0_f64 - t25150 * t7720 / 6.0_f64 - t6954 * t28086 / 3.0_f64 - t6954 * t28090 / 3.0_f64 - t1923 * t25139 * t7719 / 6.0_f64 - t1923 * t6973 * t28089 / 3.0_f64 - t1923 * t1926 * t101303 / 6.0_f64 + 2.0_f64 / 3.0_f64 * t6963 * t28078;
    t101309
}
