//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta546 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1923;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1924;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1925;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1926;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1927;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta546<F: Float>(t7575: F, t7719: F, t2122: F, t28089: F, t28150: F, t1923: F, t2123: F, t25162: F, t26792: F, t26795: F, t28093: F, t28147: F, t28154: F, t29364: F, t29367: F, t6954: F, t6963: F, t7576: F, t7579: F, t7702: F, t8144: F, t8147: F, t13272: F, t7565: F, t38: F, t8142: F, t2247: F, t26749: F, t26755: F, t28105: F, t28109: F, t28112: F, t28116: F, t28119: F, t28133: F, t28141: F, t6960: F, t7566: F, t7706: F, t7709: F, t5: F, t117: F, t1310: F, t1843: F, t2127: F, t27136: F, t27139: F, t27152: F, t27156: F, t27834: F, t27835: F, t28022: F, t28045: F, t28058: F, t28060: F, t508: F, t5517: F, t649: F, t7584: F, t8152: F, t8233: F, t116: F, t8151: F, t2126: F, t670: F, t1518: F, t27060: F, t28212: F, t28214: F, t28216: F, t28218: F, t28221: F, t28223: F, t28225: F, t28227: F, t28229: F, t4292: F, t7586: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t29372, t29375, t29380, t29387) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1923::<F>(t7575, t7719, t2122, t28089, t28150, t1923, t2123, t25162, t26792, t26795, t28093, t28147, t28154, t29364, t29367, t6954, t6963, t7576, t7579, t7702, t8144, t8147);
        let (t29388, t29411, t29412, t29419) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1924::<F>(t13272, t7565, t38, t8142, t2247, t2123, t26749, t26755, t28105, t28109, t28112, t28116, t28119, t28133, t28141, t6960, t6963, t7566, t7576, t7579, t7706, t7709, t8144);
        let (t29421, t29422, t29425) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1925::<F>(t5, t29387, t29419, t117, t1310, t1843, t2127, t27136, t27139, t27152, t27156, t27834, t27835, t28022, t28045, t28058, t28060, t508, t5517, t649, t7584, t8152, t8233);
        let t29427 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1926::<F>(t116, t8151);
        let t29432 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1927::<F>(t2126, t670);
        let t29437 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1928::<F>(t1518, t27060, t28212, t28214, t28216, t28218, t28221, t28223, t28225, t28227, t28229, t29422, t29427, t29432, t4292, t670, t7586);
    (t29372, t29375, t29380, t29388, t29411, t29412, t29421, t29422, t29425, t29427, t29432, t29437)
}
