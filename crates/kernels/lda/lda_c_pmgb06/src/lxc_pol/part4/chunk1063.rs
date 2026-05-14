//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1063/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1063<F: Float>(t13000: F, t15872: F, t5083: F, t1: F, t5085: F, t13043: F, t5094: F, t12991: F, t15324: F, t5499: F, t6395: F, t1972: F, t5487: F, t1423: F, t6361: F, t6365: F) -> (F, F, F, F, F, F, F, F) {
    let t15879 = 4.0 / 9.0 * t5083 * t13000 * t15872;
    let t15880 = t5085 * t1;
    let t15883 = 16.0 / 45.0 * t13043 * t5094 * t15880;
    let t15886 = 16.0 / 15.0 * t13043 * t12991 * t15324;
    let t15887 = t5499 * t6395;
    let t15888 = 4.0 / 27.0 * t15887;
    let t15890 = 4.0 / 45.0 * t1972 * t5487;
    let t15891 = t1423 * t6361;
    let t15892 = 8.0 / 135.0 * t15891;
    let t15893 = t1423 * t6365;
    (t15879, t15880, t15883, t15886, t15888, t15890, t15892, t15893)
}
