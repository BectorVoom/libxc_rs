//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta644 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta644<F: Float>(t12283: F, t19962: F, t19882: F, t19996: F, t3866: F, t40018: F, t6371: F, t12189: F, t6375: F, t40138: F, t6396: F, t19951: F) -> (F, F, F, F, F, F, F) {
        let (t56933, t56935, t56937, t56946, t56953, t56959, t56961) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2184::<F>(t12283, t19962, t19882, t19996, t3866, t40018, t6371, t12189, t6375, t40138, t6396, t19951);
    (t56933, t56935, t56937, t56946, t56953, t56959, t56961)
}
