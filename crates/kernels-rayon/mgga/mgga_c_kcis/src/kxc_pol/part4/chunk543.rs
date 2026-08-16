//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 543/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk543(t233: f64, t2806: f64, t1008: f64, t296: f64, t1121: f64, t1133: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2807 = t233 * t2806;
    let t2808 = t2807 / 8.0_f64;
    let t2809 = t1008 * t1008;
    let t2810 = t296 * t296;
    let t2811 = 1.0_f64 / t2810;
    let t2812 = t2809 * t2811;
    let t2815 = t1121 * t1133;
    (t2808, t2809, t2810, t2811, t2812, t2815)
}
