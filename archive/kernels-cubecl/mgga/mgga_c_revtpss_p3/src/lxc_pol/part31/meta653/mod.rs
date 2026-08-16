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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta653<F: Float>(t25759: F, t77425: F, t100987: F, t27375: F, t106625: F, t29598: F, t94245: F, t1711: F, t4343: F, t106561: F, t27799: F, t105923: F, t106596: F, t1940: F, t1963: F, t20256: F, t2403: F, t25206: F, t27770: F, t27793: F, t27800: F, t29939: F, t29949: F, t29953: F, t4541: F, t7087: F, t98637: F, t11064: F, t27384: F, t106533: F, t18875: F, t4433: F, t892: F, t1113: F, t5962: F, t18392: F, t33: F, t100981: F, t106565: F, t6079: F, t105930: F, t106487: F, t106496: F, t25440: F, t25445: F, t27158: F, t27368: F, t27382: F, t27764: F, t27802: F, t27806: F, t29970: F, t6416: F, t775: F, t106501: F, t77441: F, t4537: F, t106539: F, t27364: F, t27773: F, t27777: F, t29705: F, t29940: F, t29946: F, t29967: F, t50080: F, t7091: F, t7200: F, t7783: F, t7862: F, t7869: F, t92819: F, t99555: F, t77408: F, t890: F, t5966: F, t6075: F, t106610: F, t18435: F, t27763: F, t18498: F, t106554: F, t18838: F, t106482: F, t106516: F, t27810: F, t27817: F, t29964: F, t7207: F, t93404: F, t265: F, t502: F, t107868: F, t1469: F, t18281: F, t2003: F, t27822: F, t29978: F, t4186: F, t57: F, t5825: F, t606: F, t7215: F, t7877: F, dens_threshold: F, rho1: F, zeta_threshold: F, t22279: F, t28167: F, t8996: F, t29506: F, t7313: F, t105850: F, t105859: F, t105863: F, t105866: F, t105889: F, t105894: F, t105897: F, t107881: F, t118: F, t1310: F, t13426: F, t18220: F, t18227: F, t18232: F, t18245: F, t1932: F, t2007: F, t21658: F, t29573: F, t508: F, t5884: F, t671: F, t6765: F, t6983: F, t6985: F, t7007: F, t7221: F, t7746: F, t1843: F, t28042: F, t651: F, t2322: F, t30005: F, t4254: F, t30004: F, t27123: F, t7742: F, t27126: F, t28063: F, t7732: F) -> (F, F, F, F, F, F, F, F) {
        let (t107882, t107885, t107892, t107895, t107901, t107908, t107919) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2176::<F>(t25759, t77425, t100987, t27375, t106625, t29598, t94245, t1711, t4343, t106561, t27799, t105923);
        let t107922 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2177::<F>(t106596, t107882, t107885, t107892, t107895, t107901, t107908, t107919, t1940, t1963, t20256, t2403, t25206, t27770, t27793, t27800, t29939, t29949, t29953, t4541, t7087, t98637);
        let (t107924, t107927, t107930, t107934, t107939, t107943) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2178::<F>(t11064, t1711, t27384, t106533, t25759, t100987, t18875, t4433, t892, t1113, t5962, t18392, t33);
        let t107963 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2179::<F>(t100981, t106565, t1113, t6079, t105930, t106487, t106496, t107924, t107927, t107930, t107934, t107939, t107943, t1940, t1963, t2403, t25206, t25440, t25445, t27158, t27368, t27382, t27764, t27802, t27806, t29970, t6416, t7087);
        let t108001 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2180::<F>(t6416, t775, t106501, t27799, t25759, t77441, t1711, t4537, t106539, t1113, t1940, t1963, t2403, t25206, t25440, t27364, t27773, t27777, t29705, t29940, t29946, t29967, t50080, t7091, t7200, t7783, t7862, t7869, t92819, t99555);
        let (t108002, t108005, t108009, t108021, t108028, t108030, t108033) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2181::<F>(t25759, t77408, t6416, t890, t1113, t5966, t6075, t106610, t27799, t18435, t27763, t18498);
        let t108047 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2182::<F>(t106554, t27799, t18838, t33, t106482, t106516, t108002, t108005, t108009, t108021, t108028, t108030, t108033, t1711, t1940, t1963, t2403, t27158, t27364, t27368, t27382, t27810, t27817, t29964, t4541, t7091, t7207, t7783, t93404);
        let t108062 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2183::<F>(t33, t265, t502, t107922, t107963, t108001, t108047, t107868, t1469, t18281, t2003, t27822, t29978, t4186, t57, t5825, t606, t7215, t7877, dens_threshold, rho1, zeta_threshold);
        let t108071 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2184::<F>(t22279, t28167, t8996, t29506, t7313, t105850, t105859, t105863, t105866, t105889, t105894, t105897, t107881, t108062, t118, t1310, t13426, t18220, t18227, t18232, t18245, t1932, t2007, t21658, t29573, t508, t5884, t671, t6765, t6983, t6985, t7007, t7221, t7746);
        let (t108076, t108078, t108080, t108083, t108085, t108087, t108089) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2185::<F>(t1843, t28042, t651, t2322, t30005, t4254, t1310, t30004, t27123, t7742, t27126, t28063, t7732);
    (t108071, t108076, t108078, t108080, t108083, t108085, t108087, t108089)
}
