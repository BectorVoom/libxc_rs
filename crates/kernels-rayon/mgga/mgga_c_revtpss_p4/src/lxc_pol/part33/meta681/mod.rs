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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta681(t108879: f64, t2122: f64, t101237: f64, t101240: f64, t101243: f64, t104215: f64, t104226: f64, t108872: f64, t108876: f64, t108941: f64, t108945: f64, t1923: f64, t2123: f64, t26792: f64, t28154: f64, t29380: f64, t29532: f64, t30689: f64, t6954: f64, t7575: f64, t92568: f64, t96804: f64, t28150: f64, t8143: f64, t108978: f64, t108986: f64, t101230: f64, t104203: f64, t104208: f64, t104314: f64, t104332: f64, t108966: f64, t108975: f64, t108983: f64, t108990: f64, t25162: f64, t26795: f64, t28147: f64, t5: f64, t111468: f64, t111493: f64, t111521: f64, t111548: f64, t111577: f64, t111623: f64, t117: f64, t105859: f64, t105863: f64, t105889: f64, t105894: f64, t105897: f64, t108067: f64, t108068: f64, t108076: f64, t1310: f64, t13426: f64, t18227: f64, t18245: f64, t21891: f64, t27060: f64, t29432: f64, t29444: f64, t30716: f64, t34446: f64, t4248: f64, t4293: f64, t508: f64, t5787: f64, t5887: f64, t7586: f64, t7591: f64, t8158: f64, t8237: f64, t116: f64, t30715: f64, t108078: f64, t108080: f64, t108083: f64, t108085: f64, t108087: f64, t108089: f64, t108099: f64, t108103: f64, t108105: f64, t108107: f64, t108109: f64, t108111: f64, t108117: f64, t1843: f64, t29422: f64, t29456: f64, t30944: f64, t4292: f64, t649: f64, t651: f64, t671: f64, t7732: f64, t8233: f64, t5883: f64, t7583: f64, t108129: f64, t108681: f64, t108685: f64, t108687: f64, t108691: f64, t108693: f64, t108712: f64, t108716: f64, t108718: f64, t108721: f64, t108723: f64, t108725: f64, t108727: f64, t2163: f64, t21814: f64, t21882: f64, t30724: f64, t5517: f64, t5877: f64, t7683: f64, t8152: f64, t670: f64, t8151: f64, t104115: f64, t109012: f64, t109014: f64, t109024: f64, t109029: f64, t109035: f64, t109038: f64, t109039: f64, t1519: f64, t18235: f64, t1911: f64, t21881: f64, t2322: f64, t29427: f64, t29437: f64, t29459: f64, t30951: f64, t4254: f64, t4257: f64, t5920: f64, t109041: f64, t109043: f64, t109045: f64, t109047: f64, t109049: f64, t109052: f64, t109054: f64, t109058: f64, t109060: f64, t109063: f64, t109074: f64, t109078: f64, t109081: f64, t1518: f64, t18242: f64, t29337: f64, t30963: f64, t5921: f64, t109087: f64, t109090: f64, t109092: f64, t109095: f64, t109099: f64, t109103: f64, t109107: f64, t109110: f64, t109112: f64, t109117: f64, t109121: f64, t109124: f64, t109126: f64, t109128: f64, t1502: f64, t2127: f64, t2165: f64, t21658: f64, t22506: f64, t4246: f64, t6765: f64, t7584: f64, t109204: f64, t109222: f64, t109224: f64, t109226: f64, t109228: f64, t109230: f64, t109233: f64, t109235: f64, t109237: f64, t109239: f64, t109241: f64, t109244: f64, t109246: f64, t109248: f64, t109250: f64, t109252: f64, t109254: f64, t109256: f64, t109135: f64, t109138: f64, t109140: f64, t109142: f64, t109144: f64, t109147: f64, t109149: f64, t109152: f64, t109155: f64, t109157: f64, t109158: f64, t109159: f64, t109162: f64, t109164: f64, t109167: f64, t109169: f64, t1453: f64, t18232: f64, t30959: f64, t569: f64, t30: f64, t265: f64, t393: f64, t107868: f64, t106638: f64, t1469: f64, t18281: f64, t2129: f64, t28998: f64, t30727: f64, t4186: f64, t45: f64, t5825: f64, t606: f64, t7594: f64, t8161: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t111652 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2221(t108879, t2122, t101237, t101240, t101243, t104215, t104226, t108872, t108876, t108941, t108945, t1923, t2123, t26792, t28154, t29380, t29532, t30689, t6954, t7575, t92568, t96804);
        let t111680 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2222(t28150, t8143, t108978, t2122, t108986, t101230, t104203, t104208, t104314, t104332, t108966, t108975, t108983, t108990, t25162, t26792, t26795, t28147, t28154, t29380);
        let (t111685, t111690) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2223(t5, t111468, t111493, t111521, t111548, t111577, t111623, t111652, t111680, t117, t105859, t105863, t105889, t105894, t105897, t108067, t108068, t108076, t1310, t13426, t18227, t18245, t21891, t27060, t29432, t29444, t30716, t34446, t4248, t4293, t508, t5787, t5887, t7586, t7591, t8158, t8237);
        let (t111696, t111704) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2224(t116, t30715, t108078, t108080, t108083, t108085, t108087, t108089, t108099, t108103, t108105, t108107, t108109, t108111, t108117, t1843, t29422, t29456, t30944, t4248, t4292, t649, t651, t671, t7732, t8233);
        let (t111708, t111717) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2225(t5883, t7583, t108129, t108681, t108685, t108687, t108691, t108693, t108712, t108716, t108718, t108721, t108723, t108725, t108727, t1310, t2163, t21814, t21882, t30724, t508, t5517, t5877, t7586, t7683, t8152);
        let (t111734, t111746) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2226(t670, t8151, t104115, t109012, t109014, t109024, t109029, t109035, t109038, t109039, t1519, t18235, t1911, t2163, t21881, t2322, t29427, t29437, t29459, t30944, t30951, t4248, t4254, t4257, t4293, t5920, t651, t7586, t7683);
        let t111762 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2227(t109041, t109043, t109045, t109047, t109049, t109052, t109054, t109058, t109060, t109063, t109074, t109078, t109081, t1518, t18242, t2322, t27060, t29337, t29432, t30963, t4254, t5921, t651, t7586);
        let t111770 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2228(t109087, t109090, t109092, t109095, t109099, t109103, t109107, t109110, t109112, t109117, t109121, t109124, t109126, t109128, t1502, t2127, t2165, t21658, t22506, t29337, t4246, t6765, t7584, t8233);
        let t111788 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2229(t104115, t109204, t109222, t109224, t109226, t109228, t109230, t111696, t111734, t1518, t21881, t27060, t29427, t29432, t34446, t4292, t5920, t670, t7586);
        let t111790 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2230(t109233, t109235, t109237, t109239, t109241, t109244, t109246, t109248, t109250, t109252, t109254, t109256, t111685, t111708);
        let t111796 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2231(t109135, t109138, t109140, t109142, t109144, t109147, t109149, t109152, t109155, t109157, t109158, t109159, t109162, t109164, t109167, t109169, t111788, t111790, t1453, t18232, t30959, t569, t7586);
        let t111809 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2232(t30, t265, t393, t107868, t106638, t1469, t18281, t2129, t28998, t30727, t4186, t45, t5825, t606, t7594, t8161, dens_threshold, rho0, zeta_threshold);
    (t111690, t111704, t111717, t111746, t111762, t111770, t111796, t111809)
}
