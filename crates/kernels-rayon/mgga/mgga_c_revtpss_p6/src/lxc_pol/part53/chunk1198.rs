//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1198/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1198(t32099: f64, t7898: f64, t25082: f64, t27153: f64, t37110: f64, t33913: f64, t7316: f64, t28173: f64, t8568: f64, t33974: f64, t531: f64, t2014: f64, t7238: f64) -> (f64, f64, f64, f64, f64) {
    let t127302 = 3.0_f64 * t7898 * t32099;
    let t127305 = 6.0_f64 * t25082 * t37110 * t27153;
    let t127306 = t33913 * t7316;
    let t127308 = t8568 * t28173;
    let t127310 = t531 * t33974;
    let t127313 = 3.0_f64 * t2014 * t127310 * t7238;
    (t127302, t127305, t127306, t127308, t127313)
}
