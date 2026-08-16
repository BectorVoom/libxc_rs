//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1060/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1060(t3014: f64, t6205: f64, t2926: f64, t6141: f64, t342: f64, t6343: f64, t6271: f64, t73: f64, t11249: f64, t6305: f64) -> (f64, f64, f64, f64, f64) {
    let t19303 = t6205 * t3014;
    let t19330 = t6141 * t2926;
    let t19351 = t342 * t6343;
    let t19446 = t6271 * t73;
    let t19450 = t6305 * t11249;
    (t19303, t19330, t19351, t19446, t19450)
}
