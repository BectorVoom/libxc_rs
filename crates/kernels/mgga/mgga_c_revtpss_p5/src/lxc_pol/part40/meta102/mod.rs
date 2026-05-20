//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta102 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk554;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta102<F: Float>(t123: F, t2434: F, t781: F, t124: F, t68: F, t138: F) -> (F, F, F, F) {
        let t2435 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk554::<F>(t123, t2434);
        let (t2437, t2438, t2439) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk555::<F>(t2435, t781, t124, t68, t138);
    (t2435, t2437, t2438, t2439)
}
