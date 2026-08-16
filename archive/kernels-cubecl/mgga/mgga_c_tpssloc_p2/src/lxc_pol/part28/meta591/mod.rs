//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1887;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta591<F: Float>(t1408: F, t2749: F, t13191: F, t25014: F, t13196: F, t13471: F, t25: F, t25373: F, t57921: F, t1530: F, t2249: F, t16596: F, t81547: F) -> (F, F, F, F, F, F, F) {
        let (t87961, t87978, t87981, t87984, t87988, t87994, t87998) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1887::<F>(t1408, t2749, t13191, t25014, t13196, t13471, t25, t25373, t57921, t1530, t2249, t16596, t81547);
    (t87961, t87978, t87981, t87984, t87988, t87994, t87998)
}
