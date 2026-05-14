//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 292/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk292<F: Float>(t696: F, t980: F, t109: F, t660: F, t265: F, t659: F, t260: F, t666: F) -> (F, F, F, F, F, F) {
    let t982 = 1.1696447245269292 * t696 * t980;
    let t986 = t109 * t660;
    let t990 = t659 * t265;
    let t991 = 1.0 / t990;
    let t992 = t260 * t991;
    let t993 = t666 * t666;
    (t982, t986, t990, t991, t992, t993)
}
