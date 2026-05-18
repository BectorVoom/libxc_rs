//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 865/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk865<F: Float>(t283: F, t3881: F, t3884: F, t3888: F, t3939: F, t3944: F, t3954: F, t3959: F, t3962: F, t3965: F, t3968: F, t3970: F, t4568: F, t4569: F, t4570: F, t4571: F, t6067: F) -> F {
    let t6102 = t3881 - t3884 - t3888 + F::new(0.0197516734986138) * t6067 * t283 + F::new(12.0) * t3939 - t4568 + t3944 + t4569 - t4570 - t3954 + t4571 - t3959 - t3962 + t3965 + t3968 + F::new(0.01084358130030174) * t3970;
    t6102
}
