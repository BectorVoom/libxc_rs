//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3080/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3080(t18520: f64, t699: f64, t2403: f64, t6011: f64, t136: f64, t3297: f64, t63357: f64, t6014: f64, t1113: f64, t63363: f64, t44938: f64, t48140: f64, t55716: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63886 = t699 * t18520;
    let t63888 = t2403 * t6011;
    let t63891 = t136 * t3297 * t63357;
    let t63893 = t2403 * t6014;
    let t63896 = t136 * t1113 * t63363;
    let t63899 = t48140 * t44938 * t55716;
    (t63886, t63888, t63891, t63893, t63896, t63899)
}
