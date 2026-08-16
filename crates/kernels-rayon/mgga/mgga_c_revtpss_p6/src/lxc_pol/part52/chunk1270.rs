//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1270/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1270(t128594: f64, t128609: f64, t128625: f64, t128647: f64, t128664: f64, t128677: f64, t128688: f64, t128713: f64, t128742: f64, t128767: f64, t128781: f64, t128796: f64, t128810: f64, t128826: f64, t128837: f64, t128860: f64, t1450: f64, t2014: f64, t532: f64) -> f64 {
    let t128867 = t2014 * t532 * (t128594 + t128609 + t128625 + t128647 + t128664 + t128677 + t128688 + t128713 + t128742 + t128767 + t128781 + t128796 + t128810 + t128826 + t128837 + t128860) * t1450;
    t128867
}
