//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1635;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1636;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1637;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1638;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta434(t12625: f64, t458: f64, t456: f64, t225: f64, t480: f64, t3568: f64, t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43858: f64, t43862: f64, t43865: f64, t43871: f64, t43877: f64, t43813: f64, t43854: f64, t43883: f64, t43886: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t43899: f64, t43902: f64, t43905: f64, t12984: f64, t3667: f64, t12976: f64, t3678: f64, t12963: f64, t1235: f64, t127: f64, t12970: f64, t371: f64, t1222: f64, t1238: f64, t12972: f64, t17693: f64, t17799: f64, t3663: f64, t372: f64, t43843: f64, t44800: f64, t44823: f64, t44829: f64, t44833: f64, t44838: f64, t482: f64, t5308: f64, t126: f64, t13099: f64, t12257: f64, t1261: f64, t247: f64, t12879: f64, t3372: f64, t3368: f64, t12287: f64, t17240: f64, t12881: f64, t3647: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44842, t44843, t44844, t44845, t44864) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1635(t12625, t458, t456, t225, t480, t3568, t43830, t43832, t43837, t43841, t43845, t43849, t43858, t43862, t43865, t43871, t43877);
        let t44877 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1636(t43813, t43854, t43883, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905);
        let (t44878, t44894) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1637(t44864, t44877, t12984, t3667, t12976, t3678, t12963, t1235, t127, t12970, t371, t1222, t1238, t12972, t17693, t17799, t3663, t372, t43843, t44800, t44823, t44829, t44833, t44838, t44844, t44845, t482, t5308);
        let (t44898, t44902, t44906, t44912, t44917) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1638(t126, t13099, t12257, t1261, t247, t12879, t3372, t3368, t1222, t12287, t17240, t12881, t3647);
    (t44842, t44843, t44845, t44878, t44894, t44898, t44902, t44906, t44912, t44917)
}
