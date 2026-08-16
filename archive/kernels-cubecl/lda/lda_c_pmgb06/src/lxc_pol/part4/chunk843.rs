//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 843/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk843<F: Float>(t110: F, t2221: F, t360: F, t4394: F, t64: F, t35: F, t2226: F, t947: F, t3577: F, t3579: F, t3601: F, t3603: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5806 = t110 * t2221;
    let t5808 = t360 * t5806 / F::cast_from(3.0_f64);
    let t5809 = t64 * t4394;
    let t5810 = t35 * t5809;
    let t5813 = t2226 * t947;
    let t5820 = F::cast_from(0.6495611111111111_f64) * t3577;
    let t5821 = F::cast_from(0.48717083333333333_f64) * t3579;
    let t5825 = F::cast_from(0.9743416666666667_f64) * t3601;
    let t5826 = F::cast_from(1.2991222222222223_f64) * t3603;
    (t5806, t5808, t5809, t5810, t5813, t5820, t5821, t5825, t5826)
}
