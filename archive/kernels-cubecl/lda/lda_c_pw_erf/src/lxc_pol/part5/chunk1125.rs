//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1125/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1125<F: Float>(t4488: F, t4490: F, t6460: F, t16863: F, t2026: F, t3965: F, t13115: F, t14034: F, t2388: F, t4475: F, t6400: F, t15697: F) -> (F, F, F, F, F) {
    let t20876 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t4488 * t4490 * t6460;
    let t20879 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3965 * t16863 * t2026;
    let t20882 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t13115 * t14034 * t2388;
    let t20885 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t13115 * t4475 * t6400;
    let t20886 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t15697;
    (t20876, t20879, t20882, t20885, t20886)
}
