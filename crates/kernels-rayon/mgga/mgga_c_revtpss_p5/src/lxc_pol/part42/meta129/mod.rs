//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk627;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta129(t1129: f64, t408: f64) -> (f64, f64, f64) {
        let (t3431, t3432, t3433) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk627(t1129, t408);
    (t3431, t3432, t3433)
}
