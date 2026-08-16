//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta625<F: Float>(t19951: F, t22833: F, t19972: F, t19976: F, t5259: F, t91100: F, t26308: F, t5234: F, t5252: F, t6396: F, t80820: F, t19962: F) -> (F, F, F, F, F, F, F) {
        let (t97208, t97210, t97212, t97214, t97217, t97219, t97221) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1882::<F>(t19951, t22833, t19972, t19976, t5259, t91100, t26308, t5234, t5252, t6396, t80820, t19962);
    (t97208, t97210, t97212, t97214, t97217, t97219, t97221)
}
