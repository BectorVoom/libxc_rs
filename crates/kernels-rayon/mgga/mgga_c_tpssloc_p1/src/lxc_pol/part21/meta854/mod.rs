//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta854 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3087;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3088;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta854(t63953: f64, t63967: f64, t63980: f64, t63994: f64, t1100: f64, t45192: f64, t48140: f64, t55716: f64, t50822: f64, t4756: f64, t3287: f64, t50846: f64, t50848: f64, t50853: f64, t63918: f64, t63921: f64, t63924: f64, t63927: f64, t63930: f64, t63933: f64, t63936: f64, t63939: f64, t43855: f64, t43859: f64, t43861: f64, t43863: f64, t44027: f64, t50903: f64, t50905: f64, t50907: f64, t50919: f64, t50921: f64, t50948: f64, t50950: f64, t50952: f64, t50954: f64, t1107: f64, t1102: f64, t14804: f64, t14801: f64, t3270: f64, t1113: f64, t136: f64, t63353: f64, t43780: f64, t43782: f64, t43816: f64, t44053: f64, t50968: f64, t50970: f64, t50972: f64, t50978: f64, t51039: f64, t51041: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63996, t63997, t64003, t64006, t64008, t64009, t64011) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3087(t63953, t63967, t63980, t63994, t1100, t45192, t48140, t55716, t50822, t4756, t3287, t50846, t50848, t50853, t63918, t63921, t63924, t63927, t63930, t63933, t63936, t63939);
        let t64027 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3088(t43855, t43859, t43861, t43863, t44027, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t50952, t50954);
        let (t64028, t64031, t64033, t64042, t64045, t64049) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3089(t1107, t63996, t1102, t4756, t14804, t14801, t3270, t64008, t1113, t136, t63353, t43780, t43782, t43816, t44053, t50968, t50970, t50972, t50978, t51039, t51041);
    (t63997, t64003, t64006, t64009, t64011, t64027, t64028, t64031, t64033, t64042, t64045, t64049)
}
