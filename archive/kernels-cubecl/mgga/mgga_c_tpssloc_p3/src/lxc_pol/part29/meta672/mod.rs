//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta672 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2248;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2249;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2250;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2251;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2252;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2253;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2254;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2255;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2256;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2257;
use chunk10::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2258;
use chunk11::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2259;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta672<F: Float>(t16311: F, t3788: F, t3850: F, t6936: F, t57554: F, t80915: F, t26233: F, t3858: F, t22783: F, t5310: F, t22760: F, t5234: F, t3795: F, t3853: F, t80886: F, t80889: F, t80900: F, t91354: F, t91357: F, t91359: F, t91362: F, t91365: F, t91366: F, t91370: F, t91374: F, t1827: F, t80914: F, t1811: F, t80775: F, t7709: F, t80766: F, t22797: F, t5227: F, t22804: F, t26277: F, t80940: F, t16308: F, t22833: F, t16123: F, t2002: F, t559: F, t80920: F, t80922: F, t80943: F, t80957: F, t80959: F, t80971: F, t80989: F, t80992: F, t80998: F, t81007: F, t91132: F, t91181: F, t91224: F, t91258: F, t91302: F, t91348: F, t12240: F, t1336: F, t16047: F, t16048: F, t16206: F, t1814: F, t2013: F, t22871: F, t26403: F, t26459: F, t3777: F, t3793: F, t5230: F, t5334: F, t544: F, t553: F, t6987: F, t6990: F, t81216: F, t81218: F, t81230: F, t91065: F, t91074: F, t91077: F, t91078: F, t91081: F, t91091: F, t225: F, t26221: F, t1307: F, t1377: F, t22633: F, t22635: F, t5353: F, t26215: F, t80650: F, t12033: F, t1386: F, t16439: F, t22630: F, t22670: F, t22913: F, t26371: F, t3882: F, t5215: F, t5321: F, t5354: F, t6963: F, t7750: F, t81318: F, t81328: F, t1985: F, t6907: F, t90739: F, t22685: F, t22686: F, t26193: F, t16018: F, t6888: F, t6889: F, t6890: F, t22674: F, t22892: F, t26189: F, t26329: F, t26229: F, t81375: F, t1324: F, t254: F, t12020: F, t1843: F, t22656: F, t26224: F, t26226: F, t5210: F, t5325: F, t5326: F, t568: F, t6955: F, t6992: F, t6993: F, t80704: F, t22724: F, t26344: F, t22643: F, t7691: F, t81195: F, t1375: F, t16030: F, t16453: F, t1842: F, t2016: F, t22653: F, t22904: F, t26348: F, t3887: F, t539: F, t55093: F, t6958: F, t81393: F, t81395: F, t81399: F, t12021: F, t12444: F, t1323: F, t1378: F, t1390: F, t16022: F, t16122: F, t16437: F, t16460: F, t16471: F, t1807: F, t1983: F, t2006: F, t22870: F, t22905: F, t26328: F, t26366: F, t26472: F, t26477: F, t26482: F, t3758: F, t3888: F, t3889: F, t3912: F, t533: F, t53866: F, t54825: F, t55150: F, t7729: F, t7749: F, t80699: F, t81307: F, t81350: F, t81365: F, t81379: F, t90505: F, t90509: F, t90512: F, t90515: F, t90519: F, t90521: F, t90525: F, t90527: F, t90530: F, t90534: F, t90573: F, t90581: F, t90582: F, t90585: F, t90594: F, t90621: F, t90626: F, t90634: F, t90639: F, t90642: F, t90646: F, t90677: F, t90725: F, t90728: F, t90732: F, t90737: F, t90741: F, t90743: F, t90803: F, t90861: F, t90902: F, t90939: F, t90985: F, t91019: F, t91059: F, t1388: F, t25988: F, t22574: F, t26162: F, t26149: F, t6876: F, t19577: F, t31035: F, t25971: F, t83886: F, t23831: F, t4028: F, t26504: F, t7687: F, t83929: F, t1874: F, t90370: F, t26114: F, t6525: F, t12734: F, t7461: F, t2314: F, t25980: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t91378, t91381, t91383, t91384, t91387, t91388) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2248::<F>(t16311, t3788, t3850, t6936, t57554, t80915, t26233, t3858, t22783, t5310, t22760, t5234);
        let t91393 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2249::<F>(t3795, t91388, t26233, t3853, t80886, t80889, t80900, t91354, t91357, t91359, t91362, t91365, t91366, t91370, t91374, t91378, t91381, t91383, t91384, t91387);
        let (t91394, t91398, t91400, t91403, t91404, t91406, t91413) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2250::<F>(t1827, t80914, t1811, t80775, t7709, t80766, t22797, t5227, t22804, t26277, t80940, t16308, t22833);
        let t91418 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2251::<F>(t16123, t2002, t559, t80920, t80922, t80943, t80957, t80959, t80971, t80989, t80992, t80998, t81007, t91394, t91398, t91400, t91403, t91404, t91406, t91413);
        let (t91421, t91427) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2252::<F>(t91132, t91181, t91224, t91258, t91302, t91348, t91393, t91418, t12240, t1336, t16047, t16048, t16123, t16206, t1814, t2013, t22871, t26403, t26459, t3777, t3793, t5230, t5334, t544, t553, t6987, t6990, t81216, t81218, t81230, t91065, t91074, t91077, t91078, t91081, t91091);
        let t91459 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2253::<F>(t225, t26221, t1307, t1377, t22633, t22635, t5353, t26215, t80650, t12033, t1386, t16439, t22630, t22670, t22913, t26371, t3882, t5215, t5321, t5354, t6963, t7750, t81318, t81328);
        let (t91469, t91478, t91482, t91486) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2254::<F>(t1985, t6907, t90739, t22685, t22686, t26193, t16018, t6888, t6889, t6890, t22674, t22892, t26189);
        let t91512 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2255::<F>(t91486, t225, t26329, t26229, t81375, t1324, t254, t12020, t1386, t16439, t1843, t22656, t22670, t26224, t26226, t5210, t5325, t5326, t568, t6955, t6992, t6993, t80704);
        let (t91531, t91557) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2256::<F>(t22724, t26344, t22643, t7691, t81195, t1375, t16030, t16453, t1842, t2016, t22653, t22904, t26348, t3882, t3887, t5215, t539, t55093, t568, t6958, t6963, t81393, t81395, t81399, t91421);
        let t91564 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2257::<F>(t12021, t12033, t12444, t1323, t1375, t1378, t1386, t1390, t16022, t16030, t16122, t16437, t16460, t16471, t1807, t1843, t1983, t2006, t2016, t22653, t22656, t22870, t22905, t22913, t26328, t26366, t26472, t26477, t26482, t3758, t3882, t3887, t3888, t3889, t3912, t5215, t5321, t533, t5353, t5354, t53866, t54825, t55150, t568, t6958, t6963, t6992, t6993, t7729, t7749, t7750, t80699, t81307, t81350, t81365, t81379, t90505, t90509, t90512, t90515, t90519, t90521, t90525, t90527, t90530, t90534, t90573, t90581, t90582, t90585, t90594, t90621, t90626, t90634, t90639, t90642, t90646, t90677, t90725, t90728, t90732, t90737, t90741, t90743, t90803, t90861, t90902, t90939, t90985, t91019, t91059, t91427, t91459, t91469, t91478, t91482, t91512, t91531, t91557);
        let (t91568, t91570, t91573, t91578, t91580) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2258::<F>(t1388, t25988, t22574, t26162, t26149, t6876, t19577, t31035, t25971, t83886, t23831, t4028);
        let (t91582, t91585, t91587, t91589, t91591, t91593) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2259::<F>(t26504, t6876, t1983, t7687, t83929, t1874, t90370, t26114, t6525, t12734, t7461, t2314, t25980);
    (t91564, t91568, t91570, t91573, t91578, t91580, t91582, t91585, t91587, t91589, t91591, t91593)
}
