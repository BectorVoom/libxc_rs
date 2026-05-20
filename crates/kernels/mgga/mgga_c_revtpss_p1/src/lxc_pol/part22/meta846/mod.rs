//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta846 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2983;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2984;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta846<F: Float>(t1882: F, t2482: F, t4104: F, t4118: F, t1398: F, t2782: F, t4086: F, t543: F, t5710: F, t1897: F, t40317: F, t10111: F, t22: F, t5759: F, t49146: F, t4100: F, t48475: F, t47423: F, t5741: F, t3923: F, t48105: F, t47371: F, t10026: F, t14141: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t49325, t49346, t49354, t49361) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2983::<F>(t1882, t2482, t4104, t4118, t1398, t2782, t4086, t543, t5710, t1897, t40317, t10111, t22, t5759);
        let (t49376, t49378, t49382, t49386, t49395, t49399) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2984::<F>(t49146, t543, t2782, t4100, t48475, t47423, t5741, t3923, t48105, t47371, t10026, t14141);
    (t49325, t49346, t49354, t49361, t49376, t49378, t49382, t49386, t49395, t49399)
}
