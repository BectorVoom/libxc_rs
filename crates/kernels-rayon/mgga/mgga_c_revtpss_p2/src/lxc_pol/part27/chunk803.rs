//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 803/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk803(t1359: f64, t9292: f64, t1363: f64, t9288: f64, t1362: f64, t3911: f64, t3920: f64, t3957: f64, t3961: f64, t124: f64, t9628: f64, t800: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9691 = 0.17073386770573548589e-1_f64 * t9292 * t1359;
    let t9692 = t1363 * t9288;
    let t9694 = 0.30356481678079769392e-1_f64 * t1362 * t9692;
    let t9695 = t3911 * t3920;
    let t9697 = t3957 * t3961;
    let t9699 = t124 * t9628;
    let t9700 = t800 * t9699;
    (t9691, t9692, t9694, t9695, t9697, t9700)
}
