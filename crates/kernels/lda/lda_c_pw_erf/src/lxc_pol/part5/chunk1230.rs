//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1230/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1230<F: Float>(t1472: F, t7613: F, t7609: F, t1466: F, t2065: F, t571: F, t6968: F, t4753: F, t7585: F, t3416: F, t2146: F, t6925: F) -> (F, F, F, F, F, F) {
    let t22169 = F::new(32.0) / F::new(81.0) * t1472 * t7613;
    let t22171 = F::new(4.0) / F::new(45.0) * t1472 * t7609;
    let t22175 = F::new(12.0) / F::new(5.0) * t571 * t1466 * t6968 * t2065;
    let t22177 = F::new(8.0) / F::new(5.0) * t4753 * t7585;
    let t22179 = F::new(8.0) / F::new(5.0) * t3416 * t7585;
    let t22181 = F::new(4.0) / F::new(15.0) * t2146 * t6925;
    (t22169, t22171, t22175, t22177, t22179, t22181)
}
