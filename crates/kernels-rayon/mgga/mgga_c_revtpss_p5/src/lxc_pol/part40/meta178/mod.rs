//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta178 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk773;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk774;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk775;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk776;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta178(t1444: f64, t676: f64, t123: f64, t3915: f64, t1363: f64, t2470: f64, t1362: f64, t1398: f64, t543: f64, t1390: f64, t828: f64, t1386: f64, t820: f64, t843: f64, t1401: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3916, t3917, t3918, t3920, t3922, t3923) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk773(t1444, t676, t123, t3915, t1363, t2470, t1362, t1398);
        let t3924 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk774(t3923, t543);
        let (t3926, t3930) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk775(t1390, t3924, t828, t1386, t820, t843);
        let (t3931, t3934) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk776(t1401, t3930, t1386, t241, t820);
    (t3916, t3917, t3918, t3920, t3922, t3923, t3924, t3926, t3930, t3931, t3934)
}
