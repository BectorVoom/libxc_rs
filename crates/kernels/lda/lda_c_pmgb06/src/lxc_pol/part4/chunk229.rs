//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 229/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk229<F: Float>(t273: F, t675: F, t350: F, t405: F, t624: F, t629: F) -> (F, F) {
    let t676 = t273 * t675;
    let t681 = -F::new(0.8630833333333333) * t624 - F::new(0.301925) * t350 - F::new(0.05501625) * t629 - F::new(0.082785) * t405;
    (t676, t681)
}
