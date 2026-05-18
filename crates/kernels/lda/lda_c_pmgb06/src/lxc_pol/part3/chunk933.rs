//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 933/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk933<F: Float>(t1343: F, t2837: F, t421: F, t1334: F, t2803: F, t409: F, t419: F, t1186: F, t2826: F, t4244: F, t4247: F, t1166: F, t1179: F) -> (F, F, F, F, F, F, F) {
    let t10813 = t1343 * t2837 * t421;
    let t10817 = F::new(0.03950778065781896) * t1334 * t2837 * t421;
    let t10820 = t409 * t2803 * t419 * t421;
    let t10823 = t2826 * t1186 * t421;
    let t10825 = t4244 * t421;
    let t10828 = F::new(0.10359818039161417) * t4247 * t421;
    let t10831 = t1179 * t1166 * t419 * t421;
    (t10813, t10817, t10820, t10823, t10825, t10828, t10831)
}
