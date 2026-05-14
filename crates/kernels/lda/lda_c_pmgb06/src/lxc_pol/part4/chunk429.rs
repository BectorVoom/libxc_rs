//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 429/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk429<F: Float>(t199: F, t718: F, t1329: F, t391: F, t566: F, t31: F, t740: F) -> (F, F, F, F) {
    let t1658 = 0.1675256410710088 * t718 * t199;
    let t1659 = t1329 * t199;
    let t1661 = t391 * t566;
    let t1669 = t31 * t740;
    (t1658, t1659, t1661, t1669)
}
