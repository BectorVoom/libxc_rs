//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta685 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2263;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2264;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2265;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2266;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2267;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2268;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2269;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2270;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2271;
use chunk9::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2272;
use chunk10::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2273;
use chunk11::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta685(t1769: f64, t1774: f64, t30882: f64, t7658: f64, t105167: f64, t105269: f64, t1214: f64, t1248: f64, t1287: f64, t1294: f64, t1794: f64, t2151: f64, t26889: f64, t26895: f64, t26922: f64, t26931: f64, t26969: f64, t29122: f64, t29186: f64, t29207: f64, t29271: f64, t30747: f64, t30771: f64, t30854: f64, t30867: f64, t30886: f64, t5284: f64, t5429: f64, t6587: f64, t6622: f64, t7627: f64, t7637: f64, t7643: f64, t7651: f64, t7652: f64, t7659: f64, t7662: f64, t97066: f64, t97343: f64, t97363: f64, t112757: f64, t7642: f64, t104521: f64, t105046: f64, t105354: f64, t105409: f64, t105499: f64, t111825: f64, t111991: f64, t1203: f64, t1775: f64, t1828: f64, t21506: f64, t26949: f64, t26979: f64, t29111: f64, t29118: f64, t30758: f64, t30763: f64, t30899: f64, t5497: f64, t6573: f64, t6579: f64, t7636: f64, t7645: f64, t7648: f64, t8197: f64, t8205: f64, t97397: f64, t5219: f64, t8190: f64, t7635: f64, t105433: f64, t105598: f64, t112535: f64, t1215: f64, t20748: f64, t2149: f64, t2150: f64, t26994: f64, t29132: f64, t29141: f64, t29179: f64, t29220: f64, t30767: f64, t30840: f64, t473: f64, t5246: f64, t7654: f64, t8201: f64, t8217: f64, t96861: f64, t97348: f64, t30923: f64, t3801: f64, t105665: f64, t105669: f64, t111864: f64, t111913: f64, t111959: f64, t112009: f64, t112051: f64, t112092: f64, t112138: f64, t112564: f64, t112602: f64, t112645: f64, t112697: f64, t112744: f64, t112787: f64, t1298: f64, t1300: f64, t1832: f64, t198: f64, t21635: f64, t27037: f64, t27041: f64, t29317: f64, t29322: f64, t336: f64, t5023: f64, t5501: f64, t6748: f64, t6752: f64, t7673: f64, t97491: f64, t97498: f64, t33: f64, t265: f64, t502: f64, t107868: f64, t108049: f64, t1469: f64, t18281: f64, t2159: f64, t29329: f64, t30936: f64, t4186: f64, t57: f64, t5825: f64, t606: f64, t7677: f64, t8227: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t109172: f64, t109176: f64, t109178: f64, t109180: f64, t109182: f64, t109194: f64, t109196: f64, t109198: f64, t109202: f64, t109262: f64, t109266: f64, t109268: f64, t109271: f64, t109274: f64, t111809: f64, t118: f64, t18220: f64, t2163: f64, t29427: f64, t4297: f64, t5884: f64, t6934: f64, t7683: f64, t7687: f64, t111690: f64, t111704: f64, t111717: f64, t111746: f64, t111762: f64, t111770: f64, t111796: f64, t2172: f64, t6936: f64, t1921: f64, t8240: f64, t30993: f64, t571: f64, t104094: f64, t111419: f64, t1456: f64, t1464: f64, t29469: f64, t3: f64, t30975: f64, t575: f64, t5790: f64, t5808: f64, t6937: f64, t6951: f64, t7691: f64, t7700: f64, t8241: f64, t8249: f64, t105818: f64, t105822: f64, t105826: f64, t105830: f64, t105834: f64, t105837: f64, t105839: f64, t105841: f64, t105843: f64, t109282: f64, t109288: f64, t2170: f64, t22556: f64, t22559: f64, t22565: f64, t22568: f64, t5802: f64, t6945: f64, t7696: f64, t8245: f64, t109291: f64, t109293: f64, t109295: f64, t109299: f64, t109305: f64, t109307: f64, t109310: f64, t109315: f64, t109319: f64, t109322: f64, t109327: f64, t109330: f64, t109333: f64, t1461: f64, t1918: f64, t29480: f64, t30985: f64, t573: f64, t5805: f64, t6948: f64, param_d: f64, t2167: f64, t1913: f64, t105792: f64, t105794: f64, t105796: f64, t105798: f64, t105800: f64, t105802: f64, t105804: f64, t1458: f64, t1914: f64, t2168: f64, t22533: f64, t22571: f64, t29490: f64) -> f64 {
        let t112846 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2263(t1769, t1774, t30882, t7658, t105167, t105269, t1214, t1248, t1287, t1294, t1794, t2151, t26889, t26895, t26922, t26931, t26969, t29122, t29186, t29207, t29271, t30747, t30771, t30854, t30867, t30886, t5284, t5429, t6587, t6622, t7627, t7637, t7643, t7651, t7652, t7659, t7662, t97066, t97343, t97363);
        let t112899 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2264(t112757, t7642, t104521, t105046, t105354, t105409, t105499, t111825, t111991, t1203, t1214, t1294, t1775, t1828, t21506, t2151, t26949, t26979, t29111, t29118, t29186, t30758, t30763, t30886, t30899, t5497, t6573, t6579, t7627, t7636, t7637, t7643, t7645, t7648, t7652, t8197, t8205, t97066, t97397);
        let t112950 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2265(t5219, t8190, t30882, t7635, t105433, t105598, t112535, t1214, t1215, t1248, t1287, t1294, t1774, t1775, t1794, t20748, t2149, t2150, t26895, t26922, t26969, t26994, t29118, t29132, t29141, t29179, t29186, t29220, t30767, t30771, t30840, t30854, t30886, t473, t5246, t5284, t7637, t7643, t7651, t7654, t8201, t8217, t96861, t97348);
        let t112989 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2266(t30923, t3801, t105665, t105669, t111864, t111913, t111959, t112009, t112051, t112092, t112138, t112564, t112602, t112645, t112697, t112744, t112787, t112846, t112899, t112950, t1298, t1300, t1832, t198, t21635, t27037, t27041, t29317, t29322, t336, t5023, t5501, t6748, t6752, t7673, t97491, t97498);
        let t113002 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2267(t33, t265, t502, t107868, t112989, t108049, t1469, t18281, t2159, t29329, t30936, t4186, t57, t5825, t606, t7677, t8227, dens_threshold, rho1, zeta_threshold);
        let t113012 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2268(t109172, t109176, t109178, t109180, t109182, t109194, t109196, t109198, t109202, t109262, t109266, t109268, t109271, t109274, t111809, t113002, t118, t18220, t2163, t29427, t4297, t5884, t6934, t7683, t7687);
        let (t113015, t113019, t113022) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2269(t111690, t111704, t111717, t111746, t111762, t111770, t111796, t113012, t2172, t6936, t1921, t8240);
        let t113026 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2270(t30993, t571, t104094, t111419, t113015, t113019, t113022, t1456, t1464, t1921, t29469, t3, t30975, t575, t5790, t5808, t6937, t6951, t7691, t7700, t8241, t8249);
        let t113039 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2271(t105818, t105822, t105826, t105830, t105834, t105837, t105839, t105841, t105843, t109282, t109288, t2170, t22556, t22559, t22565, t22568, t5802, t6945, t7696, t8245);
        let t113050 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2272(t109291, t109293, t109295, t109299, t109305, t109307, t109310, t109315, t109319, t109322, t109327, t109330, t109333, t113015, t1461, t1918, t29480, t30985, t573, t5805, t6948, t7696, t8245, param_d);
        let t113060 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2273(t2167, t6951, t1913, t8249, t105792, t105794, t105796, t105798, t105800, t105802, t105804, t113039, t113050, t1458, t1914, t2168, t2172, t22533, t22571, t29490);
        let tv4rho3sigma8 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2274(t113026, t113060);
    tv4rho3sigma8
}
