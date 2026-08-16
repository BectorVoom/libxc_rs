//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 723/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk723(t265: f64, t729: f64, t9692: f64, t731: f64, t8232: f64, t768: f64, t1882: f64, t2563: f64, t2559: f64, t724: f64, t9587: f64, t2594: f64, t9578: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9819 = t729 * t265 * t9692;
    let t9822 = t8232 * t731;
    let t9824 = t8232 * t768;
    let t9826 = t1882 * t2563;
    let t9828 = t1882 * t2559;
    let t9831 = t724 * t265 * t9587;
    let t9835 = t2594 * t265 * t9578;
    (t9819, t9822, t9824, t9826, t9828, t9831, t9835)
}
