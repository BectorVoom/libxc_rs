//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta876 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2780;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2781;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta876<F: Float>(t22026: F, t46802: F, t9794: F, t46694: F, t6850: F, t22294: F, t48823: F, t9816: F, t1398: F, t6843: F, t22245: F, t808: F, t9736: F, t22236: F, t6884: F, t9741: F, t14104: F, t47856: F, t13729: F, t2782: F, t556: F, t5774: F, t2439: F, t3895: F, t6896: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t74677, t74682, t74698, t74700, t74711) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2780::<F>(t22026, t46802, t9794, t46694, t6850, t22294, t48823, t9816, t1398, t6843, t22245, t808, t9736);
        let (t74714, t74717, t74733, t74744, t74757) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2781::<F>(t22236, t808, t9736, t6884, t9741, t14104, t47856, t13729, t2782, t556, t5774, t2439, t3895, t6896);
    (t74677, t74682, t74698, t74700, t74711, t74714, t74717, t74733, t74744, t74757)
}
