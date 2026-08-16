//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1184/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1184(t348: f64, t350: f64, t7281: f64, t7307: f64, t365: f64, t7310: f64, t21358: f64, t35: f64, t64: f64, t18625: f64, t18628: f64, t18630: f64, t21376: f64, t21379: f64, t21382: f64, t360: f64, t8263: f64) -> (f64, f64, f64, f64) {
    let t21385 = t348 * t7281 * t350;
    let t21386 = 2.923025_f64 * t21385;
    let t21388 = t348 * t7307 * t350;
    let t21389 = 0.48717083333333333_f64 * t21388;
    let t21391 = t365 * t7310 * t350;
    let t21394 = t35 * t64 * t21358;
    let t21397 = -3.0_f64 * t18625 + 44.0712_f64 * t18628 - 17.62848_f64 * t18630 + 2.0_f64 * t21376 + t21379 / 6.0_f64 + 14.6904_f64 * t21382 + t21386 + t21389 + 0.73452_f64 * t21391 - t360 * t21394 / 2.0_f64 + t8263;
    (t21386, t21389, t21394, t21397)
}
