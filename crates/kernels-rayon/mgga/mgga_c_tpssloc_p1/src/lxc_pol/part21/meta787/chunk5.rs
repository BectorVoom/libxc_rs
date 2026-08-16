//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2743/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2743(t57850: f64, t57873: f64, t157: f64, t182: f64, t145: f64, t185: f64, t46125: f64, t46128: f64, t46130: f64, t16576: f64, t751: f64, t46132: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57874 = t57850 + t57873;
    let t57877 = 0.19751673498613801407e-1_f64 * t57874 * t157 * t182;
    let t57879 = t145 * t57874 * t185;
    let t57880 = 0.70178683471615754484e1_f64 * t46125;
    let t57885 = 0.36622894612013090108e-3_f64 * t46128;
    let t57886 = 0.10389515463408878255e3_f64 * t46130;
    let t57887 = t16576 * t751;
    let t57888 = 2.0_f64 * t57887;
    let t57889 = 0.65061487801810439052e-1_f64 * t46132;
    (t57877, t57879, t57880, t57885, t57886, t57888, t57889)
}
