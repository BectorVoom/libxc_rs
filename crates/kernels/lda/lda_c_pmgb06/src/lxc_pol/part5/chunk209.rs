//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 209/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk209<F: Float>(t27: F, t627: F, t402: F, t350: F, t405: F, t624: F) -> (F, F, F) {
    let t628 = t627 * t27;
    let t629 = t628 * t402;
    let t632 = -F::new(0.632975) * t624 - F::new(0.29896666666666666) * t350 - F::new(0.1023875) * t629 - F::new(0.08215666666666667) * t405;
    (t628, t629, t632)
}
