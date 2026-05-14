//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 976/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk976<F: Float>(t1916: F, t3223: F, t1920: F, t1179: F, t161: F, t4840: F, t495: F, t1447: F, t5180: F, t1847: F, t607: F, t500: F, t1451: F, t5194: F, t1455: F, t1467: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12870 = t3223 * t1916;
    let t12878 = t3223 * t1920;
    let t12898 = t161 * t1179 * t495 * t4840;
    let t12908 = t1447 * t5180;
    let t12912 = t1847 * t607;
    let t12913 = t12912 * t500;
    let t12915 = t5194 * t1451;
    let t12917 = t5194 * t1455;
    let t12919 = t5194 * t1467;
    (t12870, t12878, t12898, t12908, t12912, t12913, t12915, t12917, t12919)
}
