//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1519;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1520;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1521;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1522;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1523;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1524;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1525;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta494(t12291: f64, t1341: f64, t1343: f64, t16285: f64, t1827: f64, t19855: f64, t20492: f64, t20497: f64, t20556: f64, t20570: f64, t3790: f64, t40449: f64, t5235: f64, t54020: f64, t54793: f64, t6417: f64, t6422: f64, t74290: f64, t80076: f64, t80085: f64, t80189: f64, t80193: f64, t820: f64, t80265: f64, t80303: f64, t80330: f64, t80352: f64, t80375: f64, t80399: f64, t80442: f64, t1336: f64, t1825: f64, t1838: f64, t19657: f64, t19815: f64, t20490: f64, t20553: f64, t20622: f64, t20630: f64, t3792: f64, t5234: f64, t5334: f64, t5335: f64, t5344: f64, t544: f64, t54930: f64, t553: f64, t6420: f64, t6451: f64, t6456: f64, t74289: f64, t74937: f64, t74949: f64, t12249: f64, t1375: f64, t1378: f64, t1380: f64, t16047: f64, t16428: f64, t1814: f64, t1834: f64, t1840: f64, t1842: f64, t1843: f64, t19743: f64, t20029: f64, t20060: f64, t20473: f64, t20495: f64, t20594: f64, t20595: f64, t20613: f64, t20616: f64, t20625: f64, t20635: f64, t20648: f64, t20651: f64, t20661: f64, t3887: f64, t3897: f64, t40541: f64, t5215: f64, t5321: f64, t562: f64, t564: f64, t568: f64, t57653: f64, t6361: f64, t6378: f64, t6388: f64, t6415: f64, t6434: f64, t6440: f64, t6448: f64, t6458: f64, t6461: f64, t74849: f64, t74930: f64, t75008: f64, t75124: f64, t79993: f64, t80048: f64, t80164: f64, t80175: f64, t80181: f64, t80185: f64, t6439: f64, t12021: f64, t1807: f64, t20044: f64, t20601: f64, t20609: f64, t20662: f64, t40591: f64, t539: f64, t6460: f64, t74860: f64, t74908: f64, t6324: f64, t1390: f64, t193: f64, t20085: f64, t39658: f64, t39660: f64, t39844: f64, t39856: f64, t40224: f64, t40228: f64, t40230: f64, t40611: f64, t5160: f64, t533: f64, t6463: f64, t80112: f64, t80113: f64, t80114: f64, t80115: f64, t80116: f64, t1268: f64, t1458: f64, t1774: f64, t1849: f64, t19451: f64, t20293: f64, t20296: f64, t20347: f64, t20350: f64, t20720: f64, t22425: f64, t28002: f64, t4028: f64, t510: f64, t513: f64, t5460: f64, t5493: f64, t574: f64, t6287: f64, t6295: f64, t6468: f64, t652: f64, t67001: f64, t7458: f64, t7676: f64, t79713: f64, t79817: f64, t79825: f64, t79829: f64, t79855: f64, t79891: f64, t79903: f64, t79915: f64, t79926: f64, t79939: f64, t79988: f64, t88: f64, t89: f64, t79729: f64, t1401: f64, t16524: f64, t1851: f64, t20162: f64, t22445: f64, t22448: f64, t28893: f64, t3941: f64, t5371: f64, t5456: f64, t55388: f64, t577: f64, t75784: f64, t1398: f64, t1852: f64, t1858: f64, t22431: f64, t22453: f64, t3: f64, t580: f64, t6471: f64, t6483: f64, t67000: f64, t75768: f64, t75774: f64, t75780: f64) -> f64 {
        let t80474 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1519(t12291, t1341, t1343, t16285, t1827, t19855, t20492, t20497, t20556, t20570, t3790, t40449, t5235, t54020, t54793, t6417, t6422, t74290, t80076, t80085, t80189, t80193, t820);
        let (t80477, t80482) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1520(t80265, t80303, t80330, t80352, t80375, t80399, t80442, t80474, t1336, t1825, t1838, t19657, t19815, t20490, t20553, t20622, t20630, t3792, t5234, t5334, t5335, t5344, t544, t54930, t553, t6420, t6451, t6456, t74289, t74937, t74949);
        let t80489 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1521(t12249, t1336, t1375, t1378, t1380, t16047, t16428, t1814, t1825, t1834, t1840, t1842, t1843, t19657, t19743, t19815, t20029, t20060, t20473, t20495, t20594, t20595, t20613, t20616, t20625, t20635, t20648, t20651, t20661, t3887, t3897, t40541, t5215, t5234, t5321, t5334, t562, t564, t568, t57653, t6361, t6378, t6388, t6415, t6434, t6440, t6448, t6458, t6461, t74849, t74930, t75008, t75124, t79993, t80048, t80076, t80164, t80175, t80181, t80185, t80189, t80193, t80482);
        let t80521 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1522(t6439, t12021, t1375, t1807, t1843, t20044, t20060, t20601, t20609, t20662, t40591, t5215, t5321, t539, t568, t6440, t6460, t6461, t74860, t74908, t80477);
        let t80534 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1523(t6324, t1390, t193, t20085, t39658, t39660, t39844, t39856, t40224, t40228, t40230, t40611, t5160, t533, t6463, t80112, t80113, t80114, t80115, t80116, t80489, t80521);
        let t80558 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1524(t1268, t1458, t1774, t1849, t19451, t20293, t20296, t20347, t20350, t20720, t22425, t28002, t4028, t510, t513, t5460, t5493, t574, t6287, t6295, t6468, t652, t67001, t7458, t7676, t79713, t79817, t79825, t79829, t79855, t79891, t79903, t79915, t79926, t79939, t79988, t80534, t88, t89);
        let (t80559, t80591) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1525(t79729, t80558, t1401, t1458, t16524, t1851, t20162, t20347, t22445, t22448, t28893, t3941, t5371, t5456, t5493, t55388, t577, t75784, t79817, t79825);
        let tv4rho44 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1526(t1398, t1852, t1858, t22431, t22453, t3, t580, t6471, t6483, t67000, t75768, t75774, t75780, t80559, t80591);
    tv4rho44
}
