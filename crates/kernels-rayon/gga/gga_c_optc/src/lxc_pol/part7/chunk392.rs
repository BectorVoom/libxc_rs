//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 392/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk392(t43: f64, t1884: f64, t1885: f64, t1891: f64, t47: f64, t99: f64, t553: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t1895 = piecewise3(t44, 0.0_f64, 4.0_f64 / 9.0_f64 * t1884 * t1885 + 4.0_f64 / 3.0_f64 * t47 * t1891);
    let t1896 = 1.0_f64 / t99;
    let t1897 = t553 * t553;
    (t1895, t1896, t1897)
}
