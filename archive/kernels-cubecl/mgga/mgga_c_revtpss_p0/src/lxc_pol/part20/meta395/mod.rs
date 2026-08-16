//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta395 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1451;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1452;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1453;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1454;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1455;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1456;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta395<F: Float>(t41491: F, t983: F, t11502: F, t11601: F, t981: F, t2922: F, t275: F, t2875: F, t2925: F, t11506: F, t15542: F, t3006: F, t2918: F, t2924: F, t2926: F, t41306: F, t41308: F, t41312: F, t41316: F, t41320: F, t41323: F, t41327: F, t41330: F, t41332: F, t41334: F, t41336: F, t41341: F, t41344: F, t41347: F, t41350: F, t41353: F, t41356: F, t41359: F, t41361: F, t41363: F, t41365: F, t41367: F, t41369: F, t324: F, t300: F, t291: F, t11545: F, t914: F, t936: F, t41481: F, t41483: F, t41485: F, t41488: F, t41490: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41493, t41496, t41500, t41505, t41509) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1451::<F>(t41491, t983, t11502, t11601, t981, t2922, t275, t2875, t2925, t11506, t15542, t3006);
        let (t41510, t41513, t41525) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1452::<F>(t2918, t2924, t2926, t41306, t41308, t41312, t41316, t41320, t41323, t41327, t41330, t41332, t41334, t41336);
        let t41538 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1453::<F>(t41341, t41344, t41347, t41350, t41353, t41356, t41359, t41361, t41363, t41365, t41367, t41369);
        let (t41540, t41542, t41554) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1454::<F>(t324, t41525, t41538, t300, t41306, t41308, t41312, t41316, t41320, t41323, t41327, t41330, t41332, t41334, t41336);
        let t41567 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1455::<F>(t41341, t41344, t41347, t41350, t41353, t41356, t41359, t41361, t41363, t41365, t41367, t41369);
        let (t41570, t41573, t41574) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1456::<F>(t291, t41554, t41567, t11545, t914, t936, t41481, t41483, t41485, t41488, t41490, t41493, t41496, t41505, t41509, t41513, t41542);
    (t41493, t41496, t41500, t41505, t41509, t41510, t41513, t41540, t41542, t41570, t41573, t41574)
}
