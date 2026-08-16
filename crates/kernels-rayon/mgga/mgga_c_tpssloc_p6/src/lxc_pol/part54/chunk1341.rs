//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1341/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1341(t1992: f64, t550: f64, t6976: f64, t90946: f64, t22704: f64, t22705: f64, t32744: f64, t120437: f64, t1352: f64, t22633: f64, t26403: f64, t3807: f64) -> (f64, f64, f64, f64) {
    let t120456 = 0.16449340668482264365e-1_f64 * t1992 * t6976 * t90946 * t550;
    let t120458 = t22704 * t22705 * t32744;
    let t120459 = 0.82246703342411321825e-2_f64 * t120458;
    let t120463 = 0.3289868133696452873e-1_f64 * t22633 * t6976 * t120437 * t1352;
    let t120467 = 0.3289868133696452873e-1_f64 * t22633 * t6976 * t26403 * t3807;
    (t120456, t120459, t120463, t120467)
}
