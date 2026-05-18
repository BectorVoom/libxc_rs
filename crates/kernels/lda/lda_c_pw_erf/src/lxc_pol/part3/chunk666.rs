//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 666/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk666<F: Float>(t203: F, t4024: F, t184: F, t221: F, t1519: F, t511: F, t1508: F, t515: F, t1513: F, t1522: F, t563: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4025 = t203 * t4024;
    let t4026 = t4025 * t184;
    let t4028 = F::new(2.0) / F::new(15.0) * t4026 * t221;
    let t4029 = t511 * t1519;
    let t4030 = F::new(4.0) / F::new(45.0) * t4029;
    let t4031 = t1508 * t515;
    let t4032 = F::new(4.0) / F::new(15.0) * t4031;
    let t4033 = t1513 * t515;
    let t4034 = F::new(8.0) / F::new(15.0) * t4033;
    let t4035 = t1522 * t563;
    let t4036 = t4035 * t184;
    let t4038 = F::new(4.0) / F::new(5.0) * t4036 * t221;
    (t4025, t4026, t4028, t4029, t4030, t4031, t4032, t4033, t4034, t4035, t4036, t4038)
}
