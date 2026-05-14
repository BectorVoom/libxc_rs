//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 941/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk941<F: Float>(t11073: F, t11314: F, t11620: F, t2032: F, t2829: F, t2832: F, t6323: F, t6337: F, t6793: F, t6806: F, t6811: F, t7041: F, t7045: F, t7049: F, t7053: F, t7064: F, t7067: F, t7069: F, t7071: F, t7074: F, t7076: F, t7262: F, t7269: F) -> (F,) {
    let t11632 = -0.03412591035063918 * t11073 + 0.03412591035063918 * t6337 + 0.10237773105191754 * t6323 - t7041 + 0.04991874779241519 * t6793 - t7045 + 0.02466859483068398 * t6806 - 0.14975624337724558 * t6811 + t7049 / 18.0 + t7053 - t7064 - t11620 / 6.0 - t7067 / 18.0 + t7069 / 18.0 + t11314 * t7269 / 6.0 - t2832 * t7262 / 6.0 + t7071 / 18.0 - t7074 + t7076 / 6.0 - t2829 * t2032 / 6.0;
    (t11632,)
}
