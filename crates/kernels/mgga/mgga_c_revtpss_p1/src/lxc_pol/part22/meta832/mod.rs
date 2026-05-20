//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta832 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2954;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2955;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta832<F: Float>(t2661: F, t3992: F, t4057: F, t5651: F, t1389: F, t1882: F, t46856: F, t543: F, t685: F, t72: F, t13874: F, t3989: F, t13805: F, t46609: F, t5608: F, t4004: F, t9934: F, t13854: F, t9962: F, t13834: F, t13999: F, t125: F, t13920: F) -> (F, F, F, F, F, F, F, F) {
        let (t48557, t48563, t48565) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2954::<F>(t2661, t3992, t4057, t5651, t1389, t1882, t46856, t543, t685, t72, t13874, t3989);
        let (t48573, t48577, t48591, t48593, t48595) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2955::<F>(t13805, t2661, t46609, t5608, t4004, t9934, t13854, t9962, t13834, t13999, t125, t13920);
    (t48557, t48563, t48565, t48573, t48577, t48591, t48593, t48595)
}
