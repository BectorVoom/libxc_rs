//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1253/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1253(t18154: f64, t799: f64, t15614: f64, t2532: f64, t14049: f64, t14052: f64, t11854: f64, t13562: f64, t13564: f64, t13635: f64, t13639: f64, t16285: f64, t20813: f64, t21779: f64, t21782: f64, t21785: f64, t21788: f64, t21790: f64, t21792: f64, t21796: f64, t21799: f64, t21802: f64, t21805: f64, t21808: f64, t21813: f64, t21817: f64, t21822: f64, t21827: f64) -> (f64, f64, f64, f64, f64) {
    let t22446 = 4.0_f64 / 15.0_f64 * t18154 * t799;
    let t22448 = 8.0_f64 / 5.0_f64 * t15614 * t2532;
    let t22449 = 16.0_f64 / 135.0_f64 * t14049;
    let t22450 = 32.0_f64 / 135.0_f64 * t14052;
    let t22475 = 0.04_f64 * t11854 * t13635 * t20813 - 0.008888888888888889_f64 * t11854 * t13639 * t20813 + 0.05925925925925926_f64 * t13562 + 0.044444444444444446_f64 * t13564 + 0.8638_f64 * t21779 + 1.2957_f64 * t21782 + 0.21595_f64 * t21785 - 0.4319_f64 * t21788 + 0.023994444444444443_f64 * t21790 - 0.07198333333333333_f64 * t21792 - 0.8638_f64 * t21796 + 0.47988888888888886_f64 * t21799 - 0.8638_f64 * t21802 - 0.07198333333333333_f64 * t21805 + 0.14396666666666666_f64 * t21808 - 0.10664197530864197_f64 * t21813 + 0.23994444444444443_f64 * t21817 + 0.07198333333333333_f64 * t21822 - 0.023994444444444443_f64 * t21827 + 0.10666666666666667_f64 * t16285;
    (t22446, t22448, t22449, t22450, t22475)
}
