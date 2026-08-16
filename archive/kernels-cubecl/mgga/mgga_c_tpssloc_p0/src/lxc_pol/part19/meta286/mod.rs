//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1053;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1054;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta286<F: Float>(t12259: F, t1352: F, t3773: F, t68: F, t3850: F, t562: F, t12240: F, t3806: F, t5248: F, t1339: F, t836: F, t1336: F) -> (F, F, F, F, F, F) {
        let (t12260, t12267) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1053::<F>(t12259, t1352, t3773, t68);
        let (t12273, t12279, t12282, t12283) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1054::<F>(t3850, t562, t1352, t12240, t3806, t5248, t1339, t836, t1336);
    (t12260, t12267, t12273, t12279, t12282, t12283)
}
