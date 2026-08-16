//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 492/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk492(t1469: f64, t2275: f64, t2282: f64, t2299: f64, t2306: f64, t116: f64, t1501: f64) -> (f64, f64, f64, f64, f64) {
    let t4201 = t2275 * t1469;
    let t4210 = t2282 * t1469;
    let t4227 = t2299 * t1469;
    let t4232 = t2306 * t1469;
    let t4248 = t1501 * t116;
    (t4201, t4210, t4227, t4232, t4248)
}
