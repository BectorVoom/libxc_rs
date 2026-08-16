//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 677/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk677(t118: f64, t9781: f64, t1986: f64, t1994: f64, t2283: f64, t2412: f64, t128: f64, t1910: f64, t2001: f64, t675: f64, t2286: f64, t1934: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9782 = t118 * t9781;
    let t9783 = t1986 * t9782;
    let t9784 = t1994 * t9783;
    let t9785 = 0.53205749866622299248e-5_f64 * t9784;
    let t9786 = t2412 * t2283;
    let t9787 = 0.85129199786595678796e-5_f64 * t9786;
    let t9788 = t128 * t1910;
    let t9789 = t118 * t9788;
    let t9790 = t2001 * t9789;
    let t9791 = t675 * t9790;
    let t9792 = 0.42564599893297839398e-5_f64 * t9791;
    let t9793 = t2412 * t2286;
    let t9794 = 0.25538759935978703638e-4_f64 * t9793;
    let t9795 = t1986 * t1934;
    (t9783, t9785, t9787, t9790, t9792, t9794, t9795)
}
