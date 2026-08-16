//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1451/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1451(t31651: f64, t9656: f64, t4861: f64, t9634: f64, t3746: f64, t5204: f64, t3738: f64, t3785: f64, t11343: f64, t11349: f64, t11354: f64, t22754: f64, t26865: f64, t31051: f64, t31652: f64, t9632: f64, t9636: f64, t9639: f64, t9663: f64, t9670: f64, sigma0: f64) -> f64 {
    let t31656 = t31651 * t9656;
    let t31659 = t4861 * sigma0;
    let t31660 = t9634 * t31659;
    let t31663 = t5204 * t3746;
    let t31670 = t3785 * t3738;
    let t31685 = -704.0_f64 / 27.0_f64 * t9639 * t31656 - 6400.0_f64 / 81.0_f64 * t9639 * t31660 + 1600.0_f64 / 81.0_f64 * t31663 * t9636 - 6400.0_f64 / 243.0_f64 * t9663 * t31660 - 6400.0_f64 / 243.0_f64 * t9670 * t31660 - 256.0_f64 / 81.0_f64 * t31670 * t11343 + 1600.0_f64 / 81.0_f64 * t26865 * t11349 - 128.0_f64 / 27.0_f64 * t26865 * t11354 - 6400.0_f64 / 81.0_f64 * t9632 * t31660 + 704.0_f64 / 81.0_f64 * t9663 * t31652 - 704.0_f64 / 81.0_f64 * t9670 * t31656 - 64.0_f64 / 3.0_f64 * t22754 * t31051;
    t31685
}
