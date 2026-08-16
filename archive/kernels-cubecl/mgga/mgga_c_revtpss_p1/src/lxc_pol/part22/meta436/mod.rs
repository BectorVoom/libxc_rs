//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2064;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2065;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2066;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta436<F: Float>(t1544: F, t2394: F, t10698: F, t828: F, t10811: F, t4462: F, t4416: F, t808: F, t10886: F, t2703: F, t4458: F, t10678: F, t10682: F, t10687: F, t10692: F, t14759: F, t14761: F, t14765: F, t14769: F, t851: F, t10769: F, t836: F, t2749: F, t2746: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14772, t14774, t14777, t14779, t14780, t14783, t14784) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2064::<F>(t1544, t2394, t10698, t828, t10811, t4462, t4416, t808, t10886, t2703, t4458, t10678, t10682, t10687, t10692, t14759, t14761, t14765, t14769, t851);
        let t14785 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2065::<F>(t10769, t828);
        let (t14786, t14787, t14788, t14791) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2066::<F>(t1544, t836, t2749, t14785, t2746, t828);
    (t14772, t14774, t14777, t14779, t14780, t14783, t14784, t14785, t14786, t14787, t14788, t14791)
}
