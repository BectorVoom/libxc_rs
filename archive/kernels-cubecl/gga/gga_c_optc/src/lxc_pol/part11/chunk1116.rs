//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1116/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1116<F: Float>(t3132: F, t3133: F, t46715: F, t1442: F, t9123: F, t26881: F, t1111: F, t5289: F, t530: F, t3108: F, t45811: F, t12105: F, t4363: F) -> (F, F, F, F, F, F) {
    let t46717 = t3132 * t46715 * t3133;
    let t46729 = t9123 * t1442;
    let t46733 = t26881 * t1442;
    let t46792 = t1111 * t530 * t5289;
    let t46810 = t45811 * t3108;
    let t46820 = t4363 * t12105;
    (t46717, t46729, t46733, t46792, t46810, t46820)
}
