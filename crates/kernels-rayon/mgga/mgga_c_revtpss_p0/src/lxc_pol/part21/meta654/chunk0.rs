//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2441/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2441(t11727: f64, t3106: f64, t3223: f64, t3230: f64, t11817: f64, t3224: f64, t1024: f64, t11961: f64, t3042: f64, t3056: f64, t225: f64, t11274: f64, t12009: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42338 = t3106 * t11727;
    let t42340 = t3223 * t3230;
    let t42346 = t3224 * t11817;
    let t42355 = t1024 * t11961;
    let t42358 = t3042 * t3056;
    let t42359 = t42358 * t225;
    let t42369 = t11274 * t12009;
    (t42338, t42340, t42346, t42355, t42358, t42359, t42369)
}
