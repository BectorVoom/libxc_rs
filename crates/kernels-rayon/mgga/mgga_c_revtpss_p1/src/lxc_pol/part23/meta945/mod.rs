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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta945(t81509: f64, t81511: f64, t81514: f64, t81516: f64, t81518: f64, t81521: f64, t81523: f64, t81525: f64, t81527: f64, t81530: f64, t81533: f64, t81536: f64, t58226: f64, t68454: f64, t68456: f64, t68538: f64, t68540: f64, t68548: f64, t68550: f64, t68567: f64, t68583: f64, t68585: f64, t68590: f64, t81539: f64, t1161: f64, t1169: f64, t17089: f64, t1757: f64, t20521: f64, t20526: f64, t24331: f64, t24363: f64, t24366: f64, t3447: f64, t45080: f64, t45197: f64, t5120: f64, t5181: f64, t58317: f64, t6506: f64, t6535: f64, t69354: f64, t81128: f64, t81130: f64, t81132: f64, t81134: f64, t81136: f64, t81138: f64, t81678: f64, t81691: f64, t81705: f64, t81717: f64, t81729: f64, t81740: f64, t1160: f64, t24453: f64, t1170: f64, t12423: f64, t12481: f64, t24411: f64, t24431: f64, t24436: f64, t45174: f64, t58307: f64, t58336: f64, t6487: f64, t6519: f64, t81148: f64, t81150: f64, t81152: f64, t81252: f64, t81307: f64, t81352: f64, t81558: f64, t81560: f64, t81562: f64, t1168: f64, t1187: f64, t12470: f64, t12486: f64, t12491: f64, t16965: f64, t17097: f64, t17154: f64, t1756: f64, t20382: f64, t20615: f64, t20659: f64, t20662: f64, t20671: f64, t20672: f64, t24414: f64, t24423: f64, t3452: f64, t3496: f64, t3521: f64, t5180: f64, t58237: f64, t58259: f64, t6502: f64, t6534: f64, t69504: f64, t81566: f64, t24362: f64, t3479: f64, t24407: f64, t3523: f64, t12553: f64, t20625: f64, t20665: f64, t20668: f64, t20675: f64, t20679: f64, t24330: f64, t24376: f64, t24408: f64, t3477: f64, t45157: f64, t45159: f64, t45177: f64, t5142: f64, t5163: f64, t5185: f64, t58247: f64, t58262: f64, t6538: f64, t69359: f64, t69371: f64, t43771: f64, t43814: f64, t43817: f64, t68255: f64, t68257: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81399: f64, t81401: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64, t81416: f64, t68262: f64, t68277: f64, t68312: f64, t68332: f64, t68334: f64, t68336: f64, t68368: f64, t68370: f64, t81423: f64, t81425: f64, t81427: f64, t81429: f64, t56176: f64, t81439: f64, t81442: f64, t81445: f64, t81448: f64, t81451: f64, t81454: f64, t81457: f64, t81460: f64, t81463: f64, t81466: f64, t81469: f64, t56183: f64, t56236: f64, t58404: f64, t68389: f64, t68399: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t43888: f64, t58153: f64, t58165: f64, t58411: f64, t81242: f64, t81245: f64, t81489: f64, t81491: f64, t81494: f64, t81496: f64, t81499: f64, t81501: f64, t58452: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t81754 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3104(t81509, t81511, t81514, t81516, t81518, t81521, t81523, t81525, t81527, t81530, t81533, t81536);
        let t81766 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3105(t58226, t68454, t68456, t68538, t68540, t68548, t68550, t68567, t68583, t68585, t68590, t81539);
        let t81781 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3106(t1161, t1169, t17089, t1757, t20521, t20526, t24331, t24363, t24366, t3447, t45080, t45197, t5120, t5181, t58317, t6506, t6535, t69354, t81128, t81130, t81132, t81134, t81136, t81138, t81678, t81691, t81705, t81717, t81729, t81740, t81754, t81766);
        let t81796 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3107(t1160, t24453, t1170, t12423, t12481, t24411, t24431, t24436, t45174, t58307, t58336, t6487, t6519, t81148, t81150, t81152, t81252, t81307, t81352, t81558, t81560, t81562);
        let t81835 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3108(t1168, t1187, t12470, t12481, t12486, t12491, t16965, t17097, t17154, t1756, t1757, t20382, t20615, t20659, t20662, t20671, t20672, t24363, t24414, t24423, t24436, t3452, t3496, t3521, t5180, t5181, t58237, t58259, t6502, t6519, t6534, t69504, t81566);
        let t81877 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3109(t24362, t3479, t24407, t3523, t1168, t1187, t12470, t12486, t12553, t17097, t17154, t20625, t20665, t20668, t20672, t20675, t20679, t24330, t24376, t24408, t24411, t3477, t3496, t3521, t45157, t45159, t45177, t5142, t5163, t5180, t5185, t58247, t58262, t6538, t69359, t69371);
        let (t81904, t81917) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3110(t43771, t43814, t43817, t68255, t68257, t81156, t81158, t81162, t81167, t81399, t81401, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214, t81416);
        let t81931 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3111(t68262, t68277, t68312, t68332, t68334, t68336, t68368, t68370, t81423, t81425, t81427, t81429);
        let t81944 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3112(t56176, t81439, t81442, t81445, t81448, t81451, t81454, t81457, t81460, t81463, t81466, t81469);
        let (t81957, t81969) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3113(t56183, t56236, t58404, t68389, t68399, t81224, t81228, t81230, t81232, t81234, t81236, t43888, t58153, t58165, t58411, t81242, t81245, t81489, t81491, t81494, t81496, t81499, t81501);
        let t81983 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3114(t81509, t81511, t81514, t81516, t81518, t81521, t81523, t81525, t81527, t81530, t81533, t81536);
        let t81995 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3115(t58452, t68454, t68456, t68538, t68540, t68548, t68550, t68567, t68583, t68585, t68590, t81539);
    (t81781, t81796, t81835, t81877, t81904, t81917, t81931, t81944, t81957, t81969, t81983, t81995)
}
