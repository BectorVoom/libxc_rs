//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1087/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1087(t32655: f64, t7741: f64, t28042: f64, t8692: f64, t13426: f64, t8460: f64, t18227: f64, t27123: f64, t28219: f64, t32392: f64, t7742: f64, t32394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t125381 = 4.0_f64 * t32655 * t7741;
    let t125383 = 4.0_f64 * t8692 * t28042;
    let t125384 = t13426 * t8460;
    let t125385 = 2.0_f64 * t125384;
    let t125386 = t18227 * t8460;
    let t125387 = 2.0_f64 * t125386;
    let t125388 = t27123 * t8460;
    let t125389 = 2.0_f64 * t125388;
    let t125390 = t28219 * t8460;
    let t125391 = 2.0_f64 * t125390;
    let t125405 = 4.0_f64 * t32392 * t7742;
    let t125407 = 4.0_f64 * t32394 * t7742;
    (t125381, t125383, t125385, t125387, t125389, t125391, t125405, t125407)
}
