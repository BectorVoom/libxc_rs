//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1801;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta569<F: Float>(t81912: F, t1878: F, t81982: F, t25120: F, t6604: F, t81962: F, t7500: F, t81911: F, t81928: F, t81934: F, t81943: F, t22690: F, t23122: F, t4119: F, t841: F) -> (F, F, F, F, F, F, F, F) {
        let (t87414, t87420, t87425, t87432, t87437, t87438, t87440, t87443) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1801::<F>(t81912, t1878, t81982, t25120, t6604, t81962, t7500, t81911, t81928, t81934, t81943, t22690, t23122, t4119, t841);
    (t87414, t87420, t87425, t87432, t87437, t87438, t87440, t87443)
}
