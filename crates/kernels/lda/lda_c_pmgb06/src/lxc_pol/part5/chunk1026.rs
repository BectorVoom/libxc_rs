//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1026/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1026<F: Float>(t405: F, t7850: F, t7844: F, t7834: F, t7837: F, t13370: F, t14127: F, t19364: F, t19366: F, t19368: F, t19373: F, t19379: F, t19383: F, t19387: F, t19391: F, t19398: F, t19400: F, t19402: F, t9938: F, t9981: F, t9986: F) -> (F,) {
    let t21131 = t405 * t7850;
    let t21133 = t405 * t7844;
    let t21135 = t405 * t7834;
    let t21137 = t405 * t7837;
    let t21139 = 0.14396666666666666 * t19364 + 0.023994444444444443 * t19366 - 0.07198333333333333 * t19368 + 0.47988888888888886 * t19373 - 0.10664197530864197 * t19379 + 0.23994444444444443 * t19383 - 0.8638 * t19387 - 0.8638 * t19391 + 0.09597777777777777 * t13370 - t14127 - 0.035991666666666665 * t19398 + 0.013330246913580247 * t19400 - 0.047988888888888886 * t19402 + 0.019753086419753086 * t9938 + t9981 + t9986 + 0.02666666666666667 * t21131 - 0.008888888888888889 * t21133 + 0.0019753086419753087 * t21135 + 0.0044444444444444444 * t21137;
    (t21139,)
}
