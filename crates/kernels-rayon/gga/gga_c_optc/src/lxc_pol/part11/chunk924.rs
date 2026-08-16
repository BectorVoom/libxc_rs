//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 924/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk924(t17314: f64, t275: f64, t176: f64, t16824: f64, t16826: f64, t16828: f64, t16860: f64, t16864: f64, t16866: f64, t16869: f64, t16877: f64, t17039: f64, t17043: f64, t17249: f64, t277: f64, t364: f64, t95: f64, t962: f64, sigma0: f64) -> (f64, f64) {
    let t17315 = t17314 * t275;
    let t17317 = t176 * t17315 * sigma0;
    let t17320 = 0.25844881434903430496e-2_f64 * t95 * t277 * t17249 * t962 + t17317 * t364 / 2.0_f64 + t17043 + t16824 + t16826 + t16828 + t16860 + t16864 - t16866 - t16869 + t16877 + t17039;
    (t17317, t17320)
}
