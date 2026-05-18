//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1187/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1187<F: Float>(t2642: F, t955: F, t31: F, t8101: F, t99: F, t405: F, t6218: F, t15416: F, t15418: F, t15421: F, t15423: F, t15427: F, t15429: F, t15431: F, t15433: F, t15435: F, t1619: F, t473: F, t9724: F) -> (F, F) {
    let t15644 = t955 * t2642;
    let t15650 = t99 * t31 * t8101;
    let t15654 = t405 * t6218;
    let t15662 = t9724 - F::new(0.015996296296296297) * t15416 + F::new(0.014814814814814815) * t15644 - F::new(0.010664197530864198) * t15418 - F::new(0.09597777777777777) * t15421 + F::new(0.03199259259259259) * t15423 + F::new(0.10666666666666667) * t15650 * t473 * t15431 + F::new(0.008888888888888889) * t15654 - F::new(0.017777777777777778) * t15650 * t1619 * t15427 - F::new(0.19195555555555555) * t15429 + F::new(0.5758666666666666) * t15433 + F::new(0.023994444444444443) * t15435;
    (t15650, t15662)
}
