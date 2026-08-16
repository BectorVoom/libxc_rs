//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1134/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1134(t3279: f64, t4049: f64, t14011: f64, t3232: f64, t1125: f64, t14024: f64, t3139: f64, t9026: f64, t4028: f64, t14007: f64, t3261: f64, t3291: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14516 = t4049 * t3279;
    let t14518 = t14011 * t3232;
    let t14520 = t1125 * t14024;
    let t14522 = t3139 * t9026;
    let t14523 = t4028 * t14522;
    let t14525 = t14007 * t3261;
    let t14529 = t14007 * t3291;
    (t14516, t14518, t14520, t14522, t14523, t14525, t14529)
}
