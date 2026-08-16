//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1187/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1187(t2642: f64, t955: f64, t31: f64, t8101: f64, t99: f64, t405: f64, t6218: f64, t15416: f64, t15418: f64, t15421: f64, t15423: f64, t15427: f64, t15429: f64, t15431: f64, t15433: f64, t15435: f64, t1619: f64, t473: f64, t9724: f64) -> (f64, f64) {
    let t15644 = t955 * t2642;
    let t15650 = t99 * t31 * t8101;
    let t15654 = t405 * t6218;
    let t15662 = t9724 - 0.015996296296296297_f64 * t15416 + 0.014814814814814815_f64 * t15644 - 0.010664197530864198_f64 * t15418 - 0.09597777777777777_f64 * t15421 + 0.03199259259259259_f64 * t15423 + 0.10666666666666667_f64 * t15650 * t473 * t15431 + 0.008888888888888889_f64 * t15654 - 0.017777777777777778_f64 * t15650 * t1619 * t15427 - 0.19195555555555555_f64 * t15429 + 0.5758666666666666_f64 * t15433 + 0.023994444444444443_f64 * t15435;
    (t15650, t15662)
}
