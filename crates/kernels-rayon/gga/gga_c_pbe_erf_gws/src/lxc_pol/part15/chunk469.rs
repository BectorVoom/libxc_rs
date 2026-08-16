//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 469/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk469(t707: f64, t719: f64, t256: f64, t19: f64, t535: f64, t336: f64, t714: f64, t247: f64, t24: f64, t712: f64, t1243: f64, t1251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1910 = t707 * t719;
    let t1911 = t1910 * t256;
    let t1913 = t535 * t19;
    let t1914 = t1913 * t336;
    let t1915 = t1914 * t714;
    let t1917 = t247 * t719;
    let t1918 = t24 * t1917;
    let t1920 = 0.12155555555555555555e0_f64 * t712 * t1918;
    let t1923 = -0.43111111111111111111e-1_f64 * t1243 + 0.18777777777777777778e0_f64 * t1251;
    (t1910, t1911, t1913, t1914, t1915, t1917, t1918, t1920, t1923)
}
