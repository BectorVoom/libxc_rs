//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1163/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1163(t13426: f64, t8460: f64, t18227: f64, t27123: f64, t28219: f64, t28019: f64, t4147: f64, t8567: f64, t8995: f64, t28166: f64, t32110: f64, t7732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t125384 = t13426 * t8460;
    let t125385 = 2.0_f64 * t125384;
    let t125386 = t18227 * t8460;
    let t125387 = 2.0_f64 * t125386;
    let t125388 = t27123 * t8460;
    let t125389 = 2.0_f64 * t125388;
    let t125390 = t28219 * t8460;
    let t125391 = 2.0_f64 * t125390;
    let t125428 = t4147 * t28019;
    let t125478 = t8567 * t8995;
    let t125496 = t8567 * t28166;
    let t125507 = 2.0_f64 * t7732 * t32110;
    (t125385, t125387, t125389, t125391, t125428, t125478, t125496, t125507)
}
