//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta148 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk755;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta148<F: Float>(t2932: F, t950: F, t2978: F, t60: F, t344: F, t2987: F, t340: F, t974: F, t247: F, t375: F, t1043: F, t2775: F, t2770: F, t3061: F, t1022: F, t3131: F, t3188: F, t1932: F, t360: F, t193: F, t336: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4497, t4509, t4510, t4518, t4546, t4582) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk755::<F>(t2932, t950, t2978, t60, t344, t2987, t340, t974, t247, t375);
        let (t4583, t4588, t4594, t4673, t4684, t4700) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk756::<F>(t1043, t2775, t2770, t3061, t1022, t3131, t3188, t1932, t360, t193, t336);
    (t4497, t4509, t4510, t4518, t4546, t4582, t4583, t4588, t4594, t4673, t4684, t4700)
}
