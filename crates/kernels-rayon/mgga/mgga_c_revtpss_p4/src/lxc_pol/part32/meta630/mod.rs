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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta630(t30391: f64, t689: f64, t93314: f64, t93302: f64, t2718: f64, t7997: f64, t103212: f64, t103521: f64, t103529: f64, t103543: f64, t103547: f64, t106275: f64, t14587: f64, t1580: f64, t25383: f64, t26550: f64, t27353: f64, t28400: f64, t28425: f64, t30357: f64, t62604: f64, t62637: f64, t7420: f64, t7766: f64, t95945: f64, t95948: f64, t110242: f64, t110261: f64, t110281: f64, t110306: f64, t110330: f64, t110348: f64, t110365: f64, t110466: f64, t110499: f64, t110519: f64, t110551: f64, t110576: f64, t110607: f64, t110635: f64, t110665: f64, t892: f64, t198: f64, t205: f64, t8019: f64, t102854: f64, t105906: f64, t106534: f64, t106540: f64, t106546: f64, t106562: f64, t106590: f64, t106593: f64, t106606: f64, t1468: f64, t1940: f64, t26425: f64, t26585: f64, t26590: f64, t27160: f64, t28291: f64, t28456: f64, t28472: f64, t29599: f64, t29719: f64, t30: f64, t7432: f64, t7787: f64, t95511: f64, t106494: f64, t102888: f64, t106490: f64, t106498: f64, t106502: f64, t106520: f64, t106528: f64, t106572: f64, t106583: f64, t106602: f64, t2403: f64, t27166: f64, t27376: f64, t27387: f64, t27391: f64, t27395: f64, t28460: f64, t30420: f64, t605: f64, t8020: f64, t106554: f64, t106565: f64, t106610: f64, t107793: f64, t107805: f64, t18435: f64, t18498: f64, t18838: f64, t207: f64, t2071: f64, t27375: f64, t4541: f64, t5962: f64, t6075: f64, t7428: f64, t77408: f64, t77425: f64, t77441: f64, t775: f64, t95964: f64, t103586: f64, t105923: f64, t106561: f64, t106625: f64, t110177: f64, t1544: f64, t1583: f64, t18392: f64, t18875: f64, t27384: f64, t29598: f64, t30439: f64, t4343: f64, t4433: f64, t4537: f64, t50080: f64, t5966: f64, t6079: f64, t890: f64, t95976: f64, t265: f64, t393: f64, t110158: f64, t110196: f64, t1469: f64, t18281: f64, t2078: f64, t28523: f64, t30463: f64, t4186: f64, t45: f64, t5825: f64, t606: f64, t7449: f64, t8040: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t107974: f64, t108002: f64, t108005: f64, t108021: f64, t108033: f64, t108043: f64, t110150: f64, t110154: f64, t110165: f64, t1711: f64, t20256: f64, t27800: f64, t29946: f64, t29967: f64, t7862: f64, t107882: f64, t107885: f64, t107895: f64, t107939: f64, t107943: f64, t107947: f64, t107985: f64, t108028: f64, t108036: f64, t27773: f64, t27777: f64, t27810: f64, t27817: f64, t29949: f64, t7200: f64, t107901: f64, t107919: f64, t107924: f64, t107930: f64, t107988: f64, t108009: f64, t108030: f64, t1113: f64, t27793: f64, t29953: f64, t29964: f64, t6416: f64, t7207: f64, t107892: f64, t107908: f64, t107927: f64, t107934: f64, t107958: f64, t107970: f64, t27764: f64, t27770: f64, t27802: f64, t27806: f64, t29939: f64, t29970: f64, t30471: f64, t33: f64, t7869: f64, t502: f64, t2085: f64, t28578: f64, t30503: f64, t57: f64, t7468: f64, t8059: f64, rho1: f64) -> (f64, f64) {
        let t110694 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2031(t30391, t689, t93314, t93302, t2718, t7997, t103212, t103521, t103529, t103543, t103547, t106275, t14587, t1580, t25383, t26550, t27353, t28400, t28425, t30357, t62604, t62637, t7420, t7766, t95945, t95948);
        let t110698 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2032(t110242, t110261, t110281, t110306, t110330, t110348, t110365, t110466, t110499, t110519, t110551, t110576, t110607, t110635, t110665, t110694);
        let (t110699, t110704, t110711) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2033(t110698, t892, t198, t205, t8019, t102854, t105906, t106534, t106540, t106546, t106562, t106590, t106593, t106606, t1468, t1940, t26425, t26585, t26590, t27160, t28291, t28456, t28472, t29599, t29719, t30, t7432, t7787, t95511);
        let (t110717, t110745) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2034(t106494, t26425, t102888, t106490, t106498, t106502, t106520, t106528, t106572, t106583, t106602, t1940, t2403, t27166, t27376, t27387, t27391, t27395, t28291, t28460, t30420, t605, t7432, t8020);
        let t110792 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2035(t106554, t106565, t106610, t107793, t107805, t110698, t18435, t18498, t18838, t1940, t198, t207, t2071, t2403, t26425, t26585, t26590, t27375, t28291, t28460, t30420, t4541, t5962, t6075, t7428, t7432, t77408, t77425, t77441, t775, t892, t95964);
        let t110839 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2036(t102854, t103586, t105923, t106561, t106625, t110177, t1544, t1583, t18392, t18875, t1940, t2071, t2403, t26585, t26590, t27384, t28456, t28460, t29598, t30439, t4343, t4433, t4537, t4541, t50080, t5966, t6079, t7428, t7432, t8020, t890, t95976);
        let (t110840, t110853) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2037(t30, t265, t393, t110792, t110839, t110158, t110196, t110711, t110745, t1469, t18281, t2078, t28523, t30463, t4186, t45, t5825, t606, t7449, t8040, dens_threshold, rho0, zeta_threshold);
        let t110883 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2038(t107974, t108002, t108005, t108021, t108033, t108043, t110150, t110154, t110165, t1711, t1940, t20256, t2071, t2403, t26425, t26585, t27800, t28291, t28456, t29946, t29967, t7432, t7862, t95511);
        let t110920 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2039(t107882, t107885, t107895, t107939, t107943, t107947, t107985, t108028, t108036, t1940, t2071, t2403, t26425, t27773, t27777, t27810, t27817, t28460, t28472, t29949, t30420, t7200, t7428, t8020);
        let t110954 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2040(t102888, t107901, t107919, t107924, t107930, t107988, t108009, t108030, t110177, t110717, t1113, t1940, t2071, t2403, t26425, t27793, t28291, t28472, t29953, t29964, t30420, t4541, t6416, t7207, t7428, t7432, t95976);
        let t110989 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2041(t102854, t102888, t107892, t107908, t107927, t107934, t107958, t107970, t110699, t110704, t1940, t2071, t2403, t26425, t26585, t26590, t27764, t27770, t27802, t27806, t28291, t28460, t29939, t29970, t30471, t33, t4541, t50080, t7428, t7869);
        let t111004 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2042(t33, t265, t502, t110840, t110883, t110920, t110954, t110989, t1469, t18281, t2085, t28578, t30503, t4186, t57, t5825, t606, t7468, t8059, dens_threshold, rho1, zeta_threshold);
    (t110853, t111004)
}
