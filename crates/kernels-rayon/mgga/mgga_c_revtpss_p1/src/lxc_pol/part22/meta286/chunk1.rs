//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1700/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1700(t1444: f64, t2438: f64, t138: f64, t9674: f64, t4075: f64, t556: f64, t786: f64) -> (f64, f64, f64, f64, f64) {
    let t9675 = t2438 * t1444;
    let t9676 = t138 * t9675;
    let t9677 = t9674 * t9676;
    let t9679 = t556 * t4075;
    let t9680 = t786 * t9679;
    (t9675, t9676, t9677, t9679, t9680)
}
