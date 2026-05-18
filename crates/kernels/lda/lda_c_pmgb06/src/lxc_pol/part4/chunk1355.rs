//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1355/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1355<F: Float>(t4731: F, t493: F, t5486: F, t1981: F, t5441: F, t1444: F, t6748: F, t176: F, t1826: F, t5312: F, t13836: F, t13838: F) -> (F, F, F, F, F, F) {
    let t17819 = F::new(2.0) / F::new(45.0) * t493 * t5486 * t4731;
    let t17822 = F::new(8.0) / F::new(45.0) * t1981 * t5486 * t5441;
    let t17824 = F::new(8.0) / F::new(45.0) * t1444 * t6748;
    let t17828 = F::new(8.0) / F::new(45.0) * t493 * t5312 * t176 * t1826;
    let t17829 = F::new(8.0) / F::new(135.0) * t13836;
    let t17830 = F::new(8.0) / F::new(27.0) * t13838;
    (t17819, t17822, t17824, t17828, t17829, t17830)
}
