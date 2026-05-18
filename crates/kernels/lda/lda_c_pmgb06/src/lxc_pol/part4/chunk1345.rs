//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1345/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1345<F: Float>(t17666: F, t1423: F, t6255: F, t1594: F, t1966: F, t2604: F, t439: F, t9647: F, t13708: F, t13710: F, t17643: F, t17647: F, t17650: F, t17652: F, t17653: F, t17655: F, t17657: F, t17661: F, t17662: F, t17665: F) -> (F, F, F, F, F, F) {
    let t17667 = F::new(8.0) / F::new(45.0) * t17666;
    let t17668 = t1423 * t6255;
    let t17669 = F::new(4.0) / F::new(15.0) * t17668;
    let t17674 = F::new(4.0) / F::new(5.0) * t439 * t1966 * t9647 * t2604 * t1594;
    let t17675 = F::new(8.0) / F::new(405.0) * t13708;
    let t17676 = F::new(8.0) / F::new(135.0) * t13710;
    let t17677 = t17643 - t17647 - t17650 + t17652 - t17653 - t17655 - t17657 + t17661 + t17662 + t17665 + t17667 - t17669 + t17674 - t17675 - t17676;
    (t17667, t17669, t17674, t17675, t17676, t17677)
}
