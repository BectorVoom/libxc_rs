//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 766/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk766<F: Float>(t242: F, t7045: F, t153: F, t156: F, t168: F, t245: F, t3378: F, t4084: F, t4091: F, t5887: F, t5891: F, t5892: F, t6080: F, t7025: F, t7032: F, t7035: F, t7038: F, t7043: F) -> (F, F) {
    let t7046 = t7045 * t242;
    let t7049 = -F::cast_from(0.011938374665504766_f64) * t168 * t245 * t7025 + F::cast_from(0.42708890021612717_f64) * t153 * t156 * t6080 - F::cast_from(0.0837628205355044_f64) * t7032 * t242 - F::cast_from(0.0837628205355044_f64) * t7035 + t4091 - F::cast_from(0.5694518669548363_f64) * t7038 - F::cast_from(0.053059442957798957_f64) * t4084 + t5887 + F::cast_from(1.328721022894618_f64) * t3378 + F::cast_from(0.019897291109174608_f64) * t7043 + F::cast_from(0.0837628205355044_f64) * t7046 - t5891 - F::cast_from(0.1675256410710088_f64) * t5892;
    (t7046, t7049)
}
