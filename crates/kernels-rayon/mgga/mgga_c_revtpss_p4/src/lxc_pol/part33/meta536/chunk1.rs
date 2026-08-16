//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1890/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1890(t5265: f64, t7618: f64, t1219: f64, t8172: f64, t5357: f64, t7607: f64, t5378: f64, t7624: f64, t1785: f64, t7623: f64) -> (f64, f64, f64, f64, f64) {
    let t29023 = t7618 * t5265;
    let t29027 = t8172 * t1219;
    let t29031 = t7607 * t5357;
    let t29034 = t7624 * t5378;
    let t29037 = t1785 * t7623;
    (t29023, t29027, t29031, t29034, t29037)
}
