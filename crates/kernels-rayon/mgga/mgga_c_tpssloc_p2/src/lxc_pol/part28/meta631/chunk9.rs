//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1986/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1986(t87581: f64, t87583: f64, t2047: f64, t4233: f64, t87601: f64, t87603: f64, t13176: f64, t24270: f64, t2617: f64, t26608: f64, t26656: f64, t4166: f64, t4281: f64, t4291: f64, t7102: f64, t81656: f64, t81670: f64, t81691: f64, t829: f64, t84995: f64, t87575: f64, t87578: f64, t87589: f64, t87609: f64, t9632: f64) -> (f64, f64) {
    let t92738 = 0.16449340668482264365e-1_f64 * t87581;
    let t92739 = 0.15352717957250113407e0_f64 * t87583;
    let t92745 = t2047 * t4233;
    let t92749 = 0.16449340668482264365e-1_f64 * t87601;
    let t92754 = 0.15352717957250113407e0_f64 * t87603;
    let t92759 = 0.3289868133696452873e-1_f64 * t81656 - 0.3289868133696452873e-1_f64 * t87575 - 0.16449340668482264365e-1_f64 * t87578 + t92738 - t92739 + 0.16449340668482264365e-1_f64 * t81670 - 0.6579736267392905746e-1_f64 * t87589 + 2.0_f64 * t4281 * t26656 * t9632 - 2.0_f64 * t4291 * t92745 * t829 + t92749 - 2.0_f64 * t13176 * t7102 - 2.0_f64 * t4166 * t24270 + t92754 - 2.0_f64 * t2617 * t26608 - t84995 + 0.82246703342411321825e-2_f64 * t81691 + 0.3289868133696452873e-1_f64 * t87609;
    (t92745, t92759)
}
