//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1812;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta341(t287: f64, t2922: f64, t275: f64, t11132: f64, t2912: f64, t698: f64, t240: f64, t624: f64, t281: f64, t283: f64, t2909: f64, t3252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11298, t11299) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1812(t287, t2922, t275);
        let (t11304, t11326, t11334, t11335, t11337, t11338, t11339, t11341) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1813(t11132, t2912, t698, t240, t624, t281, t283, t2909, t3252);
    (t11298, t11299, t11304, t11326, t11334, t11335, t11337, t11338, t11339, t11341)
}
