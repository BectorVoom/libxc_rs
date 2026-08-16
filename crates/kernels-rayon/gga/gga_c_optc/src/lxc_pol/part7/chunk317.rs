//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 317/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk317(t1002: f64, t1010: f64, t1015: f64, t277: f64, t355: f64, t364: f64, t776: f64, t802: f64, t842: f64, t844: f64, t849: f64, t95: f64, t960: f64, t962: f64, t984: f64, t989: f64, t995: f64, t999: f64) -> f64 {
    let t1018 = -t776 + t802 + t842 + t844 - t849 + 0.25844881434903430496e-2_f64 * t95 * t277 * t960 * t962 + t984 * t364 / 2.0_f64 - 4.0_f64 / 3.0_f64 * t355 * t989 + t995 + t999 * t1002 / 6.0_f64 + 50.0_f64 / 27.0_f64 * t1010 * t1015;
    t1018
}
