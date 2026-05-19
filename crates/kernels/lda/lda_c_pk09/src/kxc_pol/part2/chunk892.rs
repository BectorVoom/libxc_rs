//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 892/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk892<F: Float>(t2362: F, t4502: F, t3148: F, t1101: F, t4411: F, t4413: F, t4421: F, t8595: F, t8597: F, t8600: F, t8602: F, t8604: F, t8606: F, t8608: F, t9056: F, t9060: F, t9394: F, t9409: F, t9411: F, t98: F) -> F {
    let t9414 = t2362 * t4502;
    let t9415 = t9414 * t3148;
    let t9417 = -F::cast_from(0.09983749558483038_f64) * t4411 - t4413 / F::new(9.0) + t4421 / F::new(6.0) + t9394 * t98 / F::new(6.0) - F::cast_from(0.02466859483068398_f64) * t8595 - F::cast_from(0.02466859483068398_f64) * t8597 + F::cast_from(0.14975624337724558_f64) * t8600 + F::cast_from(0.29951248675449116_f64) * t8602 + F::cast_from(0.14975624337724558_f64) * t8604 + F::cast_from(0.14975624337724558_f64) * t8606 + F::cast_from(0.29951248675449116_f64) * t8608 + t1101 * t9056 / F::new(6.0) + t1101 * t9060 / F::new(3.0) + t9409 * t9411 / F::new(6.0) + t9415 / F::new(6.0);
    t9417
}
