//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1375/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1375<F: Float>(t2414: F, t740: F, t1193: F, t1354: F, t6716: F, t81: F, t118: F, t415: F, t6946: F, t15116: F, t3: F, t6928: F) -> (F, F, F, F, F, F, F) {
    let t18057 = t740 * t2414;
    let t18059 = t18057 * t1193 * t1354;
    let t18061 = t81 * t6716;
    let t18062 = t18061 * t118;
    let t18064 = t6946 * t415;
    let t18066 = t3 * t15116;
    let t18069 = t6928 * t415;
    (t18057, t18059, t18061, t18062, t18064, t18066, t18069)
}
