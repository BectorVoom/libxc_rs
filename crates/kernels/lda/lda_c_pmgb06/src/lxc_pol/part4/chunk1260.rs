//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1260/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1260<F: Float>(t12807: F, t132: F, t137: F, t13979: F, t822: F, t12816: F, t16541: F, t16543: F, t16548: F, t16550: F, t16555: F, t16557: F, t16559: F, t16560: F, t16561: F, t16562: F, t16566: F, t16568: F) -> (F, F, F, F) {
    let t16569 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t12807;
    let t16573 = t132 * t137 * t13979 * t822 / F::cast_from(15.0_f64);
    let t16574 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t12816;
    let t16575 = t16541 - t16543 - t16548 + t16550 - t16555 + t16557 - t16559 + t16560 - t16561 - t16562 + t16566 + t16568 - t16569 - t16573 - t16574;
    (t16569, t16573, t16574, t16575)
}
