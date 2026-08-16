//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1299;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta305<F: Float>(t251: F, t4503: F, t786: F, t2453: F, t2797: F, t231: F, t281: F, t68: F, t836: F, t2783: F, t860: F, t760: F, t9323: F, t9318: F, t2609: F, t717: F, t162: F, t9544: F, t158: F, t755: F, t9586: F, t2619: F, t2622: F, t2629: F, t9863: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10529, t10530, t10535, t10539, t10542, t10552) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1299::<F>(t251, t4503, t786, t2453, t2797, t231, t281, t68, t836, t2783, t860, t760, t9323);
        let (t10554, t10563, t10566, t10568, t10569, t10577) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1300::<F>(t760, t9318, t2609, t717, t162, t9544, t158, t755, t9586, t2619, t2622, t2629, t9863);
    (t10529, t10530, t10535, t10539, t10542, t10552, t10554, t10563, t10566, t10568, t10569, t10577)
}
