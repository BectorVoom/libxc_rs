//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 890/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk890<F: Float>(t224: F, t3145: F, t3120: F, t441: F, t1455: F, t3223: F, t1467: F, t1447: F, t3174: F, t3226: F, t1423: F, t3210: F) -> (F, F, F, F, F, F, F, F) {
    let t9370 = t3145 * t224;
    let t9373 = t441 * t3120;
    let t9379 = t3223 * t1455;
    let t9381 = t3223 * t1467;
    let t9383 = t1447 * t3174;
    let t9385 = t3226 * t1467;
    let t9393 = t3226 * t1455;
    let t9395 = t1423 * t3210;
    (t9370, t9373, t9379, t9381, t9383, t9385, t9393, t9395)
}
