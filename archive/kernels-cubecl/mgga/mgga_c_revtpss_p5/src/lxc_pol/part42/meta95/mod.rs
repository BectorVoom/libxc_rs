//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk536;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta95<F: Float>(t2435: F, t781: F, t124: F, t68: F, t138: F) -> (F, F, F) {
        let (t2437, t2438, t2439) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk536::<F>(t2435, t781, t124, t68, t138);
    (t2437, t2438, t2439)
}
