//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1228/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1228(t4062: f64, t571: f64, t7488: f64, t13750: f64, t22121: f64, t22125: f64, t22129: f64, t22133: f64, t22137: f64, t22141: f64, t22143: f64, t22145: f64, t22146: f64, t22148: f64, t22150: f64) -> (f64, f64) {
    let t22152 = t571 * t4062 * t7488;
    let t22153 = 16.0_f64 / 27.0_f64 * t22152;
    let t22154 = t22121 + t22125 - t22129 + t22133 + t22137 - t22141 - t13750 + t22143 + t22145 + t22146 + t22148 + t22150 - t22153;
    (t22153, t22154)
}
