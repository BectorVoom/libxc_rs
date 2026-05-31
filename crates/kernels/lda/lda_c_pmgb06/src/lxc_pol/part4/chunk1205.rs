//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1205/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1205<F: Float>(t13000: F, t15872: F, t5083: F, t1: F, t5085: F, t13043: F, t5094: F, t12991: F, t15324: F, t5499: F, t6395: F, t1972: F, t5487: F) -> (F, F, F, F, F, F) {
    let t15879 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5083 * t13000 * t15872;
    let t15880 = t5085 * t1;
    let t15883 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t13043 * t5094 * t15880;
    let t15886 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t13043 * t12991 * t15324;
    let t15887 = t5499 * t6395;
    let t15888 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t15887;
    let t15890 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1972 * t5487;
    (t15879, t15880, t15883, t15886, t15888, t15890)
}
