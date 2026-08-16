//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1184/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1184(t50935: f64, t822: f64, t824: f64, t838: f64, t13984: f64, t2331: f64, t328: f64, t356: f64, t3971: f64, t3976: f64, t15636: f64, t3973: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t50936 = t822 * t50935;
    let t50942 = t824 * t838;
    let t50943 = t822 * t50942;
    let t50944 = t50943 * t13984;
    let t50948 = t356 * t328 * t2331 * t3971;
    let t50949 = t50948 * t3976;
    let t50956 = t3973 * t15636;
    (t50936, t50942, t50943, t50944, t50948, t50949, t50956)
}
