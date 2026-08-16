//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 969/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk969(t5047: f64, t5071: f64, t5903: f64, t5916: f64, t5917: f64, t5922: f64, t5925: f64, t9628: f64, t9746: f64, t9753: f64, t9756: f64, t9922: f64, t9925: f64, t9929: f64, t9933: f64, t9936: f64, t9943: f64) -> f64 {
    let t10314 = -t5917 + t5922 + t5903 + t5916 + 0.3056501876701794_f64 * t5047 - t5925 + 0.1018833958900598_f64 * t5071 + 3.0646056102413666_f64 * t9922 - 3.0646056102413666_f64 * t9925 - 3.0646056102413666_f64 * t9929 + 4.59690841536205_f64 * t9933 - 3.0646056102413666_f64 * t9936 + 0.3056501876701794_f64 * t9746 + 0.1018833958900598_f64 * t9753 + 0.3056501876701794_f64 * t9756 + 0.6113003753403587_f64 * t9628 - 1.0215352034137888_f64 * t9943;
    t10314
}
