//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1119/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1119<F: Float>(t4463: F, t81: F, t199: F, t5522: F, t566: F, t122: F, t1669: F, t2116: F, t421: F, t5900: F, t1147: F, t794: F) -> (F, F, F, F, F, F) {
    let t14239 = t81 * t4463;
    let t14240 = t14239 * t199;
    let t14242 = t5522 * t566;
    let t14245 = t122 * t1669 * t2116;
    let t14275 = t5900 * t421;
    let t14277 = t1147 * t794;
    (t14239, t14240, t14242, t14245, t14275, t14277)
}
