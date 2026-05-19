//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 863/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk863<F: Float>(t3679: F, t642: F, t1147: F, t934: F, t940: F, t623: F, t8165: F, t36: F, t138: F, t28: F, t4238: F, t3689: F) -> (F, F, F, F, F, F) {
    let t8699 = t3679 * t642;
    let t8701 = t934 * t1147;
    let t8702 = t940 * t8701;
    let t8704 = t623 * t8165;
    let t8707 = F::powf(t36, -F::new(2.5));
    let t8710 = t8707 * t28 * t4238 * t138;
    let t8712 = t3689 * t642;
    (t8699, t8701, t8702, t8704, t8710, t8712)
}
