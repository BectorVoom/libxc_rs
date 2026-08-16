//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta468<F: Float>(t1184: F, t52: F, t460: F, t24682: F, t3548: F, t7310: F, t2127: F, t3545: F, t2132: F, t607: F, t2136: F, t3535: F, t7338: F) -> (F, F, F, F, F, F, F) {
        let (t24683, t24684, t24685, t24690, t24704, t24712, t24716) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1758::<F>(t1184, t52, t460, t24682, t3548, t7310, t2127, t3545, t2132, t607, t2136, t3535, t7338);
    (t24683, t24684, t24685, t24690, t24704, t24712, t24716)
}
