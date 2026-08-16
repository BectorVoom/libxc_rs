//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta758 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2544;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2545;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2546;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2547;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta758(t1113: f64, t136: f64, t71189: f64, t71201: f64, t71191: f64, t71195: f64, t71199: f64, t71468: f64, t71470: f64, t71472: f64, t71474: f64, t71477: f64, t71480: f64, t71483: f64, t1102: f64, t5999: f64, t14801: f64, t14804: f64, t45192: f64, t48140: f64, t68513: f64, t50822: f64, t44938: f64, t43777: f64, t43859: f64, t43895: f64, t50919: f64, t50948: f64, t71203: f64, t71206: f64, t43816: f64, t51040: f64, t51051: f64, t63361: f64, t63382: f64, t63384: f64, t63398: f64, t63400: f64, t64074: f64, t64076: f64, t64087: f64, t64089: f64, t71343: f64, t71396: f64, t71428: f64, t71440: f64, t71467: f64, t51402: f64, t6024: f64, t21961: f64, t44162: f64, t21810: f64, t3259: f64, t50834: f64, t51137: f64, t63291: f64, t63306: f64, t63308: f64, t63841: f64, t63843: f64, t63845: f64, t71333: f64, t71335: f64, t71337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71486, t71489, t71494) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2544(t1113, t136, t71189, t71201, t71191, t71195, t71199, t71468, t71470, t71472, t71474, t71477, t71480, t71483);
        let (t71499, t71501, t71505, t71508, t71511, t71515) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2545(t1102, t5999, t14801, t14804, t45192, t48140, t68513, t50822, t44938, t43777, t43859, t43895, t50919, t50948, t71203, t71206);
        let t71527 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2546(t43816, t51040, t51051, t63361, t63382, t63384, t63398, t63400, t64074, t64076, t64087, t64089);
        let (t71530, t71543, t71545) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2547(t71343, t71396, t71428, t71440, t71467, t71494, t71515, t71527, t51402, t6024, t21961, t44162);
        let (t71547, t71558) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2548(t21810, t3259, t50834, t51137, t63291, t63306, t63308, t63841, t63843, t63845, t71333, t71335, t71337);
    (t71486, t71489, t71499, t71501, t71505, t71508, t71511, t71530, t71543, t71545, t71547, t71558)
}
