//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1205;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta300(t251: f64, t4503: f64, t786: f64, t2453: f64, t2797: f64, t231: f64, t281: f64, t68: f64, t836: f64, t2783: f64, t860: f64, t760: f64, t9323: f64, t9318: f64, t2609: f64, t717: f64, t162: f64, t9544: f64, t158: f64, t755: f64, t9586: f64, t2619: f64, t2622: f64, t2629: f64, t9863: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10529, t10530, t10535, t10539, t10542, t10552) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1205(t251, t4503, t786, t2453, t2797, t231, t281, t68, t836, t2783, t860, t760, t9323);
        let (t10554, t10563, t10566, t10568, t10569, t10577) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1206(t760, t9318, t2609, t717, t162, t9544, t158, t755, t9586, t2619, t2622, t2629, t9863);
    (t10529, t10530, t10535, t10539, t10542, t10552, t10554, t10563, t10566, t10568, t10569, t10577)
}
