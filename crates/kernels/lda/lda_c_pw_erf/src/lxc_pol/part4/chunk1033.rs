//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1033/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1033<F: Float>(t2070: F, t807: F, t185: F, t834: F, t211: F, t548: F, t812: F, t1397: F, t5211: F, t4039: F, t795: F, t1: F, t3: F, t4713: F, t604: F, t1635: F, t4537: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14043 = t2070 * t807;
    let t14044 = t185 * t14043;
    let t14048 = t2070 * t834;
    let t14049 = t211 * t14048;
    let t14052 = t548 * t2070 * t812;
    let t14075 = t5211 * t1397;
    let t14089 = t795 * t4039;
    let t14093 = t4713 * t1 * t3 * t604;
    let t14095 = t4537 * t1635;
    (t14043, t14044, t14048, t14049, t14052, t14075, t14089, t14093, t14095)
}
