//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1138;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1139;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta346<F: Float>(t154: F, t1995: F, t205: F, t12247: F, t551: F, t236: F, t1336: F, t240: F, t3792: F, t10021: F, t1361: F, t22843: F, t241: F, t67: F, t1339: F, t2690: F, t3788: F, t6924: F, t246: F, t39037: F, t522: F, t2221: F, t3824: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40025, t40041, t40044, t40046, t40059, t40070) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1138::<F>(t154, t1995, t205, t12247, t551, t236, t1336, t240, t3792, t10021, t1361, t22843, t241, t67);
        let (t40123, t40159, t40168, t40224, t40227) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1139::<F>(t10021, t1336, t1339, t2690, t3788, t67, t6924, t246, t39037, t522, t2221, t3824);
    (t40025, t40041, t40044, t40046, t40059, t40070, t40123, t40159, t40168, t40224, t40227)
}
