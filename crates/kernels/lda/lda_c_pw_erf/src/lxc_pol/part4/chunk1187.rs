//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1187/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1187<F: Float>(t12475: F, t12963: F, t2030: F, t13032: F, t1294: F, t2402: F, t5175: F, t6875: F, t595: F, t6611: F, t1383: F, t184: F, t202: F, t6669: F, t551: F, t13041: F) -> (F, F, F, F, F, F, F, F) {
    let t17546 = 32.0 / 45.0 * t12475 * t12963 * t2030;
    let t17547 = 64.0 / 135.0 * t13032;
    let t17548 = t2402 * t1294;
    let t17549 = 16.0 / 45.0 * t17548;
    let t17550 = t6875 * t5175;
    let t17551 = 8.0 / 9.0 * t17550;
    let t17553 = 8.0 / 15.0 * t6611 * t595;
    let t17555 = 4.0 / 15.0 * t2402 * t1383;
    let t17557 = t202 * t6669 * t184;
    let t17559 = 8.0 / 15.0 * t17557 * t551;
    let t17560 = 16.0 / 45.0 * t13041;
    (t17546, t17547, t17549, t17551, t17553, t17555, t17559, t17560)
}
