//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1138/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1138<F: Float>(t11007: F, t11065: F, t14935: F, t14939: F, t14942: F, t14944: F, t14947: F, t283: F, t8482: F, t8519: F, t8520: F, t8526: F, t8529: F, t8531: F, t8534: F, t8538: F, t8541: F, t8543: F) -> F {
    let t14956 = F::cast_from(0.0197516734986138_f64) * t14935 * t283 + t8482 - t8519 + F::cast_from(0.01084358130030174_f64) * t14939 - F::cast_from(240.0_f64) * t8520 - F::cast_from(0.5848223622634646_f64) * t14942 - F::cast_from(17.315859105681465_f64) * t14944 - F::cast_from(1.1696447245269292_f64) * t14947 + t8526 - F::cast_from(24.0_f64) * t8529 + F::cast_from(32.0_f64) * t8531 + t8534 - F::cast_from(8.0_f64) * t8538 + F::cast_from(12.0_f64) * t8541 + F::cast_from(120.0_f64) * t8543 + F::cast_from(2.0_f64) * t11007 - F::cast_from(32.0_f64) * t11065;
    t14956
}
