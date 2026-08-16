//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta136 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk660;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk661;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk662;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk663;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk664;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta136(t193: f64, t202: f64, t2378: f64, t2423: f64, t2426: f64, t2486: f64, t2518: f64, t2530: f64, t2537: f64, t2665: f64, t2752: f64, t5527: f64, t5544: f64, t5596: f64, t5599: f64, t5660: f64, t5664: f64, t766: f64, t870: f64, t5526: f64, t2770: f64, t5392: f64, t2768: f64, t123: f64, t2775: f64, t882: f64, t5398: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5668 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk660(t193, t202, t2378, t2423, t2426, t2486, t2518, t2530, t2537, t2665, t2752, t5527, t5544, t5596, t5599, t5660, t5664, t766, t870);
        let t5669 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk661(t5526, t5668);
        let t5677 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk662(t2770, t5392);
        let (t5678, t5679, t5681) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk663(t2768, t5677, t123, t2775, t5392);
        let (t5682, t5683, t5685) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk664(t5681, t882, t123, t5398, t883);
    (t5669, t5677, t5678, t5679, t5681, t5682, t5683, t5685)
}
