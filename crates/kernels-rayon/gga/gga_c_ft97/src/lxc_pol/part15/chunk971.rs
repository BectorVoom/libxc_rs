//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 971/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk971(t1775: f64, t21607: f64, t21599: f64, t21610: f64, t21573: f64, t21581: f64, t2: f64, t21399: f64, t21597: f64, t21595: f64, t21592: f64, t21577: f64, t458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t80911 = t1775 * t21607;
    let t80913 = t1775 * t21599;
    let t80915 = t1775 * t21610;
    let t80942 = t1775 * t21573;
    let t80961 = t1775 * t21581;
    let t80963 = t2 * t21399;
    let t81006 = t1775 * t21597;
    let t81008 = t1775 * t21595;
    let t81010 = t1775 * t21592;
    let t81040 = t458 * t21577;
    (t80911, t80913, t80915, t80942, t80961, t80963, t81006, t81008, t81010, t81040)
}
