//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 594/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk594(t4884: f64, t818: f64, t2520: f64, t4868: f64, t2444: f64, t3640: f64, t4770: f64, t4774: f64, t4778: f64, t232: f64, t4818: f64, t799: f64) -> (f64, f64, f64, f64, f64) {
    let t4885 = t4884 * t818;
    let t4888 = t4868 * t2520;
    let t4895 = t2444 + 0.11872222222222222222e-1_f64 * t3640 - 0.11872222222222222222e-1_f64 * t4770 + 0.35616666666666666666e-1_f64 * t4774 - 0.17808333333333333333e-1_f64 * t4778;
    let t4897 = 0.62182e-1_f64 * t4895 * t232;
    let t4898 = t4818 * t799;
    (t4885, t4888, t4895, t4897, t4898)
}
