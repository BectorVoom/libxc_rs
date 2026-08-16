//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 892/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk892<F: Float>(t27: F, t2789: F, t29: F, t563: F, t115: F, t2786: F, t562: F, t1190: F, t4189: F, t1187: F, t4197: F, t8173: F) -> (F, F, F, F, F, F) {
    let t10512 = t2789 * t27;
    let t10515 = F::cast_from(0.1254_f64) * t10512 * t29 * t563;
    let t10518 = F::cast_from(0.32511111111111113_f64) * t562 * t2786 * t115;
    let t10520 = F::cast_from(0.2508_f64) * t4189 * t1190;
    let t10522 = F::cast_from(0.39013333333333333_f64) * t1187 * t4197;
    let t10524 = t8173 * t115;
    (t10512, t10515, t10518, t10520, t10522, t10524)
}
