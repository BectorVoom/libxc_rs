//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta102 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk553;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk554;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta102(t123: f64, t2434: f64, t781: f64, t124: f64, t68: f64, t138: f64) -> (f64, f64, f64, f64) {
        let t2435 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk553(t123, t2434);
        let (t2437, t2438, t2439) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk554(t2435, t781, t124, t68, t138);
    (t2435, t2437, t2438, t2439)
}
