//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta639 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2216;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2217;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2218;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2219;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2220;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2221;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2222;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta639<F: Float>(t101218: F, t2122: F, t101204: F, t101234: F, t101237: F, t101240: F, t101243: F, t101252: F, t101360: F, t10309: F, t2121: F, t2123: F, t25162: F, t26792: F, t26795: F, t28093: F, t28147: F, t28154: F, t607: F, t7576: F, t7579: F, t96752: F, t96757: F, t96804: F, t1479: F, t2282: F, t101303: F, t101376: F, t13312: F, t13392: F, t13396: F, t15936: F, t1923: F, t1927: F, t2251: F, t2258: F, t25117: F, t25146: F, t25150: F, t26776: F, t26783: F, t26786: F, t26789: F, t29355: F, t29363: F, t29364: F, t29367: F, t6954: F, t6977: F, t72: F, t7571: F, t7702: F, t8143: F, t8144: F, t8147: F, t92612: F, t96733: F, t5: F, t104194: F, t104222: F, t104249: F, t104274: F, t104303: F, t104330: F, t117: F, t101504: F, t101506: F, t101508: F, t101510: F, t101512: F, t101514: F, t101517: F, t101519: F, t101521: F, t101524: F, t101526: F, t101528: F, t104163: F, t670: F, t7583: F, t101530: F, t101532: F, t101534: F, t101536: F, t101538: F, t101540: F, t104115: F, t104138: F, t13514: F, t1518: F, t2371: F, t27060: F, t29427: F, t29432: F, t4292: F, t7586: F, t96706: F, t13429: F, t18153: F, t2127: F, t2163: F, t27056: F, t29456: F, t4254: F, t569: F, t651: F, t8233: F, t97661: F, t97663: F, t97666: F, t98421: F, t98426: F, t98428: F, t98430: F, t98432: F, t98439: F, t98440: F, t98442: F, t98449: F, t98452: F, t30: F, t265: F, t393: F, t100927: F, t1469: F, t2129: F, t26809: F, t28998: F, t4186: F, t45: F, t606: F, t7594: F, t8161: F, t99565: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1204: F, t8190: F, t2142: F, t5284: F, t3153: F, t1276: F, t42859: F, t13038: F, t2149: F, t11249: F, t29157: F, t73: F, t1203: F, t471: F, t355: F, t1214: F, t12713: F, t1294: F, t1295: F, t16750: F, t17848: F, t17875: F, t17963: F, t26889: F, t26895: F, t26988: F, t26994: F, t29141: F, t29158: F, t29174: F, t29194: F, t29195: F, t29200: F, t29212: F, t3551: F, t3738: F, t5457: F, t5458: F, t5465: F, t7636: F, t7637: F, t7643: F, t7651: F, t7652: F, t8202: F, t97034: F, t97304: F, t97348: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t104359 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2216::<F>(t101218, t2122, t101204, t101234, t101237, t101240, t101243, t101252, t101360, t10309, t2121, t2123, t25162, t26792, t26795, t28093, t28147, t28154, t607, t7576, t7579, t96752, t96757, t96804);
        let t104403 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2217::<F>(t1479, t2282, t101303, t101376, t13312, t13392, t13396, t15936, t1923, t1927, t2122, t2123, t2251, t2258, t25117, t25146, t25150, t26776, t26783, t26786, t26789, t29355, t29363, t29364, t29367, t6954, t6977, t72, t7571, t7702, t8143, t8144, t8147, t92612, t96733);
        let (t104408, t104409) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2218::<F>(t5, t104194, t104222, t104249, t104274, t104303, t104330, t104359, t104403, t117, t101504, t101506, t101508, t101510, t101512, t101514, t101517, t101519, t101521, t101524, t101526, t101528, t104163);
        let (t104416, t104427) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2219::<F>(t670, t7583, t101530, t101532, t101534, t101536, t101538, t101540, t104115, t104138, t13514, t1518, t2371, t27060, t29427, t29432, t4292, t7586, t96706);
        let t104433 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2220::<F>(t104409, t104427, t13429, t1518, t18153, t2127, t2163, t2371, t27056, t29456, t4254, t569, t651, t8233, t97661, t97663, t97666, t98421, t98426, t98428, t98430, t98432, t98439, t98440, t98442, t98449, t98452);
        let t104450 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2221::<F>(t30, t265, t393, t100927, t13312, t1469, t2129, t2258, t26809, t28998, t4186, t45, t606, t7594, t8161, t99565, dens_threshold, rho0, zeta_threshold);
        let (t104465, t104473, t104480, t104482, t104483, t104490) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2222::<F>(t1204, t8190, t2142, t5284, t3153, t1276, t42859, t13038, t2149, t11249, t29157, t73);
        let (t104504, t104509) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2223::<F>(t1203, t471, t355, t104465, t104473, t104482, t104483, t104490, t1214, t12713, t1294, t1295, t16750, t17848, t17875, t17963, t2142, t26889, t26895, t26988, t26994, t29141, t29158, t29174, t29194, t29195, t29200, t29212, t3551, t3738, t5457, t5458, t5465, t7636, t7637, t7643, t7651, t7652, t8202, t97034, t97304, t97348);
    (t104408, t104416, t104433, t104450, t104473, t104480, t104483, t104490, t104504, t104509)
}
