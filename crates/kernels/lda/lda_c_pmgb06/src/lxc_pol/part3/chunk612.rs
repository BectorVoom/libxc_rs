//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 612/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk612<F: Float>(t3427: F, t3440: F, t465: F, t137: F, t132: F, t1586: F, t1639: F, t166: F, t161: F, t1554: F, t530: F, t1587: F, t489: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3441 = t3427 + t3440;
    let t3442 = t465 * t3441;
    let t3443 = t137 * t3442;
    let t3445 = t132 * t3443 / F::new(30.0);
    let t3446 = t1639 * t1586;
    let t3447 = t166 * t3446;
    let t3449 = t161 * t3447 / F::new(10.0);
    let t3450 = t1554 * t530;
    let t3451 = t161 * t3450;
    let t3452 = t3451 / F::new(45.0);
    let t3453 = t489 * t1587;
    (t3441, t3442, t3443, t3445, t3446, t3447, t3449, t3450, t3451, t3452, t3453)
}
