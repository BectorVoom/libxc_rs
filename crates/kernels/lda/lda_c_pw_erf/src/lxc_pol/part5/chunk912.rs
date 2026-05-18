//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 912/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk912<F: Float>(t3892: F, t56: F, t1953: F, t506: F, t2070: F, t594: F, t211: F, t543: F, t185: F, t3964: F, t668: F) -> (F, F, F, F, F) {
    let t9836 = t56 * t3892;
    let t9847 = t1953 * t506;
    let t9933 = t2070 * t594;
    let t9934 = t211 * t9933;
    let t9946 = t2070 * t543;
    let t9947 = t185 * t9946;
    let t10011 = t3964 * t668;
    (t9836, t9847, t9934, t9947, t10011)
}
