//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1042;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1043;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta290<F: Float>(t10521: F, t231: F, t268: F, t2798: F, t251: F, t4503: F, t786: F, t2723: F, t2453: F, t2797: F, t281: F, t68: F, t836: F, t2783: F, t860: F, t2801: F, t2645: F, t675: F, t760: F, t9323: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10524, t10529, t10533, t10535, t10538) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1042::<F>(t10521, t231, t268, t2798, t251, t4503, t786, t2723, t2453, t2797, t281, t68, t836);
        let (t10539, t10542, t10543, t10548, t10552) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1043::<F>(t10535, t10538, t2783, t860, t786, t2801, t231, t2645, t268, t675, t2798, t760, t9323);
    (t10524, t10529, t10533, t10535, t10539, t10542, t10543, t10548, t10552)
}
