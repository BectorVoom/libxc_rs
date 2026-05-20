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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta464<F: Float>(t9775: F, t9981: F, t1398: F, t2661: F, t3992: F, t4010: F, t9956: F, t3938: F, t47218: F, t221: F, t4018: F, t4019: F, t9891: F, t1389: F, t3964: F, t40604: F, t3961: F, t9741: F, t13783: F, t1388: F, t1390: F, t1399: F, t3934: F, t46532: F, t46682: F, t47282: F, t47284: F, t47286: F, t47296: F, t47298: F, t47302: F, t47304: F, t47306: F, t47318: F, t5673: F, t828: F, t9984: F, t46654: F, t46714: F, t46782: F, t46848: F, t46911: F, t47212: F, t47279: F, t10111: F, t22: F, t4092: F, t39515: F, t4083: F, t10043: F, t9303: F, t10014: F, t10019: F, t268: F, t4101: F, t543: F, t675: F, t9890: F, t10139: F, t281: F, t4056: F, t68: F, t9898: F, t14192: F, t555: F, t786: F, t9994: F, t10023: F, t4003: F, t10115: F, t1441: F, t213: F, t546: F, t10008: F, t545: F, t689: F, t869: F, t4093: F, t9292: F, t10065: F, t10073: F, t1432: F, t1433: F, t39497: F, t1385: F, t10061: F, t10069: F, t2782: F, t4086: F, t46407: F, t46565: F, t5744: F, t1428: F, t588: F, t10049: F, t820: F, t9912: F, t4066: F, t4104: F, t4100: F, t46433: F, t10022: F, t2453: F, t46507: F, t686: F, t72: F, t39644: F, t8779: F, t4107: F, t9288: F, t10107: F, t9285: F, t39494: F, t4096: F, t40270: F, t4089: F, t1437: F, t4114: F, t46902: F, t47188: F) -> (F, F, F, F) {
        let (t47320, t47325, t47329, t47333) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1764::<F>(t9775, t9981, t1398, t2661, t3992, t4010, t9956, t3938, t47218, t221, t4018, t4019, t9891);
        let t47340 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1765::<F>(t1389, t3964, t40604, t3961, t9741, t13783, t1388, t1390, t1399, t3934, t46532, t46682, t47282, t47284, t47286, t47296, t47298, t47302, t47304, t47306, t47318, t47320, t47325, t47329, t47333, t5673, t828, t9984);
        let (t47343, t47348, t47351) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1766::<F>(t46654, t46714, t46782, t46848, t46911, t47212, t47279, t47340, t10111, t22, t4092, t39515, t4083);
        let (t47352, t47354, t47359, t47364) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1767::<F>(t10043, t9303, t10014, t10019, t268, t4101, t543, t675, t9890, t10139, t281, t4056, t68);
        let (t47369, t47375, t47379, t47381) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1768::<F>(t675, t9898, t268, t4101, t543, t14192, t555, t786, t9994, t10023, t4003, t10115, t1441);
        let t47383 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1769::<F>(t213, t47343, t47348, t47351, t47352, t47354, t47359, t47364, t47369, t47375, t47379, t47381, t546);
        let (t47387, t47389, t47391, t47395, t47396) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1770::<F>(t10008, t545, t689, t869, t4093, t9292, t10065, t10073, t1432, t1433, t39497, t1385);
        let (t47403, t47407, t47411, t47413, t47417) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1771::<F>(t10061, t10069, t2782, t4086, t46407, t543, t4003, t46565, t5744, t10073, t10111, t1428, t588);
        let t47418 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1772::<F>(t10049, t1399, t47387, t47389, t47391, t47395, t47396, t47403, t47407, t47411, t47413, t47417, t820, t9912);
        let (t47424, t47427, t47432) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1773::<F>(t4066, t4086, t786, t4104, t2782, t4100, t46433, t10022, t2453, t281, t4003, t46507);
        let (t47436, t47442, t47444, t47450) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1774::<F>(t10008, t1432, t686, t72, t268, t39644, t546, t555, t8779, t4107, t9288, t10107, t3964, t9285);
        let t47457 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1775::<F>(t39494, t3964, t4096, t40270, t4089, t1437, t4114, t46902, t47188, t47424, t47427, t47432, t47436, t47442, t47444, t47450, t820);
    (t47343, t47383, t47418, t47457)
}
