//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 865/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk865(t9176: f64, t11551: f64, t3257: f64, t3803: f64, t12039: f64, t13124: f64, t4394: f64, t6608: f64, t6610: f64, t860: f64, t9182: f64, t13172: f64, t824: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13569 = 35.0_f64 / 72.0_f64 * t9176;
    let t13571 = t3257 * t3803 * t11551;
    let t13575 = 7.0_f64 / 96.0_f64 * t12039;
    let t13578 = t13124 * t4394;
    let t13580 = t6608 * t13578 * t6610;
    let t13582 = t13580 * t860 / 96.0_f64;
    let t13583 = 35.0_f64 / 144.0_f64 * t9182;
    let t13585 = t13172 * t824;
    (t13569, t13571, t13575, t13578, t13580, t13582, t13583, t13585)
}
