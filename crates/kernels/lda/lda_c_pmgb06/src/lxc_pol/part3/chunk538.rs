//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 538/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk538<F: Float>(t118: F, t2778: F, t1179: F, t55: F, t1767: F, t32: F) -> (F, F, F, F) {
    let t2779 = t2778 * t118;
    let t2780 = F::new(0.00011865309871651405) * t2779;
    let t2781 = t55 * t1179;
    let t2786 = t32 * t1767;
    (t2779, t2780, t2781, t2786)
}
