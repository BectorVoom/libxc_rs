//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2972/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2972(t13910: f64, t808: f64, t9736: f64, t14026: f64, t9744: f64, t13821: f64, t13999: f64, t13716: f64, t1413: f64, t547: f64, t807: f64, t550: f64, t9794: f64) -> (f64, f64, f64, f64, f64) {
    let t49056 = t9736 * t808 * t13910;
    let t49058 = t9744 * t14026;
    let t49062 = t13999 * t13821;
    let t49066 = t807 * t547 * t1413 * t13716;
    let t49068 = t9794 * t550;
    (t49056, t49058, t49062, t49066, t49068)
}
