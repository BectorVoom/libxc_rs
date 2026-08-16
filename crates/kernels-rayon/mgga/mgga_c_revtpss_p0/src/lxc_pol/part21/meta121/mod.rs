//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta121 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk785;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk786;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta121(t2881: f64, t2897: f64, t2889: f64, t923: f64, t240: f64, t68: f64, t281: f64, t283: f64, t698: f64, t931: f64, t1014: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t2898, t2900, t2902, t2904, t2905, t2906) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk785(t2881, t2897, t2889, t923, t240, t68, t281, t283, t698, t931);
        let t2908 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk786(t1014, t240);
    (t2898, t2900, t2902, t2904, t2905, t2906, t2908)
}
