//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1174/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1174(t405: f64, t7850: f64, t7844: f64, t7834: f64, t7837: f64, t13370: f64, t14127: f64, t19364: f64, t19366: f64, t19368: f64, t19373: f64, t19379: f64, t19383: f64, t19387: f64, t19391: f64, t19398: f64, t19400: f64, t19402: f64, t9938: f64, t9981: f64, t9986: f64) -> f64 {
    let t21131 = t405 * t7850;
    let t21133 = t405 * t7844;
    let t21135 = t405 * t7834;
    let t21137 = t405 * t7837;
    let t21139 = 0.14396666666666666_f64 * t19364 + 0.023994444444444443_f64 * t19366 - 0.07198333333333333_f64 * t19368 + 0.47988888888888886_f64 * t19373 - 0.10664197530864197_f64 * t19379 + 0.23994444444444443_f64 * t19383 - 0.8638_f64 * t19387 - 0.8638_f64 * t19391 + 0.09597777777777777_f64 * t13370 - t14127 - 0.035991666666666665_f64 * t19398 + 0.013330246913580247_f64 * t19400 - 0.047988888888888886_f64 * t19402 + 0.019753086419753086_f64 * t9938 + t9981 + t9986 + 0.02666666666666667_f64 * t21131 - 0.008888888888888889_f64 * t21133 + 0.0019753086419753087_f64 * t21135 + 0.0044444444444444444_f64 * t21137;
    t21139
}
