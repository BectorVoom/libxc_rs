//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 224/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk224(t25: f64, t596: f64, t88: f64, t90: f64, t29: f64, t17: f64, t2: f64, t579: f64, t66: f64) -> (f64, f64, f64, f64, f64) {
    let t598 = 6.0_f64 * t25 * t596;
    let t602 = 1.0_f64 / t90 / t88;
    let t603 = t29 * t602;
    let t604 = t2 * t17;
    let t624 = 1.0_f64 / t66 / t579;
    (t598, t602, t603, t604, t624)
}
