//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2390/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2390(t10705: f64, t10716: f64, t10697: f64, t136: f64, t10627: f64, t221: f64, t2674: f64, t2452: f64, t9720: f64, t225: f64, t268: f64, t2665: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40681 = t10716 * t10705;
    let t40683 = t10697 * t136;
    let t40686 = t2674 * t40683 * t221 * t10627;
    let t40688 = t9720 * t2452;
    let t40689 = t40688 * t225;
    let t40690 = t268 * t40689;
    let t40691 = t40690 * t2665;
    (t40681, t40683, t40686, t40688, t40689, t40690, t40691)
}
