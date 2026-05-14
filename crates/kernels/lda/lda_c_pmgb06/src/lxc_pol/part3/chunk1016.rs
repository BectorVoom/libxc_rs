//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1016/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1016<F: Float>(t13922: F, t2952: F, t439: F, t5482: F, t2970: F, t6494: F, t3104: F, t6498: F, t4619: F, t464: F, t2961: F, t1972: F, t2877: F, t2876: F, t493: F, t5486: F) -> (F, F, F, F, F, F, F) {
    let t13923 = 10.0 / 27.0 * t13922;
    let t13926 = t439 * t5482 * t2952 / 15.0;
    let t13929 = 2.0 / 15.0 * t439 * t6494 * t2970;
    let t13932 = t439 * t6498 * t3104 / 9.0;
    let t13933 = t4619 * t464;
    let t13936 = t439 * t13933 * t2961 / 9.0;
    let t13938 = 2.0 / 15.0 * t1972 * t2877;
    let t13941 = 2.0 / 15.0 * t493 * t5486 * t2876;
    (t13923, t13926, t13929, t13932, t13936, t13938, t13941)
}
