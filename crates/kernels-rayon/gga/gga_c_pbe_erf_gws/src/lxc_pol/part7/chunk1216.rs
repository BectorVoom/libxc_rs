//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1216/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1216(t2132: f64, t6110: f64, t822: f64, t2138: f64, t6336: f64, t6535: f64, t6472: f64, t6800: f64, t6449: f64, t875: f64, t2319: f64, t6266: f64) -> (f64, f64, f64, f64, f64) {
    let t21591 = t6110 * t2132;
    let t21592 = t822 * t21591;
    let t21594 = t21592 * t2138 / 24.0_f64;
    let t21596 = t6336 * t6535 / 4.0_f64;
    let t21597 = t6472 * t2132;
    let t21598 = t6800 * t21597;
    let t21600 = t21598 * t2138 / 24.0_f64;
    let t21601 = t6449 * t875;
    let t21605 = t2319 * t6266;
    (t21594, t21596, t21600, t21601, t21605)
}
