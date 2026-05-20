//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta542<F: Float>(t2014: F, t29583: F, t2034: F, t22483: F, t30: F, t5966: F, t1963: F, t1544: F, t1583: F) -> (F, F, F, F, F, F) {
        let (t29585, t29589, t29590, t29591, t29592, t29598) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1927::<F>(t2014, t29583, t2034, t22483, t30, t5966, t1963, t1544, t1583);
    (t29585, t29589, t29590, t29591, t29592, t29598)
}
