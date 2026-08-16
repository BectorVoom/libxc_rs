//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta854 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3087;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3088;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta854<F: Float>(t63953: F, t63967: F, t63980: F, t63994: F, t1100: F, t45192: F, t48140: F, t55716: F, t50822: F, t4756: F, t3287: F, t50846: F, t50848: F, t50853: F, t63918: F, t63921: F, t63924: F, t63927: F, t63930: F, t63933: F, t63936: F, t63939: F, t43855: F, t43859: F, t43861: F, t43863: F, t44027: F, t50903: F, t50905: F, t50907: F, t50919: F, t50921: F, t50948: F, t50950: F, t50952: F, t50954: F, t1107: F, t1102: F, t14804: F, t14801: F, t3270: F, t1113: F, t136: F, t63353: F, t43780: F, t43782: F, t43816: F, t44053: F, t50968: F, t50970: F, t50972: F, t50978: F, t51039: F, t51041: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t63996, t63997, t64003, t64006, t64008, t64009, t64011) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3087::<F>(t63953, t63967, t63980, t63994, t1100, t45192, t48140, t55716, t50822, t4756, t3287, t50846, t50848, t50853, t63918, t63921, t63924, t63927, t63930, t63933, t63936, t63939);
        let t64027 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3088::<F>(t43855, t43859, t43861, t43863, t44027, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t50952, t50954);
        let (t64028, t64031, t64033, t64042, t64045, t64049) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3089::<F>(t1107, t63996, t1102, t4756, t14804, t14801, t3270, t64008, t1113, t136, t63353, t43780, t43782, t43816, t44053, t50968, t50970, t50972, t50978, t51039, t51041);
    (t63997, t64003, t64006, t64009, t64011, t64027, t64028, t64031, t64033, t64042, t64045, t64049)
}
