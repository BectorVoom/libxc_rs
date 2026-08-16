//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3024/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3024(t14362: f64, t9572: f64, t37: f64, t4391: f64, t14767: f64, t221: f64, t10703: f64, t2674: f64, t2661: f64, t2662: f64, t2754: f64, t4352: f64) -> (f64, f64, f64, f64) {
    let t50901 = t14362 * t9572;
    let t50903 = t37 * t4391;
    let t50931 = t221 * t14767;
    let t50933 = t2674 * t10703 * t50931;
    let t50937 = t2661 * t2662 * t4352 * t2754;
    (t50901, t50903, t50933, t50937)
}
