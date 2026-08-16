//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1213/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1213(t21535: f64, t2250: f64, t6640: f64, t2216: f64, t6480: f64, t21447: f64, t2155: f64, t858: f64, t867: f64, t19646: f64, t346: f64, t2124: f64, t822: f64) -> (f64, f64, f64, f64) {
    let t21536 = t2250 * t21535;
    let t21537 = t21536 * t6640;
    let t21539 = t6480 * t2216;
    let t21540 = 35.0_f64 / 36.0_f64 * t21539;
    let t21544 = t2155 * t867 * t858 * t21447 / 16.0_f64;
    let t21560 = t19646 * t346;
    let t21563 = t822 * t21560 * t2124 / 32.0_f64;
    (t21537, t21540, t21544, t21563)
}
