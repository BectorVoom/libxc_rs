//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 607/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk607<F: Float>(t2899: F, t2901: F, t2903: F, t2905: F, t2907: F, t2915: F, t2921: F, t2926: F, t2930: F, t2935: F, t2941: F, t3369: F) -> (F, F) {
    let t3380 = F::cast_from(0.11197407407407407_f64) * t2899;
    let t3381 = -F::new(0.21595) * t2930 + F::new(0.21595) * t2935 - F::cast_from(0.07198333333333333_f64) * t2905 + F::cast_from(0.14396666666666666_f64) * t2921 - F::cast_from(0.07198333333333333_f64) * t2926 - F::cast_from(0.047988888888888886_f64) * t2901 + F::cast_from(0.035991666666666665_f64) * t2907 + F::cast_from(0.023994444444444443_f64) * t2903 - F::cast_from(0.03999074074074074_f64) * t2915 - F::cast_from(0.035991666666666665_f64) * t2941 - t3380;
    let t3382 = t3369 + t3381;
    (t3380, t3382)
}
