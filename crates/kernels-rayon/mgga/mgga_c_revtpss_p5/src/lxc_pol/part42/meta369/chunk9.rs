//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1205/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1205(t10871: f64, t836: f64, t18426: f64, t4364: f64, t221: f64, t2485: f64, t5978: f64, t2484: f64, t10552: f64, t10554: f64, t14317: f64, t18261: f64, t18262: f64, t18265: f64, t18267: f64, t18300: f64, t18301: f64, t18308: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> (f64, f64, f64, f64) {
    let t18525 = t10871 * t836;
    let t18527 = t4364 * t18426 * t18525;
    let t18531 = t2485 * t221 * t5978;
    let t18532 = t2484 * t18531;
    let t18534 = t18261 + t18262 + t18265 + t18267 - t9278 + t9308 + t9316 + t9329 + t9333 + t18300 + t18301 + t14317 + t18308 - t10552 + t10554;
    (t18525, t18527, t18532, t18534)
}
