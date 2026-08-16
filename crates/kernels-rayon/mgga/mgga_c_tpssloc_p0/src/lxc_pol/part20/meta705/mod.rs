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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta705(t12365: f64, t5289: f64, t1827: f64, t39955: f64, t16261: f64, t16398: f64, t12289: f64, t1336: f64, t836: f64, t16235: f64, t1811: f64, t40005: f64, t12251: f64, t12267: f64, t12429: f64, t16155: f64, t16233: f64, t16244: f64, t16405: f64, t3783: f64, t3805: f64, t40188: f64, t40190: f64, t40206: f64, t40282: f64, t5245: f64, t5252: f64, t5301: f64, t12283: f64, t16265: f64, t1351: f64, t3719: f64, t16257: f64, t1358: f64, t16347: f64, t40281: f64, t5259: f64, t1361: f64, t242: f64, t12178: f64, t12255: f64, t12303: f64, t12371: f64, t16305: f64, t16311: f64, t16312: f64, t19735: f64, t19876: f64, t3803: f64, t3807: f64, t40168: f64, t40285: f64, t40293: f64, t40295: f64, t5246: f64, t54258: f64, t12189: f64, t5206: f64, t40406: f64, t5202: f64, t16115: f64, t3726: f64, t12199: f64, t16111: f64, t1804: f64, t2585: f64, t3732: f64, t46853: f64, t5308: f64, t40343: f64, t40347: f64, t40350: f64, t40351: f64, t40356: f64, t40360: f64, t40366: f64, t40372: f64, t40376: f64, t40387: f64, t40401: f64, t40402: f64, t40404: f64, t40407: f64, t40410: f64, t40415: f64, t40422: f64, t16118: f64, t9577: f64, t212: f64, t5187: f64, t12225: f64, t2586: f64, t16100: f64, t782: f64, t16103: f64, t16081: f64, t16090: f64, t16093: f64, t16097: f64, t2566: f64, t1307: f64, t16018: f64, t16084: f64, t213: f64, t221: f64, t40423: f64, t40425: f64, t40429: f64, t40431: f64, t5195: f64, t16094: f64, t686: f64, t16095: f64, t2559: f64, t5194: f64, t5198: f64, t118: f64, t3739: f64, t794: f64, t16086: f64, t12214: f64, t67: f64, t792: f64, t3734: f64, t133: f64, t1799: f64, t40369: f64, t6600: f64, t131: f64, t205: f64, t40024: f64, t12012: f64, t12156: f64, t1315: f64, t16101: f64, t210: f64, t214: f64, t46838: f64, t5196: f64, t53856: f64, t54284: f64, t225: f64, t40042: f64, t12177: f64, t40046: f64, t16391: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t54556, t54557, t54561, t54567, t54582) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2678(t12365, t5289, t1827, t39955, t16261, t16398, t12289, t1336, t836, t16235, t1811, t40005);
        let t54584 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2679(t12251, t12267, t12429, t16155, t16233, t16244, t16405, t3783, t3805, t40188, t40190, t40206, t40282, t5245, t5252, t5301, t54556, t54557, t54561, t54567, t54582);
        let (t54585, t54591, t54607, t54609, t54612, t54614) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2680(t12283, t16265, t1351, t3719, t16257, t16398, t1358, t16347, t40281, t5259, t1336, t1361, t242);
        let t54625 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2681(t12178, t12255, t12303, t12371, t16305, t16311, t16312, t19735, t19876, t3803, t3805, t3807, t40168, t40285, t40293, t40295, t5246, t5301, t54258, t54585, t54591, t54607, t54609, t54612, t54614);
        let (t54631, t54633, t54635, t54638, t54639, t54643) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2682(t12189, t5206, t40406, t5202, t16115, t3726, t12199, t16111, t1804, t40005, t2585, t3732, t46853, t5308);
        let (t54647, t54658) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2683(t54643, t40343, t40347, t40350, t40351, t40356, t54631, t54633, t54635, t54638, t54639, t40360, t40366, t40372, t40376, t40387, t40401, t40402, t40404, t40407, t40410, t40415, t40422);
        let (t54663, t54665, t54668, t54671, t54673, t54676) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2684(t16118, t9577, t212, t5187, t12225, t2586, t16100, t782, t16103, t16081, t16090, t16093, t16097, t2566);
        let t54687 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2685(t1307, t16018, t16084, t213, t221, t3719, t40423, t40425, t40429, t40431, t5195, t54663, t54668, t54671, t54673, t54676);
        let (t54690, t54698, t54702, t54705) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2686(t1307, t16094, t54665, t686, t16095, t3719, t2559, t5194, t5198, t118, t16018, t3739, t794);
        let (t54711, t54721, t54725) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2687(t16081, t16086, t12214, t67, t792, t16095, t3734, t686, t133, t1799, t40369, t6600);
        let t54736 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2688(t131, t205, t40024, t12012, t12156, t1315, t16084, t16101, t210, t214, t221, t3734, t46838, t5195, t5196, t53856, t54284, t54690, t54698, t54702, t54705, t54711, t54721, t54725);
        let (t54738, t54739, t54744, t54745, t54750) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2689(t54647, t54658, t54687, t54736, t225, t1336, t242, t40042, t12177, t40046, t16391, t16398);
    (t54584, t54625, t54738, t54739, t54744, t54745, t54750)
}
