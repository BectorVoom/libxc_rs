//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2314;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2315;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2316;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2317;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2318;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta529<F: Float>(t16750: F, t482: F, t371: F, t372: F, t1803: F, t3666: F, t1208: F, t5215: F, t225: F, t480: F, t3678: F, t5327: F, t5323: F, t1235: F, t1238: F, t12800: F, t12976: F, t1791: F, t1808: F, t3644: F, t3663: F, t3667: F, t5320: F, t5391: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17278, t17280, t17283) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2314::<F>(t16750, t482, t371, t372, t1803, t3666);
        let t17288 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2315::<F>(t1208, t5215);
        let t17289 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2316::<F>(t17288, t225);
        let t17290 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2317::<F>(t17289, t480);
        let (t17296, t17298, t17299) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2318::<F>(t3678, t5327, t5323, t1235, t1238, t12800, t12976, t17280, t17283, t17290, t1791, t1808, t3644, t3663, t3667, t5320, t5391);
    (t17278, t17280, t17283, t17288, t17289, t17290, t17296, t17298, t17299)
}
