//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 999/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk999<F: Float>(t11877: F, t2872: F, t493: F, t1898: F, t3213: F, t161: F, t3004: F, t843: F, t9350: F, t11859: F, t11861: F, t11865: F, t11867: F, t11869: F, t11872: F, t11874: F, t11876: F) -> (F, F, F, F, F) {
    let t11880 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t11877 * t2872;
    let t11881 = t3213 * t1898;
    let t11882 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t11881;
    let t11884 = t161 * t3004 * t843;
    let t11885 = F::cast_from(4.0_f64) / F::cast_from(405.0_f64) * t11884;
    let t11886 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t9350;
    let t11887 = t11859 - t11861 + t11865 - t11867 + t11869 - t11872 - t11874 - t11876 + t11880 + t11882 + t11885 + t11886;
    (t11880, t11882, t11885, t11886, t11887)
}
