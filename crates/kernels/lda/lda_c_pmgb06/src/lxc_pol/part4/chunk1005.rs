//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1005/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1005<F: Float>(t123: F, t4435: F, t722: F, t14281: F, t199: F, t4429: F, t566: F, t2833: F, t868: F, t1152: F, t1808: F, t642: F, t902: F, t2164: F, t247: F, t1200: F, t2281: F) -> (F, F, F, F, F, F, F, F) {
    let t14672 = t123 * t722 * t4435;
    let t14696 = t123 * t14281 * t199;
    let t14699 = t123 * t4429 * t566;
    let t14702 = t123 * t2833 * t868;
    let t14705 = t123 * t1152 * t1808;
    let t14707 = t642 * t902;
    let t14709 = t247 * t2164;
    let t14712 = t123 * t2281 * t1200;
    (t14672, t14696, t14699, t14702, t14705, t14707, t14709, t14712)
}
