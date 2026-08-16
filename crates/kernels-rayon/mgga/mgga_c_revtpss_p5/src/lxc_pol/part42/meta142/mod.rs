//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta142 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk660;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk661;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta142(t1364: f64, t3911: f64, t1426: f64, t556: f64, t786: f64, t1444: f64, t676: f64, t123: f64, t1363: f64, t2470: f64, t1362: f64, t1386: f64, t820: f64, t843: f64, t1401: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3912, t3914, t3915) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk660(t1364, t3911, t1426, t556, t786);
        let (t3916, t3917, t3918, t3920, t3922, t3930) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk661(t1444, t676, t123, t3915, t1363, t2470, t1362, t1386, t820, t843);
        let (t3931, t3934) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk662(t1401, t3930, t1386, t241, t820);
    (t3912, t3914, t3915, t3916, t3917, t3918, t3920, t3922, t3930, t3931, t3934)
}
