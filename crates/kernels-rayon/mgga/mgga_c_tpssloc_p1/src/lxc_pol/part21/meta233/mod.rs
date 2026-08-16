//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1386;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1387;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1388;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1389;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1390;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1391;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1392;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta233(t5727: f64, t893: f64, t2844: f64, t5694: f64, t2842: f64, t2848: f64, t4335: f64, t5679: f64, t5683: f64, t5687: f64, t1568: f64, t932: f64, t2868: f64, t2875: f64, t4384: f64, t5699: f64, t5706: f64, t5712: f64, t5714: f64, t5718: f64, t5721: f64, t5724: f64, t2888: f64, t2892: f64, t324: f64, t1580: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5729, t5730) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1386(t5727, t893, t2844, t5694);
        let (t5732, t5737, t5742, t5743) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1387(t2842, t5730, t2848, t4335, t5679, t5683, t5687, t1568, t932);
        let t5758 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1388(t2868, t2875, t4335, t4384, t5679, t5683, t5687, t5699, t5706, t5712, t5714, t5718, t5721, t5724);
        let t5759 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1389(t5758, t932);
        let t5762 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1390(t2888, t5742);
        let t5769 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1391(t2892, t4335, t5679, t5683, t5687);
        let (t5770, t5774) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1392(t324, t5769, t1580);
    (t5729, t5730, t5732, t5737, t5742, t5743, t5758, t5759, t5762, t5769, t5770, t5774)
}
