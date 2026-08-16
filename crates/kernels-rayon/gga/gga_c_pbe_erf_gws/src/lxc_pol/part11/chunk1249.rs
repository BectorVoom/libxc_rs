//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1249/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1249(t13126: f64, t3786: f64, t4395: f64, t860: f64, t13293: f64, t36962: f64, t45584: f64, t28269: f64, t3065: f64, t49794: f64, t858: f64, t45133: f64, t9016: f64) -> (f64, f64, f64, f64, f64) {
    let t49819 = t13126 * t4395 * t3786 * t860 / 16.0_f64;
    let t49826 = 11.0_f64 / 96.0_f64 * t36962 * t13293;
    let t49828 = 7.0_f64 / 4.0_f64 * t45584;
    let t49832 = t28269 * t3065 * t858 * t49794 / 8.0_f64;
    let t49837 = t9016 * t45133 / 4.0_f64;
    (t49819, t49826, t49828, t49832, t49837)
}
