//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1018/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1018(t141: f64, t22052: f64, t659: f64, t661: f64, t6923: f64, t2030: f64, t6870: f64, t2070: f64, t6893: f64, t2020: f64, t6892: f64, t2026: f64) -> (f64, f64, f64, f64, f64) {
    let t22233 = t659 * t141 * t22052;
    let t22236 = t6923 * t661;
    let t22238 = t2030 * t6870;
    let t22240 = t6893 * t2070;
    let t22242 = t2020 * t6892;
    let t22243 = t22242 * t2026;
    (t22233, t22236, t22238, t22240, t22243)
}
