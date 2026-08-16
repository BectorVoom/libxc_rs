//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 627/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk627(t1075: f64, t5154: f64, t2976: f64, t5122: f64, t2980: f64, t4068: f64, t5108: f64, t5112: f64, t5115: f64, t389: f64, t1460: f64, t4144: f64) -> (f64, f64, f64, f64, f64) {
    let t5155 = t5154 * t1075;
    let t5158 = t5122 * t2976;
    let t5165 = t2980 + 0.11872222222222222222e-1_f64 * t4068 - 0.11872222222222222222e-1_f64 * t5108 + 0.35616666666666666666e-1_f64 * t5112 - 0.17808333333333333333e-1_f64 * t5115;
    let t5167 = 0.62182e-1_f64 * t5165 * t389;
    let t5169 = 2.0_f64 * t4144 * t1460;
    (t5155, t5158, t5165, t5167, t5169)
}
