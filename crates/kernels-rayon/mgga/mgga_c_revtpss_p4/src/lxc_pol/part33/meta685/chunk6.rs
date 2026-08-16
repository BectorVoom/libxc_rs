//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2269/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2269(t111690: f64, t111704: f64, t111717: f64, t111746: f64, t111762: f64, t111770: f64, t111796: f64, t113012: f64, t2172: f64, t6936: f64, t1921: f64, t8240: f64) -> (f64, f64, f64) {
    let t113015 = t111690 + t111704 + t111717 + t111746 + t111762 + t111770 + t111796 + t113012;
    let t113019 = t6936 * t2172;
    let t113022 = t8240 * t1921;
    (t113015, t113019, t113022)
}
