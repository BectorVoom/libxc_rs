//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta744 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2468;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2469;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2470;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2471;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2472;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta744(t17611: f64, t4641: f64, t10480: f64, t21391: f64, t248: f64, t3101: f64, t1041: f64, t10457: f64, t21118: f64, t1616: f64, t607: f64, t10403: f64, t10408: f64, t10413: f64, t1618: f64, t17151: f64, t17177: f64, t17182: f64, t17923: f64, t3070: f64, t3071: f64, t42397: f64, t42483: f64, t5685: f64, t61744: f64, t61754: f64, t61768: f64, t61782: f64, t62850: f64, t70082: f64, t70086: f64, t21390: f64, t376: f64, t10952: f64, t17607: f64, t17712: f64, t21503: f64, t21551: f64, t3039: f64, t3048: f64, t3117: f64, t42347: f64, t4582: f64, t4585: f64, t4590: f64, t4594: f64, t4650: f64, t61784: f64, t61794: f64, t61796: f64, t62091: f64, t1409: f64, t16558: f64, t3966: f64, t5398: f64, t20234: f64, t1023: f64, t10390: f64, t17637: f64, t17643: f64, t21134: f64, t21403: f64, t21532: f64, t21574: f64, t42508: f64, t4583: f64, t4644: f64, t48607: f64, t49854: f64, t69643: f64, t884: f64, t5392: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70214, t70227, t70241, t70268) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2468(t17611, t4641, t10480, t21391, t248, t3101, t1041, t10457, t21118, t1616, t607, t10403, t10408, t10413, t1618, t17151, t17177, t17182, t17923, t3070, t3071, t42397, t42483, t5685, t61744, t61754, t61768, t61782, t62850, t70082, t70086);
        let (t70273, t70296) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2469(t21390, t376, t10952, t1616, t17607, t17712, t21503, t21551, t3039, t3048, t3117, t42347, t4582, t4585, t4590, t4594, t4650, t61784, t61794, t61796, t62091);
        let (t70316, t70321, t70330) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2470(t1409, t16558, t3966, t5398, t20234, t607);
        let t70335 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2471(t1023, t10390, t1041, t17637, t17643, t21134, t21403, t21532, t21574, t3070, t3071, t42397, t42483, t42508, t4582, t4583, t4644, t4650, t48607, t49854, t5685, t69643, t70316, t70321, t70330, t884);
        let t70339 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2472(t3966, t5392);
    (t70214, t70227, t70241, t70268, t70273, t70296, t70316, t70321, t70330, t70335, t70339)
}
