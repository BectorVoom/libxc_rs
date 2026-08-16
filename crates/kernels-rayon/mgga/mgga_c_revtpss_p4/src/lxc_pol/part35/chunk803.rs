//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 803/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk803(t15014: f64, t2439: f64, t1569: f64, t2453: f64, t2458: f64, t2435: f64, t4322: f64, t1596: f64, t2873: f64, t1614: f64, t2942: f64, t1606: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15015 = t2439 * t15014;
    let t15017 = t2453 * t1569;
    let t15018 = t15017 * t2458;
    let t15063 = t2435 * t4322;
    let t15101 = t1596 * t2873;
    let t15104 = t1614 * t2942;
    let t15123 = t2439 * t1606;
    (t15015, t15018, t15063, t15101, t15104, t15123)
}
