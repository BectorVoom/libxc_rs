//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2229;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta618<F: Float>(t40722: F, t40726: F, t12858: F, t2528: F, t2371: F, t40729: F, t40733: F, t2745: F, t776: F, t4205: F, t9909: F, t2553: F, t868: F, t40736: F, t10126: F, t12854: F, t1877: F, t2522: F, t40732: F, t4119: F, t4307: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t46228, t46232, t46235, t46237, t46238, t46239, t46240, t46245, t46252) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2229::<F>(t40722, t40726, t12858, t2528, t2371, t40729, t40733, t2745, t776, t4205, t9909, t2553, t868);
        let (t46256, t46257) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2230::<F>(t40736, t10126, t12854, t1877, t2522, t2745, t40732, t4119, t4307, t46235, t46237, t46238, t46239, t46240, t46245, t46252);
    (t46228, t46232, t46235, t46237, t46238, t46239, t46245, t46256, t46257)
}
