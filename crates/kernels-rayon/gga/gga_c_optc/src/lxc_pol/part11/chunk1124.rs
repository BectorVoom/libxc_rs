//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1124/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1124(t3563: f64, t4611: f64, t16572: f64, t714: f64, t16433: f64, t22892: f64, t16429: f64, t2007: f64, t16373: f64, t2030: f64, t16330: f64, t6799: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48162 = t4611 * t3563;
    let t48183 = t16572 * t714;
    let t48212 = t22892 * t16433;
    let t48214 = t2007 * t16429;
    let t48260 = t2030 * t16373;
    let t48262 = t6799 * t16330;
    (t48162, t48183, t48212, t48214, t48260, t48262)
}
