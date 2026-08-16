//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1299/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1299(t7583: f64, t88383: f64, t23384: f64, t28684: f64, t1920: f64, t28474: f64, t968: f64, t5914: f64, t6703: f64, t28492: f64, t28500: f64, t28648: f64, t82431: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99834 = t88383 * t7583;
    let t99864 = t23384 * t28684;
    let t99877 = t1920 * t968 * t28474;
    let t99895 = t6703 * t5914;
    let t99948 = t23384 * t28492;
    let t99956 = t23384 * t28500;
    let t99960 = t82431 * t28648;
    (t99834, t99864, t99877, t99895, t99948, t99956, t99960)
}
