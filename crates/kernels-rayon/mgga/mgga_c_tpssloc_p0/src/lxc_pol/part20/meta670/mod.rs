//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta670 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2517;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2518;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2519;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2520;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta670(t136: f64, t3297: f64, t50964: f64, t2403: f64, t4772: f64, t14792: f64, t699: f64, t1113: f64, t50929: f64, t50826: f64, t50919: f64, t43727: f64, t43729: f64, t43748: f64, t43750: f64, t50828: f64, t50832: f64, t50834: f64, t50897: f64, t50900: f64, t50903: f64, t50905: f64, t50907: f64, t50912: f64, t50917: f64, t50921: f64, t50926: f64, t50931: f64, t50934: f64, t50948: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43816: f64, t43820: f64, t50937: f64, t50940: f64, t50946: f64, t50950: f64, t50952: f64, t50954: f64, t50957: f64, t50961: f64, t50966: f64, t50994: f64, t51000: f64, t51004: f64, t1100: f64, t1107: f64, t51034: f64, t51037: f64, t51040: f64, t51041: f64, t51043: f64, t51046: f64, t50845: f64, t50877: f64, t50902: f64, t50942: f64, t50974: f64, t50996: f64, t51032: f64, t1147: f64, t1156: f64, t1164: f64, t14831: f64, t3411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51049, t51051, t51053, t51056, t51078) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2517(t136, t3297, t50964, t2403, t4772, t14792, t699, t1113, t50929, t50826, t50919, t43727, t43729, t43748, t43750, t50828, t50832, t50834, t50897, t50900, t50903, t50905, t50907, t50912, t50917, t50921, t50926, t50931, t50934);
        let t51098 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2518(t50948, t43780, t43782, t43784, t43786, t43788, t43816, t43820, t50937, t50940, t50946, t50950, t50952, t50954, t50957, t50961, t50966, t50994, t51000, t51004);
        let (t51100, t51102, t51104) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2519(t51078, t51098, t1100, t1107, t51034, t51037, t51040, t51041, t51043, t51046, t51049, t51051, t51053, t51056);
        let (t51107, t51111, t51113) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2520(t50845, t50877, t50902, t50942, t50974, t50996, t51032, t51104, t1147, t1156, t1164, t14831, t3411);
    (t51049, t51051, t51053, t51056, t51100, t51102, t51107, t51111, t51113)
}
