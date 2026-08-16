//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 503/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk503(t2025: f64, t2037: f64, t2029: f64, t287: f64, t2028: f64, t758: f64, t154: f64, t277: f64, t486: f64, t276: f64, t301: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2038 = t2037 * t2025;
    let t2039 = t2029 * t287;
    let t2040 = t2028 * t2039;
    let t2041 = t758 * t2040;
    let t2045 = t154 * t486 * t277;
    let t2047 = t276 * t2045 / 432.0_f64;
    let t2048 = t67 * t301;
    (t2038, t2039, t2040, t2041, t2045, t2047, t2048)
}
