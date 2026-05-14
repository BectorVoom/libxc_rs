//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1167/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1167<F: Float>(t1531: F, t2106: F, t5077: F, t5086: F, t15862: F, t5091: F, t5118: F, t822: F, t1386: F, t2599: F, t3458: F, t1381: F, t5068: F, t5090: F, t5493: F, t2604: F, t3032: F) -> (F, F, F, F, F, F) {
    let t17614 = 8.0 / 45.0 * t5077 * t2106 * t1531 * t5086;
    let t17616 = 8.0 / 45.0 * t15862 * t5091;
    let t17617 = t5118 * t822;
    let t17620 = 8.0 / 45.0 * t5077 * t17617 * t1386;
    let t17621 = t3458 * t2599;
    let t17624 = 4.0 / 15.0 * t5068 * t17621 * t1381;
    let t17627 = 8.0 / 45.0 * t5068 * t5090 * t5493;
    let t17628 = t3032 * t2604;
    (t17614, t17616, t17620, t17624, t17627, t17628)
}
