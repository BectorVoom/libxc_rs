//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 952/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk952(t8691: f64, t1329: f64, t2176: f64, t238: f64, t242: f64, t3309: f64, t779: f64, t226: f64, t8646: f64, t1342: f64, t2213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8692 = 0.33114e0_f64 * t8691;
    let t8693 = t2176 * t1329;
    let t8695 = t238 * t242 * t8693;
    let t8697 = t779 * t3309;
    let t8699 = t238 * t242 * t8697;
    let t8701 = t226 * t8646;
    let t8703 = t238 * t242 * t8701;
    let t8706 = t238 * t2213 * t1342;
    (t8692, t8693, t8695, t8697, t8699, t8701, t8703, t8706)
}
