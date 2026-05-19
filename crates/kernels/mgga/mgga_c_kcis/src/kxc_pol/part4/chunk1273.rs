//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1273/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1273<F: Float>(t11543: F, t5546: F, t11576: F, t5578: F, t1334: F, t5574: F, t3861: F, t1907: F, t3893: F, t3862: F, t5577: F, t11581: F) -> (F, F, F, F, F) {
    let t16251 = F::new(4.0) * t11543 * t5546;
    let t16253 = F::cast_from(0.32163648644302209644e2_f64) * t11576 * t5578;
    let t16254 = t5574 * t1334;
    let t16256 = F::new(4.0) * t3861 * t16254;
    let t16257 = t1907 * t3893;
    let t16259 = F::new(2.0) * t3861 * t16257;
    let t16260 = t5577 * t3862;
    let t16262 = F::cast_from(0.96490945932906628932e2_f64) * t11581 * t16260;
    (t16251, t16253, t16256, t16259, t16262)
}
