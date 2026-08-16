//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta722 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2569;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2570;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2571;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2572;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2573;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2574;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2575;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2576;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2577;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta722(t1667: f64, t9709: f64, t14712: f64, t699: f64, t2403: f64, t4778: f64, t14750: f64, t690: f64, t14754: f64, t14745: f64, t2394: f64, t4725: f64, t14727: f64, t4730: f64, t14737: f64, t14741: f64, t14732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50846, t50848, t50853, t50903) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2569(t1667, t9709, t14712, t699, t2403, t4778, t14750, t690);
        let t50905 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2570(t14754, t690);
        let t50907 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2571(t14745, t690);
        let t50919 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2572(t2394, t4725);
        let t50921 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2573(t14727, t690);
        let t50948 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2574(t2394, t4730);
        let t50950 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2575(t14737, t690);
        let t50952 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2576(t14741, t690);
        let t50954 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2577(t14732, t690);
    (t50846, t50848, t50853, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t50952, t50954)
}
