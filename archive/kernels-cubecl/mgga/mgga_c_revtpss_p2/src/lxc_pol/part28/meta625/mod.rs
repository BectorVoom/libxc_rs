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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta625<F: Float>(t7105: F, t816: F, t15670: F, t1972: F, t4857: F, t7125: F, t25495: F, t4845: F, t15749: F, t7117: F, t25490: F, t15666: F, t27479: F, t3215: F, t1028: F, t15606: F, t15975: F, t27498: F, t27528: F, t27532: F, t3208: F, t93548: F, t93813: F, t25577: F, t4817: F, t15711: F, t7132: F, t15655: F, t16060: F, t7111: F, t25539: F, t4924: F, t1656: F, t1665: F, t1675: F, t3220: F, t4854: F, t4887: F, t93592: F, t93691: F, t93715: F, t93722: F, t16219: F, t139: F, t27526: F, t3252: F, t4574: F, t1014: F, t4579: F, t15130: F, t15135: F, t15140: F, t15145: F, t15149: F, t15154: F, t15651: F, t27527: F, t27531: F, t53321: F, t93736: F, t100004: F, t100035: F, t100058: F, t100085: F, t100109: F, t100133: F, t100163: F, t100187: F, t100216: F, t100233: F, t100254: F, t100282: F, t100310: F, t1089: F, t16183: F, t16295: F, t16314: F, t1983: F, t1984: F, t1985: F, t25461: F, t25634: F, t25651: F, t25687: F, t25692: F, t25699: F, t27415: F, t27423: F, t27579: F, t27580: F, t27616: F, t27621: F, t27634: F, t3325: F, t3326: F, t359: F, t4742: F, t4941: F, t5016: F, t7135: F, t7144: F, t7145: F, t7151: F, t7160: F, t7167: F, t7168: F, t7821: F, t7829: F, t93436: F, t93498: F, t94068: F, t999: F, t99953: F, t99969: F, t99970: F, t1035: F, t27543: F, t1043: F, t1096: F, t16249: F, t1696: F, t25464: F, t25605: F, t25611: F, t27647: F, t27651: F, t27664: F, t27680: F, t27688: F, t3270: F, t4758: F, t4764: F, t4975: F, t7102: F, t7817: F, t7818: F, t93509: F, t93901: F, t93904: F, t93959: F, t94023: F, t99877: F, t3151: F, t7828: F, t7150: F, t99708: F, t16255: F, t1647: F, t1652: F, t25476: F, t25591: F, t25640: F, t25662: F, t27445: F, t27550: F, t27606: F, t27656: F, t3076: F, t3133: F, t3304: F, t4947: F, t4976: F, t7140: F, t7153: F, t7833: F, t93464: F, t93528: F, t93884: F, t93963: F, t93983: F, t988: F, t99762: F, t11239: F, t1678: F, t1078: F, t1982: F, t16287: F, t16292: F, t16322: F, t1651: F, t25466: F, t25586: F, t25674: F, t25678: F, t27422: F, t27545: F, t27609: F, t27679: F, t3059: F, t4773: F, t7159: F, t93881: F, t93994: F, t989: F, t3143: F, t7810: F, t1977: F, t994: F, t11627: F, t99682: F, t12132: F, t15886: F, t16344: F, t16554: F, t16592: F, t16605: F, t1978: F, t25484: F, t25487: F, t25671: F, t27419: F, t27557: F, t27604: F, t27635: F, t27642: F, t27669: F, t3318: F, t4743: F, t4983: F, t7137: F, t7837: F, t93459: F, t99685: F, t99735: F, t16243: F, t16352: F, t1986: F, t25621: F, t25625: F, t25658: F, t27426: F, t27599: F, t27661: F, t27676: F, t27687: F, t3042: F, t3043: F, t3271: F, t7156: F, t7812: F, t94095: F, t11223: F, t7143: F, t3057: F, t11120: F, t11213: F, t25480: F, t25617: F, t27556: F, t27568: F, t27627: F, t27631: F, t27692: F, t27699: F, t27703: F, t3060: F, t3067: F, t3075: F, t4946: F, t93867: F, t93928: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t100315, t100321, t100324, t100327, t100329, t100332, t100334) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2223::<F>(t7105, t816, t15670, t1972, t4857, t7125, t25495, t4845, t15749, t7117, t25490, t15666);
        let t100337 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2224::<F>(t27479, t3215, t100315, t100321, t100324, t100327, t100329, t100332, t100334, t1028, t15606, t15975, t27498, t27528, t27532, t3208, t93548, t93813);
        let t100364 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2225::<F>(t25577, t4817, t15711, t7132, t15655, t1972, t16060, t7111, t25539, t4924, t1028, t1656, t1665, t1675, t25495, t27479, t3220, t4854, t4887, t93592, t93691, t93715, t93722);
        let t100399 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2226::<F>(t16219, t7111, t139, t27526, t3252, t4574, t1014, t4579, t15130, t15135, t15140, t15145, t15149, t15154, t15651, t1665, t25490, t27527, t27531, t4854, t53321, t7117, t93736);
        let t100403 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2227::<F>(t100004, t100035, t100058, t100085, t100109, t100133, t100163, t100187, t100216, t100233, t100254, t100282, t100310, t100337, t100364, t100399);
        let t100425 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2228::<F>(t100403, t1089, t16183, t16295, t16314, t1983, t1984, t1985, t25461, t25634, t25651, t25687, t25692, t25699, t27415, t27423, t27579, t27580, t27616, t27621, t27634, t3325, t3326, t359, t4742, t4941, t5016, t7135, t7144, t7145, t7151, t7160, t7167, t7168, t7821, t7829, t93436, t93498, t94068, t999, t99953, t99969, t99970);
        let t100471 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2229::<F>(t1035, t27543, t1043, t1089, t1096, t16249, t1696, t25461, t25464, t25605, t25611, t25692, t27415, t27579, t27647, t27651, t27664, t27680, t27688, t3270, t3325, t4758, t4764, t4975, t7102, t7144, t7151, t7160, t7167, t7817, t7818, t93509, t93901, t93904, t93959, t94023, t99877);
        let (t100490, t100513) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2230::<F>(t3151, t7828, t7150, t99708, t1089, t16255, t1647, t1652, t25476, t25591, t25605, t25611, t25634, t25640, t25662, t27445, t27550, t27579, t27606, t27656, t3076, t3133, t3304, t4758, t4947, t4976, t7140, t7145, t7153, t7833, t93464, t93528, t93884, t93963, t93983, t988, t99762, t99877);
        let t100560 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2231::<F>(t11239, t1678, t1078, t1982, t1096, t16287, t16292, t16322, t1651, t1652, t25464, t25466, t25586, t25591, t25651, t25674, t25678, t25692, t27422, t27545, t27609, t27679, t3059, t3325, t4773, t7102, t7140, t7144, t7145, t7151, t7159, t7160, t7817, t7821, t7828, t93881, t93994, t989, t999);
        let t100606 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2232::<F>(t3143, t7810, t1977, t994, t11627, t1983, t99682, t1089, t12132, t15886, t16344, t1652, t16554, t16592, t16605, t1978, t25461, t25476, t25484, t25487, t25651, t25671, t27419, t27557, t27604, t27635, t27642, t27669, t3133, t3151, t3304, t3318, t4743, t4983, t7137, t7140, t7167, t7837, t93459, t99685, t99735);
        let t100650 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2233::<F>(t1096, t16243, t16352, t1678, t1986, t25476, t25591, t25621, t25625, t25658, t25699, t27426, t27599, t27616, t27661, t27676, t27680, t27687, t3042, t3043, t3271, t5016, t7102, t7145, t7151, t7156, t7160, t7812, t7821, t94095, t988, t999);
        let (t100690, t100696) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2234::<F>(t11223, t7143, t3057, t7810, t11120, t994, t1096, t11213, t1696, t1985, t25464, t25480, t25617, t25640, t25699, t27419, t27556, t27568, t27609, t27627, t27631, t27692, t27699, t27703, t3060, t3067, t3075, t3270, t3326, t4946, t7145, t7151, t7159, t7160, t7818, t7821, t93867, t93928, t94095, t988);
    (t100403, t100425, t100471, t100490, t100513, t100560, t100606, t100650, t100690, t100696)
}
