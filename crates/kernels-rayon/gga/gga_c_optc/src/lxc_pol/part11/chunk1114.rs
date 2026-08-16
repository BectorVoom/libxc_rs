//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1114/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1114(t5301: f64, t7878: f64, t1179: f64, t3137: f64, t3186: f64, t5407: f64, t27515: f64, t3244: f64, t5355: f64, t3169: f64, t5344: f64, t3138: f64, t5280: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46297 = t7878 * t5301;
    let t46298 = t1179 * t46297;
    let t46314 = t3186 * t3137 * t5407;
    let t46390 = t3244 * t27515 * t5355;
    let t46413 = t5344 * t3169;
    let t46452 = t5280 * t3138;
    (t46297, t46298, t46314, t46390, t46413, t46452)
}
