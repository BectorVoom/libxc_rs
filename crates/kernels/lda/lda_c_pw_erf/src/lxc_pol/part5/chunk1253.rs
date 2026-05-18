//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1253/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1253<F: Float>(t18154: F, t799: F, t15614: F, t2532: F, t14049: F, t14052: F, t11854: F, t13562: F, t13564: F, t13635: F, t13639: F, t16285: F, t20813: F, t21779: F, t21782: F, t21785: F, t21788: F, t21790: F, t21792: F, t21796: F, t21799: F, t21802: F, t21805: F, t21808: F, t21813: F, t21817: F, t21822: F, t21827: F) -> (F, F, F, F, F) {
    let t22446 = F::new(4.0) / F::new(15.0) * t18154 * t799;
    let t22448 = F::new(8.0) / F::new(5.0) * t15614 * t2532;
    let t22449 = F::new(16.0) / F::new(135.0) * t14049;
    let t22450 = F::new(32.0) / F::new(135.0) * t14052;
    let t22475 = F::new(0.04) * t11854 * t13635 * t20813 - F::new(0.008888888888888889) * t11854 * t13639 * t20813 + F::new(0.05925925925925926) * t13562 + F::new(0.044444444444444446) * t13564 + F::new(0.8638) * t21779 + F::new(1.2957) * t21782 + F::new(0.21595) * t21785 - F::new(0.4319) * t21788 + F::new(0.023994444444444443) * t21790 - F::new(0.07198333333333333) * t21792 - F::new(0.8638) * t21796 + F::new(0.47988888888888886) * t21799 - F::new(0.8638) * t21802 - F::new(0.07198333333333333) * t21805 + F::new(0.14396666666666666) * t21808 - F::new(0.10664197530864197) * t21813 + F::new(0.23994444444444443) * t21817 + F::new(0.07198333333333333) * t21822 - F::new(0.023994444444444443) * t21827 + F::new(0.10666666666666667) * t16285;
    (t22446, t22448, t22449, t22450, t22475)
}
