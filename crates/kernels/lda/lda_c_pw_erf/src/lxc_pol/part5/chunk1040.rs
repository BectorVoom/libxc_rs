//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1040/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1040<F: Float>(t3416: F, t6894: F, t1519: F, t2407: F, t184: F, t563: F, t811: F, t3899: F, t571: F, t6194: F, t4738: F, t4946: F) -> (F, F, F, F, F) {
    let t18523 = t3416 * t6894;
    let t18551 = t2407 * t1519;
    let t18555 = t811 * t563 * t184;
    let t18575 = t571 * t3899 * t6194;
    let t18584 = t4738 * t4946;
    (t18523, t18551, t18555, t18575, t18584)
}
