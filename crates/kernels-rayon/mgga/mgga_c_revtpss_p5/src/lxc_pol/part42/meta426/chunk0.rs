//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1488/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1488(t116926: f64, t8312: f64, t116929: f64, t8316: f64, t10241: f64, t104: f64, t46089: f64, t655: f64, t10199: f64, t2339: f64, t31027: f64, t31430: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t117184 = t116926 * t8312;
    let t117186 = t116929 * t8316;
    let t117218 = t104 * t10241;
    let t117461 = t46089 * t655;
    let t117544 = t10199 * t2339;
    let t117918 = 20.0_f64 / 9.0_f64 * t31027 * t31430;
    (t117184, t117186, t117218, t117461, t117544, t117918)
}
