//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 777/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk777(t138: f64, t4637: f64, t1: f64, t9735: f64, t123: f64, t4599: f64, t4626: f64, t6799: f64, t6: f64, t2024: f64, t4623: f64, t1256: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13124 = t4637 * t138;
    let t13129 = t9735 * t1;
    let t13136 = t123 * t4599;
    let t13158 = t6799 * t4626;
    let t13160 = t6 * t4599;
    let t13174 = t4623 * t2024;
    let t13185 = t2024 * t1256;
    (t13124, t13129, t13136, t13158, t13160, t13174, t13185)
}
