//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 811/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk811(t1284: f64, t1811: f64, t1209: f64, t1789: f64, t371: f64, t676: f64, t1235: f64, t1769: f64, t3565: f64, t225: f64, t480: f64, t1804: f64, t3655: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17191 = t1284 * t1811;
    let t17192 = t1209 * t17191;
    let t17303 = t371 * t676 * t1789;
    let t17304 = t1235 * t17303;
    let t17306 = t1769 * t3565;
    let t17307 = t17306 * t225;
    let t17308 = t17307 * t480;
    let t17340 = t1804 * t3655;
    (t17192, t17304, t17306, t17307, t17308, t17340)
}
