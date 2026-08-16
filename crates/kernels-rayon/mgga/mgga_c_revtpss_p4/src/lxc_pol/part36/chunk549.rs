//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 549/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk549(t1469: f64, t3362: f64, t3367: f64, t1130: f64, t1719: f64, t1723: f64, t3390: f64, t3407: f64, t1729: f64, t698: f64, t1160: f64, t1737: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5046 = t3362 * t1469;
    let t5051 = t3367 * t1469;
    let t5063 = t1719 * t1130;
    let t5071 = t3390 * t1723;
    let t5087 = t3407 * t1723;
    let t5093 = t698 * t1729;
    let t5120 = t1737 * t1160;
    (t5046, t5051, t5063, t5071, t5087, t5093, t5120)
}
