//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1020/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1020(t7573: f64, t9503: f64, t2824: f64, t3727: f64, t3739: f64, t3747: f64, t3753: f64, t3757: f64, t7806: f64, t9508: f64, t9513: f64, t9521: f64, t9523: f64, t9527: f64, t9528: f64, t9533: f64, t9535: f64, t9538: f64, t9542: f64, t9545: f64, t9549: f64, t9552: f64, t9558: f64, sigma2: f64) -> (f64, f64, f64) {
    let t9561 = t7573 * sigma2;
    let t9562 = t9561 * t9503;
    let t9565 = 176.0_f64 / 81.0_f64 * t3747 * t9513 + 352.0_f64 / 243.0_f64 * t3753 * t9508 + 176.0_f64 / 81.0_f64 * t3757 * t9513 - 200.0_f64 / 9.0_f64 * t9521 * t9523 - 16.0_f64 / 3.0_f64 * t9527 * t9528 + 100.0_f64 / 3.0_f64 * t9533 * t9535 - 80.0_f64 / 3.0_f64 * t9538 * t3727 * t2824 - 500.0_f64 / 3.0_f64 * t9542 * t9523 + 32.0_f64 * t7806 * t9545 + 200.0_f64 * t9549 * t9535 + 32.0_f64 * t7806 * t9552 - 200.0_f64 * t9549 * t9523 - 112.0_f64 / 3.0_f64 * t9558 * t9528 - 128.0_f64 / 81.0_f64 * t3739 * t9562;
    (t9561, t9562, t9565)
}
