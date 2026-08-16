//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta625 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2223;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2224;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2225;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2226;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2227;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2228;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2229;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2230;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2231;
use chunk9::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2232;
use chunk10::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2233;
use chunk11::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2234;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta625(t7105: f64, t816: f64, t15670: f64, t1972: f64, t4857: f64, t7125: f64, t25495: f64, t4845: f64, t15749: f64, t7117: f64, t25490: f64, t15666: f64, t27479: f64, t3215: f64, t1028: f64, t15606: f64, t15975: f64, t27498: f64, t27528: f64, t27532: f64, t3208: f64, t93548: f64, t93813: f64, t25577: f64, t4817: f64, t15711: f64, t7132: f64, t15655: f64, t16060: f64, t7111: f64, t25539: f64, t4924: f64, t1656: f64, t1665: f64, t1675: f64, t3220: f64, t4854: f64, t4887: f64, t93592: f64, t93691: f64, t93715: f64, t93722: f64, t16219: f64, t139: f64, t27526: f64, t3252: f64, t4574: f64, t1014: f64, t4579: f64, t15130: f64, t15135: f64, t15140: f64, t15145: f64, t15149: f64, t15154: f64, t15651: f64, t27527: f64, t27531: f64, t53321: f64, t93736: f64, t100004: f64, t100035: f64, t100058: f64, t100085: f64, t100109: f64, t100133: f64, t100163: f64, t100187: f64, t100216: f64, t100233: f64, t100254: f64, t100282: f64, t100310: f64, t1089: f64, t16183: f64, t16295: f64, t16314: f64, t1983: f64, t1984: f64, t1985: f64, t25461: f64, t25634: f64, t25651: f64, t25687: f64, t25692: f64, t25699: f64, t27415: f64, t27423: f64, t27579: f64, t27580: f64, t27616: f64, t27621: f64, t27634: f64, t3325: f64, t3326: f64, t359: f64, t4742: f64, t4941: f64, t5016: f64, t7135: f64, t7144: f64, t7145: f64, t7151: f64, t7160: f64, t7167: f64, t7168: f64, t7821: f64, t7829: f64, t93436: f64, t93498: f64, t94068: f64, t999: f64, t99953: f64, t99969: f64, t99970: f64, t1035: f64, t27543: f64, t1043: f64, t1096: f64, t16249: f64, t1696: f64, t25464: f64, t25605: f64, t25611: f64, t27647: f64, t27651: f64, t27664: f64, t27680: f64, t27688: f64, t3270: f64, t4758: f64, t4764: f64, t4975: f64, t7102: f64, t7817: f64, t7818: f64, t93509: f64, t93901: f64, t93904: f64, t93959: f64, t94023: f64, t99877: f64, t3151: f64, t7828: f64, t7150: f64, t99708: f64, t16255: f64, t1647: f64, t1652: f64, t25476: f64, t25591: f64, t25640: f64, t25662: f64, t27445: f64, t27550: f64, t27606: f64, t27656: f64, t3076: f64, t3133: f64, t3304: f64, t4947: f64, t4976: f64, t7140: f64, t7153: f64, t7833: f64, t93464: f64, t93528: f64, t93884: f64, t93963: f64, t93983: f64, t988: f64, t99762: f64, t11239: f64, t1678: f64, t1078: f64, t1982: f64, t16287: f64, t16292: f64, t16322: f64, t1651: f64, t25466: f64, t25586: f64, t25674: f64, t25678: f64, t27422: f64, t27545: f64, t27609: f64, t27679: f64, t3059: f64, t4773: f64, t7159: f64, t93881: f64, t93994: f64, t989: f64, t3143: f64, t7810: f64, t1977: f64, t994: f64, t11627: f64, t99682: f64, t12132: f64, t15886: f64, t16344: f64, t16554: f64, t16592: f64, t16605: f64, t1978: f64, t25484: f64, t25487: f64, t25671: f64, t27419: f64, t27557: f64, t27604: f64, t27635: f64, t27642: f64, t27669: f64, t3318: f64, t4743: f64, t4983: f64, t7137: f64, t7837: f64, t93459: f64, t99685: f64, t99735: f64, t16243: f64, t16352: f64, t1986: f64, t25621: f64, t25625: f64, t25658: f64, t27426: f64, t27599: f64, t27661: f64, t27676: f64, t27687: f64, t3042: f64, t3043: f64, t3271: f64, t7156: f64, t7812: f64, t94095: f64, t11223: f64, t7143: f64, t3057: f64, t11120: f64, t11213: f64, t25480: f64, t25617: f64, t27556: f64, t27568: f64, t27627: f64, t27631: f64, t27692: f64, t27699: f64, t27703: f64, t3060: f64, t3067: f64, t3075: f64, t4946: f64, t93867: f64, t93928: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t100315, t100321, t100324, t100327, t100329, t100332, t100334) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2223(t7105, t816, t15670, t1972, t4857, t7125, t25495, t4845, t15749, t7117, t25490, t15666);
        let t100337 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2224(t27479, t3215, t100315, t100321, t100324, t100327, t100329, t100332, t100334, t1028, t15606, t15975, t27498, t27528, t27532, t3208, t93548, t93813);
        let t100364 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2225(t25577, t4817, t15711, t7132, t15655, t1972, t16060, t7111, t25539, t4924, t1028, t1656, t1665, t1675, t25495, t27479, t3220, t4854, t4887, t93592, t93691, t93715, t93722);
        let t100399 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2226(t16219, t7111, t139, t27526, t3252, t4574, t1014, t4579, t15130, t15135, t15140, t15145, t15149, t15154, t15651, t1665, t25490, t27527, t27531, t4854, t53321, t7117, t93736);
        let t100403 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2227(t100004, t100035, t100058, t100085, t100109, t100133, t100163, t100187, t100216, t100233, t100254, t100282, t100310, t100337, t100364, t100399);
        let t100425 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2228(t100403, t1089, t16183, t16295, t16314, t1983, t1984, t1985, t25461, t25634, t25651, t25687, t25692, t25699, t27415, t27423, t27579, t27580, t27616, t27621, t27634, t3325, t3326, t359, t4742, t4941, t5016, t7135, t7144, t7145, t7151, t7160, t7167, t7168, t7821, t7829, t93436, t93498, t94068, t999, t99953, t99969, t99970);
        let t100471 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2229(t1035, t27543, t1043, t1089, t1096, t16249, t1696, t25461, t25464, t25605, t25611, t25692, t27415, t27579, t27647, t27651, t27664, t27680, t27688, t3270, t3325, t4758, t4764, t4975, t7102, t7144, t7151, t7160, t7167, t7817, t7818, t93509, t93901, t93904, t93959, t94023, t99877);
        let (t100490, t100513) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2230(t3151, t7828, t7150, t99708, t1089, t16255, t1647, t1652, t25476, t25591, t25605, t25611, t25634, t25640, t25662, t27445, t27550, t27579, t27606, t27656, t3076, t3133, t3304, t4758, t4947, t4976, t7140, t7145, t7153, t7833, t93464, t93528, t93884, t93963, t93983, t988, t99762, t99877);
        let t100560 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2231(t11239, t1678, t1078, t1982, t1096, t16287, t16292, t16322, t1651, t1652, t25464, t25466, t25586, t25591, t25651, t25674, t25678, t25692, t27422, t27545, t27609, t27679, t3059, t3325, t4773, t7102, t7140, t7144, t7145, t7151, t7159, t7160, t7817, t7821, t7828, t93881, t93994, t989, t999);
        let t100606 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2232(t3143, t7810, t1977, t994, t11627, t1983, t99682, t1089, t12132, t15886, t16344, t1652, t16554, t16592, t16605, t1978, t25461, t25476, t25484, t25487, t25651, t25671, t27419, t27557, t27604, t27635, t27642, t27669, t3133, t3151, t3304, t3318, t4743, t4983, t7137, t7140, t7167, t7837, t93459, t99685, t99735);
        let t100650 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2233(t1096, t16243, t16352, t1678, t1986, t25476, t25591, t25621, t25625, t25658, t25699, t27426, t27599, t27616, t27661, t27676, t27680, t27687, t3042, t3043, t3271, t5016, t7102, t7145, t7151, t7156, t7160, t7812, t7821, t94095, t988, t999);
        let (t100690, t100696) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2234(t11223, t7143, t3057, t7810, t11120, t994, t1096, t11213, t1696, t1985, t25464, t25480, t25617, t25640, t25699, t27419, t27556, t27568, t27609, t27627, t27631, t27692, t27699, t27703, t3060, t3067, t3075, t3270, t3326, t4946, t7145, t7151, t7159, t7160, t7818, t7821, t93867, t93928, t94095, t988);
    (t100403, t100425, t100471, t100490, t100513, t100560, t100606, t100650, t100690, t100696)
}
