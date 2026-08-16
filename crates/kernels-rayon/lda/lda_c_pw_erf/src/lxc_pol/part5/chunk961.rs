//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 961/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk961(t12814: f64, t10467: f64, t1996: f64, t519: f64, t10463: f64, t1972: f64, t10313: f64, t1967: f64, t197: f64, t518: f64, t5210: f64, t1124: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12815 = 8.0_f64 / 135.0_f64 * t12814;
    let t12838 = t519 * t10467 * t1996;
    let t12839 = 8.0_f64 / 135.0_f64 * t12838;
    let t12862 = t519 * t10463 * t1972;
    let t12863 = 16.0_f64 / 135.0_f64 * t12862;
    let t12869 = t519 * t10313 * t197 * t1967;
    let t12870 = 8.0_f64 / 81.0_f64 * t12869;
    let t12874 = t5210 * t518;
    let t12916 = t1124 * t213;
    (t12815, t12839, t12863, t12870, t12874, t12916)
}
