//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2098;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta587<F: Float>(t46206: F, t4199: F, t9494: F, t12945: F, t2427: F, t12858: F, t2528: F, t2371: F, t13123: F, t9885: F, t1409: F, t2516: F, t4194: F, t607: F, t9722: F, t2535: F, t4205: F, t9868: F, t193: F, t776: F, t707: F, t9862: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46207, t46208, t46218, t46235, t46237, t46278, t46291) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2098::<F>(t46206, t4199, t9494, t12945, t2427, t12858, t2528, t2371, t13123, t9885, t1409, t2516, t4194, t607);
        let (t46292, t46302, t46311, t46336, t46341, t46369) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2099::<F>(t46291, t4199, t9722, t12858, t2535, t4205, t9868, t193, t776, t1409, t707, t9862);
    (t46207, t46208, t46218, t46235, t46237, t46278, t46292, t46302, t46311, t46336, t46341, t46369)
}
