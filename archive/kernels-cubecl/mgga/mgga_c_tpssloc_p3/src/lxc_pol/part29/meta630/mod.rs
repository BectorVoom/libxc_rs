//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta630<F: Float>(t12813: F, t1873: F, t3941: F, t55341: F, t12524: F, t26542: F, t22479: F, t5371: F, t66940: F, t7769: F, t55353: F, t7015: F) -> (F, F, F, F, F, F) {
        let (t86625, t86629, t86631, t86633, t86635, t86637) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2077::<F>(t12813, t1873, t3941, t55341, t12524, t26542, t22479, t5371, t66940, t7769, t55353, t7015);
    (t86625, t86629, t86631, t86633, t86635, t86637)
}
