//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 962/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk962<F: Float>(t11903: F, t5067: F, t432: F, t5051: F, t132: F, t1547: F, t1873: F, t5385: F, t588: F, t97: F, t208: F, t213: F, t4463: F, t579: F, t5381: F, t4159: F, t871: F) -> (F, F, F, F, F, F, F) {
    let t11904 = t11903 * t5067;
    let t11914 = t432 * t5051;
    let t11917 = t132 * t1547 * t1873;
    let t11920 = t5385 * t97 * t588;
    let t11928 = t4463 * t579 * t208 * t213;
    let t11930 = t5381 * t97 * t588;
    let t11944 = t871 * t4159;
    (t11904, t11914, t11917, t11920, t11928, t11930, t11944)
}
