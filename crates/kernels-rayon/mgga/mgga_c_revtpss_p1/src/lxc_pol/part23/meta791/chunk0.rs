//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2607/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2607(t18495: f64, t2652: f64, t18500: f64, t18493: f64, t221: f64, t2674: f64, t40683: f64, t18441: f64, t9775: f64, t18437: f64, t2661: f64, t2662: f64, t4352: f64, t4424: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61630 = t2652 * t18495;
    let t61632 = t2652 * t18500;
    let t61639 = t221 * t18493;
    let t61641 = t2674 * t40683 * t61639;
    let t61645 = t9775 * t18441;
    let t61660 = t2652 * t18437;
    let t61669 = t2661 * t2662 * t4352 * t4424;
    (t61630, t61632, t61641, t61645, t61660, t61669)
}
