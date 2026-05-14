//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1066/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1066<F: Float>(t12225: F, t12227: F, t19935: F, t19937: F, t19939: F, t19941: F, t19943: F, t19944: F, t19945: F, t19946: F, t19947: F, t9483: F, t12240: F, t12246: F, t19948: F, t19949: F, t19954: F, t19956: F, t19958: F, t19960: F, t19963: F, t19967: F, t19969: F, t19970: F) -> (F, F) {
    let t21938 = 0.18233333333333332 * t9483 - t12225 - 2.0 / 3.0 * t12227 - t19935 + t19937 - t19939 + t19941 + t19943 + t19944 + t19945 + t19946 + t19947;
    let t21940 = t19948 + t19949 + t19954 - t19956 + t19958 + t19960 + t19963 + t12240 + t12246 - t19967 + t19969 - t19970;
    (t21938, t21940)
}
