//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 715/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk715<F: Float>(t5212: F, t2064: F, t2106: F, t137: F, t132: F, t2090: F, t831: F, t2631: F, t432: F, t3306: F, t5196: F, t5207: F, t5209: F, t5215: F, t5217: F, t5219: F, t5222: F, t5304: F, t5328: F, t5330: F, t5342: F) -> (F, F, F, F, F, F, F, F) {
    let t6570 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t5212;
    let t6571 = t2106 * t2064;
    let t6572 = t137 * t6571;
    let t6574 = t132 * t6572 / F::cast_from(15.0_f64);
    let t6576 = t831 * t2090 / F::cast_from(15.0_f64);
    let t6578 = t432 * t2631 / F::cast_from(15.0_f64);
    let t6579 = t3306 / F::cast_from(135.0_f64);
    let t6580 = t5196 + t5207 + t5209 + t6570 + t5215 + t5217 + t5219 + t5222 - t5304 - t6574 - t6576 - t6578 - t6579 - t5328 - t5330 - t5342;
    (t6570, t6571, t6572, t6574, t6576, t6578, t6579, t6580)
}
