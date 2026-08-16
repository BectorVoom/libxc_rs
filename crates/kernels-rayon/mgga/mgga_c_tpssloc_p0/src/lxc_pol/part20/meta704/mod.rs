//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta704 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2674;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2675;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2676;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta704(t54467: f64, t15892: f64, t2535: f64, t2528: f64, t40225: f64, t15921: f64, t588: f64, t40231: f64, t15971: f64, t40224: f64, t40230: f64, t54455: f64, t54457: f64, t54459: f64, t54461: f64, t54463: f64, t54464: f64, t54465: f64, t54466: f64, t12156: f64, t12157: f64, t12161: f64, t12303: f64, t1307: f64, t1345: f64, t1365: f64, t16018: f64, t16186: f64, t16191: f64, t16192: f64, t16195: f64, t16202: f64, t1799: f64, t1819: f64, t19708: f64, t1995: f64, t3719: f64, t3734: f64, t3839: f64, t3844: f64, t5187: f64, t5272: f64, t5278: f64, t5280: f64, t68: f64, t6924: f64, t12012: f64, t12147: f64, t12164: f64, t1347: f64, t1348: f64, t16176: f64, t16196: f64, t16199: f64, t1821: f64, t225: f64, t3847: f64, t5279: f64, t5283: f64, t53856: f64, t54311: f64, t54377: f64, t54391: f64, t54415: f64, t54426: f64, t54440: f64, t54454: f64, t546: f64, t548: f64, t550: f64, t12364: f64, t5234: f64, t1354: f64, t16288: f64, t3858: f64, t1351: f64, t12168: f64, t12413: f64, t1341: f64, t1343: f64, t1363: f64, t16101: f64, t16208: f64, t16224: f64, t16311: f64, t16394: f64, t221: f64, t3778: f64, t3803: f64, t3805: f64, t3870: f64, t40160: f64, t5246: f64, t5248: f64, t5250: f64, t5301: f64, t53958: f64, t54284: f64, t54293: f64, t54295: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54468, t54470, t54472, t54473, t54475, t54476, t54478, t54479) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2674(t54467, t15892, t2535, t2528, t40225, t15921, t588, t40231, t15971, t40224, t40230, t54455, t54457, t54459, t54461, t54463, t54464, t54465, t54466);
        let t54525 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2675(t12156, t12157, t12161, t12303, t1307, t1345, t1365, t16018, t16186, t16191, t16192, t16195, t16202, t1799, t1819, t19708, t1995, t3719, t3734, t3839, t3844, t5187, t5272, t5278, t5280, t68, t6924);
        let t54527 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2676(t12012, t12147, t12164, t1347, t1348, t16176, t16186, t16196, t16199, t1819, t1821, t225, t3839, t3847, t5272, t5278, t5279, t5283, t53856, t54311, t54377, t54391, t54415, t54426, t54440, t54454, t54479, t54525, t546, t548, t550);
        let t54552 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2677(t12364, t5234, t1354, t16288, t3858, t1351, t3734, t12012, t12168, t12413, t1341, t1343, t1363, t16101, t16208, t16224, t16311, t16394, t1799, t221, t3719, t3778, t3803, t3805, t3870, t40160, t5187, t5246, t5248, t5250, t5301, t53958, t54284, t54293, t54295, t54527, t820);
    (t54468, t54470, t54472, t54473, t54475, t54476, t54478, t54527, t54552)
}
