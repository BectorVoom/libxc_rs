//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta395 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1451;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1452;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1453;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1454;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1455;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1456;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta395(t41491: f64, t983: f64, t11502: f64, t11601: f64, t981: f64, t2922: f64, t275: f64, t2875: f64, t2925: f64, t11506: f64, t15542: f64, t3006: f64, t2918: f64, t2924: f64, t2926: f64, t41306: f64, t41308: f64, t41312: f64, t41316: f64, t41320: f64, t41323: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41341: f64, t41344: f64, t41347: f64, t41350: f64, t41353: f64, t41356: f64, t41359: f64, t41361: f64, t41363: f64, t41365: f64, t41367: f64, t41369: f64, t324: f64, t300: f64, t291: f64, t11545: f64, t914: f64, t936: f64, t41481: f64, t41483: f64, t41485: f64, t41488: f64, t41490: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41493, t41496, t41500, t41505, t41509) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1451(t41491, t983, t11502, t11601, t981, t2922, t275, t2875, t2925, t11506, t15542, t3006);
        let (t41510, t41513, t41525) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1452(t2918, t2924, t2926, t41306, t41308, t41312, t41316, t41320, t41323, t41327, t41330, t41332, t41334, t41336);
        let t41538 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1453(t41341, t41344, t41347, t41350, t41353, t41356, t41359, t41361, t41363, t41365, t41367, t41369);
        let (t41540, t41542, t41554) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1454(t324, t41525, t41538, t300, t41306, t41308, t41312, t41316, t41320, t41323, t41327, t41330, t41332, t41334, t41336);
        let t41567 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1455(t41341, t41344, t41347, t41350, t41353, t41356, t41359, t41361, t41363, t41365, t41367, t41369);
        let (t41570, t41573, t41574) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1456(t291, t41554, t41567, t11545, t914, t936, t41481, t41483, t41485, t41488, t41490, t41493, t41496, t41505, t41509, t41513, t41542);
    (t41493, t41496, t41500, t41505, t41509, t41510, t41513, t41540, t41542, t41570, t41573, t41574)
}
