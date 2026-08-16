//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1303/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1303(t1851: f64, t8299: f64, t30581: f64, t580: f64, t2212: f64, t6470: f64, t110919: f64, t111289: f64, t111291: f64, t111293: f64, t111842: f64, t112062: f64, t1396: f64, t1398: f64, t1858: f64, t20149: f64, t30350: f64, t30616: f64, t6471: f64, t6483: f64, t8200: f64, t8217: f64) -> f64 {
    let t112065 = t1851 * t8299;
    let t112073 = t30581 * t580;
    let t112074 = t6470 * t2212;
    let t112075 = t1398 * (t111842 + t112062) + 2.0_f64 * t112065 + t1396 * t30616 + t110919 + t20149 * t2212 + t111289 + t8200 * t6483 + t111291 + 2.0_f64 * t30350 * t1858 + t6471 * t8217 + t112073 + t112074 + t111293;
    t112075
}
