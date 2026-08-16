//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta130 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk706;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta130(t2875: f64, t935: f64, t2874: f64, t273: f64, t276: f64, t918: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t916: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2876, t2878, t2880, t2881, t2882, t2884, t2889, t2890) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk706(t2875, t935, t2874, t273, t276, t918, t2846, t2848, t2855, t2860, t2864, t916);
    (t2876, t2878, t2880, t2881, t2882, t2884, t2889, t2890)
}
