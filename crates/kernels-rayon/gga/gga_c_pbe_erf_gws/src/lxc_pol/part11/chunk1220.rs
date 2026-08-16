//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1220/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1220(t44629: f64, t44672: f64, t2157: f64, t3717: f64, t11478: f64, t2170: f64, t3138: f64, t13347: f64, t2168: f64, t13334: f64, t3131: f64, t3139: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49371 = 7.0_f64 / 24.0_f64 * t44629;
    let t49372 = 7.0_f64 / 12.0_f64 * t44672;
    let t49374 = t2157 * t3717;
    let t49378 = t3138 * t2170 * t11478 * t49374 / 4.0_f64;
    let t49382 = t2168 * t2170 * t11478 * t13347 / 8.0_f64;
    let t49387 = t3138 * t3139 * t3131 * t2157 * t13334 / 12.0_f64;
    (t49371, t49372, t49374, t49378, t49382, t49387)
}
