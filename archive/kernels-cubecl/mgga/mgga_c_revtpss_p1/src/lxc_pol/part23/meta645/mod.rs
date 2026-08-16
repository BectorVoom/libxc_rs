//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2369;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta645<F: Float>(t40113: F, t760: F, t2523: F, t9419: F, t2598: F, t9321: F, t9387: F, t2495: F, t39875: F, t9367: F, t10565: F, t606: F, t706: F) -> (F, F, F, F, F, F, F, F) {
        let (t40115, t40121, t40129, t40131, t40132, t40135, t40137, t40139) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2369::<F>(t40113, t760, t2523, t9419, t2598, t9321, t9387, t2495, t39875, t9367, t10565, t606, t706);
    (t40115, t40121, t40129, t40131, t40132, t40135, t40137, t40139)
}
