//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1298/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1298<F: Float>(t1476: F, t15426: F, t17043: F, t17037: F, t506: F, t350: F, t6828: F, t1820: F, t1830: F, t2546: F, t947: F, t2542: F) -> (F, F, F, F, F, F) {
    let t17049 = t15426 * t1476 * t17043;
    let t17052 = t15426 * t506 * t17037;
    let t17054 = t350 * t6828;
    let t17057 = t1830 * t1476 * t1820;
    let t17059 = t947 * t2546;
    let t17061 = t947 * t2542;
    (t17049, t17052, t17054, t17057, t17059, t17061)
}
