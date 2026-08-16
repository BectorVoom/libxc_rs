//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1112/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1112(t3107: f64, t45811: f64, t27351: f64, t3234: f64, t5355: f64, t1162: f64, t5285: f64, t7274: f64, t5289: f64, t3138: f64, t5417: f64, t44090: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45812 = t45811 * t3107;
    let t45885 = t3234 * t27351 * t5355;
    let t45954 = t1162 * t7274 * t5285;
    let t45968 = t1162 * t7274 * t5289;
    let t46007 = t5417 * t3138;
    let t46014 = t44090 * t3107;
    (t45812, t45885, t45954, t45968, t46007, t46014)
}
