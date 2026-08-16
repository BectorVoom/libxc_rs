//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta120 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk660;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk661;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta120(t2904: f64, t698: f64, t931: f64, t1014: f64, t240: f64, t913: f64, t275: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t2905, t2906, t2908) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk660(t2904, t698, t931, t1014, t240);
        let (t2922, t2923, t2924) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk661(t913, t275);
    (t2905, t2906, t2908, t2922, t2923, t2924)
}
