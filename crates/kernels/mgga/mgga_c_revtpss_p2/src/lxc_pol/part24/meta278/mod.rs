//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1052;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta278<F: Float>(t18742: F, t2782: F, t18681: F, t231: F, t2783: F, t18677: F, t2723: F, t4503: F, t6041: F, t72: F, t686: F, t874: F) -> (F, F, F, F, F, F, F) {
        let (t18743, t18746, t18747, t18750, t18751, t18761, t18763) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1052::<F>(t18742, t2782, t18681, t231, t2783, t18677, t2723, t4503, t6041, t72, t686, t874);
    (t18743, t18746, t18747, t18750, t18751, t18761, t18763)
}
