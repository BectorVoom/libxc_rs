//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 980/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk980(t1486: f64, t9602: f64, t1287: f64, t9922: f64, t9929: f64, t5047: f64, t5071: f64, t5348: f64, t5361: f64, t5362: f64, t5367: f64, t5370: f64, t9628: f64, t9746: f64, t9753: f64, t9756: f64, t9925: f64, t9933: f64, t9936: f64, t9943: f64) -> (f64, f64) {
    let t10474 = t1486 * t9602;
    let t10475 = t10474 * t1287;
    let t10479 = 8.0_f64 * t9922;
    let t10481 = 8.0_f64 * t9929;
    let t10489 = -t5362 + t5367 + t5348 + t5361 + 0.821419393556371_f64 * t5047 - t5370 + 0.2738064645187903_f64 * t5071 + t10479 - 8.0_f64 * t9925 - t10481 + 12.0_f64 * t9933 - 8.0_f64 * t9936 + 0.821419393556371_f64 * t9746 + 0.2738064645187903_f64 * t9753 + 0.821419393556371_f64 * t9756 + 1.642838787112742_f64 * t9628 - 2.6666666666666665_f64 * t9943;
    (t10475, t10489)
}
