//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta138 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta138<F: Float>(t5705: F, t894: F, t2815: F, t5698: F, t901: F, t2826: F, t5677: F, t136: F, t5681: F, t908: F, t5685: F, t2810: F, t2823: F, t4335: F, t4384: F, t5679: F, t5683: F, t5687: F, t5699: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5706, t5712, t5714, t5717, t5718, t5720, t5721, t5723, t5724, t5726) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk668::<F>(t5705, t894, t2815, t5698, t901, t2826, t5677, t136, t5681, t908, t5685, t2810, t2823, t4335, t4384, t5679, t5683, t5687, t5699);
    (t5706, t5712, t5714, t5717, t5718, t5720, t5721, t5723, t5724, t5726)
}
