//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1209/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1209(t1493: f64, t1925: f64, t119457: f64, t644: f64, t1497: f64, t8442: f64, t640: f64, t1469: f64, t92669: f64, t32591: f64, t4186: f64, t8621: f64, t8622: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128401 = t1925 * t1493;
    let t128403 = t119457 * t128401 * t644;
    let t128409 = t1925 * t1497;
    let t128411 = t8442 * t128409 * t644;
    let t128415 = t119457 * t128409 * t640;
    let t128424 = t8442 * t92669 * t1469;
    let t128428 = t8442 * t32591 * t4186;
    let t128434 = t8621 * t8622 * t1469;
    (t128403, t128411, t128415, t128424, t128428, t128434)
}
