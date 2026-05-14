//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1012/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1012<F: Float>(t1972: F, t6391: F, t6268: F, t6395: F, t17734: F, t17736: F, t17738: F, t10321: F, t493: F, t6113: F, t6119: F, t10335: F, t17771: F, t20888: F, t20890: F, t1385: F, t439: F, t6217: F, t822: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20892 = 2.0 / 15.0 * t1972 * t6391;
    let t20894 = 4.0 / 15.0 * t6268 * t6395;
    let t20895 = 8.0 / 45.0 * t17734;
    let t20896 = 8.0 / 45.0 * t17736;
    let t20897 = 4.0 / 27.0 * t17738;
    let t20898 = 8.0 / 1215.0 * t10321;
    let t20901 = t493 * t6119 * t6113 / 5.0;
    let t20902 = 8.0 / 1215.0 * t10335;
    let t20903 = t17771 / 15.0;
    let t20904 = -t20888 - t20890 - t20892 + t20894 + t20895 + t20896 - t20897 + t20898 + t20901 + t20902 + t20903;
    let t20914 = t439 * t1385 * t6217 * t822 / 15.0;
    (t20892, t20894, t20895, t20896, t20897, t20898, t20901, t20902, t20903, t20904, t20914)
}
