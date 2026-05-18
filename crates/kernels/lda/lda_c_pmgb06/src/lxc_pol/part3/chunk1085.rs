//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1085/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1085<F: Float>(t1925: F, t3198: F, t9774: F, t1179: F, t161: F, t4840: F, t495: F, t9777: F, t12879: F, t12881: F, t12883: F, t12885: F, t12887: F, t12889: F, t12892: F) -> (F, F, F, F, F) {
    let t12894 = t3198 * t1925 / F::new(15.0);
    let t12895 = t9774 / F::new(45.0);
    let t12898 = t161 * t1179 * t495 * t4840;
    let t12899 = F::new(8.0) / F::new(45.0) * t12898;
    let t12900 = F::new(2.0) / F::new(15.0) * t9777;
    let t12901 = -t12879 - t12881 - t12883 - t12885 + t12887 - t12889 - t12892 - t12894 + t12895 + t12899 + t12900;
    (t12894, t12895, t12899, t12900, t12901)
}
