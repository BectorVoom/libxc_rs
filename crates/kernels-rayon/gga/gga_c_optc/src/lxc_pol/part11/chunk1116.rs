//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1116/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1116(t3132: f64, t3133: f64, t46715: f64, t1442: f64, t9123: f64, t26881: f64, t1111: f64, t5289: f64, t530: f64, t3108: f64, t45811: f64, t12105: f64, t4363: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46717 = t3132 * t46715 * t3133;
    let t46729 = t9123 * t1442;
    let t46733 = t26881 * t1442;
    let t46792 = t1111 * t530 * t5289;
    let t46810 = t45811 * t3108;
    let t46820 = t4363 * t12105;
    (t46717, t46729, t46733, t46792, t46810, t46820)
}
