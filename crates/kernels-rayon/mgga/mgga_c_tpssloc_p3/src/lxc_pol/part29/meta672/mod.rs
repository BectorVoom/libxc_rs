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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta672(t16311: f64, t3788: f64, t3850: f64, t6936: f64, t57554: f64, t80915: f64, t26233: f64, t3858: f64, t22783: f64, t5310: f64, t22760: f64, t5234: f64, t3795: f64, t3853: f64, t80886: f64, t80889: f64, t80900: f64, t91354: f64, t91357: f64, t91359: f64, t91362: f64, t91365: f64, t91366: f64, t91370: f64, t91374: f64, t1827: f64, t80914: f64, t1811: f64, t80775: f64, t7709: f64, t80766: f64, t22797: f64, t5227: f64, t22804: f64, t26277: f64, t80940: f64, t16308: f64, t22833: f64, t16123: f64, t2002: f64, t559: f64, t80920: f64, t80922: f64, t80943: f64, t80957: f64, t80959: f64, t80971: f64, t80989: f64, t80992: f64, t80998: f64, t81007: f64, t91132: f64, t91181: f64, t91224: f64, t91258: f64, t91302: f64, t91348: f64, t12240: f64, t1336: f64, t16047: f64, t16048: f64, t16206: f64, t1814: f64, t2013: f64, t22871: f64, t26403: f64, t26459: f64, t3777: f64, t3793: f64, t5230: f64, t5334: f64, t544: f64, t553: f64, t6987: f64, t6990: f64, t81216: f64, t81218: f64, t81230: f64, t91065: f64, t91074: f64, t91077: f64, t91078: f64, t91081: f64, t91091: f64, t225: f64, t26221: f64, t1307: f64, t1377: f64, t22633: f64, t22635: f64, t5353: f64, t26215: f64, t80650: f64, t12033: f64, t1386: f64, t16439: f64, t22630: f64, t22670: f64, t22913: f64, t26371: f64, t3882: f64, t5215: f64, t5321: f64, t5354: f64, t6963: f64, t7750: f64, t81318: f64, t81328: f64, t1985: f64, t6907: f64, t90739: f64, t22685: f64, t22686: f64, t26193: f64, t16018: f64, t6888: f64, t6889: f64, t6890: f64, t22674: f64, t22892: f64, t26189: f64, t26329: f64, t26229: f64, t81375: f64, t1324: f64, t254: f64, t12020: f64, t1843: f64, t22656: f64, t26224: f64, t26226: f64, t5210: f64, t5325: f64, t5326: f64, t568: f64, t6955: f64, t6992: f64, t6993: f64, t80704: f64, t22724: f64, t26344: f64, t22643: f64, t7691: f64, t81195: f64, t1375: f64, t16030: f64, t16453: f64, t1842: f64, t2016: f64, t22653: f64, t22904: f64, t26348: f64, t3887: f64, t539: f64, t55093: f64, t6958: f64, t81393: f64, t81395: f64, t81399: f64, t12021: f64, t12444: f64, t1323: f64, t1378: f64, t1390: f64, t16022: f64, t16122: f64, t16437: f64, t16460: f64, t16471: f64, t1807: f64, t1983: f64, t2006: f64, t22870: f64, t22905: f64, t26328: f64, t26366: f64, t26472: f64, t26477: f64, t26482: f64, t3758: f64, t3888: f64, t3889: f64, t3912: f64, t533: f64, t53866: f64, t54825: f64, t55150: f64, t7729: f64, t7749: f64, t80699: f64, t81307: f64, t81350: f64, t81365: f64, t81379: f64, t90505: f64, t90509: f64, t90512: f64, t90515: f64, t90519: f64, t90521: f64, t90525: f64, t90527: f64, t90530: f64, t90534: f64, t90573: f64, t90581: f64, t90582: f64, t90585: f64, t90594: f64, t90621: f64, t90626: f64, t90634: f64, t90639: f64, t90642: f64, t90646: f64, t90677: f64, t90725: f64, t90728: f64, t90732: f64, t90737: f64, t90741: f64, t90743: f64, t90803: f64, t90861: f64, t90902: f64, t90939: f64, t90985: f64, t91019: f64, t91059: f64, t1388: f64, t25988: f64, t22574: f64, t26162: f64, t26149: f64, t6876: f64, t19577: f64, t31035: f64, t25971: f64, t83886: f64, t23831: f64, t4028: f64, t26504: f64, t7687: f64, t83929: f64, t1874: f64, t90370: f64, t26114: f64, t6525: f64, t12734: f64, t7461: f64, t2314: f64, t25980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91378, t91381, t91383, t91384, t91387, t91388) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2248(t16311, t3788, t3850, t6936, t57554, t80915, t26233, t3858, t22783, t5310, t22760, t5234);
        let t91393 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2249(t3795, t91388, t26233, t3853, t80886, t80889, t80900, t91354, t91357, t91359, t91362, t91365, t91366, t91370, t91374, t91378, t91381, t91383, t91384, t91387);
        let (t91394, t91398, t91400, t91403, t91404, t91406, t91413) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2250(t1827, t80914, t1811, t80775, t7709, t80766, t22797, t5227, t22804, t26277, t80940, t16308, t22833);
        let t91418 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2251(t16123, t2002, t559, t80920, t80922, t80943, t80957, t80959, t80971, t80989, t80992, t80998, t81007, t91394, t91398, t91400, t91403, t91404, t91406, t91413);
        let (t91421, t91427) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2252(t91132, t91181, t91224, t91258, t91302, t91348, t91393, t91418, t12240, t1336, t16047, t16048, t16123, t16206, t1814, t2013, t22871, t26403, t26459, t3777, t3793, t5230, t5334, t544, t553, t6987, t6990, t81216, t81218, t81230, t91065, t91074, t91077, t91078, t91081, t91091);
        let t91459 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2253(t225, t26221, t1307, t1377, t22633, t22635, t5353, t26215, t80650, t12033, t1386, t16439, t22630, t22670, t22913, t26371, t3882, t5215, t5321, t5354, t6963, t7750, t81318, t81328);
        let (t91469, t91478, t91482, t91486) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2254(t1985, t6907, t90739, t22685, t22686, t26193, t16018, t6888, t6889, t6890, t22674, t22892, t26189);
        let t91512 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2255(t91486, t225, t26329, t26229, t81375, t1324, t254, t12020, t1386, t16439, t1843, t22656, t22670, t26224, t26226, t5210, t5325, t5326, t568, t6955, t6992, t6993, t80704);
        let (t91531, t91557) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2256(t22724, t26344, t22643, t7691, t81195, t1375, t16030, t16453, t1842, t2016, t22653, t22904, t26348, t3882, t3887, t5215, t539, t55093, t568, t6958, t6963, t81393, t81395, t81399, t91421);
        let t91564 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2257(t12021, t12033, t12444, t1323, t1375, t1378, t1386, t1390, t16022, t16030, t16122, t16437, t16460, t16471, t1807, t1843, t1983, t2006, t2016, t22653, t22656, t22870, t22905, t22913, t26328, t26366, t26472, t26477, t26482, t3758, t3882, t3887, t3888, t3889, t3912, t5215, t5321, t533, t5353, t5354, t53866, t54825, t55150, t568, t6958, t6963, t6992, t6993, t7729, t7749, t7750, t80699, t81307, t81350, t81365, t81379, t90505, t90509, t90512, t90515, t90519, t90521, t90525, t90527, t90530, t90534, t90573, t90581, t90582, t90585, t90594, t90621, t90626, t90634, t90639, t90642, t90646, t90677, t90725, t90728, t90732, t90737, t90741, t90743, t90803, t90861, t90902, t90939, t90985, t91019, t91059, t91427, t91459, t91469, t91478, t91482, t91512, t91531, t91557);
        let (t91568, t91570, t91573, t91578, t91580) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2258(t1388, t25988, t22574, t26162, t26149, t6876, t19577, t31035, t25971, t83886, t23831, t4028);
        let (t91582, t91585, t91587, t91589, t91591, t91593) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2259(t26504, t6876, t1983, t7687, t83929, t1874, t90370, t26114, t6525, t12734, t7461, t2314, t25980);
    (t91564, t91568, t91570, t91573, t91578, t91580, t91582, t91585, t91587, t91589, t91591, t91593)
}
