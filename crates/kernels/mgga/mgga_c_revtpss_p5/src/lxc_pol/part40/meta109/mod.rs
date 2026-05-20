//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta109 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk571;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta109<F: Float>(t2435: F, t2439: F, t2502: F, t2504: F, t2509: F, t2511: F, t730: F, t722: F, t164: F, t172: F, t2538: F, t123: F, t147: F, t2434: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2548, t2549, t2552, t2553, t2554, t2555, t2556, t2557, t2562) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk571::<F>(t2435, t2439, t2502, t2504, t2509, t2511, t730, t722, t164, t172, t2538, t123, t147, t2434);
    (t2548, t2549, t2552, t2553, t2554, t2555, t2556, t2557, t2562)
}
