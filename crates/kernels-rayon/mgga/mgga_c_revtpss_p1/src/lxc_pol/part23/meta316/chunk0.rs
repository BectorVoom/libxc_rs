//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1604/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1604(t1466: f64, t2246: f64, t2275: f64, t4186: f64, t580: f64, t9342: f64, t2282: f64, t10389: f64, t1469: f64, t2299: f64, t10398: f64, t2306: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13272 = t1466 * t2246;
    let t13302 = t2275 * t4186;
    let t13309 = 2.0_f64 * t580;
    let t13310 = 6.0_f64 * t9342;
    let t13324 = t2282 * t4186;
    let t13368 = t10389 * t1469;
    let t13371 = t2299 * t4186;
    let t13378 = t10398 * t1469;
    let t13381 = t2306 * t4186;
    (t13272, t13302, t13309, t13310, t13324, t13368, t13371, t13378, t13381)
}
