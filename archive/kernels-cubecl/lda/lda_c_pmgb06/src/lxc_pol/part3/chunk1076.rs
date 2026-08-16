//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1076/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1076<F: Float>(t12784: F, t161: F, t489: F, t4936: F, t12758: F, t12760: F, t12763: F, t12766: F, t12768: F, t12771: F, t12775: F, t12778: F, t12783: F) -> (F, F, F) {
    let t12785 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t12784;
    let t12787 = t161 * t489 * t4936;
    let t12788 = t12787 / F::cast_from(15.0_f64);
    let t12789 = t12758 - t12760 - t12763 - t12766 - t12768 - t12771 - t12775 - t12778 - t12783 - t12785 - t12788;
    (t12785, t12788, t12789)
}
