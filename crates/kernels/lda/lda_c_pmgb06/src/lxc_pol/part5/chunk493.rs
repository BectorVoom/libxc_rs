//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 493/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk493<F: Float>(t2578: F, t453: F, t36: F, t1522: F, t1856: F, t2572: F, t2576: F) -> (F, F, F) {
    let t2579 = t453 * t2578;
    let t2580 = t36 * t2579;
    let t2582 = -t1522 - 0.0012594444444444445 * t1856 + 0.0012594444444444445 * t2572 - 0.003778333333333333 * t2576 + 0.0018891666666666666 * t2580;
    (t2579, t2580, t2582)
}
