//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1259/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1259<F: Float>(t13007: F, t6630: F, t1636: F, t2563: F, t12752: F, t12784: F, t12787: F, t1593: F, t2648: F, t1386: F, t5077: F, t15855: F, t5079: F) -> (F, F, F, F, F, F, F) {
    let t16556 = t13007 * t6630;
    let t16557 = F::new(16.0) / F::new(135.0) * t16556;
    let t16558 = t2563 * t1636;
    let t16559 = F::new(2.0) / F::new(45.0) * t16558;
    let t16560 = F::new(8.0) / F::new(405.0) * t12752;
    let t16561 = F::new(8.0) / F::new(135.0) * t12784;
    let t16562 = F::new(2.0) / F::new(45.0) * t12787;
    let t16563 = t1593 * t2648;
    let t16566 = F::new(4.0) / F::new(45.0) * t5077 * t16563 * t1386;
    let t16568 = F::new(8.0) / F::new(45.0) * t15855 * t5079;
    (t16557, t16559, t16560, t16561, t16562, t16566, t16568)
}
