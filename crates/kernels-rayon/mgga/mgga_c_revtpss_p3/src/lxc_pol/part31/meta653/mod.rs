//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta653 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2176;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2177;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2178;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2179;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2180;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2181;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2182;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2183;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2184;
use chunk9::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2185;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta653(t25759: f64, t77425: f64, t100987: f64, t27375: f64, t106625: f64, t29598: f64, t94245: f64, t1711: f64, t4343: f64, t106561: f64, t27799: f64, t105923: f64, t106596: f64, t1940: f64, t1963: f64, t20256: f64, t2403: f64, t25206: f64, t27770: f64, t27793: f64, t27800: f64, t29939: f64, t29949: f64, t29953: f64, t4541: f64, t7087: f64, t98637: f64, t11064: f64, t27384: f64, t106533: f64, t18875: f64, t4433: f64, t892: f64, t1113: f64, t5962: f64, t18392: f64, t33: f64, t100981: f64, t106565: f64, t6079: f64, t105930: f64, t106487: f64, t106496: f64, t25440: f64, t25445: f64, t27158: f64, t27368: f64, t27382: f64, t27764: f64, t27802: f64, t27806: f64, t29970: f64, t6416: f64, t775: f64, t106501: f64, t77441: f64, t4537: f64, t106539: f64, t27364: f64, t27773: f64, t27777: f64, t29705: f64, t29940: f64, t29946: f64, t29967: f64, t50080: f64, t7091: f64, t7200: f64, t7783: f64, t7862: f64, t7869: f64, t92819: f64, t99555: f64, t77408: f64, t890: f64, t5966: f64, t6075: f64, t106610: f64, t18435: f64, t27763: f64, t18498: f64, t106554: f64, t18838: f64, t106482: f64, t106516: f64, t27810: f64, t27817: f64, t29964: f64, t7207: f64, t93404: f64, t265: f64, t502: f64, t107868: f64, t1469: f64, t18281: f64, t2003: f64, t27822: f64, t29978: f64, t4186: f64, t57: f64, t5825: f64, t606: f64, t7215: f64, t7877: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t22279: f64, t28167: f64, t8996: f64, t29506: f64, t7313: f64, t105850: f64, t105859: f64, t105863: f64, t105866: f64, t105889: f64, t105894: f64, t105897: f64, t107881: f64, t118: f64, t1310: f64, t13426: f64, t18220: f64, t18227: f64, t18232: f64, t18245: f64, t1932: f64, t2007: f64, t21658: f64, t29573: f64, t508: f64, t5884: f64, t671: f64, t6765: f64, t6983: f64, t6985: f64, t7007: f64, t7221: f64, t7746: f64, t1843: f64, t28042: f64, t651: f64, t2322: f64, t30005: f64, t4254: f64, t30004: f64, t27123: f64, t7742: f64, t27126: f64, t28063: f64, t7732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t107882, t107885, t107892, t107895, t107901, t107908, t107919) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2176(t25759, t77425, t100987, t27375, t106625, t29598, t94245, t1711, t4343, t106561, t27799, t105923);
        let t107922 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2177(t106596, t107882, t107885, t107892, t107895, t107901, t107908, t107919, t1940, t1963, t20256, t2403, t25206, t27770, t27793, t27800, t29939, t29949, t29953, t4541, t7087, t98637);
        let (t107924, t107927, t107930, t107934, t107939, t107943) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2178(t11064, t1711, t27384, t106533, t25759, t100987, t18875, t4433, t892, t1113, t5962, t18392, t33);
        let t107963 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2179(t100981, t106565, t1113, t6079, t105930, t106487, t106496, t107924, t107927, t107930, t107934, t107939, t107943, t1940, t1963, t2403, t25206, t25440, t25445, t27158, t27368, t27382, t27764, t27802, t27806, t29970, t6416, t7087);
        let t108001 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2180(t6416, t775, t106501, t27799, t25759, t77441, t1711, t4537, t106539, t1113, t1940, t1963, t2403, t25206, t25440, t27364, t27773, t27777, t29705, t29940, t29946, t29967, t50080, t7091, t7200, t7783, t7862, t7869, t92819, t99555);
        let (t108002, t108005, t108009, t108021, t108028, t108030, t108033) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2181(t25759, t77408, t6416, t890, t1113, t5966, t6075, t106610, t27799, t18435, t27763, t18498);
        let t108047 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2182(t106554, t27799, t18838, t33, t106482, t106516, t108002, t108005, t108009, t108021, t108028, t108030, t108033, t1711, t1940, t1963, t2403, t27158, t27364, t27368, t27382, t27810, t27817, t29964, t4541, t7091, t7207, t7783, t93404);
        let t108062 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2183(t33, t265, t502, t107922, t107963, t108001, t108047, t107868, t1469, t18281, t2003, t27822, t29978, t4186, t57, t5825, t606, t7215, t7877, dens_threshold, rho1, zeta_threshold);
        let t108071 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2184(t22279, t28167, t8996, t29506, t7313, t105850, t105859, t105863, t105866, t105889, t105894, t105897, t107881, t108062, t118, t1310, t13426, t18220, t18227, t18232, t18245, t1932, t2007, t21658, t29573, t508, t5884, t671, t6765, t6983, t6985, t7007, t7221, t7746);
        let (t108076, t108078, t108080, t108083, t108085, t108087, t108089) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2185(t1843, t28042, t651, t2322, t30005, t4254, t1310, t30004, t27123, t7742, t27126, t28063, t7732);
    (t108071, t108076, t108078, t108080, t108083, t108085, t108087, t108089)
}
