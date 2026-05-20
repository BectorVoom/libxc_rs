//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta681 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2221;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2222;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2223;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2224;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2225;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2226;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2227;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2228;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2229;
use chunk9::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2230;
use chunk10::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2231;
use chunk11::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta681<F: Float>(t108879: F, t2122: F, t101237: F, t101240: F, t101243: F, t104215: F, t104226: F, t108872: F, t108876: F, t108941: F, t108945: F, t1923: F, t2123: F, t26792: F, t28154: F, t29380: F, t29532: F, t30689: F, t6954: F, t7575: F, t92568: F, t96804: F, t28150: F, t8143: F, t108978: F, t108986: F, t101230: F, t104203: F, t104208: F, t104314: F, t104332: F, t108966: F, t108975: F, t108983: F, t108990: F, t25162: F, t26795: F, t28147: F, t5: F, t111468: F, t111493: F, t111521: F, t111548: F, t111577: F, t111623: F, t117: F, t105859: F, t105863: F, t105889: F, t105894: F, t105897: F, t108067: F, t108068: F, t108076: F, t1310: F, t13426: F, t18227: F, t18245: F, t21891: F, t27060: F, t29432: F, t29444: F, t30716: F, t34446: F, t4248: F, t4293: F, t508: F, t5787: F, t5887: F, t7586: F, t7591: F, t8158: F, t8237: F, t116: F, t30715: F, t108078: F, t108080: F, t108083: F, t108085: F, t108087: F, t108089: F, t108099: F, t108103: F, t108105: F, t108107: F, t108109: F, t108111: F, t108117: F, t1843: F, t29422: F, t29456: F, t30944: F, t4292: F, t649: F, t651: F, t671: F, t7732: F, t8233: F, t5883: F, t7583: F, t108129: F, t108681: F, t108685: F, t108687: F, t108691: F, t108693: F, t108712: F, t108716: F, t108718: F, t108721: F, t108723: F, t108725: F, t108727: F, t2163: F, t21814: F, t21882: F, t30724: F, t5517: F, t5877: F, t7683: F, t8152: F, t670: F, t8151: F, t104115: F, t109012: F, t109014: F, t109024: F, t109029: F, t109035: F, t109038: F, t109039: F, t1519: F, t18235: F, t1911: F, t21881: F, t2322: F, t29427: F, t29437: F, t29459: F, t30951: F, t4254: F, t4257: F, t5920: F, t109041: F, t109043: F, t109045: F, t109047: F, t109049: F, t109052: F, t109054: F, t109058: F, t109060: F, t109063: F, t109074: F, t109078: F, t109081: F, t1518: F, t18242: F, t29337: F, t30963: F, t5921: F, t109087: F, t109090: F, t109092: F, t109095: F, t109099: F, t109103: F, t109107: F, t109110: F, t109112: F, t109117: F, t109121: F, t109124: F, t109126: F, t109128: F, t1502: F, t2127: F, t2165: F, t21658: F, t22506: F, t4246: F, t6765: F, t7584: F, t109204: F, t109222: F, t109224: F, t109226: F, t109228: F, t109230: F, t109233: F, t109235: F, t109237: F, t109239: F, t109241: F, t109244: F, t109246: F, t109248: F, t109250: F, t109252: F, t109254: F, t109256: F, t109135: F, t109138: F, t109140: F, t109142: F, t109144: F, t109147: F, t109149: F, t109152: F, t109155: F, t109157: F, t109158: F, t109159: F, t109162: F, t109164: F, t109167: F, t109169: F, t1453: F, t18232: F, t30959: F, t569: F, t30: F, t265: F, t393: F, t107868: F, t106638: F, t1469: F, t18281: F, t2129: F, t28998: F, t30727: F, t4186: F, t45: F, t5825: F, t606: F, t7594: F, t8161: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
        let t111652 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2221::<F>(t108879, t2122, t101237, t101240, t101243, t104215, t104226, t108872, t108876, t108941, t108945, t1923, t2123, t26792, t28154, t29380, t29532, t30689, t6954, t7575, t92568, t96804);
        let t111680 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2222::<F>(t28150, t8143, t108978, t2122, t108986, t101230, t104203, t104208, t104314, t104332, t108966, t108975, t108983, t108990, t25162, t26792, t26795, t28147, t28154, t29380);
        let (t111685, t111690) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2223::<F>(t5, t111468, t111493, t111521, t111548, t111577, t111623, t111652, t111680, t117, t105859, t105863, t105889, t105894, t105897, t108067, t108068, t108076, t1310, t13426, t18227, t18245, t21891, t27060, t29432, t29444, t30716, t34446, t4248, t4293, t508, t5787, t5887, t7586, t7591, t8158, t8237);
        let (t111696, t111704) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2224::<F>(t116, t30715, t108078, t108080, t108083, t108085, t108087, t108089, t108099, t108103, t108105, t108107, t108109, t108111, t108117, t1843, t29422, t29456, t30944, t4248, t4292, t649, t651, t671, t7732, t8233);
        let (t111708, t111717) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2225::<F>(t5883, t7583, t108129, t108681, t108685, t108687, t108691, t108693, t108712, t108716, t108718, t108721, t108723, t108725, t108727, t1310, t2163, t21814, t21882, t30724, t508, t5517, t5877, t7586, t7683, t8152);
        let (t111734, t111746) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2226::<F>(t670, t8151, t104115, t109012, t109014, t109024, t109029, t109035, t109038, t109039, t1519, t18235, t1911, t2163, t21881, t2322, t29427, t29437, t29459, t30944, t30951, t4248, t4254, t4257, t4293, t5920, t651, t7586, t7683);
        let t111762 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2227::<F>(t109041, t109043, t109045, t109047, t109049, t109052, t109054, t109058, t109060, t109063, t109074, t109078, t109081, t1518, t18242, t2322, t27060, t29337, t29432, t30963, t4254, t5921, t651, t7586);
        let t111770 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2228::<F>(t109087, t109090, t109092, t109095, t109099, t109103, t109107, t109110, t109112, t109117, t109121, t109124, t109126, t109128, t1502, t2127, t2165, t21658, t22506, t29337, t4246, t6765, t7584, t8233);
        let t111788 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2229::<F>(t104115, t109204, t109222, t109224, t109226, t109228, t109230, t111696, t111734, t1518, t21881, t27060, t29427, t29432, t34446, t4292, t5920, t670, t7586);
        let t111790 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2230::<F>(t109233, t109235, t109237, t109239, t109241, t109244, t109246, t109248, t109250, t109252, t109254, t109256, t111685, t111708);
        let t111796 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2231::<F>(t109135, t109138, t109140, t109142, t109144, t109147, t109149, t109152, t109155, t109157, t109158, t109159, t109162, t109164, t109167, t109169, t111788, t111790, t1453, t18232, t30959, t569, t7586);
        let t111809 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2232::<F>(t30, t265, t393, t107868, t106638, t1469, t18281, t2129, t28998, t30727, t4186, t45, t5825, t606, t7594, t8161, dens_threshold, rho0, zeta_threshold);
    (t111690, t111704, t111717, t111746, t111762, t111770, t111796, t111809)
}
