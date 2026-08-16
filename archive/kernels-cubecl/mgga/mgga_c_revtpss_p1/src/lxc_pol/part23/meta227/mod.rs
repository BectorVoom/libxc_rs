//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta227 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1333;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1334;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1335;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1336;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1337;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1338;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta227<F: Float>(t1188: F, t6518: F, t3503: F, t3510: F, t5044: F, t5093: F, t6423: F, t6427: F, t6431: F, t6443: F, t6450: F, t6456: F, t6458: F, t6462: F, t6465: F, t6468: F, t3523: F, t1161: F, t1180: F, t1745: F, t1757: F, t3452: F, t3477: F, t3496: F, t3521: F, t435: F, t5120: F, t5158: F, t6435: F, t6437: F, t6441: F, t6473: F, t6476: F, t6481: F, t6487: F, t6503: F, t6506: F, t6514: F, t300: F, t1765: F, t5192: F, t3495: F, t1196: F, t1179: F, t3520: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t6519 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1333::<F>(t1188, t6518);
        let t6534 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1334::<F>(t3503, t3510, t5044, t5093, t6423, t6427, t6431, t6443, t6450, t6456, t6458, t6462, t6465, t6468);
        let t6535 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1335::<F>(t1188, t6534);
        let t6538 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1336::<F>(t3523, t6518);
        let t6541 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1337::<F>(t1161, t1180, t1745, t1757, t3452, t3477, t3496, t3521, t435, t5120, t5158, t6435, t6437, t6441, t6473, t6476, t6481, t6487, t6503, t6506, t6514, t6519, t6535, t6538);
        let (t6542, t6544, t6546, t6548, t6550, t6552, t6554, t6555) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1338::<F>(t300, t6541, t6514, t1765, t5192, t1188, t3495, t6518, t1196, t1179, t6534, t3520);
    (t6519, t6534, t6535, t6538, t6542, t6544, t6546, t6548, t6550, t6552, t6554, t6555)
}
