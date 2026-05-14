//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 609/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk609<F: Float>(t1251: F, t4722: F, t1458: F, t197: F, t1245: F, t1333: F, t4574: F, t1484: F, t219: F, t1351: F, t2066: F, t514: F, t211: F, t2071: F, t4567: F, t548: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5141 = t4722 * t1251;
    let t5146 = t1458 * t197;
    let t5147 = t5146 * t1245;
    let t5160 = t4574 * t1333;
    let t5165 = t1484 * t219;
    let t5166 = t5165 * t1351;
    let t5170 = t514 * t2066;
    let t5172 = 8.0 / 45.0 * t211 * t5170;
    let t5175 = t4567 * t2071;
    let t5176 = t548 * t5175;
    (t5141, t5146, t5147, t5160, t5165, t5166, t5170, t5172, t5175, t5176)
}
