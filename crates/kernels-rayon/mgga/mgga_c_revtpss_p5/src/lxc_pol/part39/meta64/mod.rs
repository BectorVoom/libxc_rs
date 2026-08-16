//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta64 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk386;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta64(t1225: f64, t606: f64, t1012: f64, t1204: f64, t225: f64, t480: f64, t1209: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t1226, t1227, t1230, t1231, t1234) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk386(t1225, t606, t1012, t1204, t225, t480, t1209);
        let t1235 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk387(t1234, t480);
    (t1226, t1227, t1230, t1231, t1234, t1235)
}
