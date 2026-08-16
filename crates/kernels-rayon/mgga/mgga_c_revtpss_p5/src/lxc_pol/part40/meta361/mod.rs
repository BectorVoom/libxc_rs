//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1262;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1263;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1264;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta361(t15113: f64, t918: f64, t2889: f64, t4598: f64, t2897: f64, t4606: f64, t4614: f64, t1606: f64, t2439: f64, t4580: f64, t689: f64, t4575: f64, t2852: f64, t4186: f64, t606: f64, t2850: f64, t128: f64, t2258: f64, t4573: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15114, t15116, t15119, t15121, t15123, t15125) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1262(t15113, t918, t2889, t4598, t2897, t4606, t4614, t1606, t2439, t4580, t689);
        let t15127 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1263(t4575, t689);
        let (t15128, t15130, t15132) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1264(t15127, t2852, t4186, t606, t2850, t128);
        let (t15135, t15137) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1265(t2258, t4573, t2850, t128);
    (t15114, t15116, t15119, t15121, t15123, t15125, t15127, t15128, t15130, t15132, t15135, t15137)
}
