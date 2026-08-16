//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 856/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk856(t13468: f64, t3138: f64, t13220: f64, t6659: f64, t858: f64, t884: f64, t11925: f64, t3128: f64, t11869: f64, t1113: f64, t13140: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13470 = t3138 * t13468 / 16.0_f64;
    let t13473 = t6659 * t858 * t13220;
    let t13475 = t884 * t13473 / 4.0_f64;
    let t13478 = 3.0_f64 / 16.0_f64 * t3128 * t11925;
    let t13479 = 7.0_f64 / 96.0_f64 * t11869;
    let t13480 = t1113 * t13140;
    let t13481 = t905 * t13480;
    (t13470, t13473, t13475, t13478, t13479, t13481)
}
