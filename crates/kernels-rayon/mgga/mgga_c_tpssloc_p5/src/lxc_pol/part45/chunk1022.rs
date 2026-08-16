//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1022/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1022(t115550: f64, t22633: f64, t22635: f64, t31558: f64, t90506: f64, t1992: f64, t26989: f64, t3888: f64, t22716: f64, t8612: f64, t114178: f64, t114188: f64, t114193: f64, t114209: f64, t114217: f64, t114220: f64, t115540: f64, t115542: f64, t115547: f64, t1375: f64, t1385: f64, t2015: f64, t24138: f64, t31555: f64, t31601: f64, t31641: f64, t3758: f64, t3882: f64, t3887: f64) -> f64 {
    let t115551 = 0.82246703342411321824e-2_f64 * t115550;
    let t115554 = t22633 * t22635 * t31558 * t90506;
    let t115558 = t1992 * t22635 * t26989 * t3888;
    let t115566 = t22716 * t8612;
    let t115567 = 0.63969658155208805863e-1_f64 * t115566;
    let t115570 = -t114178 + 2.0_f64 * t1375 * t3887 * t24138 * t2015 - t115540 - 0.16449340668482264365e-1_f64 * t115542 + t114188 + 0.3289868133696452873e-1_f64 * t115547 + t115551 + t114193 - 0.6579736267392905746e-1_f64 * t115554 - 0.49348022005446793095e-1_f64 * t115558 + 4.0_f64 * t3758 * t31555 + 4.0_f64 * t1375 * t3887 * t31641 * t1385 + t115567 - t114209 + t114217 - t114220 + 4.0_f64 * t3882 * t31601;
    t115570
}
