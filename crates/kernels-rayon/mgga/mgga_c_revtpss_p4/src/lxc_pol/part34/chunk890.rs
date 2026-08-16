//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 890/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk890(t2970: f64, t6173: f64, t3014: f64, t6205: f64, t2926: f64, t6141: f64, t342: f64, t6343: f64, t6234: f64, t993: f64, t225: f64, t3011: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19275 = t6173 * t2970;
    let t19303 = t6205 * t3014;
    let t19330 = t6141 * t2926;
    let t19351 = t342 * t6343;
    let t19462 = t6234 * t993;
    let t19463 = t19462 * t225;
    let t19467 = t3011 * t6205;
    (t19275, t19303, t19330, t19351, t19462, t19463, t19467)
}
