//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1013/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1013<F: Float>(t13883: F, t1447: F, t4728: F, t4732: F, t13861: F, t13863: F, t13865: F, t13867: F, t13869: F, t13872: F, t13875: F, t13878: F, t13882: F, t5442: F, t5499: F, t1916: F, t3226: F) -> (F, F, F, F, F, F) {
    let t13884 = 4.0 / 45.0 * t13883;
    let t13885 = t1447 * t4728;
    let t13886 = 4.0 / 45.0 * t13885;
    let t13887 = t1447 * t4732;
    let t13888 = 2.0 / 45.0 * t13887;
    let t13889 = t13861 - t13863 - t13865 - t13867 + t13869 - t13872 - t13875 + t13878 - t13882 - t13884 - t13886 - t13888;
    let t13891 = t5499 * t5442;
    let t13892 = 2.0 / 9.0 * t13891;
    let t13893 = t3226 * t1916;
    (t13884, t13886, t13888, t13889, t13892, t13893)
}
