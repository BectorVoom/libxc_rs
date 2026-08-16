//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta845 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2980;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2981;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2982;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta845<F: Float>(t14066: F, t545: F, t689: F, t869: F, t1398: F, t14141: F, t14143: F, t2434: F, t10049: F, t14145: F, t1882: F, t2482: F, t14230: F, t2782: F, t46456: F, t1385: F, t14155: F, t1432: F, t2470: F, t1892: F, t4056: F, t4086: F, t543: F, t10069: F, t14225: F, t10013: F, t14224: F, t48073: F, t4100: F, t49213: F, t10136: F, t14114: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t49252, t49256, t49260) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2980::<F>(t14066, t545, t689, t869, t1398, t14141, t14143, t2434, t10049, t14145, t1882, t2482);
        let (t49263, t49268, t49273, t49283) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2981::<F>(t14230, t2782, t46456, t1385, t14066, t14155, t1432, t2470, t1892, t4056, t4086, t543);
        let (t49289, t49296, t49308, t49313, t49321) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2982::<F>(t10069, t14225, t10013, t14224, t2782, t48073, t543, t4100, t4086, t49213, t10136, t14114);
    (t49252, t49256, t49260, t49263, t49268, t49273, t49283, t49289, t49296, t49308, t49313, t49321)
}
