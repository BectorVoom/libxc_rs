//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta945 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3104;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3105;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3106;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3107;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3108;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3109;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3110;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3111;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3112;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3113;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3114;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3115;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta945<F: Float>(t81509: F, t81511: F, t81514: F, t81516: F, t81518: F, t81521: F, t81523: F, t81525: F, t81527: F, t81530: F, t81533: F, t81536: F, t58226: F, t68454: F, t68456: F, t68538: F, t68540: F, t68548: F, t68550: F, t68567: F, t68583: F, t68585: F, t68590: F, t81539: F, t1161: F, t1169: F, t17089: F, t1757: F, t20521: F, t20526: F, t24331: F, t24363: F, t24366: F, t3447: F, t45080: F, t45197: F, t5120: F, t5181: F, t58317: F, t6506: F, t6535: F, t69354: F, t81128: F, t81130: F, t81132: F, t81134: F, t81136: F, t81138: F, t81678: F, t81691: F, t81705: F, t81717: F, t81729: F, t81740: F, t1160: F, t24453: F, t1170: F, t12423: F, t12481: F, t24411: F, t24431: F, t24436: F, t45174: F, t58307: F, t58336: F, t6487: F, t6519: F, t81148: F, t81150: F, t81152: F, t81252: F, t81307: F, t81352: F, t81558: F, t81560: F, t81562: F, t1168: F, t1187: F, t12470: F, t12486: F, t12491: F, t16965: F, t17097: F, t17154: F, t1756: F, t20382: F, t20615: F, t20659: F, t20662: F, t20671: F, t20672: F, t24414: F, t24423: F, t3452: F, t3496: F, t3521: F, t5180: F, t58237: F, t58259: F, t6502: F, t6534: F, t69504: F, t81566: F, t24362: F, t3479: F, t24407: F, t3523: F, t12553: F, t20625: F, t20665: F, t20668: F, t20675: F, t20679: F, t24330: F, t24376: F, t24408: F, t3477: F, t45157: F, t45159: F, t45177: F, t5142: F, t5163: F, t5185: F, t58247: F, t58262: F, t6538: F, t69359: F, t69371: F, t43771: F, t43814: F, t43817: F, t68255: F, t68257: F, t81156: F, t81158: F, t81162: F, t81167: F, t81399: F, t81401: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F, t81416: F, t68262: F, t68277: F, t68312: F, t68332: F, t68334: F, t68336: F, t68368: F, t68370: F, t81423: F, t81425: F, t81427: F, t81429: F, t56176: F, t81439: F, t81442: F, t81445: F, t81448: F, t81451: F, t81454: F, t81457: F, t81460: F, t81463: F, t81466: F, t81469: F, t56183: F, t56236: F, t58404: F, t68389: F, t68399: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t43888: F, t58153: F, t58165: F, t58411: F, t81242: F, t81245: F, t81489: F, t81491: F, t81494: F, t81496: F, t81499: F, t81501: F, t58452: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t81754 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3104::<F>(t81509, t81511, t81514, t81516, t81518, t81521, t81523, t81525, t81527, t81530, t81533, t81536);
        let t81766 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3105::<F>(t58226, t68454, t68456, t68538, t68540, t68548, t68550, t68567, t68583, t68585, t68590, t81539);
        let t81781 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3106::<F>(t1161, t1169, t17089, t1757, t20521, t20526, t24331, t24363, t24366, t3447, t45080, t45197, t5120, t5181, t58317, t6506, t6535, t69354, t81128, t81130, t81132, t81134, t81136, t81138, t81678, t81691, t81705, t81717, t81729, t81740, t81754, t81766);
        let t81796 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3107::<F>(t1160, t24453, t1170, t12423, t12481, t24411, t24431, t24436, t45174, t58307, t58336, t6487, t6519, t81148, t81150, t81152, t81252, t81307, t81352, t81558, t81560, t81562);
        let t81835 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3108::<F>(t1168, t1187, t12470, t12481, t12486, t12491, t16965, t17097, t17154, t1756, t1757, t20382, t20615, t20659, t20662, t20671, t20672, t24363, t24414, t24423, t24436, t3452, t3496, t3521, t5180, t5181, t58237, t58259, t6502, t6519, t6534, t69504, t81566);
        let t81877 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3109::<F>(t24362, t3479, t24407, t3523, t1168, t1187, t12470, t12486, t12553, t17097, t17154, t20625, t20665, t20668, t20672, t20675, t20679, t24330, t24376, t24408, t24411, t3477, t3496, t3521, t45157, t45159, t45177, t5142, t5163, t5180, t5185, t58247, t58262, t6538, t69359, t69371);
        let (t81904, t81917) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3110::<F>(t43771, t43814, t43817, t68255, t68257, t81156, t81158, t81162, t81167, t81399, t81401, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214, t81416);
        let t81931 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3111::<F>(t68262, t68277, t68312, t68332, t68334, t68336, t68368, t68370, t81423, t81425, t81427, t81429);
        let t81944 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3112::<F>(t56176, t81439, t81442, t81445, t81448, t81451, t81454, t81457, t81460, t81463, t81466, t81469);
        let (t81957, t81969) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3113::<F>(t56183, t56236, t58404, t68389, t68399, t81224, t81228, t81230, t81232, t81234, t81236, t43888, t58153, t58165, t58411, t81242, t81245, t81489, t81491, t81494, t81496, t81499, t81501);
        let t81983 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3114::<F>(t81509, t81511, t81514, t81516, t81518, t81521, t81523, t81525, t81527, t81530, t81533, t81536);
        let t81995 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3115::<F>(t58452, t68454, t68456, t68538, t68540, t68548, t68550, t68567, t68583, t68585, t68590, t81539);
    (t81781, t81796, t81835, t81877, t81904, t81917, t81931, t81944, t81957, t81969, t81983, t81995)
}
