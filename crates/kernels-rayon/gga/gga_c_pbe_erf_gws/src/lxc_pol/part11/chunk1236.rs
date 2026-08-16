//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1236/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1236(t3065: f64, t49529: f64, t858: f64, t8978: f64, t13285: f64, t2118: f64, t2277: f64, t3257: f64, t44970: f64, t44977: f64, t45574: f64, t49474: f64, t49528: f64, t49538: f64, t49540: f64, t49545: f64, t49550: f64, t6158: f64, t6637: f64, t9499: f64) -> (f64, f64) {
    let t49555 = t8978 * t3065 * t858 * t49529 / 16.0_f64;
    let t49556 = t2277 * t3257 * t45574 * t13285 / 192.0_f64 + t49528 + t6637 * t9499 * t2118 * t49529 / 128.0_f64 + t49538 - t49540 - t6637 * t9499 * t6158 * t49474 / 96.0_f64 - t49545 - 7.0_f64 / 288.0_f64 * t44970 + t49550 + 7.0_f64 / 48.0_f64 * t44977 + t49555;
    (t49555, t49556)
}
