//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3048/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3048(t14593: f64, t2470: f64, t874: f64, t1558: f64, t2482: f64, t2801: f64, t2815: f64, t10547: f64, t14606: f64, t10538: f64, t14605: f64, t49180: f64) -> (f64, f64, f64, f64) {
    let t51587 = t874 * t14593 * t2470;
    let t51598 = t2482 * t2815 * t1558 * t2801;
    let t51600 = t14606 * t10547;
    let t51603 = t49180 * t14605 * t10538;
    (t51587, t51598, t51600, t51603)
}
