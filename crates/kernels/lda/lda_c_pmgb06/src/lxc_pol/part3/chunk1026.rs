//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1026/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1026<F: Float>(t1512: F, t2066: F, t2043: F, t1447: F, t5282: F, t2912: F, t2918: F, t2991: F, t493: F, t851: F, t1444: F, t5337: F) -> (F, F, F, F, F) {
    let t12199 = t1512 * t2066 / F::cast_from(10.0_f64);
    let t12201 = t1512 * t2043 / F::cast_from(10.0_f64);
    let t12202 = t1447 * t5282;
    let t12203 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t12202;
    let t12208 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t493 * t2991 * t851 * t2918 * t2912;
    let t12210 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1444 * t5337;
    (t12199, t12201, t12203, t12208, t12210)
}
