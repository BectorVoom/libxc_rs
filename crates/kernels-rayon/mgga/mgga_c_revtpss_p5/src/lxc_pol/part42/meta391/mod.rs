//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1319;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1320;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1321;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1322;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1323;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1324;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta391(t6426: f64, t689: f64, t6430: f64, t1120: f64, t20272: f64, t128: f64, t12256: f64, t5819: f64, t606: f64, t12305: f64, t12268: f64, t3360: f64, t4186: f64, t5046: f64, t6421: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t20285 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1319(t6426, t689);
        let t20287 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1320(t6430, t689);
        let t20290 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1321(t1120, t20272, t128);
        let (t20293, t20295) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1322(t12256, t5819, t606, t12305, t128);
        let (t20298, t20300) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1323(t12268, t5819, t606, t3360, t128);
        let (t20302, t20304) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1324(t4186, t5046, t3360, t128);
        let (t20306, t20308) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1325(t606, t6421, t1120, t128);
    (t20285, t20287, t20290, t20293, t20295, t20298, t20300, t20302, t20304, t20306, t20308)
}
