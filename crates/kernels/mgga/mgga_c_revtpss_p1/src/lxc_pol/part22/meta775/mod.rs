//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta775 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2863;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta775<F: Float>(t11262: F, t3600: F, t3605: F, t3617: F, t675: F, t1261: F, t247: F, t3363: F, t3609: F, t44169: F, t1263: F, t215: F, t1122: F, t3711: F, t3713: F, t12657: F, t1284: F, t3624: F, t221: F, t461: F, t462: F, t624: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t44675, t44693, t44696, t44698, t44701) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2863::<F>(t11262, t3600, t3605, t3617, t675, t1261, t247, t3363, t3609, t44169, t1263, t215);
        let (t44704, t44751, t44769, t44797) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2864::<F>(t1122, t1261, t247, t44701, t11262, t3711, t3713, t12657, t1284, t3624, t221, t461, t462, t624);
    (t44675, t44693, t44696, t44698, t44701, t44704, t44751, t44769, t44797)
}
