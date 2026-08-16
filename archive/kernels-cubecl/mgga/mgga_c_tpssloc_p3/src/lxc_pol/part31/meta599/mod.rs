//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1844;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta599<F: Float>(t86928: F, t86940: F, t86942: F, t86950: F, t86967: F, t225: F, t26708: F, t87028: F, t87066: F, t87100: F, t87153: F, t87165: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t92415, t92425, t92426, t92431, t92434, t92439, t92486, t92491, t92502, t92515, t92530) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1844::<F>(t86928, t86940, t86942, t86950, t86967, t225, t26708, t87028, t87066, t87100, t87153, t87165);
    (t92415, t92425, t92426, t92431, t92434, t92439, t92486, t92491, t92502, t92515, t92530)
}
