//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2252;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta601<F: Float>(t12079: F, t24078: F, t1668: F, t3302: F, t357: F, t19572: F, t4982: F, t6299: F, t4893: F, t12168: F, t1651: F, t19556: F) -> (F, F, F, F, F, F, F) {
        let (t24079, t24083, t24084, t24089, t24090, t24093, t24098) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2252::<F>(t12079, t24078, t1668, t3302, t357, t19572, t4982, t6299, t4893, t12168, t1651, t19556);
    (t24079, t24083, t24084, t24089, t24090, t24093, t24098)
}
