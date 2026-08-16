//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1097/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1097<F: Float>(t2918: F, t518: F, t12531: F, t5138: F, t2952: F, t5077: F, t5078: F, t9885: F, t9887: F, t1179: F, t132: F, t441: F, t4829: F) -> (F, F, F, F, F) {
    let t13068 = t518 * t2918;
    let t13071 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5138 * t13068 * t12531;
    let t13074 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5077 * t5078 * t2952;
    let t13075 = t9885 / F::cast_from(15.0_f64);
    let t13076 = t9887 / F::cast_from(15.0_f64);
    let t13079 = t132 * t1179 * t441 * t4829;
    (t13071, t13074, t13075, t13076, t13079)
}
