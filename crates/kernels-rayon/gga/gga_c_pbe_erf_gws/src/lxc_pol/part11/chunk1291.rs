//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1291/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1291(t13142: f64, t13290: f64, t2501: f64, t2503: f64, t28413: f64, t28487: f64, t326: f64, t338: f64, t353: f64, t36340: f64, t376: f64, t39490: f64, t39510: f64, t39521: f64, t39523: f64, t46892: f64, t46914: f64, t46928: f64, t48985: f64, t49464: f64, t826: f64, t829: f64, t830: f64, t833: f64, t844: f64, t8659: f64) -> f64 {
    let t50709 = 35.0_f64 / 72.0_f64 * t36340 + t326 * t49464 * t826 * t833 / 96.0_f64 + t13142 * t2503 / 24.0_f64 + t8659 * t829 * t830 * t2501 * t13290 / 12.0_f64 + 455.0_f64 / 162.0_f64 * t28413 - 7.0_f64 / 12.0_f64 * t46892 - 7.0_f64 / 12.0_f64 * t46914 + 7.0_f64 / 12.0_f64 * t46928 + 35.0_f64 / 36.0_f64 * t39490 + 455.0_f64 / 324.0_f64 * t28487 - 35.0_f64 / 36.0_f64 * t39510 - 35.0_f64 / 18.0_f64 * t39521 - 35.0_f64 / 72.0_f64 * t39523 - t844 * t338 * t353 * t376 * t48985 / 48.0_f64;
    t50709
}
