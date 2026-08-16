//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2095;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2096;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2097;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta448(t1614: f64, t2942: f64, t11354: f64, t1600: f64, t2881: f64, t11358: f64, t2880: f64, t4606: f64, t918: f64, t2889: f64, t4598: f64, t2897: f64, t4614: f64, t1606: f64, t2439: f64, t4580: f64, t689: f64, t4575: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15104, t15107, t15108, t15110, t15111, t15114, t15116, t15118) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2095(t1614, t2942, t11354, t1600, t2881, t11358, t2880, t4606, t918, t2889, t4598, t2897);
        let (t15119, t15121, t15123) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2096(t15118, t918, t2889, t4614, t1606, t2439);
        let t15125 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2097(t4580, t689);
        let t15127 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2098(t4575, t689);
    (t15104, t15107, t15108, t15110, t15111, t15114, t15116, t15119, t15121, t15123, t15125, t15127)
}
