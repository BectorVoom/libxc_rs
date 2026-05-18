//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 877/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk877<F: Float>(t134: F, t138: F, t9175: F, t139: F, t3259: F, t1437: F, t1830: F, t455: F, t1530: F, t1710: F, t485: F, t1687: F, t1730: F) -> (F, F, F, F, F, F, F, F) {
    let t9177 = t138 * t9175 * t134;
    let t9178 = F::new(0.01959135802469136) * t9177;
    let t9188 = t139 * t3259;
    let t9189 = t1437 * t1437;
    let t9190 = F::new(1.0) / t9189;
    let t9215 = t1830 * t455;
    let t9220 = F::new(1.0) / t1437 / t1530;
    let t9266 = t485 * t1710;
    let t9340 = t1687 * t1730;
    (t9177, t9178, t9188, t9190, t9215, t9220, t9266, t9340)
}
