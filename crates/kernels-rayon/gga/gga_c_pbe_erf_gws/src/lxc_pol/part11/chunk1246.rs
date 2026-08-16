//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1246/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1246(t3065: f64, t49508: f64, t858: f64, t8978: f64, t12069: f64, t13414: f64, t3123: f64, t46451: f64, t11787: f64, t36659: f64, t36641: f64, t13252: f64, t37632: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49745 = t8978 * t3065 * t858 * t49508 / 24.0_f64;
    let t49761 = t13414 * t12069 / 4.0_f64;
    let t49763 = t3123 * t46451 / 24.0_f64;
    let t49765 = t36659 * t11787 / 8.0_f64;
    let t49767 = t36641 * t11787 / 8.0_f64;
    let t49768 = t37632 * t13252;
    (t49745, t49761, t49763, t49765, t49767, t49768)
}
