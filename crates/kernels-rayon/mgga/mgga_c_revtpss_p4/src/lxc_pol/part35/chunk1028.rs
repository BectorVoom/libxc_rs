//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1028/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1028(t197: f64, t531: f64, t2013: f64, t2411: f64, t30: f64, t1946: f64, t2684: f64, t7043: f64, t820: f64, t843: f64, t240: f64, t7036: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25081 = t197 * t531;
    let t25082 = t2013 * t25081;
    let t25207 = t2411 * t30;
    let t25219 = t1946 * t2684;
    let t25222 = t820 * t7043 * t843;
    let t25227 = t7036 * t240;
    (t25081, t25082, t25207, t25219, t25222, t25227)
}
