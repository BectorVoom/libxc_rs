//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta751 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2519;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2520;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2521;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2522;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2523;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2524;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta751(t14735: f64, t5398: f64, t123: f64, t3240: f64, t16558: f64, t4723: f64, t14748: f64, t1088: f64, t18210: f64, t3966: f64, t21758: f64, t607: f64, t4728: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71181, t71183) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2519(t14735, t5398, t123, t3240);
        let (t71185, t71187) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2520(t16558, t4723, t123, t3240);
        let (t71189, t71191) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2521(t14748, t5398, t1088, t123);
        let (t71193, t71195) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2522(t18210, t3966, t123, t3240);
        let (t71197, t71199) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2523(t21758, t607, t123, t3240);
        let (t71201, t71203) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2524(t16558, t4728, t1088, t123);
    (t71181, t71183, t71185, t71187, t71189, t71191, t71193, t71195, t71197, t71199, t71201, t71203)
}
