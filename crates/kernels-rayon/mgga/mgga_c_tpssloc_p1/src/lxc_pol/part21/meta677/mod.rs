//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2483;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta677(t4199: f64, t9494: f64, t13471: f64, t870: f64, t12945: f64, t2427: f64, t12858: f64, t2528: f64, t2371: f64, t4205: f64, t9909: f64, t13123: f64, t9885: f64, t12908: f64, t12924: f64, t4101: f64, t9912: f64, t1409: f64, t2516: f64, t4194: f64, t607: f64, t9722: f64, t12887: f64, t172: f64, t763: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46208, t46213, t46217, t46234, t46236, t46244, t46278) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2483(t4199, t9494, t13471, t870, t12945, t2427, t12858, t2528, t2371, t4205, t9909, t13123, t9885);
        let (t46283, t46285, t46291, t46302, t46308) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2484(t12908, t12924, t4101, t9912, t1409, t2516, t4194, t607, t4199, t9722, t12887, t172, t763);
    (t46208, t46213, t46217, t46234, t46236, t46244, t46278, t46283, t46285, t46291, t46302, t46308)
}
