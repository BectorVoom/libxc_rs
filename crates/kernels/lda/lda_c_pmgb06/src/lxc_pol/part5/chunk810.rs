//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 810/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk810<F: Float>(t2563: F, t844: F, t2469: F, t4588: F, t493: F, t1972: F, t2466: F, t498: F, t7300: F, t496: F, t2470: F, t3189: F, t7284: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7633 = t2563 * t844 / F::cast_from(10.0_f64);
    let t7634 = t4588 * t2469;
    let t7636 = t493 * t7634 / F::cast_from(9.0_f64);
    let t7638 = t1972 * t2466 / F::cast_from(15.0_f64);
    let t7639 = t498 * t7300;
    let t7640 = t496 * t7639;
    let t7642 = t493 * t7640 / F::cast_from(45.0_f64);
    let t7644 = t1972 * t2470 / F::cast_from(9.0_f64);
    let t7645 = t3189 * t7284;
    (t7633, t7634, t7636, t7638, t7639, t7640, t7642, t7644, t7645)
}
