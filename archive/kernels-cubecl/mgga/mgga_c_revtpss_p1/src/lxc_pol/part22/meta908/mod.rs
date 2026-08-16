//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta908 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3109;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta908<F: Float>(t11710: F, t15958: F, t3091: F, t3316: F, t4746: F, t4891: F, t16381: F, t3090: F, t11262: F, t3127: F, t4874: F, t15758: F, t16055: F, t1063: F, t15833: F, t3172: F, t11779: F, t4845: F, t15749: F, t3211: F, t16148: F, t4837: F, t11656: F, t15769: F, t16199: F, t372: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t54553, t54570, t54578, t54599, t54623) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3109::<F>(t11710, t15958, t3091, t3316, t4746, t4891, t16381, t3090, t11262, t3127, t4874, t15758, t16055);
        let (t54638, t54646, t54648, t54651, t54656, t54658) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3110::<F>(t1063, t15833, t3172, t11779, t4845, t15749, t3211, t16148, t4837, t11656, t15769, t16199, t372);
    (t54553, t54570, t54578, t54599, t54623, t54638, t54646, t54648, t54651, t54656, t54658)
}
