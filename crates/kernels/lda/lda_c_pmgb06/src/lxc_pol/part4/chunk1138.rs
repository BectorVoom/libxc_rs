//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1138/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1138<F: Float>(t11007: F, t11065: F, t14935: F, t14939: F, t14942: F, t14944: F, t14947: F, t283: F, t8482: F, t8519: F, t8520: F, t8526: F, t8529: F, t8531: F, t8534: F, t8538: F, t8541: F, t8543: F) -> F {
    let t14956 = F::new(0.0197516734986138) * t14935 * t283 + t8482 - t8519 + F::new(0.01084358130030174) * t14939 - F::new(240.0) * t8520 - F::new(0.5848223622634646) * t14942 - F::new(17.315859105681465) * t14944 - F::new(1.1696447245269292) * t14947 + t8526 - F::new(24.0) * t8529 + F::new(32.0) * t8531 + t8534 - F::new(8.0) * t8538 + F::new(12.0) * t8541 + F::new(120.0) * t8543 + F::new(2.0) * t11007 - F::new(32.0) * t11065;
    t14956
}
