//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta140 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk898;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk899;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk900;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk901;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta140<F: Float>(t1353: F, t221: F, t3979: F, t3978: F, t247: F, t2682: F, t550: F, t548: F, t1408: F, t820: F, t843: F, t1416: F, t1386: F, t240: F, t1398: F, t543: F, t2661: F, t1384: F, t544: F, t235: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3981, t3982, t3987, t3989) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk898::<F>(t1353, t221, t3979, t3978, t247, t2682, t550, t548, t1408, t820, t843);
        let (t3990, t3992) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk899::<F>(t1416, t3989, t1386, t240);
        let (t3994, t3995, t3996, t3999) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk900::<F>(t1398, t543, t550, t3992, t2661, t1384, t544);
        let t4000 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk901::<F>(t235, t3999);
    (t3981, t3982, t3987, t3989, t3990, t3992, t3994, t3995, t3996, t3999, t4000)
}
