//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1050/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1050<F: Float>(t12485: F, t5138: F, t5139: F, t11904: F, t5072: F, t11903: F, t5137: F, t5140: F, t1414: F, t1639: F, t5068: F, t5071: F) -> (F, F, F, F) {
    let t12491 = t5138 * t5139 * t12485 / F::new(9.0);
    let t12493 = F::new(4.0) / F::new(15.0) * t11904 * t5072;
    let t12494 = t11903 * t5137;
    let t12496 = F::new(2.0) / F::new(9.0) * t12494 * t5140;
    let t12497 = t1639 * t1414;
    let t12500 = F::new(4.0) / F::new(15.0) * t5068 * t12497 * t5071;
    (t12491, t12493, t12496, t12500)
}
