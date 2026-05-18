//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 205/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk205<F: Float>(t199: F, t391: F, t414: F, t29: F, t419: F, t110: F, t115: F, t331: F, t396: F) -> (F, F, F, F, F, F) {
    let t558 = F::new(0.0837628205355044) * t391 * t199;
    let t561 = t414 / F::new(2.0);
    let t562 = t419 * t29;
    let t563 = t110 * t115;
    let t565 = F::new(0.03135) * t562 * t563;
    let t566 = t561 + t565 + t331 - t396;
    (t558, t561, t562, t563, t565, t566)
}
