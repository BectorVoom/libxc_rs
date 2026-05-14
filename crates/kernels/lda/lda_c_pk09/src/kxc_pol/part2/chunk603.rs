//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 603/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk603<F: Float>(t1447: F, t305: F, t5039: F, t5045: F, t5068: F, t5161: F, t5190: F, t5208: F, t5212: F, t1290: F, t5081: F, t1304: F, t1435: F, t1283: F, t5031: F, t1287: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6054 = t1447 * t1447;
    let t6055 = 1.0 / t6054;
    let t6056 = t6055 * t305;
    let t6060 = 1.5625 * t5039;
    let t6062 = 1.0416666666666667 * t5045;
    let t6068 = 0.3472222222222222 * t5068;
    let t6078 = 0.64 * t5039;
    let t6082 = 4.277978922036907 * t5161;
    let t6091 = 0.4266666666666667 * t5045;
    let t6092 = 0.35649824350307563 * t5190;
    let t6097 = 3.2084841915276807 * t5208;
    let t6098 = 3.2084841915276807 * t5212;
    let t6100 = 0.14222222222222222 * t5068;
    let t6107 = t1290 * t5081;
    let t6109 = t1304 * t1435;
    let t6116 = t1283 * t5031;
    let t6117 = t6116 * t1287;
    (t6056, t6060, t6062, t6068, t6078, t6082, t6091, t6092, t6097, t6098, t6100, t6107, t6109, t6117)
}
