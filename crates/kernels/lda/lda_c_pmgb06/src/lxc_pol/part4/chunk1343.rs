//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1343/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1343<F: Float>(t5442: F, t6268: F, t1594: F, t2574: F, t2864: F, t439: F, t15345: F, t1897: F, t486: F, t6851: F, t13686: F, t1887: F, t2108: F) -> (F, F, F, F, F, F) {
    let t17643 = F::new(8.0) / F::new(45.0) * t6268 * t5442;
    let t17647 = F::new(4.0) / F::new(45.0) * t439 * t2864 * t2574 * t1594;
    let t17650 = F::new(8.0) / F::new(15.0) * t439 * t1897 * t15345;
    let t17651 = t486 * t6851;
    let t17652 = F::new(4.0) / F::new(45.0) * t17651;
    let t17653 = F::new(4.0) / F::new(45.0) * t13686;
    let t17655 = F::new(2.0) / F::new(15.0) * t1887 * t2108;
    (t17643, t17647, t17650, t17652, t17653, t17655)
}
