//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 964/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk964<F: Float>(t2571: F, t947: F, t2575: F, t350: F, t6221: F, t5044: F, t831: F, t5302: F, t802: F, t1848: F, t1933: F, t1423: F, t6491: F) -> (F, F, F, F, F, F, F) {
    let t15418 = t947 * t2571;
    let t15423 = t947 * t2575;
    let t15435 = t350 * t6221;
    let t15467 = t831 * t5044;
    let t15472 = t802 * t5302;
    let t15481 = t1848 * t1933;
    let t15519 = t1423 * t6491;
    (t15418, t15423, t15435, t15467, t15472, t15481, t15519)
}
