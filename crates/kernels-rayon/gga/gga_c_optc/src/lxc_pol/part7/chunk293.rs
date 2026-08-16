//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 293/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk293(t289: f64, t314: f64, t854: f64, t860: f64, t862: f64, t867: f64, t874: f64, t878: f64, t885: f64, t891: f64, t893: f64, t899: f64) -> f64 {
    let t902 = -t854 * t289 / 36.0_f64 + t860 + t862 * t867 / 288.0_f64 + 0.35500316489081544176e-1_f64 * t874 * t878 - 0.14488602482981263091e-1_f64 * t885 * t314 + t891 + 0.18110753103726578864e-2_f64 * t893 * t899;
    t902
}
