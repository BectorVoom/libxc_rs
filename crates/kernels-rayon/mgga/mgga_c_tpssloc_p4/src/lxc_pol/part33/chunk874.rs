//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 874/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk874(t118: f64, t5527: f64, t794: f64, t9549: f64, t5544: f64, t2576: f64, t2563: f64, t5555: f64, t252: f64, t5584: f64, t120: f64, t5593: f64, t9638: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16783 = t118 * t794 * t5527;
    let t16784 = t9549 * t16783;
    let t16791 = t118 * t794 * t5544;
    let t16792 = t2576 * t16791;
    let t16794 = t2563 * t5555;
    let t16815 = t252 * t5584;
    let t16839 = t120 * t5584;
    let t16848 = t9638 * t5593;
    (t16784, t16792, t16794, t16815, t16839, t16848)
}
