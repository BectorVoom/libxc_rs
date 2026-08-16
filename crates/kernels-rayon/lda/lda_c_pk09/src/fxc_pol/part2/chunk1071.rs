//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1071/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1071(t11311: f64, t7275: f64, t11073: f64, t11314: f64, t2032: f64, t2829: f64, t2832: f64, t6323: f64, t6337: f64, t6793: f64, t6806: f64, t6811: f64, t7041: f64, t7045: f64, t7049: f64, t7053: f64, t7064: f64, t7067: f64, t7069: f64, t7071: f64, t7074: f64, t7076: f64, t7262: f64, t7269: f64) -> f64 {
    let t11620 = t11311 * t7275;
    let t11632 = -0.03412591035063918_f64 * t11073 + 0.03412591035063918_f64 * t6337 + 0.10237773105191754_f64 * t6323 - t7041 + 0.04991874779241519_f64 * t6793 - t7045 + 0.02466859483068398_f64 * t6806 - 0.14975624337724558_f64 * t6811 + t7049 / 18.0_f64 + t7053 - t7064 - t11620 / 6.0_f64 - t7067 / 18.0_f64 + t7069 / 18.0_f64 + t11314 * t7269 / 6.0_f64 - t2832 * t7262 / 6.0_f64 + t7071 / 18.0_f64 - t7074 + t7076 / 6.0_f64 - t2829 * t2032 / 6.0_f64;
    t11632
}
