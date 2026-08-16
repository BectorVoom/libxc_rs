//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta706 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2532;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2533;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta706<F: Float>(t46456: F, t786: F, t10026: F, t1398: F, t268: F, t4101: F, t543: F, t793: F, t10073: F, t10084: F, t555: F, t9898: F, t14192: F, t2782: F, t9994: F, t544: F, t9989: F, t4003: F, t215: F, t4056: F) -> (F, F, F, F, F, F, F, F) {
        let (t46458, t46463, t46465, t46469) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2532::<F>(t46456, t786, t10026, t1398, t268, t4101, t543, t793, t10073, t10084, t555, t9898);
        let (t46472, t46475, t46478, t46490) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2533::<F>(t14192, t2782, t46469, t9994, t544, t9989, t4003, t215, t268, t4056, t4101, t543);
    (t46458, t46463, t46465, t46469, t46472, t46475, t46478, t46490)
}
