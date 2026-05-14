//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1220/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1220<F: Float>(t16899: F, t16904: F, t16908: F, t16913: F, t16916: F, t16919: F, t16921: F, t16923: F, t16926: F, t16928: F, t16931: F, t16933: F, t16935: F, t16937: F, t16939: F, t16941: F, t16942: F, t16943: F, t16945: F, t16952: F, t16954: F, t16956: F, t16959: F, t16961: F, t16963: F, t16965: F, t16967: F, t16969: F, t16971: F, t16972: F) -> (F, F) {
    let t18290 = t16899 + t16904 + t16908 + t16913 - t16916 - t16919 + t16921 + t16923 + t16926 + t16928 + t16931 - t16933 - t16935 - t16937 - t16939;
    let t18291 = -t16941 - t16942 - t16943 + t16945 - t16952 - t16954 - t16956 - t16959 - t16961 + t16963 + t16965 + t16967 + t16969 + t16971 + t16972;
    (t18290, t18291)
}
