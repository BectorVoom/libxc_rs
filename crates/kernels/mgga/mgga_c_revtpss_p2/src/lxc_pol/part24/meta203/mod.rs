//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta203<F: Float>(t760: F, t9318: F, t162: F, t9544: F, t158: F, t755: F, t9586: F, t2629: F, t9863: F, t9866: F, t9575: F, t9572: F) -> (F, F, F, F, F, F, F, F) {
        let (t10554, t10565, t10566, t10568, t10577, t10582, t10584, t10586) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk937::<F>(t760, t9318, t162, t9544, t158, t755, t9586, t2629, t9863, t9866, t9575, t9572);
    (t10554, t10565, t10566, t10568, t10577, t10582, t10584, t10586)
}
