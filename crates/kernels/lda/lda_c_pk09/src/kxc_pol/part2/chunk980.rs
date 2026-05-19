//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 980/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk980<F: Float>(t1486: F, t9602: F, t1287: F, t9922: F, t9929: F, t5047: F, t5071: F, t5348: F, t5361: F, t5362: F, t5367: F, t5370: F, t9628: F, t9746: F, t9753: F, t9756: F, t9925: F, t9933: F, t9936: F, t9943: F) -> (F, F) {
    let t10474 = t1486 * t9602;
    let t10475 = t10474 * t1287;
    let t10479 = F::new(8.0) * t9922;
    let t10481 = F::new(8.0) * t9929;
    let t10489 = -t5362 + t5367 + t5348 + t5361 + F::cast_from(0.821419393556371_f64) * t5047 - t5370 + F::cast_from(0.2738064645187903_f64) * t5071 + t10479 - F::new(8.0) * t9925 - t10481 + F::new(12.0) * t9933 - F::new(8.0) * t9936 + F::cast_from(0.821419393556371_f64) * t9746 + F::cast_from(0.2738064645187903_f64) * t9753 + F::cast_from(0.821419393556371_f64) * t9756 + F::cast_from(1.642838787112742_f64) * t9628 - F::cast_from(2.6666666666666665_f64) * t9943;
    (t10475, t10489)
}
