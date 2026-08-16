//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta151 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk768;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta151(t2880: f64, t6113: f64, t2884: f64, t4571: f64, t6094: f64, t6098: f64, t6102: f64, t916: f64, t2897: f64, t923: f64, t2908: f64, t6092: f64, t141: f64, t6096: f64, t930: f64, t6100: f64, t2892: f64, t2905: f64, t4620: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6114, t6120) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk768(t2880, t6113, t2884, t4571, t6094, t6098, t6102);
        let (t6121, t6127, t6129, t6132, t6133, t6135, t6136, t6138, t6139, t6141) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk769(t6120, t916, t2897, t6113, t923, t2908, t6092, t141, t6096, t930, t6100, t2892, t2905, t4571, t4620, t6094, t6098, t6102, t6114);
    (t6114, t6120, t6121, t6127, t6129, t6132, t6133, t6135, t6136, t6138, t6139, t6141)
}
