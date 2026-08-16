//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta41 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk292;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk293;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta41(t287: f64, t275: f64, t276: f64, t902: f64, t273: f64, t240: f64, t696: f64, t281: f64, t283: f64, t346: f64, t290: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t913, t914, t915, t916, t921, t923, t926, t928, t929, t930) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk292(t287, t275, t276, t902, t273, t240, t696, t281, t283, t346);
        let t935 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk293(t290);
    (t913, t914, t915, t916, t921, t923, t926, t928, t929, t930, t935)
}
