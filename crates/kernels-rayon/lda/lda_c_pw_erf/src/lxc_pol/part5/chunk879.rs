//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 879/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk879(t13: f64, t3130: f64, t8185: f64, t902: f64, t911: f64, t1030: f64, t2983: f64, t400: f64, t8171: f64, t3148: f64, t333: f64, t904: f64, t907: f64) -> (f64, f64, f64) {
    let t8244 = 6207.00176468474_f64 * t13 / t902 / t911 * t8185 * t3130;
    let t8248 = 623.3672123775311_f64 * t400 * t2983 * t8171 * t1030;
    let t8260 = 64.32729728860441_f64 * t904 * t3148 * t907 * t333;
    (t8244, t8248, t8260)
}
