//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 557/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk557(t921: f64, t925: f64, t287: f64, t530: f64, t321: f64, t320: f64, t327: f64, t301: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2740 = t921 * t925;
    let t2742 = t530 * t287;
    let t2743 = t321 * t2742;
    let t2745 = 0.19318136643975017455e-1_f64 * t320 * t2743;
    let t2746 = t327 * t327;
    let t2747 = 1.0_f64 / t2746;
    let t2748 = t2747 * t301;
    (t2740, t2742, t2743, t2745, t2746, t2747, t2748)
}
