//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 963/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk963<F: Float>(t11330: F, t11336: F, t11341: F, t11344: F, t11355: F, t11357: F, t11364: F, t1227: F, t2233: F, t342: F, t35: F, t3559: F, t3588: F, t360: F, t5756: F, t5763: F, t780: F) -> F {
    let t11366 = -F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t11330 + t11336 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t360 * t35 * t5763 * t342 - F::cast_from(8.81424_f64) * t11341 - t11344 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t360 * t35 * t2233 * t1227 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t360 * t35 * t780 * t3559 + t11355 - F::cast_from(17.62848_f64) * t11357 + F::cast_from(30.0_f64) * t360 * t35 * t5756 * t3588 + F::cast_from(44.0712_f64) * t11364;
    t11366
}
