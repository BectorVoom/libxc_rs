//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1156/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1156<F: Float>(t2497: F, t5305: F, t1972: F, t6387: F, t6391: F, t6268: F, t6395: F, t17734: F, t17736: F, t17738: F, t10321: F, t493: F, t6113: F, t6119: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20888 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5305 * t2497;
    let t20890 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1972 * t6387;
    let t20892 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1972 * t6391;
    let t20894 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t6268 * t6395;
    let t20895 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t17734;
    let t20896 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t17736;
    let t20897 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t17738;
    let t20898 = F::cast_from(8.0_f64) / F::cast_from(1215.0_f64) * t10321;
    let t20901 = t493 * t6119 * t6113 / F::cast_from(5.0_f64);
    (t20888, t20890, t20892, t20894, t20895, t20896, t20897, t20898, t20901)
}
