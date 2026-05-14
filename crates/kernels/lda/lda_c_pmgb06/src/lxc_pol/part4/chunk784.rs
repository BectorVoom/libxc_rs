//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 784/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk784<F: Float>(t1297: F, t787: F, t2255: F, t384: F, t1309: F, t5790: F, t69: F, t5806: F, t109: F, t370: F) -> (F, F, F, F, F, F) {
    let t5843 = t787 * t1297;
    let t5846 = t2255 * t384;
    let t5849 = t787 * t1309;
    let t5852 = t69 * t5790;
    let t5855 = 1.1495033333333333 * t69 * t5806;
    let t5858 = t109 * t370;
    (t5843, t5846, t5849, t5852, t5855, t5858)
}
