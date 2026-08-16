//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk967;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk968;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk969;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk970;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta259(t8283: f64, param_d: f64, t116: f64, t2178: f64, t670: f64, t117: f64, t8273: f64, t1459: f64, t1461: f64, t2187: f64, t2189: f64, t572: f64, t573: f64, t1843: f64, t114: f64, t1513: f64, t8259: f64, t1504: f64, t8268: f64, t8257: f64, t8258: f64, t8267: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t8289 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk967(t8283, param_d);
        let t8295 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk968(t116, t2178);
        let (t8296, t8299, t8302, t8353) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk969(t670, t8295, t117, t8273, t1459, t1461, t2187, t2189, t572, t573, t8289, t1843, t2178);
        let (t8355, t8358, t8362) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk970(t114, t1513, t8259, t1504, t8268, t8257, t8258, t8267);
    (t8289, t8295, t8296, t8299, t8302, t8353, t8355, t8358, t8362)
}
