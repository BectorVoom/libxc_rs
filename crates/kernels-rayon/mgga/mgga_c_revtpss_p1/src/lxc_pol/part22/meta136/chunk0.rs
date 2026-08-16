//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 908/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk908(t3230: f64, t351: f64, t1054: f64, t1058: f64, t1014: f64, t2857: f64, t2251: f64, t1012: f64, t1010: f64, t614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3231 = t351 * t3230;
    let t3234 = t1054 * t1058;
    let t3236 = t1014 * t2857;
    let t3237 = t3236 * t2251;
    let t3238 = t1012 * t3237;
    let t3241 = t614 * t1010;
    (t3231, t3234, t3236, t3237, t3238, t3241)
}
