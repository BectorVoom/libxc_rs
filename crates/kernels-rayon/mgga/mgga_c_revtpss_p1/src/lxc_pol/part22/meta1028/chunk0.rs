//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3604/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3604(t12254: f64, t141: f64, t68265: f64, t43881: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64, t68287: f64, t68292: f64) -> (f64, f64) {
    let t68402 = t141 * t12254 * t68265;
    let t68415 = 4.0_f64 / 3.0_f64 * t68253 + 4.0_f64 / 27.0_f64 * t68255 - 8.0_f64 / 81.0_f64 * t68257 + t43881 - 20.0_f64 / 81.0_f64 * t68262 + 10.0_f64 / 27.0_f64 * t68267 + 8.0_f64 * t68271 + 4.0_f64 / 3.0_f64 * t68275 - 4.0_f64 / 9.0_f64 * t68277 - 4.0_f64 / 9.0_f64 * t68282 - 2.0_f64 / 9.0_f64 * t68287 - 4.0_f64 / 3.0_f64 * t68292;
    (t68402, t68415)
}
