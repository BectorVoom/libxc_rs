//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1635;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1636;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1637;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1638;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta434<F: Float>(t12625: F, t458: F, t456: F, t225: F, t480: F, t3568: F, t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F, t43813: F, t43854: F, t43883: F, t43886: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t43899: F, t43902: F, t43905: F, t12984: F, t3667: F, t12976: F, t3678: F, t12963: F, t1235: F, t127: F, t12970: F, t371: F, t1222: F, t1238: F, t12972: F, t17693: F, t17799: F, t3663: F, t372: F, t43843: F, t44800: F, t44823: F, t44829: F, t44833: F, t44838: F, t482: F, t5308: F, t126: F, t13099: F, t12257: F, t1261: F, t247: F, t12879: F, t3372: F, t3368: F, t12287: F, t17240: F, t12881: F, t3647: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44842, t44843, t44844, t44845, t44864) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1635::<F>(t12625, t458, t456, t225, t480, t3568, t43830, t43832, t43837, t43841, t43845, t43849, t43858, t43862, t43865, t43871, t43877);
        let t44877 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1636::<F>(t43813, t43854, t43883, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905);
        let (t44878, t44894) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1637::<F>(t44864, t44877, t12984, t3667, t12976, t3678, t12963, t1235, t127, t12970, t371, t1222, t1238, t12972, t17693, t17799, t3663, t372, t43843, t44800, t44823, t44829, t44833, t44838, t44844, t44845, t482, t5308);
        let (t44898, t44902, t44906, t44912, t44917) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1638::<F>(t126, t13099, t12257, t1261, t247, t12879, t3372, t3368, t1222, t12287, t17240, t12881, t3647);
    (t44842, t44843, t44845, t44878, t44894, t44898, t44902, t44906, t44912, t44917)
}
