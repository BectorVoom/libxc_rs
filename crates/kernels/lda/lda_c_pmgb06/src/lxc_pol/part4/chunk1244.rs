//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1244/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1244<F: Float>(t18649: F, t5874: F, t1227: F, t2707: F, t38: F, t11320: F, t11322: F, t11330: F, t11341: F, t11354: F, t11357: F, t11364: F, t11407: F, t18644: F, t18646: F, t2229: F, t4394: F, t63: F, t6989: F) -> (F, F, F) {
    let t18650 = t5874 * t18649;
    let t18656 = 5.84605 * t38 * t2707 * t1227;
    let t18663 = 4.0 * t11320 + 15.66976 * t11322 - t11330 + t18644 - 5.87616 * t11341 - t18646 + 8.0 / 3.0 * t11354 - 11.75232 * t11357 - 24.0 * t11407 * t18650 + 29.3808 * t11364 + t18656 - 29.3808 * t63 * t6989 * t1227 + 11.75232 * t63 * t2229 * t4394;
    (t18650, t18656, t18663)
}
