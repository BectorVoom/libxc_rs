//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 742/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk742<F: Float>(t4756: F, t4759: F, t4764: F, t4769: F, t4771: F, t4774: F, t4776: F, t4778: F, t4782: F, t4784: F, t4786: F, t4788: F, t4792: F, t4794: F, t4796: F, t1409: F, t794: F) -> (F, F) {
    let t5631 = t4756 - t4759 - t4764 - t4769 - t4771 - t4774 - t4776 - t4778 + t4782 + t4784 + t4786 + t4788 + t4792 + t4794 + t4796;
    let t5632 = t794 * t1409;
    (t5631, t5632)
}
