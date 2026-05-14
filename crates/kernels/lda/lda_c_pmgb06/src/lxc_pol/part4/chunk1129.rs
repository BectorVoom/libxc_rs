//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1129/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1129<F: Float>(t1080: F, t2386: F, t9509: F, t12592: F, t493: F, t16873: F, t16874: F, t16876: F, t16878: F, t16881: F, t16883: F, t16885: F, t16886: F, t16891: F, t16894: F, t16895: F, t16899: F, t16904: F, t16908: F) -> (F, F, F) {
    let t16910 = t9509 * t2386 * t1080;
    let t16913 = 88.0 / 243.0 * t493 * t12592 * t16910;
    let t16914 = t16873 + t16874 - t16876 - t16878 + t16881 + t16883 - t16885 - t16886 - t16891 + t16894 + t16895 + t16899 + t16904 + t16908 + t16913;
    (t16910, t16913, t16914)
}
