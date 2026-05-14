//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1012/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1012<F: Float>(t34: F, t3966: F, t4507: F, t811: F, t2104: F, t4571: F, t10557: F, t197: F, t4610: F, t519: F, t2070: F, t493: F, t785: F, t11898: F, t2130: F, t1318: F, t3899: F, t4942: F) -> (F, F, F, F, F, F, F, F) {
    let t12963 = t3966 * t34;
    let t12968 = t4507 * t811;
    let t12974 = t2104 * t4571;
    let t12976 = t10557 * t197;
    let t12978 = t519 * t12976 * t4610;
    let t12984 = t493 * t2070 * t785;
    let t12987 = t493 * t11898 * t2130;
    let t12990 = t1318 * t3899 * t4942;
    (t12963, t12968, t12974, t12976, t12978, t12984, t12987, t12990)
}
