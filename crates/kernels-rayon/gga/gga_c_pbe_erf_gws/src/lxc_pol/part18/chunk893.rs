//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 893/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk893(t353: f64, t9915: f64, t4386: f64, t1109: f64, t898: f64, t938: f64, t859: f64, t1105: f64, t3306: f64, t2376: f64, t2409: f64, t3060: f64, t8589: f64) -> (f64, f64, f64, f64, f64) {
    let t9916 = t353 * t9915;
    let t9917 = t4386 * t9916;
    let t9920 = t898 * t1109;
    let t9921 = t9920 * t938;
    let t9922 = t353 * t9921;
    let t9923 = t859 * t9922;
    let t9926 = t1105 * t3306;
    let t9928 = t2409 * t2376 * t9926;
    let t9932 = t2409 * t8589 * t3060;
    (t9917, t9923, t9926, t9928, t9932)
}
