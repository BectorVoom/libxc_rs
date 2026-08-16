//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta561<F: Float>(t1408: F, t2553: F, t10143: F, t606: F, t25374: F, t25365: F, t868: F, t25373: F, t58009: F, t4255: F, t22960: F, t59580: F) -> (F, F, F, F, F, F, F, F) {
        let (t86764, t86771, t86781, t86782, t86794, t86797, t86798, t86803) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1833::<F>(t1408, t2553, t10143, t606, t25374, t25365, t868, t25373, t58009, t4255, t22960, t59580);
    (t86764, t86771, t86781, t86782, t86794, t86797, t86798, t86803)
}
