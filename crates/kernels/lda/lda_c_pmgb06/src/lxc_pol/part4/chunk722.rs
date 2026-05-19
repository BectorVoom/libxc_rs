//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 722/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk722<F: Float>(t3941: F, t3945: F, t3948: F, t3955: F, t3877: F, t3881: F, t3884: F, t3888: F, t3939: F, t3944: F, t3954: F, t3959: F, t3962: F, t3965: F, t3968: F, t3970: F) -> (F, F, F, F, F) {
    let t4568 = F::new(12.0) * t3941;
    let t4569 = F::new(48.0) * t3945;
    let t4570 = F::new(80.0) * t3948;
    let t4571 = F::new(32.0) * t3955;
    let t4573 = t3877 + t3881 - t3884 - t3888 - F::new(24.0) * t3939 - t4568 - t3944 + t4569 + t4570 - t3954 - t4571 - t3959 - t3962 + t3965 + t3968 + F::cast_from(0.02168716260060348_f64) * t3970;
    (t4568, t4569, t4570, t4571, t4573)
}
