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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta685<F: Float>(t1769: F, t1774: F, t30882: F, t7658: F, t105167: F, t105269: F, t1214: F, t1248: F, t1287: F, t1294: F, t1794: F, t2151: F, t26889: F, t26895: F, t26922: F, t26931: F, t26969: F, t29122: F, t29186: F, t29207: F, t29271: F, t30747: F, t30771: F, t30854: F, t30867: F, t30886: F, t5284: F, t5429: F, t6587: F, t6622: F, t7627: F, t7637: F, t7643: F, t7651: F, t7652: F, t7659: F, t7662: F, t97066: F, t97343: F, t97363: F, t112757: F, t7642: F, t104521: F, t105046: F, t105354: F, t105409: F, t105499: F, t111825: F, t111991: F, t1203: F, t1775: F, t1828: F, t21506: F, t26949: F, t26979: F, t29111: F, t29118: F, t30758: F, t30763: F, t30899: F, t5497: F, t6573: F, t6579: F, t7636: F, t7645: F, t7648: F, t8197: F, t8205: F, t97397: F, t5219: F, t8190: F, t7635: F, t105433: F, t105598: F, t112535: F, t1215: F, t20748: F, t2149: F, t2150: F, t26994: F, t29132: F, t29141: F, t29179: F, t29220: F, t30767: F, t30840: F, t473: F, t5246: F, t7654: F, t8201: F, t8217: F, t96861: F, t97348: F, t30923: F, t3801: F, t105665: F, t105669: F, t111864: F, t111913: F, t111959: F, t112009: F, t112051: F, t112092: F, t112138: F, t112564: F, t112602: F, t112645: F, t112697: F, t112744: F, t112787: F, t1298: F, t1300: F, t1832: F, t198: F, t21635: F, t27037: F, t27041: F, t29317: F, t29322: F, t336: F, t5023: F, t5501: F, t6748: F, t6752: F, t7673: F, t97491: F, t97498: F, t33: F, t265: F, t502: F, t107868: F, t108049: F, t1469: F, t18281: F, t2159: F, t29329: F, t30936: F, t4186: F, t57: F, t5825: F, t606: F, t7677: F, t8227: F, dens_threshold: F, rho1: F, zeta_threshold: F, t109172: F, t109176: F, t109178: F, t109180: F, t109182: F, t109194: F, t109196: F, t109198: F, t109202: F, t109262: F, t109266: F, t109268: F, t109271: F, t109274: F, t111809: F, t118: F, t18220: F, t2163: F, t29427: F, t4297: F, t5884: F, t6934: F, t7683: F, t7687: F, t111690: F, t111704: F, t111717: F, t111746: F, t111762: F, t111770: F, t111796: F, t2172: F, t6936: F, t1921: F, t8240: F, t30993: F, t571: F, t104094: F, t111419: F, t1456: F, t1464: F, t29469: F, t3: F, t30975: F, t575: F, t5790: F, t5808: F, t6937: F, t6951: F, t7691: F, t7700: F, t8241: F, t8249: F, t105818: F, t105822: F, t105826: F, t105830: F, t105834: F, t105837: F, t105839: F, t105841: F, t105843: F, t109282: F, t109288: F, t2170: F, t22556: F, t22559: F, t22565: F, t22568: F, t5802: F, t6945: F, t7696: F, t8245: F, t109291: F, t109293: F, t109295: F, t109299: F, t109305: F, t109307: F, t109310: F, t109315: F, t109319: F, t109322: F, t109327: F, t109330: F, t109333: F, t1461: F, t1918: F, t29480: F, t30985: F, t573: F, t5805: F, t6948: F, param_d: F, t2167: F, t1913: F, t105792: F, t105794: F, t105796: F, t105798: F, t105800: F, t105802: F, t105804: F, t1458: F, t1914: F, t2168: F, t22533: F, t22571: F, t29490: F) -> F {
        let t112846 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2263::<F>(t1769, t1774, t30882, t7658, t105167, t105269, t1214, t1248, t1287, t1294, t1794, t2151, t26889, t26895, t26922, t26931, t26969, t29122, t29186, t29207, t29271, t30747, t30771, t30854, t30867, t30886, t5284, t5429, t6587, t6622, t7627, t7637, t7643, t7651, t7652, t7659, t7662, t97066, t97343, t97363);
        let t112899 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2264::<F>(t112757, t7642, t104521, t105046, t105354, t105409, t105499, t111825, t111991, t1203, t1214, t1294, t1775, t1828, t21506, t2151, t26949, t26979, t29111, t29118, t29186, t30758, t30763, t30886, t30899, t5497, t6573, t6579, t7627, t7636, t7637, t7643, t7645, t7648, t7652, t8197, t8205, t97066, t97397);
        let t112950 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2265::<F>(t5219, t8190, t30882, t7635, t105433, t105598, t112535, t1214, t1215, t1248, t1287, t1294, t1774, t1775, t1794, t20748, t2149, t2150, t26895, t26922, t26969, t26994, t29118, t29132, t29141, t29179, t29186, t29220, t30767, t30771, t30840, t30854, t30886, t473, t5246, t5284, t7637, t7643, t7651, t7654, t8201, t8217, t96861, t97348);
        let t112989 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2266::<F>(t30923, t3801, t105665, t105669, t111864, t111913, t111959, t112009, t112051, t112092, t112138, t112564, t112602, t112645, t112697, t112744, t112787, t112846, t112899, t112950, t1298, t1300, t1832, t198, t21635, t27037, t27041, t29317, t29322, t336, t5023, t5501, t6748, t6752, t7673, t97491, t97498);
        let t113002 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2267::<F>(t33, t265, t502, t107868, t112989, t108049, t1469, t18281, t2159, t29329, t30936, t4186, t57, t5825, t606, t7677, t8227, dens_threshold, rho1, zeta_threshold);
        let t113012 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2268::<F>(t109172, t109176, t109178, t109180, t109182, t109194, t109196, t109198, t109202, t109262, t109266, t109268, t109271, t109274, t111809, t113002, t118, t18220, t2163, t29427, t4297, t5884, t6934, t7683, t7687);
        let (t113015, t113019, t113022) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2269::<F>(t111690, t111704, t111717, t111746, t111762, t111770, t111796, t113012, t2172, t6936, t1921, t8240);
        let t113026 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2270::<F>(t30993, t571, t104094, t111419, t113015, t113019, t113022, t1456, t1464, t1921, t29469, t3, t30975, t575, t5790, t5808, t6937, t6951, t7691, t7700, t8241, t8249);
        let t113039 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2271::<F>(t105818, t105822, t105826, t105830, t105834, t105837, t105839, t105841, t105843, t109282, t109288, t2170, t22556, t22559, t22565, t22568, t5802, t6945, t7696, t8245);
        let t113050 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2272::<F>(t109291, t109293, t109295, t109299, t109305, t109307, t109310, t109315, t109319, t109322, t109327, t109330, t109333, t113015, t1461, t1918, t29480, t30985, t573, t5805, t6948, t7696, t8245, param_d);
        let t113060 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2273::<F>(t2167, t6951, t1913, t8249, t105792, t105794, t105796, t105798, t105800, t105802, t105804, t113039, t113050, t1458, t1914, t2168, t2172, t22533, t22571, t29490);
        let tv4rho3sigma8 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2274::<F>(t113026, t113060);
    tv4rho3sigma8
}
