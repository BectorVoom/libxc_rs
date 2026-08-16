//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta796 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2618;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta796<F: Float>(t10744: F, t18409: F, t808: F, t18414: F, t40521: F, t40791: F, t5989: F, t10890: F, t5985: F, t14686: F, t18525: F, t50570: F, t61956: F, t14923: F, t18428: F, t10760: F, t40627: F, t61837: F, t18527: F, t50295: F, t18353: F, t2689: F, t18394: F, t2703: F, t10777: F, t61715: F, t837: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t62069, t62072, t62089, t62095, t62105) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2618::<F>(t10744, t18409, t808, t18414, t40521, t40791, t5989, t10890, t5985, t14686, t18525, t50570, t61956);
        let (t62108, t62111, t62114, t62129, t62135, t62148) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2619::<F>(t14923, t18428, t10760, t40627, t61837, t18527, t50295, t18353, t2689, t18394, t2703, t10777, t14686, t61715, t837);
    (t62069, t62072, t62089, t62095, t62105, t62108, t62111, t62114, t62129, t62135, t62148)
}
