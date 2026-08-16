//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta392 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1481;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1482;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1483;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1484;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1485;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1486;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1487;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1488;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1489;
use chunk9::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta392(t11814: f64, t3572: f64, t11825: f64, t3523: f64, t11820: f64, t3536: f64, t11778: f64, t121: f64, t11148: f64, t1227: f64, t248: f64, t11728: f64, t11729: f64, t3570: f64, t1229: f64, t204: f64, t1090: f64, t11692: f64, t1174: f64, t1177: f64, t11779: f64, t11781: f64, t1213: f64, t1214: f64, t1216: f64, t3490: f64, t3515: f64, t3527: f64, t3578: f64, t3585: f64, t43719: f64, t43752: f64, t43792: f64, t43796: f64, t44668: f64, t44798: f64, t45250: f64, t45251: f64, t45256: f64, t45260: f64, t475: f64, t44878: f64, t44943: f64, t44999: f64, t45066: f64, t45133: f64, t45186: f64, t45246: f64, t3609: f64, t44927: f64, t3623: f64, t11880: f64, t44690: f64, t11913: f64, t11881: f64, t11883: f64, t11884: f64, t11897: f64, t11916: f64, t1244: f64, t1246: f64, t3604: f64, t3610: f64, t3612: f64, t3613: f64, t3626: f64, t44669: f64, t44673: f64, t44700: f64, t44710: f64, t44785: f64, t44786: f64, t470: f64, t491: f64, t493: f64, t11598: f64, t11608: f64, t11613: f64, t11616: f64, t11620: f64, t11621: f64, t11625: f64, t11639: f64, t11640: f64, t11868: f64, t11869: f64, t11872: f64, t11877: f64, t11888: f64, t11889: f64, t11890: f64, t11904: f64, t11907: f64, t11910: f64, t11914: f64, t11918: f64, t11919: f64, t11925: f64, t11928: f64, t11935: f64, t1201: f64, t1215: f64, t1235: f64, t1238: f64, t1241: f64, t1249: f64, t1251: f64, t1252: f64, t15429: f64, t3481: f64, t3487: f64, t3493: f64, t3565: f64, t3590: f64, t3593: f64, t3598: f64, t3600: f64, t3620: f64, t3621: f64, t3624: f64, t3625: f64, t3628: f64, t3631: f64, t44412: f64, t44657: f64, t44662: f64, t44691: f64, t44698: f64, t44701: f64, t44748: f64, t44753: f64, t44754: f64, t44774: f64, t494: f64, t498: f64, t5079: f64, t11931: f64, t225: f64, t11604: f64, t496: f64, t68: f64, t3599: f64, t11601: f64, t11599: f64, t11606: f64, t1190: f64, t3630: f64, t466: f64, t1256: f64, t193: f64, t336: f64, t3640: f64, t44161: f64, t44164: f64, t44167: f64, t44358: f64, t44375: f64, t44377: f64, t44378: f64, t44384: f64, t44388: f64, t44392: f64, t44396: f64, t44400: f64, t28: f64, t265: f64, t504: f64, t41606: f64, t43920: f64, t43990: f64, t44373: f64, t10150: f64, t1081: f64, t11122: f64, t11957: f64, t1260: f64, t2250: f64, t2756: f64, t3231: f64, t3644: f64, t39110: f64, t39448: f64, t506: f64, t52: f64, t607: f64, t873: f64, t9258: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t113: f64, t12492: f64, t12507: f64, t1266: f64, t1271: f64, t12734: f64, t1393: f64, t2314: f64, t2320: f64, t2364: f64, t3652: f64, t3660: f64, t39223: f64, t39231: f64, t39235: f64, t3929: f64, t39332: f64, t39385: f64, t39480: f64, t39524: f64, t39586: f64, t39626: f64, t39847: f64, t40615: f64, t43657: f64, t510: f64, t513: f64, t672: f64, t89: f64, t9347: f64, t9351: f64, t9419: f64, t2311: f64, t2319: f64, t107: f64, t9576: f64, t2585: f64, t667: f64, t2281: f64, t2333: f64, t2359: f64, t626: f64, t9367: f64, t9371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45262, t45264, t45266, t45271, t45283) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1481(t11814, t3572, t11825, t3523, t11820, t3536, t11778, t121, t11148, t1227, t248, t11728, t11729, t3570);
        let t45311 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1482(t1229, t204, t1090, t1227, t248, t11692, t1174, t1177, t11779, t11781, t11825, t1213, t1214, t1216, t3490, t3515, t3527, t3578, t3585, t43719, t43752, t43792, t43796, t44668, t44798, t45250, t45251, t45256, t45260, t45262, t45264, t45266, t45271, t45283, t475);
        let (t45314, t45320, t45323, t45326) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1483(t44878, t44943, t44999, t45066, t45133, t45186, t45246, t45311, t3609, t44927, t3623, t11880, t44690);
        let t45332 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1484(t11913, t44690, t11881, t11883, t11884, t11897, t11916, t1244, t1246, t3604, t3610, t3612, t3613, t3626, t44669, t44673, t44700, t44710, t44785, t44786, t44798, t45314, t45320, t45323, t45326, t470, t491, t493);
        let t45344 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1485(t11598, t11608, t11613, t11616, t11620, t11621, t11625, t11639, t11640, t11868, t11869, t11872, t11877, t11888, t11889, t11890, t11904, t11907, t11910, t11914, t11918, t11919, t11925, t11928, t11935, t1201, t1215, t1235, t1238, t1241, t1244, t1246, t1249, t1251, t1252, t15429, t3481, t3487, t3493, t3565, t3590, t3593, t3598, t3600, t3604, t3620, t3621, t3624, t3625, t3628, t3631, t44412, t44657, t44662, t44669, t44673, t44691, t44698, t44700, t44701, t44748, t44753, t44754, t44774, t45332, t491, t494, t498, t5079);
        let t45382 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1486(t11931, t225, t11604, t496, t68, t3599, t11601, t11599, t11606, t11608, t11613, t11868, t1190, t11919, t11925, t11928, t11935, t1238, t1252, t3487, t3593, t3600, t3630, t3631, t45314, t466, t498);
        let t45387 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1487(t1256, t193, t336, t3640, t44161, t44164, t44167, t44358, t44375, t44377, t44378, t44384, t44388, t44392, t44396, t44400, t45344, t45382);
        let t45402 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1488(t28, t265, t504, t41606, t43920, t43990, t44373, t45387, t10150, t1081, t11122, t11957, t1260, t2250, t2756, t3231, t3644, t39110, t39448, t506, t52, t607, t873, t9258, dens_threshold, rho1, zeta_threshold);
        let t45405 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1489(t113, t12492, t12507, t1266, t1271, t12734, t1393, t2314, t2320, t2364, t3652, t3660, t39223, t39231, t39235, t3929, t39332, t39385, t39480, t39524, t39586, t39626, t39847, t40615, t43657, t45402, t510, t513, t672, t89, t9347, t9351, t9419);
        let (t45408, t45421, t45422, t45424, t45426, t45428, t45430) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1490(t2311, t2319, t107, t9576, t2585, t667, t2281, t2333, t2359, t626, t9367, t9371);
    (t45405, t45408, t45421, t45422, t45424, t45426, t45428, t45430)
}
