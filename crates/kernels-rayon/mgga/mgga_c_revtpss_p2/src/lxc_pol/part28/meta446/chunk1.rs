//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1688/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1688(t1042: f64, t17221: f64, t3172: f64, t5269: f64, t1261: f64, t13396: f64, t5268: f64, t12256: f64, t13099: f64, t15936: f64, t1224: f64, t140: f64) -> (f64, f64, f64, f64, f64) {
    let t17222 = t1042 * t17221;
    let t17225 = t3172 * t5269;
    let t17227 = 0.3811023832717309953e-3_f64 * t1261 * t17225;
    let t17231 = t5268 * t13396;
    let t17232 = t1042 * t17231;
    let t17235 = t13099 * t12256;
    let t17236 = t17235 * t15936;
    let t17237 = t1042 * t17236;
    let t17240 = t140 * t1224;
    (t17222, t17227, t17232, t17237, t17240)
}
