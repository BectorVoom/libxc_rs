//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta409 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1513;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1514;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1515;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1516;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1517;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1518;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1519;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta409<F: Float>(t1065: F, t215: F, t1063: F, t247: F, t906: F, t11986: F, t2858: F, t11744: F, t3106: F, t373: F, t675: F, t828: F, t3115: F, t3119: F, t11688: F, t11922: F, t4892: F, t11249: F, t3151: F, t11722: F, t3188: F, t1011: F, t11268: F, t11639: F, t11656: F, t11678: F, t11871: F, t11927: F, t11933: F, t12017: F, t16020: F, t16025: F, t16067: F, t3117: F, t3136: F, t41314: F, t4915: F, t3046: F, t3316: F, t4891: F, t11923: F, t41229: F, t41241: F, t41243: F, t41449: F, t41451: F, t41453: F, t41455: F, t41459: F, t41468: F, t41472: F, t41476: F, t41481: F, t41483: F, t41485: F, t41488: F, t41490: F, t41493: F, t41496: F, t41505: F, t41509: F, t41513: F, t41542: F, t41570: F, t41573: F, t41577: F, t41580: F, t41582: F, t41585: F, t41591: F, t41657: F, t41841: F, t41845: F, t41847: F, t41849: F, t41933: F, t41864: F, t41867: F, t41871: F, t41873: F, t41876: F, t41879: F, t41882: F, t41885: F, t41888: F, t41942: F, t41947: F, t41949: F, t11238: F, t196: F) -> (F, F, F, F, F, F, F, F) {
        let (t42781, t42785, t42788, t42793) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1513::<F>(t1065, t215, t1063, t247, t906, t11986, t2858, t11744, t3106, t373, t675, t828);
        let (t42804, t42820) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1514::<F>(t3115, t3119, t42793, t11688, t11922, t4892, t11249, t3151, t11722, t3188, t1011, t11268, t11639, t11656, t11678, t11871, t11927, t11933, t12017, t16020, t16025, t16067, t3117, t3136, t41314, t42788, t4915);
        let (t42830, t42833, t42846) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1515::<F>(t3046, t3316, t4891, t11923, t11933, t41229, t41241, t41243, t41449, t41451, t41453, t41455, t41459, t41468, t41472, t41476);
        let t42847 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1516::<F>(t41481, t41483, t41485, t41488, t41490, t41493, t41496, t41505, t41509, t41513, t41542, t41570);
        let t42849 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1517::<F>(t41573, t41577, t41580, t41582, t41585, t41591, t41657, t41841, t41845, t41847, t41849, t41933);
        let t42850 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1518::<F>(t41864, t41867, t41871, t41873, t41876, t41879, t41882, t41885, t41888, t41942, t41947, t41949);
        let (t42852, t42859) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1519::<F>(t42846, t42847, t42849, t42850, t11238, t196);
    (t42781, t42785, t42804, t42820, t42830, t42833, t42852, t42859)
}
