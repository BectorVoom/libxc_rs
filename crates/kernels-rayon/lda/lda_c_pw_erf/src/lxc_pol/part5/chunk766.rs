//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 766/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk766(t242: f64, t7045: f64, t153: f64, t156: f64, t168: f64, t245: f64, t3378: f64, t4084: f64, t4091: f64, t5887: f64, t5891: f64, t5892: f64, t6080: f64, t7025: f64, t7032: f64, t7035: f64, t7038: f64, t7043: f64) -> (f64, f64) {
    let t7046 = t7045 * t242;
    let t7049 = -0.011938374665504766_f64 * t168 * t245 * t7025 + 0.42708890021612717_f64 * t153 * t156 * t6080 - 0.0837628205355044_f64 * t7032 * t242 - 0.0837628205355044_f64 * t7035 + t4091 - 0.5694518669548363_f64 * t7038 - 0.053059442957798957_f64 * t4084 + t5887 + 1.328721022894618_f64 * t3378 + 0.019897291109174608_f64 * t7043 + 0.0837628205355044_f64 * t7046 - t5891 - 0.1675256410710088_f64 * t5892;
    (t7046, t7049)
}
