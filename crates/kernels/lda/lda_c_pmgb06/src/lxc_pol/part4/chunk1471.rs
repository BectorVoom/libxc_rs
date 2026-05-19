//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1471/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1471<F: Float>(t395: F, t6104: F, t10902: F, t10934: F, t10937: F, t10940: F, t10943: F, t10946: F, t14696: F, t14699: F, t14702: F, t14705: F) -> F {
    let t18979 = t395 * t6104;
    let t18985 = -F::cast_from(0.14149184788746388_f64) * t10934 - F::cast_from(0.28298369577492777_f64) * t10937 - F::cast_from(0.14149184788746388_f64) * t10940 + F::cast_from(1.0376068845080684_f64) * t10943 + F::cast_from(1.0376068845080684_f64) * t10946 - F::cast_from(0.2133002709687175_f64) * t18979 - F::cast_from(0.5659673915498555_f64) * t14696 - F::cast_from(0.5659673915498555_f64) * t14699 - F::cast_from(0.5659673915498555_f64) * t14702 - F::cast_from(0.5659673915498555_f64) * t14705 - t10902;
    t18985
}
