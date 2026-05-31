//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 805/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk805<F: Float>(t6522: F, t760: F, t2864: F, t439: F, t3249: F, t7295: F, t3248: F, t493: F, t2002: F, t2481: F, t2485: F, t1962: F, t2480: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7562 = t6522 * t760;
    let t7563 = t2864 * t7562;
    let t7565 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t7563;
    let t7566 = t3249 * t7295;
    let t7567 = t3248 * t7566;
    let t7569 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t493 * t7567;
    let t7571 = t2002 * t2481 / F::cast_from(15.0_f64);
    let t7573 = t2002 * t2485 / F::cast_from(9.0_f64);
    let t7574 = t1962 * t2480;
    (t7562, t7563, t7565, t7566, t7567, t7569, t7571, t7573, t7574)
}
