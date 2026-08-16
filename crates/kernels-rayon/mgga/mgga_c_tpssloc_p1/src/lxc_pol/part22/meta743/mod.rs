//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta743 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2464;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2465;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2466;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2467;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta743(t5866: f64, t6739: f64, t1022: f64, t14211: f64, t14218: f64, t360: f64, t1615: f64, t883: f64, t1539: f64, t4649: f64, t17906: f64, t4644: f64, t17607: f64, t4571: f64, t1011: f64, t1019: f64, t69923: f64, t1025: f64, t1622: f64, t21405: f64, t21580: f64, t21609: f64, t3048: f64, t3117: f64, t43211: f64, t61659: f64, t61663: f64, t61665: f64, t61710: f64, t1040: f64, t21482: f64, t10876: f64, t21396: f64, t248: f64, t3101: f64, t1041: f64, t21138: f64, t3051: f64, t10403: f64, t10408: f64, t1046: f64, t18014: f64, t3071: f64, t42388: f64, t43361: f64, t4338: f64, t4343: f64, t4636: f64, t49743: f64, t5873: f64, t5880: f64, t61675: f64, t62079: f64, t21134: f64, t14508: f64, t17667: f64, t14085: f64, t17962: f64, t21597: f64, t3109: f64, t42354: f64, t4641: f64, t48431: f64, t50302: f64, t5857: f64, t5875: f64, t61677: f64, t61695: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70081, t70082, t70086, t70100, t70106, t70122, t70132) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2464(t5866, t6739, t1022, t14211, t14218, t360, t1615, t883, t1539, t4649, t17906, t4644);
        let t70151 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2465(t17607, t4571, t1011, t1019, t69923, t1025, t1622, t21405, t21580, t21609, t3048, t3117, t43211, t61659, t61663, t61665, t61710, t70132);
        let t70189 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2466(t1040, t21482, t10876, t21396, t248, t3101, t1041, t21138, t3051, t10403, t10408, t1046, t14211, t17607, t18014, t3071, t42388, t43361, t4338, t4343, t4636, t49743, t5873, t5880, t61675, t62079, t70106);
        let t70211 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2467(t1041, t21134, t248, t3051, t14508, t17667, t14085, t1622, t17962, t21405, t21580, t21597, t3109, t3117, t42354, t4641, t48431, t50302, t5857, t5875, t61677, t61695);
    (t70081, t70082, t70086, t70100, t70106, t70122, t70151, t70189, t70211)
}
