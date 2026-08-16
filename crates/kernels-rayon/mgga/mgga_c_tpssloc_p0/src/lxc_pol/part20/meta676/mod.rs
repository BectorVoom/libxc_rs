//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2550;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2551;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2552;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2553;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2554;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2555;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2556;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta676(t1670: f64, t3313: f64, t11403: f64, t3375: f64, t4832: f64, t11292: f64, t1687: f64, t50826: f64, t43727: f64, t43729: f64, t43748: f64, t43750: f64, t50828: f64, t50832: f64, t50834: f64, t50897: f64, t50900: f64, t50903: f64, t50905: f64, t50907: f64, t50912: f64, t50917: f64, t50919: f64, t50921: f64, t50926: f64, t50931: f64, t50934: f64, t50948: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43816: f64, t43942: f64, t50937: f64, t50940: f64, t50946: f64, t50950: f64, t50952: f64, t50954: f64, t50957: f64, t50961: f64, t50966: f64, t50994: f64, t51000: f64, t51004: f64, t449: f64, t11365: f64, t1694: f64, t3331: f64, t4794: f64, t1117: f64, t14913: f64, t3315: f64, t11185: f64, t14937: f64, t3265: f64, t4782: f64, t11191: f64, t11275: f64, t4785: f64, t44320: f64, t11356: f64, t11366: f64, t11434: f64, t1148: f64, t1156: f64, t15133: f64, t3334: f64, t3371: f64, t3378: f64, t436: f64, t44211: f64, t4802: f64, t4858: f64, t51107: f64, t300: f64, t51381: f64, t51411: f64, t51450: f64, t51493: f64, t51538: f64, t51617: f64, t51664: f64, t15041: f64, t3411: f64, t11126: f64, t4884: f64, t1164: f64, t44106: f64, t4882: f64, t14842: f64, t11940: f64, t4700: f64, t5095: f64, t51131: f64, t51133: f64, t51245: f64, t51248: f64, t51251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51669, t51677, t51680, t51703) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2550(t1670, t3313, t11403, t3375, t4832, t11292, t1687, t50826, t43727, t43729, t43748, t43750, t50828, t50832, t50834, t50897, t50900, t50903, t50905, t50907, t50912, t50917, t50919, t50921, t50926, t50931, t50934);
        let t51723 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2551(t50948, t43780, t43782, t43784, t43786, t43788, t43816, t43942, t50937, t50940, t50946, t50950, t50952, t50954, t50957, t50961, t50966, t50994, t51000, t51004);
        let (t51725, t51727, t51730, t51736, t51738) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2552(t449, t51703, t51723, t11365, t1694, t3331, t4794, t1117, t14913, t3313, t3315, t11185, t14937);
        let (t51741, t51744, t51765) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2553(t3265, t3313, t4782, t11191, t11275, t4785, t50826, t50919, t43727, t43729, t43748, t43750, t50828, t50832, t50834, t50897, t50900, t50903, t50905, t50907, t50912, t50917, t50921, t50926, t50931, t50934);
        let t51785 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2554(t50948, t43780, t43782, t43784, t43786, t43788, t43816, t44320, t50937, t50940, t50946, t50950, t50952, t50954, t50957, t50961, t50966, t50994, t51000, t51004);
        let t51789 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2555(t11356, t11366, t11434, t1148, t1156, t15133, t3334, t3371, t3378, t436, t44211, t4802, t4858, t51107, t51669, t51677, t51680, t51725, t51727, t51730, t51736, t51738, t51741, t51744, t51765, t51785);
        let (t51793, t51795, t51797) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2556(t300, t51381, t51411, t51450, t51493, t51538, t51617, t51664, t51789, t15041, t3411, t11126, t4884);
        let (t51800, t51802, t51803) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2557(t1164, t44106, t4882, t14842, t3411, t11940, t4700, t5095, t51131, t51133, t51245, t51248, t51251, t51793, t51795, t51797);
    (t51669, t51725, t51736, t51738, t51741, t51744, t51793, t51795, t51797, t51800, t51802, t51803)
}
