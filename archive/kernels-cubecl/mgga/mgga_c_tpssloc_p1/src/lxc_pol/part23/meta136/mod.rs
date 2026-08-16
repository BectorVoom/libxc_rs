//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta136 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk660;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk661;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk662;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk663;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk664;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta136<F: Float>(t193: F, t202: F, t2378: F, t2423: F, t2426: F, t2486: F, t2518: F, t2530: F, t2537: F, t2665: F, t2752: F, t5527: F, t5544: F, t5596: F, t5599: F, t5660: F, t5664: F, t766: F, t870: F, t5526: F, t2770: F, t5392: F, t2768: F, t123: F, t2775: F, t882: F, t5398: F, t883: F) -> (F, F, F, F, F, F, F, F) {
        let t5668 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk660::<F>(t193, t202, t2378, t2423, t2426, t2486, t2518, t2530, t2537, t2665, t2752, t5527, t5544, t5596, t5599, t5660, t5664, t766, t870);
        let t5669 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk661::<F>(t5526, t5668);
        let t5677 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk662::<F>(t2770, t5392);
        let (t5678, t5679, t5681) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk663::<F>(t2768, t5677, t123, t2775, t5392);
        let (t5682, t5683, t5685) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk664::<F>(t5681, t882, t123, t5398, t883);
    (t5669, t5677, t5678, t5679, t5681, t5682, t5683, t5685)
}
