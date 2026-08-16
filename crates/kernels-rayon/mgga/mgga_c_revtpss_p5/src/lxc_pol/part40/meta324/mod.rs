//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1106;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta324(t3369: f64, t689: f64, t3373: f64) -> (f64, f64) {
        let t12301 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1106(t3369, t689);
        let t12303 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1107(t3373, t689);
    (t12301, t12303)
}
