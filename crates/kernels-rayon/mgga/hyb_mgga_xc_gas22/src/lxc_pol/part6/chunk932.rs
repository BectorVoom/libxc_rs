//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 932/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk932(t139: f64, t8438: f64, t214: f64, t26: f64, t2950: f64, t765: f64, t1240: f64, t2018: f64, t3279: f64, t677: f64, t1319: f64, t1815: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8439 = t139 * t8438;
    let t8440 = t8439 * t214;
    let t8441 = t26 * t8440;
    let t8446 = t2950 * t765;
    let t8450 = t1240 * t2018 / 32.0_f64;
    let t8452 = t677 * t3279 / 32.0_f64;
    let t8453 = t1815 * t1319;
    (t8439, t8440, t8441, t8446, t8450, t8452, t8453)
}
