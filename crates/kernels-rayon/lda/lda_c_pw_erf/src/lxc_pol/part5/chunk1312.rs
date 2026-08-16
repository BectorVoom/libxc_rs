//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1312/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1312(t16529: f64, t21115: f64, t21116: f64, t21118: f64, t21119: f64, t21120: f64, t21121: f64, t21123: f64, t21124: f64, t21128: f64, t21129: f64, t21130: f64, t21131: f64) -> f64 {
    let t23218 = -t21115 + t21116 + t21118 - t21119 - t21120 + 4.0_f64 * t16529 + t21121 - t21123 + t21124 + t21128 + t21129 + t21130 - t21131;
    t23218
}
