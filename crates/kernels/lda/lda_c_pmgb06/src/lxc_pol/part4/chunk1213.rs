//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1213/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1213<F: Float>(t1420: F, t6365: F, t1385: F, t439: F, t5039: F, t809: F, t15962: F, t15965: F, t15968: F, t15971: F, t15973: F, t15975: F, t15978: F, t15980: F, t15982: F, t15983: F, t15984: F, t15987: F, t15990: F) -> (F, F, F) {
    let t15992 = F::new(4.0) / F::new(45.0) * t1420 * t6365;
    let t15996 = F::new(2.0) / F::new(45.0) * t439 * t1385 * t809 * t5039;
    let t15997 = t15962 - t15965 - t15968 + t15971 - t15973 + t15975 + t15978 + t15980 - t15982 - t15983 - t15984 - t15987 - t15990 - t15992 - t15996;
    (t15992, t15996, t15997)
}
