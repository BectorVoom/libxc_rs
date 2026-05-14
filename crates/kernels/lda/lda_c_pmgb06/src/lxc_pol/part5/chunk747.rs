//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 747/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk747<F: Float>(t2563: F, t844: F, t2469: F, t4588: F, t493: F, t1972: F, t2466: F, t498: F, t7300: F, t496: F, t2470: F, t3189: F, t7284: F, t1436: F, t439: F, t2002: F, t2493: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7633 = t2563 * t844 / 10.0;
    let t7634 = t4588 * t2469;
    let t7636 = t493 * t7634 / 9.0;
    let t7638 = t1972 * t2466 / 15.0;
    let t7639 = t498 * t7300;
    let t7640 = t496 * t7639;
    let t7642 = t493 * t7640 / 45.0;
    let t7644 = t1972 * t2470 / 9.0;
    let t7645 = t3189 * t7284;
    let t7646 = t1436 * t7645;
    let t7648 = 2.0 / 9.0 * t439 * t7646;
    let t7650 = 2.0 / 15.0 * t2002 * t2493;
    (t7633, t7634, t7636, t7638, t7639, t7640, t7642, t7644, t7645, t7646, t7648, t7650)
}
