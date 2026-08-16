//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1686/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1686(t225: f64, t3591: f64, t3482: f64, t3639: f64, t500: f64, t3696: f64, t588: f64, t592: f64, t1285: f64, t2223: f64, t1287: f64, t1291: f64, t9874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11925 = t3591 * t225;
    let t11928 = t3482 * t225;
    let t11947 = 1.0_f64 / t3639 / t500;
    let t11975 = t588 * t3696;
    let t11977 = t592 * t3696;
    let t11979 = t2223 * t1285;
    let t11981 = t2223 * t1287;
    let t11984 = 0.56968947174242584612e-3_f64 * t1291 * t9874;
    (t11925, t11928, t11947, t11975, t11977, t11979, t11981, t11984)
}
