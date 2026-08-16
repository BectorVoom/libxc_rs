//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1760;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1761;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1762;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1763;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1764;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta399<F: Float>(t372: F, t5296: F, t17350: F, t3767: F, t5277: F, t1285: F, t12865: F, t5302: F, t15904: F, t3623: F, t13148: F, t11249: F, t1794: F, t3172: F, t5303: F, t1261: F, t1209: F, t489: F, t370: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17649, t17654) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1760::<F>(t372, t5296, t17350, t3767);
        let t17661 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1761::<F>(t372, t5277);
        let t17693 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1762::<F>(t1285, t12865);
        let (t17694, t17708) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1763::<F>(t372, t5302, t15904, t3623);
        let t17709 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1764::<F>(t13148, t17708);
        let (t17710, t17720, t17721, t17727, t17728) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1765::<F>(t11249, t1794, t3172, t5303, t1261, t1209, t489, t3623, t370);
    (t17649, t17654, t17661, t17693, t17694, t17708, t17709, t17710, t17720, t17721, t17727, t17728)
}
