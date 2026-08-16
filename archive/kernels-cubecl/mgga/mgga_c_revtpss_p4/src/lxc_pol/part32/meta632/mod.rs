//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta632<F: Float>(t111407: F, t111416: F) -> F {
        let tv4rho3sigma7 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2055::<F>(t111407, t111416);
    tv4rho3sigma7
}
