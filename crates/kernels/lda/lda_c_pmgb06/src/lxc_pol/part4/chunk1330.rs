//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1330/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1330<F: Float>(t13035: F, t5083: F, t6642: F, t2064: F, t760: F, t332: F, t5084: F, t16821: F, t13026: F, t13031: F, t16825: F, t13020: F, t16830: F) -> (F, F, F, F, F, F, F) {
    let t17482 = F::new(4.0) / F::new(27.0) * t5083 * t13035 * t6642;
    let t17483 = t760 * t2064;
    let t17484 = t17483 * t332;
    let t17487 = F::new(4.0) / F::new(27.0) * t5083 * t5084 * t17484;
    let t17490 = F::new(2.0) / F::new(27.0) * t5083 * t5084 * t16821;
    let t17493 = F::new(16.0) / F::new(81.0) * t13026 * t13031 * t16825;
    let t17496 = F::new(8.0) / F::new(27.0) * t13020 * t5084 * t16830;
    (t17482, t17483, t17484, t17487, t17490, t17493, t17496)
}
