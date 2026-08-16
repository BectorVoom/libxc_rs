//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta46 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk287;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk288;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk289;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk290;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta46(t903: f64, t908: f64, t291: f64, t287: f64, t275: f64, t276: f64, t902: f64, t273: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t910, t912, t913, t914) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk287(t903, t908, t291, t287);
        let t915 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk288(t275, t914);
        let t916 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk289(t276);
        let t918 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk290(t902, t908);
        let (t919, t921, t923) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk291(t916, t918, t902, t273);
    (t910, t912, t913, t914, t915, t916, t918, t919, t921, t923)
}
