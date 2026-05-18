//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 961/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk961<F: Float>(t409: F, t419: F, t421: F, t6716: F, t1186: F, t7155: F, t1447: F, t6744: F, t6748: F, t6791: F, t1423: F, t6775: F) -> (F, F, F, F, F, F) {
    let t15163 = t409 * t6716 * t419 * t421;
    let t15166 = t7155 * t1186 * t421;
    let t15180 = t1447 * t6744;
    let t15182 = t1447 * t6748;
    let t15184 = t1447 * t6791;
    let t15189 = t1423 * t6775;
    (t15163, t15166, t15180, t15182, t15184, t15189)
}
