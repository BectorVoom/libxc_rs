//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta422 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1621;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1622;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1623;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1624;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1625;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1626;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1627;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1628;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1629;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1630;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta422(t1246: f64, t19189: f64, t19120: f64, t493: f64, t1243: f64, t19045: f64, t3612: f64, t5011: f64, t1755: f64, t11881: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t1729: f64, t1758: f64, t18572: f64, t19166: f64, t19170: f64, t19174: f64, t19176: f64, t19180: f64, t3604: f64, t3610: f64, t470: f64, t494: f64, t4964: f64, t5064: f64, t5073: f64, t5076: f64, t5086: f64, t6168: f64, t6257: f64, t6265: f64, t19164: f64, t1241: f64, t1235: f64, t6150: f64, t1760: f64, t5088: f64, t3598: f64, t1251: f64, t6267: f64, t6243: f64, t11606: f64, t1238: f64, t15820: f64, t1761: f64, t18287: f64, t19121: f64, t3487: f64, t3593: f64, t4945: f64, t498: f64, t5055: f64, t5060: f64, t6268: f64, t225: f64, t6151: f64, t6153: f64, t6239: f64, t1720: f64, t5052: f64, t1751: f64, t4940: f64, t18571: f64, t491: f64, t1252: f64, t14972: f64, t14980: f64, t15797: f64, t5089: f64, t6244: f64, t1256: f64, t18247: f64, t18249: f64, t18251: f64, t18257: f64, t18261: f64, t18264: f64, t18268: f64, t18270: f64, t18273: f64, t18278: f64, t18282: f64, t18285: f64, t18672: f64, t18676: f64, t18679: f64, t18909: f64, t18913: f64, t193: f64, t336: f64, t4700: f64, t5091: f64, t5095: f64, t3640: f64, t6270: f64, t11947: f64, t6274: f64, t1254: f64, t18682: f64, t18685: f64, t18688: f64, t18690: f64, t18692: f64, t18694: f64, t18696: f64, t18837: f64, t18839: f64, t18917: f64, t18920: f64, t18922: f64, t18924: f64, t18928: f64, t18930: f64, t18932: f64, t18936: f64, t18938: f64, t28: f64, t265: f64, t504: f64, t17133: f64, t1081: f64, t1260: f64, t1409: f64, t1649: f64, t16558: f64, t17141: f64, t1768: f64, t18196: f64, t3966: f64, t4324: f64, t506: f64, t5099: f64, t52: f64, t5398: f64, t5669: f64, t5966: f64, t607: f64, t6279: f64, t873: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t18188: f64, t12560: f64, t12561: f64, t12562: f64, t12563: f64, t12564: f64, t12565: f64, t9225: f64, t5385: f64, t604: f64, t5389: f64, t645: f64, t1437: f64, t4021: f64, t5445: f64, t65: f64, t67: f64, t1864: f64, t5392: f64, t628: f64, t17635: f64, t31: f64, t5399: f64, t1426: f64, t3961: f64, t3967: f64, t1410: f64, t3997: f64, t1434: f64, t3962: f64, t5393: f64, t5400: f64, t5403: f64, t642: f64, t80: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t19207 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1621(t1246, t19189, t19120, t493, t1243, t19045, t3612, t5011, t1755, t11881, t1201, t1244, t1247, t1249, t1729, t1758, t18572, t19166, t19170, t19174, t19176, t19180, t3604, t3610, t470, t494, t4964, t5064, t5073, t5076, t5086, t6168, t6257, t6265);
        let (t19209, t19211, t19214, t19220, t19226) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1622(t19164, t19207, t1241, t1235, t6150, t1760, t5088, t3598, t1251, t6267, t6243, t11606);
        let t19231 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1623(t1238, t15820, t1761, t18287, t19121, t19209, t19211, t19214, t19220, t19226, t3487, t3593, t4945, t498, t5055, t5060, t6268);
        let t19261 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1624(t225, t6151, t6153, t6239, t1720, t5052, t1751, t4940, t18571, t491, t1252, t14972, t14980, t15797, t1761, t3487, t3593, t4945, t498, t5055, t5089, t6244);
        let t19266 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1625(t19231, t19261, t1256, t18247, t18249, t18251, t18257, t18261, t18264, t18268, t18270, t18273, t18278, t18282, t18285, t18672, t18676, t18679, t18909, t18913, t193, t336, t4700, t5091, t5095);
        let t19274 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1626(t3640, t6270, t11947, t6274, t1254, t18682, t18685, t18688, t18690, t18692, t18694, t18696, t18837, t18839, t18917, t18920, t18922, t18924, t18928, t18930, t18932, t18936, t18938, t4700);
        let t19288 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1627(t28, t265, t504, t17133, t19266, t19274, t1081, t1260, t1409, t1649, t16558, t17141, t1768, t18196, t3966, t4324, t506, t5099, t52, t5398, t5669, t5966, t607, t6279, t873, dens_threshold, rho1, zeta_threshold);
        let (t19289, t19297, t19299, t19310) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1628(t18188, t19288, t12560, t12561, t12562, t12563, t12564, t12565, t9225, t5385, t604, t5389, t645);
        let (t19313, t19318, t19322, t19323, t19326, t19331) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1629(t1437, t4021, t5445, t645, t1409, t65, t67, t1864, t3966, t5392, t628, t17635);
        let (t19334, t19356) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1630(t16558, t31, t65, t5399, t628, t1426, t3961, t3967, t1410, t3997, t1434, t19322, t19323, t19326, t19331, t3962, t5393, t5400, t5403, t642, t80);
    (t19289, t19297, t19299, t19310, t19313, t19318, t19334, t19356)
}
