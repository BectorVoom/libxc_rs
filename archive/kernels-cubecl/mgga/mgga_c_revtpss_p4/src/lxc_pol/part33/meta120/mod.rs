//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta120 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk690;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk691;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta120<F: Float>(t2904: F, t698: F, t931: F, t1014: F, t240: F, t913: F, t275: F) -> (F, F, F, F, F, F) {
        let (t2905, t2906, t2908) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk690::<F>(t2904, t698, t931, t1014, t240);
        let (t2922, t2923, t2924) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk691::<F>(t913, t275);
    (t2905, t2906, t2908, t2922, t2923, t2924)
}
