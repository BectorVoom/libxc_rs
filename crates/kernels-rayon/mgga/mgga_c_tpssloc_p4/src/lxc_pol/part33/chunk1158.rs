//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1158/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1158(t28130: f64, t6976: f64, t22633: f64, t19743: f64, t3792: f64, t22897: f64, t1992: f64, t6347: f64, t6968: f64, t6637: f64, t6888: f64, t6330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28131 = t6976 * t28130;
    let t28132 = t22633 * t28131;
    let t28134 = t19743 * t3792;
    let t28135 = t22897 * t28134;
    let t28136 = t1992 * t28135;
    let t28138 = t6968 * t6347;
    let t28139 = t6637 * t28138;
    let t28140 = t6888 * t28139;
    let t28142 = t6968 * t6330;
    (t28131, t28132, t28134, t28135, t28136, t28138, t28139, t28140, t28142)
}
