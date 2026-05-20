//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta708 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2462;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2463;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta708<F: Float>(t556: F, t786: F, t9656: F, t9303: F, t9641: F, t4146: F, t46279: F, t46291: F, t1892: F, t9646: F, t9648: F, t1904: F, t47567: F, t1427: F, t1903: F, t22: F, t9647: F, t2453: F, t3908: F, t5711: F, t14296: F, t213: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47603, t47618, t47672, t47753, t47760, t47764, t47772) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2462::<F>(t556, t786, t9656, t9303, t9641, t4146, t46279, t46291, t1892, t9646, t9648, t1904, t47567);
        let (t47781, t47785, t47786, t47793, t47794) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2463::<F>(t1427, t1903, t22, t9647, t2453, t3908, t5711, t14296, t9303, t213, t556, t9656);
    (t47603, t47618, t47672, t47753, t47760, t47764, t47772, t47781, t47785, t47786, t47793, t47794)
}
