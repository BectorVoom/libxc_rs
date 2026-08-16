//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1764;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1765;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1766;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1767;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1768;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1769;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1770;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1771;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1772;
use chunk9::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1773;
use chunk10::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1774;
use chunk11::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta464(t9775: f64, t9981: f64, t1398: f64, t2661: f64, t3992: f64, t4010: f64, t9956: f64, t3938: f64, t47218: f64, t221: f64, t4018: f64, t4019: f64, t9891: f64, t1389: f64, t3964: f64, t40604: f64, t3961: f64, t9741: f64, t13783: f64, t1388: f64, t1390: f64, t1399: f64, t3934: f64, t46532: f64, t46682: f64, t47282: f64, t47284: f64, t47286: f64, t47296: f64, t47298: f64, t47302: f64, t47304: f64, t47306: f64, t47318: f64, t5673: f64, t828: f64, t9984: f64, t46654: f64, t46714: f64, t46782: f64, t46848: f64, t46911: f64, t47212: f64, t47279: f64, t10111: f64, t22: f64, t4092: f64, t39515: f64, t4083: f64, t10043: f64, t9303: f64, t10014: f64, t10019: f64, t268: f64, t4101: f64, t543: f64, t675: f64, t9890: f64, t10139: f64, t281: f64, t4056: f64, t68: f64, t9898: f64, t14192: f64, t555: f64, t786: f64, t9994: f64, t10023: f64, t4003: f64, t10115: f64, t1441: f64, t213: f64, t546: f64, t10008: f64, t545: f64, t689: f64, t869: f64, t4093: f64, t9292: f64, t10065: f64, t10073: f64, t1432: f64, t1433: f64, t39497: f64, t1385: f64, t10061: f64, t10069: f64, t2782: f64, t4086: f64, t46407: f64, t46565: f64, t5744: f64, t1428: f64, t588: f64, t10049: f64, t820: f64, t9912: f64, t4066: f64, t4104: f64, t4100: f64, t46433: f64, t10022: f64, t2453: f64, t46507: f64, t686: f64, t72: f64, t39644: f64, t8779: f64, t4107: f64, t9288: f64, t10107: f64, t9285: f64, t39494: f64, t4096: f64, t40270: f64, t4089: f64, t1437: f64, t4114: f64, t46902: f64, t47188: f64) -> (f64, f64, f64, f64) {
        let (t47320, t47325, t47329, t47333) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1764(t9775, t9981, t1398, t2661, t3992, t4010, t9956, t3938, t47218, t221, t4018, t4019, t9891);
        let t47340 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1765(t1389, t3964, t40604, t3961, t9741, t13783, t1388, t1390, t1399, t3934, t46532, t46682, t47282, t47284, t47286, t47296, t47298, t47302, t47304, t47306, t47318, t47320, t47325, t47329, t47333, t5673, t828, t9984);
        let (t47343, t47348, t47351) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1766(t46654, t46714, t46782, t46848, t46911, t47212, t47279, t47340, t10111, t22, t4092, t39515, t4083);
        let (t47352, t47354, t47359, t47364) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1767(t10043, t9303, t10014, t10019, t268, t4101, t543, t675, t9890, t10139, t281, t4056, t68);
        let (t47369, t47375, t47379, t47381) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1768(t675, t9898, t268, t4101, t543, t14192, t555, t786, t9994, t10023, t4003, t10115, t1441);
        let t47383 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1769(t213, t47343, t47348, t47351, t47352, t47354, t47359, t47364, t47369, t47375, t47379, t47381, t546);
        let (t47387, t47389, t47391, t47395, t47396) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1770(t10008, t545, t689, t869, t4093, t9292, t10065, t10073, t1432, t1433, t39497, t1385);
        let (t47403, t47407, t47411, t47413, t47417) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1771(t10061, t10069, t2782, t4086, t46407, t543, t4003, t46565, t5744, t10073, t10111, t1428, t588);
        let t47418 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1772(t10049, t1399, t47387, t47389, t47391, t47395, t47396, t47403, t47407, t47411, t47413, t47417, t820, t9912);
        let (t47424, t47427, t47432) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1773(t4066, t4086, t786, t4104, t2782, t4100, t46433, t10022, t2453, t281, t4003, t46507);
        let (t47436, t47442, t47444, t47450) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1774(t10008, t1432, t686, t72, t268, t39644, t546, t555, t8779, t4107, t9288, t10107, t3964, t9285);
        let t47457 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1775(t39494, t3964, t4096, t40270, t4089, t1437, t4114, t46902, t47188, t47424, t47427, t47432, t47436, t47442, t47444, t47450, t820);
    (t47343, t47383, t47418, t47457)
}
