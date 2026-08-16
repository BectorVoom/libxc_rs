//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1074/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1074<F: Float>(t2002: F, t2866: F, t1420: F, t5238: F, t2948: F, t439: F, t5232: F, t10412: F, t1907: F, t1908: F, t3177: F, t2979: F, t493: F, t5493: F) -> (F, F, F, F, F, F) {
    let t12758 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2002 * t2866;
    let t12760 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1420 * t5238;
    let t12763 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t2948 * t5232;
    let t12766 = t439 * t10412 * t1907 / F::cast_from(15.0_f64);
    let t12768 = t3177 * t1908 / F::cast_from(15.0_f64);
    let t12771 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t2979 * t5493;
    (t12758, t12760, t12763, t12766, t12768, t12771)
}
