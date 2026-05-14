//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1145/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1145<F: Float>(t2098: F, t739: F, t348: F, t4488: F, t4501: F, t12475: F, t12963: F, t1996: F, t12136: F, t5138: F, t108: F, t267: F, t794: F, t4497: F, t4502: F, t2471: F, t4489: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16847 = t739 * t2098;
    let t16848 = t16847 * t348;
    let t16851 = 16.0 / 27.0 * t4488 * t4501 * t16848;
    let t16854 = 32.0 / 45.0 * t12475 * t12963 * t1996;
    let t16856 = 32.0 / 45.0 * t12136 * t5138;
    let t16858 = t794 * t108 * t267;
    let t16860 = 32.0 / 45.0 * t16858 * t4497;
    let t16862 = 16.0 / 27.0 * t16858 * t4502;
    let t16863 = t4489 * t2471;
    (t16847, t16848, t16851, t16854, t16856, t16858, t16860, t16862, t16863)
}
