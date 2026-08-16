//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 857/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk857(t11846: f64, t11852: f64, t11857: f64, t11864: f64, t13456: f64, t13457: f64, t13459: f64, t13465: f64, t13470: f64, t13475: f64, t13478: f64, t13479: f64, t13481: f64, t902: f64) -> f64 {
    let t13484 = t13456 - t13457 - t13459 - 7.0_f64 / 256.0_f64 * t11846 - t13465 + 7.0_f64 / 192.0_f64 * t11852 + t13470 - 7.0_f64 / 96.0_f64 * t11857 - t13475 - 7.0_f64 / 384.0_f64 * t11864 + t13478 + t13479 + t902 * t13481 / 1536.0_f64;
    t13484
}
