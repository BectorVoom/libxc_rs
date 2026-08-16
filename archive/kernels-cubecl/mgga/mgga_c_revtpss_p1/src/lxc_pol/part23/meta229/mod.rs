//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta229 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1346;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1347;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1348;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1349;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1350;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta229<F: Float>(t467: F, t6594: F, t1785: F, t1803: F, t225: F, t6564: F, t480: F, t482: F, t6573: F, t371: F, t372: F, t1715: F, t5277: F, t1042: F, t6435: F, t6437: F, t6441: F, t6473: F, t6476: F, t6542: F, t6544: F, t6546: F, t6550: F, t6554: F, t6558: F, t1250: F, t1794: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6595, t6598, t6601, t6602, t6609, t6611) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1346::<F>(t467, t6594, t1785, t1803, t225, t6564, t480, t482, t6573, t371, t372);
        let (t6618, t6619) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1347::<F>(t1715, t5277, t1042);
        let t6622 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1348::<F>(t6435, t6437, t6441, t6473, t6476, t6542, t6544, t6546, t6550, t6554, t6558);
        let (t6624, t6625) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1349::<F>(t1250, t482, t6622, t1042);
        let t6628 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1350::<F>(t1794);
    (t6595, t6598, t6601, t6602, t6609, t6611, t6618, t6619, t6622, t6624, t6625, t6628)
}
