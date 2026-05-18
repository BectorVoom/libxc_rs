//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 574/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk574<F: Float>(t3703: F, t682: F, t963: F, t696: F, t278: F, t962: F) -> (F, F, F) {
    let t3705 = t963 * t3703 * t682;
    let t3707 = F::new(3.5089341735807875) * t696 * t3705;
    let t3709 = F::new(1.0) / t962 / t278;
    (t3705, t3707, t3709)
}
