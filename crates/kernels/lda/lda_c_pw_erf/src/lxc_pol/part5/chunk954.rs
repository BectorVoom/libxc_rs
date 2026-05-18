//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 954/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk954<F: Float>(t1245: F, t3966: F, t4487: F, t668: F, t3675: F, t521: F, t108: F, t2119: F, t267: F) -> (F, F, F, F) {
    let t12113 = t3966 * t1245;
    let t12118 = t4487 * t668;
    let t12121 = t521 * t3675;
    let t12136 = t2119 * t108 * t267;
    (t12113, t12118, t12121, t12136)
}
