//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 962/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk962<F: Float>(t4489: F, t784: F, t34: F, t3966: F, t4507: F, t811: F, t2104: F, t4571: F, t10557: F, t197: F, t2070: F, t493: F, t785: F) -> (F, F, F, F, F, F) {
    let t12956 = t4489 * t784;
    let t12963 = t3966 * t34;
    let t12968 = t4507 * t811;
    let t12974 = t2104 * t4571;
    let t12975 = F::new(8.0) / F::new(45.0) * t12974;
    let t12976 = t10557 * t197;
    let t12984 = t493 * t2070 * t785;
    (t12956, t12963, t12968, t12975, t12976, t12984)
}
