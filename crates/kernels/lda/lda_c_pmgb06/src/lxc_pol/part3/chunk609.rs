//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 609/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk609<F: Float>(t1626: F, t405: F, t1620: F, t134: F, t443: F, t147: F, t3093: F, t3116: F, t473: F, t135: F, t146: F, t3365: F) -> (F, F, F, F, F, F, F) {
    let t3398 = t405 * t1626;
    let t3400 = t405 * t1620;
    let t3403 = F::new(1.0) / t134 / t443;
    let t3404 = t147 * t3403;
    let t3405 = t3404 * t3093;
    let t3408 = t473 * t3116;
    let t3413 = F::new(0.02962962962962963) * t146 * t3365 * t135;
    (t3398, t3400, t3403, t3404, t3405, t3408, t3413)
}
