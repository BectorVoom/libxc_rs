//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta835 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2960;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2961;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta835<F: Float>(t2661: F, t3992: F, t4057: F, t5608: F, t4004: F, t5651: F, t9934: F, t47198: F, t5665: F, t5629: F, t9779: F, t5661: F, t9909: F, t47247: F, t828: F, t13967: F, t9962: F, t13941: F, t46740: F, t221: F, t47273: F, t13785: F, t9816: F, t13770: F, t9775: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48786, t48790, t48792, t48794, t48796) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2960::<F>(t2661, t3992, t4057, t5608, t4004, t5651, t9934, t47198, t5665, t5629, t9779, t5661, t9909);
        let (t48798, t48811, t48813, t48823, t48825, t48827) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2961::<F>(t47247, t828, t13967, t9962, t13941, t46740, t221, t47273, t13785, t9816, t13770, t9775);
    (t48786, t48790, t48792, t48794, t48796, t48798, t48811, t48813, t48823, t48825, t48827)
}
