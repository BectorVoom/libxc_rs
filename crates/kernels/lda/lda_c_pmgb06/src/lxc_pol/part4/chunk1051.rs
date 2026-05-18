//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1051/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1051<F: Float>(t2151: F, t3734: F, t4556: F, t980: F, t2148: F, t3711: F, t959: F, t3742: F, t968: F, t273: F, t4515: F, t698: F) -> (F, F, F, F, F, F, F) {
    let t11157 = t2151 * t3734;
    let t11160 = t4556 * t980;
    let t11162 = t2148 * t3711;
    let t11164 = t4556 * t959;
    let t11166 = t2148 * t3742;
    let t11168 = t4556 * t968;
    let t11171 = t4515 * t273 * t698;
    (t11157, t11160, t11162, t11164, t11166, t11168, t11171)
}
