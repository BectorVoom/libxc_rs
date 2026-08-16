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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1519;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1520;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1521;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1522;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1523;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1524;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1525;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta494<F: Float>(t12291: F, t1341: F, t1343: F, t16285: F, t1827: F, t19855: F, t20492: F, t20497: F, t20556: F, t20570: F, t3790: F, t40449: F, t5235: F, t54020: F, t54793: F, t6417: F, t6422: F, t74290: F, t80076: F, t80085: F, t80189: F, t80193: F, t820: F, t80265: F, t80303: F, t80330: F, t80352: F, t80375: F, t80399: F, t80442: F, t1336: F, t1825: F, t1838: F, t19657: F, t19815: F, t20490: F, t20553: F, t20622: F, t20630: F, t3792: F, t5234: F, t5334: F, t5335: F, t5344: F, t544: F, t54930: F, t553: F, t6420: F, t6451: F, t6456: F, t74289: F, t74937: F, t74949: F, t12249: F, t1375: F, t1378: F, t1380: F, t16047: F, t16428: F, t1814: F, t1834: F, t1840: F, t1842: F, t1843: F, t19743: F, t20029: F, t20060: F, t20473: F, t20495: F, t20594: F, t20595: F, t20613: F, t20616: F, t20625: F, t20635: F, t20648: F, t20651: F, t20661: F, t3887: F, t3897: F, t40541: F, t5215: F, t5321: F, t562: F, t564: F, t568: F, t57653: F, t6361: F, t6378: F, t6388: F, t6415: F, t6434: F, t6440: F, t6448: F, t6458: F, t6461: F, t74849: F, t74930: F, t75008: F, t75124: F, t79993: F, t80048: F, t80164: F, t80175: F, t80181: F, t80185: F, t6439: F, t12021: F, t1807: F, t20044: F, t20601: F, t20609: F, t20662: F, t40591: F, t539: F, t6460: F, t74860: F, t74908: F, t6324: F, t1390: F, t193: F, t20085: F, t39658: F, t39660: F, t39844: F, t39856: F, t40224: F, t40228: F, t40230: F, t40611: F, t5160: F, t533: F, t6463: F, t80112: F, t80113: F, t80114: F, t80115: F, t80116: F, t1268: F, t1458: F, t1774: F, t1849: F, t19451: F, t20293: F, t20296: F, t20347: F, t20350: F, t20720: F, t22425: F, t28002: F, t4028: F, t510: F, t513: F, t5460: F, t5493: F, t574: F, t6287: F, t6295: F, t6468: F, t652: F, t67001: F, t7458: F, t7676: F, t79713: F, t79817: F, t79825: F, t79829: F, t79855: F, t79891: F, t79903: F, t79915: F, t79926: F, t79939: F, t79988: F, t88: F, t89: F, t79729: F, t1401: F, t16524: F, t1851: F, t20162: F, t22445: F, t22448: F, t28893: F, t3941: F, t5371: F, t5456: F, t55388: F, t577: F, t75784: F, t1398: F, t1852: F, t1858: F, t22431: F, t22453: F, t3: F, t580: F, t6471: F, t6483: F, t67000: F, t75768: F, t75774: F, t75780: F) -> F {
        let t80474 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1519::<F>(t12291, t1341, t1343, t16285, t1827, t19855, t20492, t20497, t20556, t20570, t3790, t40449, t5235, t54020, t54793, t6417, t6422, t74290, t80076, t80085, t80189, t80193, t820);
        let (t80477, t80482) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1520::<F>(t80265, t80303, t80330, t80352, t80375, t80399, t80442, t80474, t1336, t1825, t1838, t19657, t19815, t20490, t20553, t20622, t20630, t3792, t5234, t5334, t5335, t5344, t544, t54930, t553, t6420, t6451, t6456, t74289, t74937, t74949);
        let t80489 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1521::<F>(t12249, t1336, t1375, t1378, t1380, t16047, t16428, t1814, t1825, t1834, t1840, t1842, t1843, t19657, t19743, t19815, t20029, t20060, t20473, t20495, t20594, t20595, t20613, t20616, t20625, t20635, t20648, t20651, t20661, t3887, t3897, t40541, t5215, t5234, t5321, t5334, t562, t564, t568, t57653, t6361, t6378, t6388, t6415, t6434, t6440, t6448, t6458, t6461, t74849, t74930, t75008, t75124, t79993, t80048, t80076, t80164, t80175, t80181, t80185, t80189, t80193, t80482);
        let t80521 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1522::<F>(t6439, t12021, t1375, t1807, t1843, t20044, t20060, t20601, t20609, t20662, t40591, t5215, t5321, t539, t568, t6440, t6460, t6461, t74860, t74908, t80477);
        let t80534 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1523::<F>(t6324, t1390, t193, t20085, t39658, t39660, t39844, t39856, t40224, t40228, t40230, t40611, t5160, t533, t6463, t80112, t80113, t80114, t80115, t80116, t80489, t80521);
        let t80558 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1524::<F>(t1268, t1458, t1774, t1849, t19451, t20293, t20296, t20347, t20350, t20720, t22425, t28002, t4028, t510, t513, t5460, t5493, t574, t6287, t6295, t6468, t652, t67001, t7458, t7676, t79713, t79817, t79825, t79829, t79855, t79891, t79903, t79915, t79926, t79939, t79988, t80534, t88, t89);
        let (t80559, t80591) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1525::<F>(t79729, t80558, t1401, t1458, t16524, t1851, t20162, t20347, t22445, t22448, t28893, t3941, t5371, t5456, t5493, t55388, t577, t75784, t79817, t79825);
        let tv4rho44 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1526::<F>(t1398, t1852, t1858, t22431, t22453, t3, t580, t6471, t6483, t67000, t75768, t75774, t75780, t80559, t80591);
    tv4rho44
}
