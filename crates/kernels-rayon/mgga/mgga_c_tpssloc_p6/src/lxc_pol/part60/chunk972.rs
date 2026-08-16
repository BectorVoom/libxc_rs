//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 972/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk972(t118903: f64, t1880: f64, t28431: f64, t6553: f64, t6571: f64, t118678: f64, t1888: f64, t232: f64, t6646: f64, t98541: f64, t22996: f64, t2632: f64) -> (f64, f64, f64, f64, f64) {
    let t126423 = 0.16449340668482264365e-1_f64 * t118903;
    let t126427 = 0.16449340668482264365e-1_f64 * t1880 * t6553 * t6571 * t28431;
    let t126433 = 0.76763589786250567036e-1_f64 * t118678;
    let t126437 = 0.16449340668482264365e-1_f64 * t1888 * t6646 * t98541 * t232;
    let t126441 = 0.3289868133696452873e-1_f64 * t1888 * t22996 * t98541 * t2632;
    (t126423, t126427, t126433, t126437, t126441)
}
