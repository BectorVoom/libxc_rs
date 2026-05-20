//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta630 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2031;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2032;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2033;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2034;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2035;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2036;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2037;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2038;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2039;
use chunk9::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2040;
use chunk10::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2041;
use chunk11::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta630<F: Float>(t30391: F, t689: F, t93314: F, t93302: F, t2718: F, t7997: F, t103212: F, t103521: F, t103529: F, t103543: F, t103547: F, t106275: F, t14587: F, t1580: F, t25383: F, t26550: F, t27353: F, t28400: F, t28425: F, t30357: F, t62604: F, t62637: F, t7420: F, t7766: F, t95945: F, t95948: F, t110242: F, t110261: F, t110281: F, t110306: F, t110330: F, t110348: F, t110365: F, t110466: F, t110499: F, t110519: F, t110551: F, t110576: F, t110607: F, t110635: F, t110665: F, t892: F, t198: F, t205: F, t8019: F, t102854: F, t105906: F, t106534: F, t106540: F, t106546: F, t106562: F, t106590: F, t106593: F, t106606: F, t1468: F, t1940: F, t26425: F, t26585: F, t26590: F, t27160: F, t28291: F, t28456: F, t28472: F, t29599: F, t29719: F, t30: F, t7432: F, t7787: F, t95511: F, t106494: F, t102888: F, t106490: F, t106498: F, t106502: F, t106520: F, t106528: F, t106572: F, t106583: F, t106602: F, t2403: F, t27166: F, t27376: F, t27387: F, t27391: F, t27395: F, t28460: F, t30420: F, t605: F, t8020: F, t106554: F, t106565: F, t106610: F, t107793: F, t107805: F, t18435: F, t18498: F, t18838: F, t207: F, t2071: F, t27375: F, t4541: F, t5962: F, t6075: F, t7428: F, t77408: F, t77425: F, t77441: F, t775: F, t95964: F, t103586: F, t105923: F, t106561: F, t106625: F, t110177: F, t1544: F, t1583: F, t18392: F, t18875: F, t27384: F, t29598: F, t30439: F, t4343: F, t4433: F, t4537: F, t50080: F, t5966: F, t6079: F, t890: F, t95976: F, t265: F, t393: F, t110158: F, t110196: F, t1469: F, t18281: F, t2078: F, t28523: F, t30463: F, t4186: F, t45: F, t5825: F, t606: F, t7449: F, t8040: F, dens_threshold: F, rho0: F, zeta_threshold: F, t107974: F, t108002: F, t108005: F, t108021: F, t108033: F, t108043: F, t110150: F, t110154: F, t110165: F, t1711: F, t20256: F, t27800: F, t29946: F, t29967: F, t7862: F, t107882: F, t107885: F, t107895: F, t107939: F, t107943: F, t107947: F, t107985: F, t108028: F, t108036: F, t27773: F, t27777: F, t27810: F, t27817: F, t29949: F, t7200: F, t107901: F, t107919: F, t107924: F, t107930: F, t107988: F, t108009: F, t108030: F, t1113: F, t27793: F, t29953: F, t29964: F, t6416: F, t7207: F, t107892: F, t107908: F, t107927: F, t107934: F, t107958: F, t107970: F, t27764: F, t27770: F, t27802: F, t27806: F, t29939: F, t29970: F, t30471: F, t33: F, t7869: F, t502: F, t2085: F, t28578: F, t30503: F, t57: F, t7468: F, t8059: F, rho1: F) -> (F, F) {
        let t110694 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2031::<F>(t30391, t689, t93314, t93302, t2718, t7997, t103212, t103521, t103529, t103543, t103547, t106275, t14587, t1580, t25383, t26550, t27353, t28400, t28425, t30357, t62604, t62637, t7420, t7766, t95945, t95948);
        let t110698 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2032::<F>(t110242, t110261, t110281, t110306, t110330, t110348, t110365, t110466, t110499, t110519, t110551, t110576, t110607, t110635, t110665, t110694);
        let (t110699, t110704, t110711) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2033::<F>(t110698, t892, t198, t205, t8019, t102854, t105906, t106534, t106540, t106546, t106562, t106590, t106593, t106606, t1468, t1940, t26425, t26585, t26590, t27160, t28291, t28456, t28472, t29599, t29719, t30, t7432, t7787, t95511);
        let (t110717, t110745) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2034::<F>(t106494, t26425, t102888, t106490, t106498, t106502, t106520, t106528, t106572, t106583, t106602, t1940, t2403, t27166, t27376, t27387, t27391, t27395, t28291, t28460, t30420, t605, t7432, t8020);
        let t110792 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2035::<F>(t106554, t106565, t106610, t107793, t107805, t110698, t18435, t18498, t18838, t1940, t198, t207, t2071, t2403, t26425, t26585, t26590, t27375, t28291, t28460, t30420, t4541, t5962, t6075, t7428, t7432, t77408, t77425, t77441, t775, t892, t95964);
        let t110839 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2036::<F>(t102854, t103586, t105923, t106561, t106625, t110177, t1544, t1583, t18392, t18875, t1940, t2071, t2403, t26585, t26590, t27384, t28456, t28460, t29598, t30439, t4343, t4433, t4537, t4541, t50080, t5966, t6079, t7428, t7432, t8020, t890, t95976);
        let (t110840, t110853) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2037::<F>(t30, t265, t393, t110792, t110839, t110158, t110196, t110711, t110745, t1469, t18281, t2078, t28523, t30463, t4186, t45, t5825, t606, t7449, t8040, dens_threshold, rho0, zeta_threshold);
        let t110883 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2038::<F>(t107974, t108002, t108005, t108021, t108033, t108043, t110150, t110154, t110165, t1711, t1940, t20256, t2071, t2403, t26425, t26585, t27800, t28291, t28456, t29946, t29967, t7432, t7862, t95511);
        let t110920 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2039::<F>(t107882, t107885, t107895, t107939, t107943, t107947, t107985, t108028, t108036, t1940, t2071, t2403, t26425, t27773, t27777, t27810, t27817, t28460, t28472, t29949, t30420, t7200, t7428, t8020);
        let t110954 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2040::<F>(t102888, t107901, t107919, t107924, t107930, t107988, t108009, t108030, t110177, t110717, t1113, t1940, t2071, t2403, t26425, t27793, t28291, t28472, t29953, t29964, t30420, t4541, t6416, t7207, t7428, t7432, t95976);
        let t110989 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2041::<F>(t102854, t102888, t107892, t107908, t107927, t107934, t107958, t107970, t110699, t110704, t1940, t2071, t2403, t26425, t26585, t26590, t27764, t27770, t27802, t27806, t28291, t28460, t29939, t29970, t30471, t33, t4541, t50080, t7428, t7869);
        let t111004 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2042::<F>(t33, t265, t502, t110840, t110883, t110920, t110954, t110989, t1469, t18281, t2085, t28578, t30503, t4186, t57, t5825, t606, t7468, t8059, dens_threshold, rho1, zeta_threshold);
    (t110853, t111004)
}
