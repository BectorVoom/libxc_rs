//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1001/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1001(t110: f64, t1756: f64, t1759: f64, t209: f64, t6475: f64, t6481: f64, t115: f64, t6568: f64, t103: f64, t193: f64, t197: f64, t2078: f64) -> (f64, f64, f64, f64) {
    let t21903 = 0.28493333333333333334e0_f64 * t209 * t110 * t1756 * t1759;
    let t21907 = 0.4274e0_f64 * t209 * t6481 * t6475;
    let t21979 = t6568 * t115;
    let t21988 = 261800.0_f64 / 729.0_f64 * t193 * t2078 * t103 * t197;
    (t21903, t21907, t21979, t21988)
}
