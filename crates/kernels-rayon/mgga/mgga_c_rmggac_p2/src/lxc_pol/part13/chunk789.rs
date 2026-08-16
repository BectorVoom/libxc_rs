//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 789/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk789(t22: f64, t235: f64, t36632: f64, t504: f64, t7191: f64, t36012: f64, t903: f64, t1179: f64, t1966: f64, t1968: f64, t483: f64, t7367: f64) -> (f64, f64, f64, f64, f64) {
    let t36634 = t235 * t36632 * t22;
    let t36639 = t504 * t7191;
    let t36646 = t903 * t36012;
    let t36662 = t1966 * t1179 * t483 * t1968;
    let t36663 = t36662 * t7367;
    (t36634, t36639, t36646, t36662, t36663)
}
