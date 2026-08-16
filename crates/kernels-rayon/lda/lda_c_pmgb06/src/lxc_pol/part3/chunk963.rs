//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 963/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk963(t11330: f64, t11336: f64, t11341: f64, t11344: f64, t11355: f64, t11357: f64, t11364: f64, t1227: f64, t2233: f64, t342: f64, t35: f64, t3559: f64, t3588: f64, t360: f64, t5756: f64, t5763: f64, t780: f64) -> f64 {
    let t11366 = -3.0_f64 / 2.0_f64 * t11330 + t11336 + 9.0_f64 / 2.0_f64 * t360 * t35 * t5763 * t342 - 8.81424_f64 * t11341 - t11344 + 9.0_f64 / 2.0_f64 * t360 * t35 * t2233 * t1227 + 3.0_f64 / 2.0_f64 * t360 * t35 * t780 * t3559 + t11355 - 17.62848_f64 * t11357 + 30.0_f64 * t360 * t35 * t5756 * t3588 + 44.0712_f64 * t11364;
    t11366
}
