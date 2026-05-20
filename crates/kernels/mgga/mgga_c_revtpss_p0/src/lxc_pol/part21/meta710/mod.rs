//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta710 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2540;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta710<F: Float>(t550: F, t9898: F, t2661: F, t46609: F, t9994: F, t3992: F, t543: F, t9890: F, t3995: F, t40488: F, t3989: F, t9944: F, t549: F, t240: F, t72: F, t4014: F, t9779: F, t221: F, t3978: F, t3979: F, t9628: F, t1408: F, t2237: F, t2482: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t46610, t46613, t46618, t46620, t46622) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2540::<F>(t550, t9898, t2661, t46609, t9994, t3992, t543, t9890, t3995, t40488, t3989, t9944);
        let (t46627, t46633, t46641, t46644) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2541::<F>(t549, t240, t72, t4014, t9779, t221, t3978, t3979, t9628, t1408, t2237, t2482);
    (t46610, t46613, t46618, t46620, t46622, t46627, t46633, t46641, t46644)
}
