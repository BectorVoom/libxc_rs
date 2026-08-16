//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1513;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1514;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1515;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1516;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1517;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1518;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1519;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta409(t1065: f64, t215: f64, t1063: f64, t247: f64, t906: f64, t11986: f64, t2858: f64, t11744: f64, t3106: f64, t373: f64, t675: f64, t828: f64, t3115: f64, t3119: f64, t11688: f64, t11922: f64, t4892: f64, t11249: f64, t3151: f64, t11722: f64, t3188: f64, t1011: f64, t11268: f64, t11639: f64, t11656: f64, t11678: f64, t11871: f64, t11927: f64, t11933: f64, t12017: f64, t16020: f64, t16025: f64, t16067: f64, t3117: f64, t3136: f64, t41314: f64, t4915: f64, t3046: f64, t3316: f64, t4891: f64, t11923: f64, t41229: f64, t41241: f64, t41243: f64, t41449: f64, t41451: f64, t41453: f64, t41455: f64, t41459: f64, t41468: f64, t41472: f64, t41476: f64, t41481: f64, t41483: f64, t41485: f64, t41488: f64, t41490: f64, t41493: f64, t41496: f64, t41505: f64, t41509: f64, t41513: f64, t41542: f64, t41570: f64, t41573: f64, t41577: f64, t41580: f64, t41582: f64, t41585: f64, t41591: f64, t41657: f64, t41841: f64, t41845: f64, t41847: f64, t41849: f64, t41933: f64, t41864: f64, t41867: f64, t41871: f64, t41873: f64, t41876: f64, t41879: f64, t41882: f64, t41885: f64, t41888: f64, t41942: f64, t41947: f64, t41949: f64, t11238: f64, t196: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42781, t42785, t42788, t42793) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1513(t1065, t215, t1063, t247, t906, t11986, t2858, t11744, t3106, t373, t675, t828);
        let (t42804, t42820) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1514(t3115, t3119, t42793, t11688, t11922, t4892, t11249, t3151, t11722, t3188, t1011, t11268, t11639, t11656, t11678, t11871, t11927, t11933, t12017, t16020, t16025, t16067, t3117, t3136, t41314, t42788, t4915);
        let (t42830, t42833, t42846) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1515(t3046, t3316, t4891, t11923, t11933, t41229, t41241, t41243, t41449, t41451, t41453, t41455, t41459, t41468, t41472, t41476);
        let t42847 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1516(t41481, t41483, t41485, t41488, t41490, t41493, t41496, t41505, t41509, t41513, t41542, t41570);
        let t42849 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1517(t41573, t41577, t41580, t41582, t41585, t41591, t41657, t41841, t41845, t41847, t41849, t41933);
        let t42850 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1518(t41864, t41867, t41871, t41873, t41876, t41879, t41882, t41885, t41888, t41942, t41947, t41949);
        let (t42852, t42859) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1519(t42846, t42847, t42849, t42850, t11238, t196);
    (t42781, t42785, t42804, t42820, t42830, t42833, t42852, t42859)
}
