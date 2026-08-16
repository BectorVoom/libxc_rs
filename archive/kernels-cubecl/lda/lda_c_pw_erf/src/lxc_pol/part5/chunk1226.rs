//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1226/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1226<F: Float>(t17985: F, t15824: F, t3965: F, t4479: F, t5424: F, t14014: F, t5220: F, t12968: F, t2021: F, t3974: F, t4516: F, t15727: F, t4475: F, t5305: F) -> (F, F, F, F, F) {
    let t22121 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t17985;
    let t22125 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t3965 * t4479 * t5424 * t15824;
    let t22129 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3965 * t14014 * t5220 * t15824;
    let t22133 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t3974 * t12968 * t2021 * t4516;
    let t22137 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t3974 * t4475 * t5305 * t15727;
    (t22121, t22125, t22129, t22133, t22137)
}
