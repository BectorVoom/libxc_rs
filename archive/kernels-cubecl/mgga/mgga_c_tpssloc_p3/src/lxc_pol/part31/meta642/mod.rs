//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta642 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta642<F: Float>(t28248: F, t81547: F, t5660: F, t606: F, t17109: F, t25: F, t5664: F, t5397: F, t776: F, t868: F, t25373: F, t23168: F, t28288: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98079, t98082, t98086, t98091, t98094, t98102, t98103, t98111, t98112, t98117) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1910::<F>(t28248, t81547, t5660, t606, t17109, t25, t5664, t5397, t776, t868, t25373, t23168, t28288);
    (t98079, t98082, t98086, t98091, t98094, t98102, t98103, t98111, t98112, t98117)
}
