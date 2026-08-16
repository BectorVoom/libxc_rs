//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2106/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2106(t29468: f64, t575: f64, t1464: f64, t8240: f64, t1921: f64, t7690: f64, t2167: f64, t5808: f64, t2172: f64, t5789: f64, t1913: f64, t7700: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t105792 = 2.0_f64 * t29468 * t575;
    let t105794 = 2.0_f64 * t8240 * t1464;
    let t105796 = 2.0_f64 * t7690 * t1921;
    let t105798 = 2.0_f64 * t2167 * t5808;
    let t105800 = 2.0_f64 * t5789 * t2172;
    let t105802 = 2.0_f64 * t1913 * t7700;
    (t105792, t105794, t105796, t105798, t105800, t105802)
}
