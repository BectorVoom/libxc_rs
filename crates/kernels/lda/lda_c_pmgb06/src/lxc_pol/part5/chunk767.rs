//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 767/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk767<F: Float>(t183: F, t6716: F, t188: F, t5215: F, t5217: F, t5219: F, t5222: F, t5304: F, t5328: F, t5330: F, t5342: F, t5349: F, t6570: F, t6574: F, t6576: F, t6578: F, t6579: F) -> (F, F) {
    let t7209 = t6716 * t183;
    let t7212 = t6570 + t5215 + t5217 + t5219 + t5222 - t5304 - t6574 - t6576 - t6578 + F::new(4.0) / F::new(3.0) * t7209 * t188 - t6579 - t5328 - t5330 - t5342 - t5349;
    (t7209, t7212)
}
