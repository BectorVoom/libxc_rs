//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta347<F: Float>(t40227: F, t12132: F, t592: F, t68: F, t6924: F, t1336: F, t1339: F, t2691: F, t10021: F, t154: F, t59: F, t3749: F) -> (F, F, F, F, F, F) {
        let (t40228, t40230, t40253, t40281, t40341, t40343) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1140::<F>(t40227, t12132, t592, t68, t6924, t1336, t1339, t2691, t10021, t154, t59, t3749);
    (t40228, t40230, t40253, t40281, t40341, t40343)
}
