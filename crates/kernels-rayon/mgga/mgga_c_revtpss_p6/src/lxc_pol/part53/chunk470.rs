//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 470/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk470(t136: f64, t854: f64, t221: f64, t775: f64, t2674: f64, t26: f64, t66: f64, t240: f64, t243: f64, t247: f64, t237: f64, t124: f64, t212: f64, t596: f64, t800: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2675 = t854 * t136;
    let t2677 = t2675 * t221 * t775;
    let t2678 = t2674 * t2677;
    let t2681 = 1.0_f64 / t66 / t26;
    let t2682 = t2681 * t240;
    let t2684 = t2682 * t243 * t247;
    let t2686 = 0.56688979511669985553e-2_f64 * t237 * t2684;
    let t2689 = t800 * t124 * t596 * t212;
    (t2675, t2677, t2678, t2681, t2682, t2684, t2686, t2689)
}
