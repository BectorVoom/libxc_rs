//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta214 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1021;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta214<F: Float>(t2826: F, t5677: F, t136: F, t5681: F, t908: F, t5685: F, t2810: F, t2823: F, t4335: F, t4384: F, t5679: F, t5683: F, t5687: F, t5699: F, t5706: F, t5712: F, t5714: F) -> (F, F, F, F, F, F, F) {
        let (t5717, t5718, t5720, t5721, t5723, t5724, t5726) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1021::<F>(t2826, t5677, t136, t5681, t908, t5685, t2810, t2823, t4335, t4384, t5679, t5683, t5687, t5699, t5706, t5712, t5714);
    (t5717, t5718, t5720, t5721, t5723, t5724, t5726)
}
