//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 844/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk844<F: Float>(t3531: F, t3534: F, t3569: F, t3573: F, t3583: F, t3586: F, t3597: F, t5820: F, t5821: F, t5825: F, t5826: F, t3505: F, t3513: F, t3515: F, t3517: F, t3521: F, t3523: F, t3525: F, t360: F, t5805: F, t5808: F, t5810: F, t5813: F) -> F {
    let t5827 = -F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3531 + t3534 / F::cast_from(6.0_f64) - F::cast_from(0.97936_f64) * t3569 + F::cast_from(0.73452_f64) * t3573 + t5820 + t5821 - F::cast_from(1.95872_f64) * t3583 - t3586 / F::cast_from(2.0_f64) - F::cast_from(2.93808_f64) * t3597 - t5825 - t5826;
    let t5829 = t5805 + t5808 - t360 * t5810 / F::cast_from(2.0_f64) - F::cast_from(0.97936_f64) * t5813 - t3505 + t3513 - t3515 - t3517 - t3521 - t3523 + t3525 + t5827;
    t5829
}
