//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1066/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1066<F: Float>(t4713: F, t607: F, t1710: F, t1959: F, t432: F, t4979: F, t9616: F, t9619: F, t12648: F, t12650: F, t12653: F, t12654: F, t12655: F, t12656: F, t12657: F) -> (F, F, F, F) {
    let t12659 = t4713 * t607;
    let t12661 = t1959 * t1710;
    let t12662 = F::new(2.0) / F::new(45.0) * t12661;
    let t12664 = t432 * t4979 / F::new(10.0);
    let t12665 = t9616 / F::new(15.0);
    let t12666 = F::new(2.0) / F::new(45.0) * t9619;
    let t12667 = -t12648 - t12650 - t12653 - t12654 + t12655 + t12656 - F::new(8.0) / F::new(405.0) * t12657 - F::new(2.0) / F::new(15.0) * t12659 + t12662 - t12664 - t12665 - t12666;
    (t12664, t12665, t12666, t12667)
}
