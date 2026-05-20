//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1393;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1394;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1395;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1396;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1397;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1398;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1399;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta220<F: Float>(t5332: F, t5341: F, t3720: F, t1248: F, t1774: F, t1250: F, t1794: F, t73: F, t1214: F, t471: F, t140: F, t1781: F, t1222: F, t127: F, t1789: F, t371: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5342, t5343) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1393::<F>(t5332, t5341, t3720);
        let t5346 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1394::<F>(t1248, t1774);
        let (t5347, t5348) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1395::<F>(t1250, t5346, t3720);
        let t5351 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1396::<F>(t1794, t73);
        let t5352 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1397::<F>(t1214, t471);
        let (t5353, t5354) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1398::<F>(t5351, t5352, t3720);
        let (t5357, t5358, t5362) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1399::<F>(t140, t1781, t1222, t127, t1789, t371);
    (t5342, t5343, t5346, t5347, t5348, t5351, t5352, t5353, t5354, t5357, t5358, t5362)
}
