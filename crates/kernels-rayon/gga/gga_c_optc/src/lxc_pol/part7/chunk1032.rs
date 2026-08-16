//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1032/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1032(t1764: f64, t22512: f64, t2002: f64, t518: f64, t517: f64, t11: f64, t2: f64, t209: f64, t6567: f64, t543: f64, t6374: f64, t1776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22513 = t1764 * t22512;
    let t22515 = t518 * t2002;
    let t22516 = t517 * t22515;
    let t22519 = f64::powf(t11, -0.25e1_f64);
    let t22522 = t22519 * t2 * t6567 * t209;
    let t22524 = t6374 * t543;
    let t22526 = t1776 * t22512;
    (t22513, t22515, t22516, t22522, t22524, t22526)
}
