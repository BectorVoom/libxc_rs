//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta670 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2517;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2518;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2519;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2520;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta670<F: Float>(t136: F, t3297: F, t50964: F, t2403: F, t4772: F, t14792: F, t699: F, t1113: F, t50929: F, t50826: F, t50919: F, t43727: F, t43729: F, t43748: F, t43750: F, t50828: F, t50832: F, t50834: F, t50897: F, t50900: F, t50903: F, t50905: F, t50907: F, t50912: F, t50917: F, t50921: F, t50926: F, t50931: F, t50934: F, t50948: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43816: F, t43820: F, t50937: F, t50940: F, t50946: F, t50950: F, t50952: F, t50954: F, t50957: F, t50961: F, t50966: F, t50994: F, t51000: F, t51004: F, t1100: F, t1107: F, t51034: F, t51037: F, t51040: F, t51041: F, t51043: F, t51046: F, t50845: F, t50877: F, t50902: F, t50942: F, t50974: F, t50996: F, t51032: F, t1147: F, t1156: F, t1164: F, t14831: F, t3411: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t51049, t51051, t51053, t51056, t51078) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2517::<F>(t136, t3297, t50964, t2403, t4772, t14792, t699, t1113, t50929, t50826, t50919, t43727, t43729, t43748, t43750, t50828, t50832, t50834, t50897, t50900, t50903, t50905, t50907, t50912, t50917, t50921, t50926, t50931, t50934);
        let t51098 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2518::<F>(t50948, t43780, t43782, t43784, t43786, t43788, t43816, t43820, t50937, t50940, t50946, t50950, t50952, t50954, t50957, t50961, t50966, t50994, t51000, t51004);
        let (t51100, t51102, t51104) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2519::<F>(t51078, t51098, t1100, t1107, t51034, t51037, t51040, t51041, t51043, t51046, t51049, t51051, t51053, t51056);
        let (t51107, t51111, t51113) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2520::<F>(t50845, t50877, t50902, t50942, t50974, t50996, t51032, t51104, t1147, t1156, t1164, t14831, t3411);
    (t51049, t51051, t51053, t51056, t51100, t51102, t51107, t51111, t51113)
}
