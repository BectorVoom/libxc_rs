//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 536/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk536(t3223: f64, t366: f64, t362: f64, t40: f64, t611: f64, t361: f64, t351: f64, t1054: f64, t1058: f64, t1014: f64, t2857: f64, t2251: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3224 = t3223 * t366;
    let t3229 = 1.0_f64 / t40 / t362 / t611;
    let t3230 = t361 * t3229;
    let t3231 = t351 * t3230;
    let t3234 = t1054 * t1058;
    let t3236 = t1014 * t2857;
    let t3237 = t3236 * t2251;
    (t3224, t3229, t3230, t3231, t3234, t3237)
}
