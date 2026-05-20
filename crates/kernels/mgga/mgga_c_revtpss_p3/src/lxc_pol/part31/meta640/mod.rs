//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta640 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta640<F: Float>(t2045: F, t5789: F, t101451: F, t98141: F, t98148: F, t98161: F, t98165: F, t98200: F, t98218: F, t98220: F, t98224: F, t98260: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t101674, t101754, t102486, t102489, t102495, t102498, t102515, t102526, t102527, t102529, t102549) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2097::<F>(t2045, t5789, t101451, t98141, t98148, t98161, t98165, t98200, t98218, t98220, t98224, t98260);
    (t101674, t101754, t102486, t102489, t102495, t102498, t102515, t102526, t102527, t102529, t102549)
}
