//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 769/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk769<F: Float>(t184: F, t5040: F, t813: F, t549: F, t563: F, t1280: F, t795: F, t4073: F, t1508: F, t808: F, t1234: F, t1294: F, t822: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5041 = t5040 * t184;
    let t5043 = F::new(4.0) / F::new(15.0) * t5041 * t813;
    let t5044 = t549 * t563;
    let t5045 = t5044 * t184;
    let t5047 = F::new(8.0) / F::new(15.0) * t5045 * t813;
    let t5049 = F::new(2.0) / F::new(15.0) * t795 * t1280;
    let t5051 = F::new(4.0) / F::new(15.0) * t4073 * t813;
    let t5053 = F::new(2.0) / F::new(15.0) * t1508 * t808;
    let t5055 = F::new(8.0) / F::new(45.0) * t795 * t1234;
    let t5057 = F::new(8.0) / F::new(45.0) * t822 * t1294;
    (t5041, t5043, t5044, t5045, t5047, t5049, t5051, t5053, t5055, t5057)
}
