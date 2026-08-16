//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1001/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1001(t43: f64, t1885: f64, t1891: f64, t1933: f64, t22014: f64, t22015: f64, t22021: f64, t22028: f64, t607: f64, t6533: f64, t6537: f64, t6541: f64, t2854: f64, t52: f64, zeta_threshold: f64) -> (f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t22032 = piecewise3(t44, 0.0_f64, -56.0_f64 / 81.0_f64 * t22014 * t22015 + 16.0_f64 / 9.0_f64 * t6533 * t1885 * t1891 - 2.0_f64 / 3.0_f64 * t1933 * t22021 - 8.0_f64 / 9.0_f64 * t6537 * t6541 + 2.0_f64 / 3.0_f64 * t607 * t22028);
    let t22034 = 1.0_f64 / t52 / t2854;
    (t22032, t22034)
}
