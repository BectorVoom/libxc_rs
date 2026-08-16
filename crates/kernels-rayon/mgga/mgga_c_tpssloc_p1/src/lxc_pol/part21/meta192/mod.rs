//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta192 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1199;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1200;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1201;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1202;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1203;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1204;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1205;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1206;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta192(t360: f64, t4649: f64, t1021: f64, t248: f64, t1020: f64, t1025: f64, t1041: f64, t1046: f64, t1618: f64, t1622: f64, t3104: f64, t3109: f64, t3114: f64, t3117: f64, t3140: f64, t3156: f64, t3160: f64, t3163: f64, t378: f64, t4617: f64, t4622: f64, t4625: f64, t4631: f64, t4636: f64, t4641: f64, t4644: f64, t4613: f64, t349: f64, t1626: f64, t225: f64, t1065: f64, t1634: f64, t3174: f64, t1057: f64, t4639: f64, t1022: f64, t3188: f64, t1629: f64, t1049: f64, t1615: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4650 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1199(t360, t4649);
        let t4652 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1200(t1021, t248, t4650);
        let t4656 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1201(t1020, t1025, t1041, t1046, t1618, t1622, t3104, t3109, t3114, t3117, t3140, t3156, t3160, t3163, t378, t4617, t4622, t4625, t4631, t4636, t4641, t4644, t4652);
        let t4657 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1202(t4613, t4656);
        let (t4658, t4660) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1203(t349, t4657, t1626, t225);
        let t4665 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1204(t1065, t1634, t3174);
        let t4669 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1205(t1057, t4639);
        let t4673 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1206(t1022, t3188);
        let (t4674, t4677) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1207(t1629, t4673, t1049, t1615);
    (t4650, t4652, t4657, t4658, t4660, t4665, t4669, t4673, t4674, t4677)
}
