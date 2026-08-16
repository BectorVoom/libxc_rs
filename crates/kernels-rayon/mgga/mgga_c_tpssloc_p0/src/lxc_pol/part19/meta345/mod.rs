//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta345 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1234;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1235;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1236;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1237;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1238;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1239;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1240;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1241;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1242;
use chunk9::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1243;
use chunk10::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1244;
use chunk11::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta345(t2617: f64, t9973: f64, t236: f64, t40931: f64, t240: f64, t812: f64, t2638: f64, t9612: f64, t831: f64, t10021: f64, t815: f64, t2686: f64, t9671: f64, t2678: f64, t2632: f64, t2681: f64, t2628: f64, t2690: f64, t2635: f64, t232: f64, t40925: f64, t2379: f64, t2553: f64, t2630: f64, t40934: f64, t40938: f64, t817: f64, t819: f64, t820: f64, t843: f64, t9607: f64, t9613: f64, t9967: f64, t9974: f64, t9978: f64, t9983: f64, t9674: f64, t2697: f64, t9618: f64, t40904: f64, t816: f64, t2629: f64, t835: f64, t9972: f64, t9667: f64, t9666: f64, t2639: f64, t9663: f64, t2701: f64, t40926: f64, t776: f64, t9516: f64, t13258: f64, t9634: f64, t9629: f64, t9975: f64, t6589: f64, t67: f64, t246: f64, t9458: f64, t10007: f64, t119: f64, t120: f64, t13262: f64, t210: f64, t2571: f64, t2643: f64, t2645: f64, t2646: f64, t2647: f64, t40972: f64, t40977: f64, t41039: f64, t41072: f64, t41161: f64, t4178: f64, t829: f64, t9621: f64, t9626: f64, t9642: f64, t9646: f64, t9647: f64, t9653: f64, t40995: f64, t41037: f64, t41077: f64, t41120: f64, t41343: f64, t10069: f64, t10077: f64, t10080: f64, t10091: f64, t10098: f64, t13390: f64, t13397: f64, t226: f64, t22997: f64, t235: f64, t2728: f64, t2732: f64, t40932: f64, t40951: f64, t40955: f64, t4291: f64, t9958: f64, t10046: f64, t814: f64, t10016: f64, t10058: f64, t10073: f64, t10081: f64, t10094: f64, t13453: f64, t255: f64, t2613: f64, t2740: f64, t41231: f64, t41333: f64, t808: f64, t860: f64, t863: f64, t9661: f64, t225: f64, t9520: f64, t10049: f64, t10054: f64, t10055: f64, t10076: f64, t10084: f64, t10097: f64, t10101: f64, t10103: f64, t10104: f64, t10112: f64, t10116: f64, t218: f64, t23175: f64, t252: f64, t259: f64, t2597: f64, t2633: f64, t2679: f64, t2684: f64, t2718: f64, t2720: f64, t2729: f64, t2733: f64, t2736: f64, t2738: f64, t2743: f64, t40890: f64, t40891: f64, t40895: f64, t40909: f64, t40917: f64, t41230: f64, t4182: f64, t4281: f64, t852: f64, t855: f64, t858: f64, t861: f64, t865: f64, t866: f64, t9584: f64, t9590: f64, t9632: f64, t9976: f64, t9981: f64, t12935: f64, t193: f64, t202: f64, t2522: f64, t39585: f64, t39590: f64, t39593: f64, t40848: f64, t40887: f64, t41252: f64, t41254: f64, t41256: f64, t41258: f64, t41260: f64, t41262: f64, t41266: f64, t766: f64, t870: f64, t9470: f64, t2378: f64, t262: f64, t39658: f64, t41270: f64, t41273: f64, t41275: f64, t41278: f64, t41281: f64, t41283: f64, t41286: f64, t41289: f64, t41292: f64, t41296: f64, t4314: f64, t868: f64, t40672: f64, t40705: f64, t40724: f64, t40756: f64, t40791: f64, t40819: f64, t10647: f64, t892: f64, t914: f64, t10650: f64, t2837: f64) -> (f64, f64, f64) {
        let (t41344, t41349, t41355, t41363, t41365) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1234(t2617, t9973, t236, t40931, t240, t812, t2638, t9612, t831, t10021, t815, t2686, t9671);
        let (t41367, t41368, t41388, t41393) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1235(t2678, t2632, t2681, t9671, t2628, t2690, t812, t2635, t232, t40925, t2379, t2553, t2630, t2686, t40934, t40938, t41344, t41349, t41355, t41363, t41365, t817, t819, t820, t843, t9607, t9613, t9967, t9974, t9978, t9983);
        let (t41395, t41397, t41399, t41404, t41410, t41414) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1236(t2686, t9674, t2697, t9618, t40904, t816, t2681, t2629, t9612, t812, t835, t9972);
        let (t41429, t41434) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1237(t41414, t9978, t9667, t9983, t2617, t9666, t2635, t2639, t9663, t232, t41367, t2630, t2681, t2701, t40926, t41395, t41397, t41399, t41404, t41410, t776, t817, t819, t820, t831, t843, t9516, t9613);
        let (t41435, t41437, t41448, t41453, t41467, t41468) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1238(t13258, t9634, t9629, t2379, t2632, t776, t9975, t6589, t67, t246, t232, t9458);
        let t41487 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1239(t10007, t119, t120, t13262, t210, t2571, t2643, t2645, t2646, t2647, t40972, t40977, t41039, t41072, t41161, t41435, t41437, t41448, t41453, t41467, t41468, t4178, t829, t9516, t9621, t9626, t9642, t9646, t9647, t9653);
        let (t41490, t41495) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1240(t40995, t41037, t41077, t41120, t41343, t41393, t41434, t41487, t10069, t10077, t10080, t10091, t10098, t13390, t13397, t226, t22997, t235, t2617, t2728, t2732, t40926, t40932, t40934, t40938, t40951, t40955, t4291, t812, t829, t9958);
        let (t41520, t41549) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1241(t10046, t814, t10016, t10058, t10073, t10081, t10094, t13453, t255, t2613, t2617, t2728, t2732, t2740, t41231, t41333, t41368, t41429, t808, t812, t860, t863, t9661);
        let t41580 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1242(t225, t9520, t10049, t10054, t10055, t10076, t10084, t10097, t10101, t10103, t10104, t10112, t10116, t218, t22997, t23175, t252, t259, t2597, t2617, t2633, t2679, t2684, t2718, t2720, t2729, t2733, t2736, t2738, t2743, t40890, t40891, t40895, t40904, t40909, t40917, t41230, t41388, t41490, t41495, t41520, t41549, t4182, t4281, t4291, t812, t829, t852, t855, t858, t860, t861, t865, t866, t9584, t9590, t9612, t9632, t9976, t9981);
        let t41591 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1243(t12935, t193, t202, t2522, t2553, t39585, t39590, t39593, t40848, t40887, t41252, t41254, t41256, t41258, t41260, t41262, t41266, t41580, t766, t870, t9470);
        let t41603 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1244(t193, t2378, t262, t39658, t40977, t41270, t41273, t41275, t41278, t41281, t41283, t41286, t41289, t41292, t41296, t4314, t776, t868, t870, t9458, t9516);
        let (t41606, t41620, t41622) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1245(t40672, t40705, t40724, t40756, t40791, t40819, t41591, t41603, t10647, t892, t914, t10650, t2837);
    (t41606, t41620, t41622)
}
