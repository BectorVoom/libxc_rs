//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 965/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk965(t3479: f64, t636: f64, t3493: f64, t3397: f64, t577: f64, t184: f64, t199: f64, t7778: f64, t3399: f64, t612: f64, t1004: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10887 = t3479 * t636;
    let t10888 = 4.0_f64 / 45.0_f64 * t10887;
    let t10889 = t3493 * t636;
    let t10890 = 8.0_f64 / 45.0_f64 * t10889;
    let t10891 = t3397 * t577;
    let t10892 = t10891 * t184;
    let t10894 = 4.0_f64 / 15.0_f64 * t10892 * t199;
    let t10895 = 16.0_f64 / 45.0_f64 * t7778;
    let t10897 = 4.0_f64 / 15.0_f64 * t3399 * t612;
    let t10898 = t562 * t1004;
    (t10888, t10890, t10894, t10895, t10897, t10898)
}
