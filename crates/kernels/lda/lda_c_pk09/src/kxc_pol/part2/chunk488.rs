//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 488/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk488<F: Float>(t3160: F, t3163: F, t741: F, t902: F, t609: F, t903: F, t904: F, t917: F, t891: F, t892: F, t896: F, t897: F, t908: F, t844: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3165 = 3.7610742193750633 * t3160 * t3163;
    let t3166 = t741 * t902;
    let t3172 = t903 * t904 * t609;
    let t3173 = t917 * t3172;
    let t3176 = t891 * t892 * t609;
    let t3177 = t917 * t3176;
    let t3190 = t896 * t897 * t609;
    let t3191 = t917 * t3190;
    let t3193 = t908 * t609;
    let t3194 = t844 * t3193;
    (t3165, t3166, t3172, t3173, t3176, t3177, t3190, t3191, t3193, t3194)
}
