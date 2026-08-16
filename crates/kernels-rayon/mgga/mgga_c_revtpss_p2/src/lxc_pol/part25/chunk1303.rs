//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1303/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1303(t2014: f64, t7238: f64, t94358: f64, t10194: f64, t10259: f64, t10260: f64, t10415: f64, t118: f64, t1310: f64, t1453: f64, t2007: f64, t2322: f64, t25078: f64, t25169: f64, t25835: f64, t508: f64, t651: f64, t671: f64, t6985: f64, t92719: f64, t92724: f64, t92727: f64, t92731: f64, t92733: f64, t92736: f64, t92737: f64, t94224: f64, t94336: f64, t94341: f64, t94348: f64, t94352: f64, t94355: f64) -> f64 {
    let t94361 = 9.0_f64 * t2014 * t94358 * t7238;
    let t94365 = -t92719 * t508 - 6.0_f64 * t2322 * t25078 - t92724 - t92727 - 2.0_f64 * t6985 * t10260 - t92731 - t92733 - t92736 - 6.0_f64 * t92737 * t671 - 6.0_f64 * t10194 * t2007 - t118 * (t94224 + t94336) - t94341 - 2.0_f64 * t651 * t2007 * t10259 + t94348 - t94352 - t94355 + 3.0_f64 * t25835 * t1453 + t94361 - 3.0_f64 * t25169 * t1310 - t10415 * t2007;
    t94365
}
