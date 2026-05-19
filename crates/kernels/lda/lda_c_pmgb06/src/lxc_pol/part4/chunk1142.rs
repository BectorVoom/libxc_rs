//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1142/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1142<F: Float>(t11110: F, t11112: F, t11115: F, t11117: F, t11119: F, t11122: F, t11124: F, t8692: F, t8693: F, t8723: F, t8724: F, t8727: F, t8733: F, t8737: F, t8738: F, t8743: F, t8746: F) -> F {
    let t15006 = t8692 - F::cast_from(1.1696447245269292_f64) * t8693 - t8723 + F::cast_from(207.79030926817757_f64) * t8724 - F::new(24.0) * t11110 + F::new(8.0) * t11112 + t8727 + t8733 - F::cast_from(0.06506148780181044_f64) * t11115 - F::cast_from(0.04337432520120696_f64) * t11117 + F::cast_from(0.9631946627535314_f64) * t11119 + F::cast_from(0.04337432520120696_f64) * t11122 + F::cast_from(0.03253074390090522_f64) * t11124 - t8737 - F::cast_from(7.017868347161575_f64) * t8738 - t8743 + t8746;
    t15006
}
