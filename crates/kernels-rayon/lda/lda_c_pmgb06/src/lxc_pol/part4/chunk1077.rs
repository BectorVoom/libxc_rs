//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1077/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1077(t3450: f64, t831: f64, t132: f64, t435: f64, t4965: f64, t432: f64, t5115: f64, t517: f64, t5415: f64, t490: f64, t5432: f64, t1504: f64, t1848: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12245 = t831 * t3450;
    let t12248 = t132 * t435 * t4965;
    let t12259 = t432 * t5115;
    let t12261 = t5415 * t517;
    let t12274 = t5432 * t490;
    let t12276 = t1848 * t1504;
    (t12245, t12248, t12259, t12261, t12274, t12276)
}
