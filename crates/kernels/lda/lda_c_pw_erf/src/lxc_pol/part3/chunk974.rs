//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 974/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk974<F: Float>(t4507: F, t811: F, t3868: F, t3974: F, t2104: F, t4571: F, t10557: F, t197: F, t4610: F, t519: F, t11808: F, t5250: F, t2070: F, t493: F, t785: F, t11898: F, t2130: F) -> (F, F, F, F, F, F) {
    let t12968 = t4507 * t811;
    let t12971 = 16.0 / 15.0 * t3974 * t12968 * t3868;
    let t12974 = t2104 * t4571;
    let t12975 = 8.0 / 45.0 * t12974;
    let t12976 = t10557 * t197;
    let t12978 = t519 * t12976 * t4610;
    let t12979 = 64.0 / 81.0 * t12978;
    let t12982 = 128.0 / 27.0 * t519 * t5250 * t11808;
    let t12984 = t493 * t2070 * t785;
    let t12985 = 32.0 / 405.0 * t12984;
    let t12987 = t493 * t11898 * t2130;
    (t12971, t12975, t12979, t12982, t12985, t12987)
}
