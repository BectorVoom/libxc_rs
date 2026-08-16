//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta977 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta977<F: Float>(t162: F, t4403: F, t50903: F, t50089: F, t14331: F, t13312: F, t4401: F, t4402: F, t50880: F, t50883: F, t50888: F, t2609: F, t5944: F) -> (F, F, F, F, F, F, F) {
        let (t62290, t62293, t62296, t62297, t62298, t62299, t62300) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3286::<F>(t162, t4403, t50903, t50089, t14331, t13312, t4401, t4402, t50880, t50883, t50888, t2609, t5944);
    (t62290, t62293, t62296, t62297, t62298, t62299, t62300)
}
