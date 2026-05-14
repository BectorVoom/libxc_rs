//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 854/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk854<F: Float>(t5047: F, t5071: F, t5903: F, t5916: F, t5917: F, t5922: F, t5925: F, t9628: F, t9746: F, t9753: F, t9756: F, t9922: F, t9925: F, t9929: F, t9933: F, t9936: F, t9943: F) -> (F,) {
    let t10314 = -t5917 + t5922 + t5903 + t5916 + 0.3056501876701794 * t5047 - t5925 + 0.1018833958900598 * t5071 + 3.0646056102413666 * t9922 - 3.0646056102413666 * t9925 - 3.0646056102413666 * t9929 + 4.59690841536205 * t9933 - 3.0646056102413666 * t9936 + 0.3056501876701794 * t9746 + 0.1018833958900598 * t9753 + 0.3056501876701794 * t9756 + 0.6113003753403587 * t9628 - 1.0215352034137888 * t9943;
    (t10314,)
}
