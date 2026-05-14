//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1030/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1030<F: Float>(t12428: F, t1351: F, t3604: F, t4521: F, t3589: F, t4048: F, t581: F, t185: F, t5128: F, t514: F, t511: F, t5184: F, t4804: F, t5378: F, t1627: F, t4537: F) -> (F, F, F, F, F, F, F) {
    let t13797 = t12428 * t1351;
    let t13812 = t4521 * t3604;
    let t13829 = t4048 * t581 * t3589;
    let t13883 = t185 * t514 * t5128;
    let t13885 = t511 * t5184;
    let t13906 = t4804 * t5378;
    let t13915 = t4537 * t1627;
    (t13797, t13812, t13829, t13883, t13885, t13906, t13915)
}
