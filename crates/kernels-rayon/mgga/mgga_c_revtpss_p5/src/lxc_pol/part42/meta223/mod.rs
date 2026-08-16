//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta223 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk864;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk865;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk866;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk867;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta223(t45: f64, t57: f64, t4399: f64, t5819: f64, t5825: f64, t766: f64, t80: f64, t770: f64, t83: f64, zeta_threshold: f64, t1544: f64, t4546: f64, t1558: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t5948, t5962) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk864(t45, t57, t4399, t5819, t5825, t766, t80, t770, t83, zeta_threshold);
        let t5966 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk865(t1544);
        let (t5970, t5977) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk866(t1544, t4546, t1558);
        let t5978 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk867(t231, t5977);
    (t5948, t5962, t5966, t5970, t5977, t5978)
}
