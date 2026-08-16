//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 511/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk511(t2938: f64, t2939: f64, t898: f64, t2400: f64, t2402: f64, t2407: f64, t2411: f64, t2415: f64, t2698: f64, t2701: f64) -> (f64, f64) {
    let t2941 = t898 * t2938 * t2939;
    let t2946 = 0.19257444444444444444e0_f64 * t2400;
    let t2951 = -0.117377e0_f64 * t2698 + 0.234754e0_f64 * t2701 + t2946 + 0.9628722222222222222e-1_f64 * t2402 - 0.9628722222222222222e-1_f64 * t2407 + 0.28886166666666666666e0_f64 * t2411 - 0.14443083333333333333e0_f64 * t2415;
    (t2941, t2951)
}
