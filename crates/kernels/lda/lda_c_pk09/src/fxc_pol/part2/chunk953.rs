//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 953/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk953<F: Float>(t11352: F, t1782: F, t11128: F, t471: F, t10959: F, t11066: F, t11073: F, t11076: F, t11529: F, t11532: F, t11535: F, t11539: F, t11542: F, t6323: F, t6337: F, t6467: F, t6508: F, t6550: F, t7107: F, t7108: F, t7112: F) -> (F, F, F) {
    let t11863 = t11352 * t1782;
    let t11866 = t471 * t11128;
    let t11883 = 0.3056501876701794 * t11066 + 0.6113003753403587 * t10959 + 3.0646056102413666 * t11529 - 3.0646056102413666 * t11532 - 3.0646056102413666 * t11535 + 4.59690841536205 * t11539 - 3.0646056102413666 * t11542 + 0.3056501876701794 * t11076 + t7107 + 0.1018833958900598 * t11073 + t7112 - 0.1018833958900598 * t6337 - 0.3056501876701794 * t6323 + 1.0215352034137888 * t6550 + t7108 - 1.0215352034137888 * t6508 + 0.1018833958900598 * t6467;
    (t11863, t11866, t11883)
}
