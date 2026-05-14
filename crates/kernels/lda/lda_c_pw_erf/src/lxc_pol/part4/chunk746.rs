//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 746/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk746<F: Float>(t348: F, t5136: F, t5141: F, t3965: F, t1458: F, t197: F, t1245: F, t3975: F, t833: F, t1321: F, t3974: F, t549: F, t743: F, t3976: F, t593: F, t1333: F, t4574: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5142 = t5136 * t348;
    let t5143 = t5141 * t5142;
    let t5145 = 32.0 / 45.0 * t3965 * t5143;
    let t5146 = t1458 * t197;
    let t5147 = t5146 * t1245;
    let t5148 = t5147 * t5142;
    let t5150 = 16.0 / 27.0 * t3965 * t5148;
    let t5151 = t3975 * t833;
    let t5152 = t5151 * t1321;
    let t5154 = 16.0 / 45.0 * t3974 * t5152;
    let t5155 = t743 * t549;
    let t5157 = t3976 * t5155 * t593;
    let t5159 = 16.0 / 45.0 * t3974 * t5157;
    let t5160 = t4574 * t1333;
    (t5143, t5145, t5146, t5147, t5148, t5150, t5152, t5154, t5155, t5157, t5159, t5160)
}
