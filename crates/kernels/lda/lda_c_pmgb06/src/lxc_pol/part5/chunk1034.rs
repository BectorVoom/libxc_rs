//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1034/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1034<F: Float>(t21337: F, t21356: F, t38: F, t56: F, t18585: F, t18589: F, t18615: F, t110: F, t7321: F, t360: F, t7317: F, t350: F, t365: F, t7278: F, t348: F, t7281: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t21358 = t21337 / 2.0 + t21356 / 2.0;
    let t21361 = 2.923025 * t38 * t56 * t21358;
    let t21366 = 8.769075 * t18585;
    let t21367 = 5.84605 * t18589;
    let t21369 = 2.923025 * t18615;
    let t21375 = t110 * t7321;
    let t21376 = t360 * t21375;
    let t21378 = t110 * t7317;
    let t21379 = t360 * t21378;
    let t21382 = t365 * t7278 * t350;
    let t21385 = t348 * t7281 * t350;
    (t21358, t21361, t21366, t21367, t21369, t21375, t21376, t21378, t21379, t21382, t21385)
}
