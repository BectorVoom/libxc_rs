//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 694/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk694(t9901: f64, t9925: f64, t515: f64, t235: f64, t128: f64, t1818: f64, t118: f64, t7418: f64, t675: f64, t1927: f64, t1986: f64, t1937: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9926 = t9901 + t9925;
    let t9927 = t515 * t9926;
    let t9928 = t235 * t9927;
    let t9929 = 0.19957069503106347607e-1_f64 * t9928;
    let t9930 = t128 * t1818;
    let t9931 = t118 * t9930;
    let t9932 = t7418 * t9931;
    let t9933 = t675 * t9932;
    let t9934 = 0.85129199786595678796e-5_f64 * t9933;
    let t9935 = t1986 * t1927;
    let t9936 = t675 * t9935;
    let t9937 = 0.25538759935978703638e-4_f64 * t9936;
    let t9938 = t1986 * t1937;
    (t9926, t9927, t9929, t9932, t9934, t9935, t9937, t9938)
}
