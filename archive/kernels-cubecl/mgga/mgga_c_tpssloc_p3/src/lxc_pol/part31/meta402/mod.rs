//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta402 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1468;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1469;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1470;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1471;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1472;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1473;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1474;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1475;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1476;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1477;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta402<F: Float>(t1246: F, t19189: F, t19120: F, t493: F, t1243: F, t19045: F, t3612: F, t5011: F, t1755: F, t11881: F, t1201: F, t1244: F, t1247: F, t1249: F, t1729: F, t1758: F, t18572: F, t19166: F, t19170: F, t19174: F, t19176: F, t19180: F, t3604: F, t3610: F, t470: F, t494: F, t4964: F, t5064: F, t5073: F, t5076: F, t5086: F, t6168: F, t6257: F, t6265: F, t19164: F, t1241: F, t1235: F, t6150: F, t1760: F, t5088: F, t3598: F, t1251: F, t6267: F, t6243: F, t11606: F, t1238: F, t15820: F, t1761: F, t18287: F, t19121: F, t3487: F, t3593: F, t4945: F, t498: F, t5055: F, t5060: F, t6268: F, t225: F, t6151: F, t6153: F, t6239: F, t1720: F, t5052: F, t1751: F, t4940: F, t18571: F, t491: F, t1252: F, t14972: F, t14980: F, t15797: F, t5089: F, t6244: F, t1256: F, t18247: F, t18249: F, t18251: F, t18257: F, t18261: F, t18264: F, t18268: F, t18270: F, t18273: F, t18278: F, t18282: F, t18285: F, t18672: F, t18676: F, t18679: F, t18909: F, t18913: F, t193: F, t336: F, t4700: F, t5091: F, t5095: F, t3640: F, t6270: F, t11947: F, t6274: F, t1254: F, t18682: F, t18685: F, t18688: F, t18690: F, t18692: F, t18694: F, t18696: F, t18837: F, t18839: F, t18917: F, t18920: F, t18922: F, t18924: F, t18928: F, t18930: F, t18932: F, t18936: F, t18938: F, t28: F, t265: F, t504: F, t17133: F, t1081: F, t1260: F, t1409: F, t1649: F, t16558: F, t17141: F, t1768: F, t18196: F, t3966: F, t4324: F, t506: F, t5099: F, t52: F, t5398: F, t5669: F, t5966: F, t607: F, t6279: F, t873: F, dens_threshold: F, rho1: F, zeta_threshold: F, t18188: F, t12560: F, t12561: F, t12562: F, t12563: F, t12564: F, t12565: F, t9225: F, t5385: F, t604: F, t5389: F, t645: F, t1437: F, t4021: F, t5445: F, t65: F, t67: F, t1864: F, t5392: F, t628: F, t17635: F, t31: F, t5399: F, t1426: F, t3961: F, t3967: F, t1410: F, t3997: F, t1434: F, t3962: F, t5393: F, t5400: F, t5403: F, t642: F, t80: F) -> (F, F, F, F, F, F, F, F) {
        let t19207 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1468::<F>(t1246, t19189, t19120, t493, t1243, t19045, t3612, t5011, t1755, t11881, t1201, t1244, t1247, t1249, t1729, t1758, t18572, t19166, t19170, t19174, t19176, t19180, t3604, t3610, t470, t494, t4964, t5064, t5073, t5076, t5086, t6168, t6257, t6265);
        let (t19209, t19211, t19214, t19220, t19226) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1469::<F>(t19164, t19207, t1241, t1235, t6150, t1760, t5088, t3598, t1251, t6267, t6243, t11606);
        let t19231 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1470::<F>(t1238, t15820, t1761, t18287, t19121, t19209, t19211, t19214, t19220, t19226, t3487, t3593, t4945, t498, t5055, t5060, t6268);
        let t19261 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1471::<F>(t225, t6151, t6153, t6239, t1720, t5052, t1751, t4940, t18571, t491, t1252, t14972, t14980, t15797, t1761, t3487, t3593, t4945, t498, t5055, t5089, t6244);
        let t19266 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1472::<F>(t19231, t19261, t1256, t18247, t18249, t18251, t18257, t18261, t18264, t18268, t18270, t18273, t18278, t18282, t18285, t18672, t18676, t18679, t18909, t18913, t193, t336, t4700, t5091, t5095);
        let t19274 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1473::<F>(t3640, t6270, t11947, t6274, t1254, t18682, t18685, t18688, t18690, t18692, t18694, t18696, t18837, t18839, t18917, t18920, t18922, t18924, t18928, t18930, t18932, t18936, t18938, t4700);
        let t19288 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1474::<F>(t28, t265, t504, t17133, t19266, t19274, t1081, t1260, t1409, t1649, t16558, t17141, t1768, t18196, t3966, t4324, t506, t5099, t52, t5398, t5669, t5966, t607, t6279, t873, dens_threshold, rho1, zeta_threshold);
        let (t19289, t19297, t19299, t19310) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1475::<F>(t18188, t19288, t12560, t12561, t12562, t12563, t12564, t12565, t9225, t5385, t604, t5389, t645);
        let (t19313, t19318, t19322, t19323, t19326, t19331) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1476::<F>(t1437, t4021, t5445, t645, t1409, t65, t67, t1864, t3966, t5392, t628, t17635);
        let (t19334, t19356) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1477::<F>(t16558, t31, t65, t5399, t628, t1426, t3961, t3967, t1410, t3997, t1434, t19322, t19323, t19326, t19331, t3962, t5393, t5400, t5403, t642, t80);
    (t19289, t19297, t19299, t19310, t19313, t19318, t19334, t19356)
}
