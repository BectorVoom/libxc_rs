//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta716 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2552;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2553;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta716<F: Float>(t2735: F, t9792: F, t1413: F, t46826: F, t1376: F, t40769: F, t3989: F, t9986: F, t10001: F, t221: F, t4019: F, t9912: F, t10111: F, t1386: F, t9720: F, t1390: F, t1399: F, t685: F, t9970: F, t9976: F, t3930: F, t9893: F, t3957: F, t9700: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46835, t46837, t46840, t46846, t46853) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2552::<F>(t2735, t9792, t1413, t46826, t1376, t40769, t3989, t9986, t10001, t221, t4019, t9912);
        let (t46856, t46859, t46861, t46863, t46865) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2553::<F>(t10111, t1386, t9720, t1390, t1399, t685, t9970, t9976, t3930, t9893, t3957, t9700);
    (t46835, t46837, t46840, t46846, t46853, t46856, t46859, t46861, t46863, t46865)
}
