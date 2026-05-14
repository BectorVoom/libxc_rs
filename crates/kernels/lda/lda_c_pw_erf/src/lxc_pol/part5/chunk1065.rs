//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1065/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1065<F: Float>(t4753: F, t7585: F, t3416: F, t2146: F, t6925: F, t2544: F, t5334: F, t10438: F, t22157: F, t22159: F, t22161: F, t22163: F, t22167: F, t22169: F, t22171: F, t22175: F) -> (F, F, F, F, F) {
    let t22177 = 8.0 / 5.0 * t4753 * t7585;
    let t22179 = 8.0 / 5.0 * t3416 * t7585;
    let t22181 = 4.0 / 15.0 * t2146 * t6925;
    let t22183 = 4.0 / 9.0 * t5334 * t2544;
    let t22184 = -t22157 - t22159 + t22161 - t22163 + t22167 + t22169 + t22171 - t22175 + t22177 + t22179 + t22181 + t22183 - t10438;
    (t22177, t22179, t22181, t22183, t22184)
}
