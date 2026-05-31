//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1461/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1461<F: Float>(t18754: F, t360: F, t6967: F, t947: F, t6970: F, t18725: F, t18729: F, t18732: F, t18735: F, t18737: F, t18741: F, t18745: F, t18748: F, t18750: F, t18752: F) -> F {
    let t18755 = t360 * t18754;
    let t18757 = t6967 * t947;
    let t18759 = t6970 * t947;
    let t18761 = -F::cast_from(0.48968_f64) * t18725 - t18729 + t18732 + t18735 - t360 * t18737 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t360 * t18741 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18745 + t18748 - t18750 + t18752 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18755 + F::cast_from(3.91744_f64) * t18757 - F::cast_from(0.97936_f64) * t18759;
    t18761
}
