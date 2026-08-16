//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1048/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1048(t136: f64, t860: f64, t2457: f64, t2710: f64, t10069: f64, t2786: f64, t10073: f64, t10111: f64, t22: f64, t870: f64, t10115: f64, t253: f64) -> (f64, f64, f64, f64, f64) {
    let t10914 = t860 * t136;
    let t10916 = t2710 * t10914 * t2457;
    let t10923 = t10069 * t2786;
    let t10925 = t10073 * t2786;
    let t10939 = 0.19637199382202157274e-3_f64 * t10111 * t870 * t22;
    let t10948 = 0.11044544084478153697e-3_f64 * t10115 * t253;
    (t10916, t10923, t10925, t10939, t10948)
}
