//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 969/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk969<F: Float>(t5047: F, t5071: F, t5903: F, t5916: F, t5917: F, t5922: F, t5925: F, t9628: F, t9746: F, t9753: F, t9756: F, t9922: F, t9925: F, t9929: F, t9933: F, t9936: F, t9943: F) -> F {
    let t10314 = -t5917 + t5922 + t5903 + t5916 + F::cast_from(0.3056501876701794_f64) * t5047 - t5925 + F::cast_from(0.1018833958900598_f64) * t5071 + F::cast_from(3.0646056102413666_f64) * t9922 - F::cast_from(3.0646056102413666_f64) * t9925 - F::cast_from(3.0646056102413666_f64) * t9929 + F::cast_from(4.59690841536205_f64) * t9933 - F::cast_from(3.0646056102413666_f64) * t9936 + F::cast_from(0.3056501876701794_f64) * t9746 + F::cast_from(0.1018833958900598_f64) * t9753 + F::cast_from(0.3056501876701794_f64) * t9756 + F::cast_from(0.6113003753403587_f64) * t9628 - F::cast_from(1.0215352034137888_f64) * t9943;
    t10314
}
