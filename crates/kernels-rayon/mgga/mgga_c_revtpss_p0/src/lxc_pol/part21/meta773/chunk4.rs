//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2749/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2749(t14738: f64, t2741: f64, t10845: f64, t14732: f64, t4423: f64, t853: f64, t2661: f64, t2662: f64, t2749: f64, t14718: f64, t14872: f64, t10777: f64, t10779: f64, t1548: f64, t2754: f64) -> (f64, f64, f64, f64, f64) {
    let t50579 = t2741 * t14738;
    let t50581 = t10845 * t14732;
    let t50582 = 0.40656002247428262579e-3_f64 * t50581;
    let t50583 = t853 * t4423;
    let t50586 = t2661 * t2662 * t50583 * t2749;
    let t50590 = t2661 * t2662 * t14718 * t14872;
    let t50594 = t10777 * t10779 * t1548 * t2754;
    (t50579, t50582, t50586, t50590, t50594)
}
