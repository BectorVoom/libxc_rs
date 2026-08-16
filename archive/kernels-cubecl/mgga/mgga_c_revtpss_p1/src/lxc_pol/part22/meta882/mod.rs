//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta882 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3055;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3056;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta882<F: Float>(t10504: F, t136: F, t2457: F, t4533: F, t14481: F, t2782: F, t861: F, t11050: F, t14987: F, t14473: F, t9303: F, t41017: F, t4481: F, t14477: F, t2435: F, t14978: F, t2465: F, t686: F, t72: F, t14480: F, t252: F, t2828: F, t10073: F, t14482: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t51726, t51729, t51731, t51733, t51739) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3055::<F>(t10504, t136, t2457, t4533, t14481, t2782, t861, t11050, t14987, t14473, t9303, t41017, t4481);
        let (t51741, t51746, t51750, t51756) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3056::<F>(t14477, t2435, t14978, t2465, t686, t72, t14480, t252, t2782, t2828, t10073, t14482);
    (t51726, t51729, t51731, t51733, t51739, t51741, t51746, t51750, t51756)
}
