//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta192 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1133;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1134;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1135;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1136;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1137;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta192(t5657: f64, t858: f64, t1528: f64, t259: f64, t4147: f64, t4268: f64, t5559: f64, t5561: f64, t5632: f64, t5637: f64, t855: f64, t1530: f64, t193: f64, t202: f64, t2378: f64, t2423: f64, t2426: f64, t2486: f64, t2518: f64, t2530: f64, t2537: f64, t2665: f64, t2752: f64, t5527: f64, t5544: f64, t5596: f64, t5599: f64, t766: f64, t870: f64, t5526: f64, t2770: f64, t5392: f64, t2768: f64, t123: f64, t2775: f64, t882: f64, t5398: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5658 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1133(t5657, t858);
        let (t5660, t5664, t5668) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1134(t1528, t259, t4147, t4268, t5559, t5561, t5632, t5637, t5658, t855, t1530, t193, t202, t2378, t2423, t2426, t2486, t2518, t2530, t2537, t2665, t2752, t5527, t5544, t5596, t5599, t766, t870);
        let t5669 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1135(t5526, t5668);
        let t5677 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1136(t2770, t5392);
        let (t5678, t5679, t5681) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1137(t2768, t5677, t123, t2775, t5392);
        let (t5682, t5683, t5685) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1138(t5681, t882, t123, t5398, t883);
    (t5658, t5660, t5664, t5669, t5677, t5678, t5679, t5681, t5682, t5683, t5685)
}
