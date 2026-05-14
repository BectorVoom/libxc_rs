//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1078/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1078<F: Float>(t123: F, t4454: F, t566: F, t4259: F, t868: F, t1156: F, t1808: F, t10895: F, t10902: F, t11589: F, t1167: F, t1200: F, t125: F, t14697: F, t14700: F, t14703: F, t14706: F, t14707: F, t14710: F, t14712: F, t14723: F, t199: F, t2285: F, t4209: F, t4269: F, t4464: F, t912: F) -> (F,) {
    let t14726 = t123 * t4454 * t566;
    let t14741 = t123 * t4259 * t868;
    let t14744 = t123 * t1156 * t1808;
    let t14746 = -t14697 - t14700 - t14703 - t14706 - t10902 - 1.279801625812305 * t14707 + t14710 + 0.15917832887339686 * t14712 - 0.031835665774679375 * t123 * t125 * t11589 * t199 - 0.09550699732403813 * t123 * t4464 * t566 + 0.15917832887339686 * t14723 + 0.3183566577467937 * t14726 - 0.09550699732403813 * t123 * t2285 * t1200 - 0.031835665774679375 * t123 * t912 * t4209 - 0.031835665774679375 * t123 * t4269 * t868 - 0.09550699732403813 * t123 * t1167 * t1808 + 0.15917832887339686 * t14741 + 0.3183566577467937 * t14744 + t10895;
    (t14746,)
}
