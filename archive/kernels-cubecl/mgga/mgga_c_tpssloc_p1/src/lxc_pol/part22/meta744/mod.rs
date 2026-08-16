//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta744 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2468;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2469;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2470;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2471;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2472;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta744<F: Float>(t17611: F, t4641: F, t10480: F, t21391: F, t248: F, t3101: F, t1041: F, t10457: F, t21118: F, t1616: F, t607: F, t10403: F, t10408: F, t10413: F, t1618: F, t17151: F, t17177: F, t17182: F, t17923: F, t3070: F, t3071: F, t42397: F, t42483: F, t5685: F, t61744: F, t61754: F, t61768: F, t61782: F, t62850: F, t70082: F, t70086: F, t21390: F, t376: F, t10952: F, t17607: F, t17712: F, t21503: F, t21551: F, t3039: F, t3048: F, t3117: F, t42347: F, t4582: F, t4585: F, t4590: F, t4594: F, t4650: F, t61784: F, t61794: F, t61796: F, t62091: F, t1409: F, t16558: F, t3966: F, t5398: F, t20234: F, t1023: F, t10390: F, t17637: F, t17643: F, t21134: F, t21403: F, t21532: F, t21574: F, t42508: F, t4583: F, t4644: F, t48607: F, t49854: F, t69643: F, t884: F, t5392: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t70214, t70227, t70241, t70268) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2468::<F>(t17611, t4641, t10480, t21391, t248, t3101, t1041, t10457, t21118, t1616, t607, t10403, t10408, t10413, t1618, t17151, t17177, t17182, t17923, t3070, t3071, t42397, t42483, t5685, t61744, t61754, t61768, t61782, t62850, t70082, t70086);
        let (t70273, t70296) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2469::<F>(t21390, t376, t10952, t1616, t17607, t17712, t21503, t21551, t3039, t3048, t3117, t42347, t4582, t4585, t4590, t4594, t4650, t61784, t61794, t61796, t62091);
        let (t70316, t70321, t70330) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2470::<F>(t1409, t16558, t3966, t5398, t20234, t607);
        let t70335 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2471::<F>(t1023, t10390, t1041, t17637, t17643, t21134, t21403, t21532, t21574, t3070, t3071, t42397, t42483, t42508, t4582, t4583, t4644, t4650, t48607, t49854, t5685, t69643, t70316, t70321, t70330, t884);
        let t70339 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2472::<F>(t3966, t5392);
    (t70214, t70227, t70241, t70268, t70273, t70296, t70316, t70321, t70330, t70335, t70339)
}
