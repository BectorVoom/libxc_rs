//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1209/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1209<F: Float>(t184: F, t203: F, t21834: F, t21866: F, t221: F, t12136: F, t6740: F, t16606: F, t2022: F, t4506: F, t18184: F, t3974: F) -> (F, F, F, F) {
    let t21871 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t203 * (t21834 + t21866) * t184 * t221;
    let t21875 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t12136 * t6740;
    let t21878 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4506 * t16606 * t2022;
    let t21881 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3974 * t18184 * t2022;
    (t21871, t21875, t21878, t21881)
}
