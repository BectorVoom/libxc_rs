//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1923;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1924;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1925;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1926;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1927;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta546(t7575: f64, t7719: f64, t2122: f64, t28089: f64, t28150: f64, t1923: f64, t2123: f64, t25162: f64, t26792: f64, t26795: f64, t28093: f64, t28147: f64, t28154: f64, t29364: f64, t29367: f64, t6954: f64, t6963: f64, t7576: f64, t7579: f64, t7702: f64, t8144: f64, t8147: f64, t13272: f64, t7565: f64, t38: f64, t8142: f64, t2247: f64, t26749: f64, t26755: f64, t28105: f64, t28109: f64, t28112: f64, t28116: f64, t28119: f64, t28133: f64, t28141: f64, t6960: f64, t7566: f64, t7706: f64, t7709: f64, t5: f64, t117: f64, t1310: f64, t1843: f64, t2127: f64, t27136: f64, t27139: f64, t27152: f64, t27156: f64, t27834: f64, t27835: f64, t28022: f64, t28045: f64, t28058: f64, t28060: f64, t508: f64, t5517: f64, t649: f64, t7584: f64, t8152: f64, t8233: f64, t116: f64, t8151: f64, t2126: f64, t670: f64, t1518: f64, t27060: f64, t28212: f64, t28214: f64, t28216: f64, t28218: f64, t28221: f64, t28223: f64, t28225: f64, t28227: f64, t28229: f64, t4292: f64, t7586: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29372, t29375, t29380, t29387) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1923(t7575, t7719, t2122, t28089, t28150, t1923, t2123, t25162, t26792, t26795, t28093, t28147, t28154, t29364, t29367, t6954, t6963, t7576, t7579, t7702, t8144, t8147);
        let (t29388, t29411, t29412, t29419) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1924(t13272, t7565, t38, t8142, t2247, t2123, t26749, t26755, t28105, t28109, t28112, t28116, t28119, t28133, t28141, t6960, t6963, t7566, t7576, t7579, t7706, t7709, t8144);
        let (t29421, t29422, t29425) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1925(t5, t29387, t29419, t117, t1310, t1843, t2127, t27136, t27139, t27152, t27156, t27834, t27835, t28022, t28045, t28058, t28060, t508, t5517, t649, t7584, t8152, t8233);
        let t29427 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1926(t116, t8151);
        let t29432 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1927(t2126, t670);
        let t29437 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1928(t1518, t27060, t28212, t28214, t28216, t28218, t28221, t28223, t28225, t28227, t28229, t29422, t29427, t29432, t4292, t670, t7586);
    (t29372, t29375, t29380, t29388, t29411, t29412, t29421, t29422, t29425, t29427, t29432, t29437)
}
