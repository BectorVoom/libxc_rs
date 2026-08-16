//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2098;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta587(t46206: f64, t4199: f64, t9494: f64, t12945: f64, t2427: f64, t12858: f64, t2528: f64, t2371: f64, t13123: f64, t9885: f64, t1409: f64, t2516: f64, t4194: f64, t607: f64, t9722: f64, t2535: f64, t4205: f64, t9868: f64, t193: f64, t776: f64, t707: f64, t9862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46207, t46208, t46218, t46235, t46237, t46278, t46291) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2098(t46206, t4199, t9494, t12945, t2427, t12858, t2528, t2371, t13123, t9885, t1409, t2516, t4194, t607);
        let (t46292, t46302, t46311, t46336, t46341, t46369) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2099(t46291, t4199, t9722, t12858, t2535, t4205, t9868, t193, t776, t1409, t707, t9862);
    (t46207, t46208, t46218, t46235, t46237, t46278, t46292, t46302, t46311, t46336, t46341, t46369)
}
