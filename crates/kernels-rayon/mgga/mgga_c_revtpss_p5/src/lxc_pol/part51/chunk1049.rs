//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1049/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1049(t120361: f64, t7150: f64, t127: f64, t32003: f64, t32004: f64, t371: f64, t32010: f64, t3215: f64, t31950: f64, t31951: f64, t31912: f64, t32013: f64) -> (f64, f64, f64, f64, f64) {
    let t120362 = t7150 * t120361;
    let t120368 = t32003 * t371 * t127 * t32004;
    let t120370 = t32010 * t3215;
    let t120374 = t31950 * t371 * t127 * t31951;
    let t120376 = t31912 * t32013;
    (t120362, t120368, t120370, t120374, t120376)
}
