//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta705 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2678;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2679;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2680;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2681;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2682;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2683;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2684;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2685;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2686;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2687;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2688;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2689;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta705<F: Float>(t12365: F, t5289: F, t1827: F, t39955: F, t16261: F, t16398: F, t12289: F, t1336: F, t836: F, t16235: F, t1811: F, t40005: F, t12251: F, t12267: F, t12429: F, t16155: F, t16233: F, t16244: F, t16405: F, t3783: F, t3805: F, t40188: F, t40190: F, t40206: F, t40282: F, t5245: F, t5252: F, t5301: F, t12283: F, t16265: F, t1351: F, t3719: F, t16257: F, t1358: F, t16347: F, t40281: F, t5259: F, t1361: F, t242: F, t12178: F, t12255: F, t12303: F, t12371: F, t16305: F, t16311: F, t16312: F, t19735: F, t19876: F, t3803: F, t3807: F, t40168: F, t40285: F, t40293: F, t40295: F, t5246: F, t54258: F, t12189: F, t5206: F, t40406: F, t5202: F, t16115: F, t3726: F, t12199: F, t16111: F, t1804: F, t2585: F, t3732: F, t46853: F, t5308: F, t40343: F, t40347: F, t40350: F, t40351: F, t40356: F, t40360: F, t40366: F, t40372: F, t40376: F, t40387: F, t40401: F, t40402: F, t40404: F, t40407: F, t40410: F, t40415: F, t40422: F, t16118: F, t9577: F, t212: F, t5187: F, t12225: F, t2586: F, t16100: F, t782: F, t16103: F, t16081: F, t16090: F, t16093: F, t16097: F, t2566: F, t1307: F, t16018: F, t16084: F, t213: F, t221: F, t40423: F, t40425: F, t40429: F, t40431: F, t5195: F, t16094: F, t686: F, t16095: F, t2559: F, t5194: F, t5198: F, t118: F, t3739: F, t794: F, t16086: F, t12214: F, t67: F, t792: F, t3734: F, t133: F, t1799: F, t40369: F, t6600: F, t131: F, t205: F, t40024: F, t12012: F, t12156: F, t1315: F, t16101: F, t210: F, t214: F, t46838: F, t5196: F, t53856: F, t54284: F, t225: F, t40042: F, t12177: F, t40046: F, t16391: F) -> (F, F, F, F, F, F, F) {
        let (t54556, t54557, t54561, t54567, t54582) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2678::<F>(t12365, t5289, t1827, t39955, t16261, t16398, t12289, t1336, t836, t16235, t1811, t40005);
        let t54584 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2679::<F>(t12251, t12267, t12429, t16155, t16233, t16244, t16405, t3783, t3805, t40188, t40190, t40206, t40282, t5245, t5252, t5301, t54556, t54557, t54561, t54567, t54582);
        let (t54585, t54591, t54607, t54609, t54612, t54614) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2680::<F>(t12283, t16265, t1351, t3719, t16257, t16398, t1358, t16347, t40281, t5259, t1336, t1361, t242);
        let t54625 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2681::<F>(t12178, t12255, t12303, t12371, t16305, t16311, t16312, t19735, t19876, t3803, t3805, t3807, t40168, t40285, t40293, t40295, t5246, t5301, t54258, t54585, t54591, t54607, t54609, t54612, t54614);
        let (t54631, t54633, t54635, t54638, t54639, t54643) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2682::<F>(t12189, t5206, t40406, t5202, t16115, t3726, t12199, t16111, t1804, t40005, t2585, t3732, t46853, t5308);
        let (t54647, t54658) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2683::<F>(t54643, t40343, t40347, t40350, t40351, t40356, t54631, t54633, t54635, t54638, t54639, t40360, t40366, t40372, t40376, t40387, t40401, t40402, t40404, t40407, t40410, t40415, t40422);
        let (t54663, t54665, t54668, t54671, t54673, t54676) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2684::<F>(t16118, t9577, t212, t5187, t12225, t2586, t16100, t782, t16103, t16081, t16090, t16093, t16097, t2566);
        let t54687 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2685::<F>(t1307, t16018, t16084, t213, t221, t3719, t40423, t40425, t40429, t40431, t5195, t54663, t54668, t54671, t54673, t54676);
        let (t54690, t54698, t54702, t54705) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2686::<F>(t1307, t16094, t54665, t686, t16095, t3719, t2559, t5194, t5198, t118, t16018, t3739, t794);
        let (t54711, t54721, t54725) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2687::<F>(t16081, t16086, t12214, t67, t792, t16095, t3734, t686, t133, t1799, t40369, t6600);
        let t54736 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2688::<F>(t131, t205, t40024, t12012, t12156, t1315, t16084, t16101, t210, t214, t221, t3734, t46838, t5195, t5196, t53856, t54284, t54690, t54698, t54702, t54705, t54711, t54721, t54725);
        let (t54738, t54739, t54744, t54745, t54750) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2689::<F>(t54647, t54658, t54687, t54736, t225, t1336, t242, t40042, t12177, t40046, t16391, t16398);
    (t54584, t54625, t54738, t54739, t54744, t54745, t54750)
}
