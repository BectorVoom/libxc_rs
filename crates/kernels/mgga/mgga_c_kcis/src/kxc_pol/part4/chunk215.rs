//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 215/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk215<F: Float>(t717: F, t89: F, t2: F, t647: F, t92: F, t651: F, t653: F, t15: F, t650: F) -> (F, F, F, F) {
    let t718 = t89 * t717;
    let t720 = t647 * t92 * t2;
    let t725 = -F::cast_from(0.66066666666666666667e-2_f64) * t651 - F::new(0.41275e-2) * t653;
    let t728 = -t720 * t650 / F::new(12.0) + t15 * t725 / F::new(2.0);
    (t718, t720, t725, t728)
}
