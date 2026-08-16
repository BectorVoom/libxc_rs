//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta935 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3167;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3168;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta935<F: Float>(t1260: F, t44843: F, t17423: F, t17426: F, t343: F, t56: F, t816: F, t13026: F, t65: F, t12256: F, t12772: F, t17634: F, t3625: F, t17395: F, t3746: F, t17689: F, t44425: F, t17435: F, t3667: F, t1235: F, t127: F, t17278: F, t371: F, t1256: F, t17311: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t57520, t57534, t57548, t57550, t57569) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3167::<F>(t1260, t44843, t17423, t17426, t343, t56, t816, t13026, t65, t12256, t12772, t17634, t3625);
        let (t57571, t57584, t57586, t57590, t57602) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3168::<F>(t17395, t3746, t17689, t3625, t44425, t17435, t3667, t1235, t127, t17278, t371, t1256, t17311);
    (t57520, t57534, t57548, t57550, t57569, t57571, t57584, t57586, t57590, t57602)
}
