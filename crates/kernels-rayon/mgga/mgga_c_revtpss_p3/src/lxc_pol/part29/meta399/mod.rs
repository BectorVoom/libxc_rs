//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1435;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1436;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1437;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta399(t15098: f64, t2924: f64, t1596: f64, t2873: f64, t2876: f64, t1614: f64, t2942: f64, t11354: f64, t1600: f64, t2881: f64, t11358: f64, t2880: f64, t4606: f64, t918: f64, t2889: f64, t4598: f64, t2897: f64, t4614: f64, t1606: f64, t2439: f64, t4580: f64, t689: f64, t4575: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15100, t15103, t15104, t15108, t15111, t15113) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1435(t15098, t2924, t1596, t2873, t2876, t1614, t2942, t11354, t1600, t2881, t11358, t2880, t4606);
        let (t15114, t15116, t15119, t15121, t15123, t15125) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1436(t15113, t918, t2889, t4598, t2897, t4606, t4614, t1606, t2439, t4580, t689);
        let t15127 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1437(t4575, t689);
    (t15100, t15103, t15104, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15125, t15127)
}
