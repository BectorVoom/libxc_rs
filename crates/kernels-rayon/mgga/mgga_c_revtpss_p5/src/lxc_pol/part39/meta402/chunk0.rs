//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1474/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1474(t31032: f64, t8269: f64, t10208: f64, t69: f64, t2340: f64, t8259: f64, t101: f64, t43: f64, t665: f64, t658: f64, t8268: f64, t2366: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31033 = t31032 * t8269;
    let t31035 = t69 * t10208;
    let t31036 = t8259 * t2340;
    let t31039 = t43 * t101;
    let t31040 = t31039 * t665;
    let t31043 = t665 * t658;
    let t31044 = t8268 * t31043;
    let t31047 = t8259 * t2366;
    (t31033, t31035, t31036, t31039, t31040, t31043, t31044, t31047)
}
