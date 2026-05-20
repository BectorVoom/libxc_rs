//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1706;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1707;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1708;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta291<F: Float>(t1353: F, t1412: F, t808: F, t9736: F, t1369: F, t2699: F, t1372: F, t3943: F, t794: F, t3946: F, t159: F, t216: F, t3989: F, t4014: F, t221: F, t3889: F, t3979: F, t3978: F, t1408: F, t2482: F, t596: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9738, t9739, t9741) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1706::<F>(t1353, t1412, t808, t9736, t1369, t2699);
        let (t9742, t9744) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1707::<F>(t1372, t9741, t3943, t794);
        let (t9745, t9747, t9748, t9753, t9761, t9762, t9765) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1708::<F>(t3946, t9744, t1412, t159, t216, t3989, t4014, t221, t3889, t3979, t3978, t1408, t2482, t596);
    (t9738, t9739, t9741, t9742, t9744, t9745, t9747, t9748, t9753, t9761, t9762, t9765)
}
