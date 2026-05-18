//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1329/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1329<F: Float>(t17427: F, t5068: F, t5069: F, t13068: F, t16386: F, t5138: F, t13672: F, t17435: F, t17430: F, t17433: F, t17434: F, t17438: F, t17440: F, t17444: F, t17448: F, t17452: F, t17455: F, t17460: F, t17465: F, t17469: F) -> (F, F, F, F) {
    let t17472 = F::new(4.0) / F::new(45.0) * t5068 * t5069 * t17427;
    let t17475 = F::new(4.0) / F::new(9.0) * t5138 * t13068 * t16386;
    let t17478 = F::new(16.0) / F::new(45.0) * t13672 * t5069 * t17435;
    let t17479 = -t17430 - t17433 + t17434 + t17438 + t17440 + t17444 + t17448 + t17452 - t17455 - t17460 - t17465 + t17469 + t17472 + t17475 - t17478;
    (t17472, t17475, t17478, t17479)
}
