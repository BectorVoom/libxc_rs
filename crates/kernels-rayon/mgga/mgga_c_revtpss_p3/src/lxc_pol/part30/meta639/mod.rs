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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2216;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2217;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2218;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2219;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2220;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2221;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2222;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta639(t101218: f64, t2122: f64, t101204: f64, t101234: f64, t101237: f64, t101240: f64, t101243: f64, t101252: f64, t101360: f64, t10309: f64, t2121: f64, t2123: f64, t25162: f64, t26792: f64, t26795: f64, t28093: f64, t28147: f64, t28154: f64, t607: f64, t7576: f64, t7579: f64, t96752: f64, t96757: f64, t96804: f64, t1479: f64, t2282: f64, t101303: f64, t101376: f64, t13312: f64, t13392: f64, t13396: f64, t15936: f64, t1923: f64, t1927: f64, t2251: f64, t2258: f64, t25117: f64, t25146: f64, t25150: f64, t26776: f64, t26783: f64, t26786: f64, t26789: f64, t29355: f64, t29363: f64, t29364: f64, t29367: f64, t6954: f64, t6977: f64, t72: f64, t7571: f64, t7702: f64, t8143: f64, t8144: f64, t8147: f64, t92612: f64, t96733: f64, t5: f64, t104194: f64, t104222: f64, t104249: f64, t104274: f64, t104303: f64, t104330: f64, t117: f64, t101504: f64, t101506: f64, t101508: f64, t101510: f64, t101512: f64, t101514: f64, t101517: f64, t101519: f64, t101521: f64, t101524: f64, t101526: f64, t101528: f64, t104163: f64, t670: f64, t7583: f64, t101530: f64, t101532: f64, t101534: f64, t101536: f64, t101538: f64, t101540: f64, t104115: f64, t104138: f64, t13514: f64, t1518: f64, t2371: f64, t27060: f64, t29427: f64, t29432: f64, t4292: f64, t7586: f64, t96706: f64, t13429: f64, t18153: f64, t2127: f64, t2163: f64, t27056: f64, t29456: f64, t4254: f64, t569: f64, t651: f64, t8233: f64, t97661: f64, t97663: f64, t97666: f64, t98421: f64, t98426: f64, t98428: f64, t98430: f64, t98432: f64, t98439: f64, t98440: f64, t98442: f64, t98449: f64, t98452: f64, t30: f64, t265: f64, t393: f64, t100927: f64, t1469: f64, t2129: f64, t26809: f64, t28998: f64, t4186: f64, t45: f64, t606: f64, t7594: f64, t8161: f64, t99565: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1204: f64, t8190: f64, t2142: f64, t5284: f64, t3153: f64, t1276: f64, t42859: f64, t13038: f64, t2149: f64, t11249: f64, t29157: f64, t73: f64, t1203: f64, t471: f64, t355: f64, t1214: f64, t12713: f64, t1294: f64, t1295: f64, t16750: f64, t17848: f64, t17875: f64, t17963: f64, t26889: f64, t26895: f64, t26988: f64, t26994: f64, t29141: f64, t29158: f64, t29174: f64, t29194: f64, t29195: f64, t29200: f64, t29212: f64, t3551: f64, t3738: f64, t5457: f64, t5458: f64, t5465: f64, t7636: f64, t7637: f64, t7643: f64, t7651: f64, t7652: f64, t8202: f64, t97034: f64, t97304: f64, t97348: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t104359 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2216(t101218, t2122, t101204, t101234, t101237, t101240, t101243, t101252, t101360, t10309, t2121, t2123, t25162, t26792, t26795, t28093, t28147, t28154, t607, t7576, t7579, t96752, t96757, t96804);
        let t104403 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2217(t1479, t2282, t101303, t101376, t13312, t13392, t13396, t15936, t1923, t1927, t2122, t2123, t2251, t2258, t25117, t25146, t25150, t26776, t26783, t26786, t26789, t29355, t29363, t29364, t29367, t6954, t6977, t72, t7571, t7702, t8143, t8144, t8147, t92612, t96733);
        let (t104408, t104409) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2218(t5, t104194, t104222, t104249, t104274, t104303, t104330, t104359, t104403, t117, t101504, t101506, t101508, t101510, t101512, t101514, t101517, t101519, t101521, t101524, t101526, t101528, t104163);
        let (t104416, t104427) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2219(t670, t7583, t101530, t101532, t101534, t101536, t101538, t101540, t104115, t104138, t13514, t1518, t2371, t27060, t29427, t29432, t4292, t7586, t96706);
        let t104433 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2220(t104409, t104427, t13429, t1518, t18153, t2127, t2163, t2371, t27056, t29456, t4254, t569, t651, t8233, t97661, t97663, t97666, t98421, t98426, t98428, t98430, t98432, t98439, t98440, t98442, t98449, t98452);
        let t104450 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2221(t30, t265, t393, t100927, t13312, t1469, t2129, t2258, t26809, t28998, t4186, t45, t606, t7594, t8161, t99565, dens_threshold, rho0, zeta_threshold);
        let (t104465, t104473, t104480, t104482, t104483, t104490) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2222(t1204, t8190, t2142, t5284, t3153, t1276, t42859, t13038, t2149, t11249, t29157, t73);
        let (t104504, t104509) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2223(t1203, t471, t355, t104465, t104473, t104482, t104483, t104490, t1214, t12713, t1294, t1295, t16750, t17848, t17875, t17963, t2142, t26889, t26895, t26988, t26994, t29141, t29158, t29174, t29194, t29195, t29200, t29212, t3551, t3738, t5457, t5458, t5465, t7636, t7637, t7643, t7651, t7652, t8202, t97034, t97304, t97348);
    (t104408, t104416, t104433, t104450, t104473, t104480, t104483, t104490, t104504, t104509)
}
