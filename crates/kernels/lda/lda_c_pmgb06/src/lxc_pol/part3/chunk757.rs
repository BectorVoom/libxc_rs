//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 757/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk757<F: Float>(t110: F, t2221: F, t360: F, t4394: F, t64: F, t35: F, t2226: F, t947: F, t3577: F, t3579: F, t3601: F, t3603: F, t3531: F, t3534: F, t3569: F, t3573: F, t3583: F, t3586: F, t3597: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5806 = t110 * t2221;
    let t5808 = t360 * t5806 / 3.0;
    let t5809 = t64 * t4394;
    let t5810 = t35 * t5809;
    let t5813 = t2226 * t947;
    let t5820 = 0.6495611111111111 * t3577;
    let t5821 = 0.48717083333333333 * t3579;
    let t5825 = 0.9743416666666667 * t3601;
    let t5826 = 1.2991222222222223 * t3603;
    let t5827 = -4.0 / 9.0 * t3531 + t3534 / 6.0 - 0.97936 * t3569 + 0.73452 * t3573 + t5820 + t5821 - 1.95872 * t3583 - t3586 / 2.0 - 2.93808 * t3597 - t5825 - t5826;
    (t5806, t5808, t5809, t5810, t5813, t5820, t5821, t5825, t5826, t5827)
}
