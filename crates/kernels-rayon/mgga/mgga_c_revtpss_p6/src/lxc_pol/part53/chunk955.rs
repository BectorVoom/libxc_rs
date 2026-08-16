//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 955/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk955(t1936: f64, t670: f64, t1518: f64, t572: f64, t26123: f64, t4292: f64, t7330: f64, t1459: f64, t7953: f64, t116: f64, t7741: f64, t117: f64, t28042: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28264 = t670 * t1936;
    let t28265 = t28264 * t1518;
    let t28267 = 6.0_f64 * t572 * t28265;
    let t28268 = t26123 * t1518;
    let t28270 = 6.0_f64 * t572 * t28268;
    let t28271 = t7330 * t4292;
    let t28273 = 6.0_f64 * t572 * t28271;
    let t28275 = 3.0_f64 * t1459 * t7953;
    let t28276 = t116 * t7741;
    let t28277 = t28276 * t670;
    let t28279 = 6.0_f64 * t572 * t28277;
    let t28280 = t117 * t28042;
    (t28264, t28265, t28267, t28268, t28270, t28271, t28273, t28275, t28277, t28279, t28280)
}
