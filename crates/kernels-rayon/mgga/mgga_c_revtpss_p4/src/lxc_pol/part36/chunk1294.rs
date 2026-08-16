//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1294/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1294(t29089: f64, t5357: f64, t21251: f64, t7607: f64, t21254: f64, t20842: f64, t7613: f64, t1234: f64, t30815: f64, t20816: f64, t7618: f64, t29020: f64, t5265: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t112433 = t29089 * t5357;
    let t112435 = t7607 * t21251;
    let t112437 = t7607 * t21254;
    let t112452 = t7613 * t20842;
    let t112456 = t1234 * t30815;
    let t112461 = t7618 * t20816;
    let t112465 = t29020 * t5265;
    (t112433, t112435, t112437, t112452, t112456, t112461, t112465)
}
