//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 590/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk590(t2305: f64, t2310: f64, t3640: f64, t3687: f64, t4770: f64, t4774: f64, t4778: f64, t4806: f64, t4809: f64, t4812: f64) -> f64 {
    let t4846 = t2305 + 0.48461111111111111112e3_f64 * t3640 - 0.48461111111111111111e3_f64 * t4770 + 0.14538333333333333333e4_f64 * t4774 - 0.72691666666666666667e3_f64 * t4778 + t2310 + 0.10488888888888888889e3_f64 * t3687 - 0.26222222222222222222e2_f64 * t4806 + 0.15733333333333333333e3_f64 * t4809 - 0.78666666666666666667e2_f64 * t4812;
    t4846
}
