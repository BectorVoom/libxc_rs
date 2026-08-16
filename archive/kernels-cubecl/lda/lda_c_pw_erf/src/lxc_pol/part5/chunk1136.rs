//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1136/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1136<F: Float>(t1318: F, t2526: F, t5269: F, t593: F, t811: F, t16050: F, t16053: F, t16058: F, t16065: F, t568: F, t7676: F, t2023: F, t6205: F) -> (F, F, F, F, F, F, F) {
    let t21001 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1318 * t5269 * t2526 * t811 * t593;
    let t21002 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t16050;
    let t21003 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t16053;
    let t21004 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t16058;
    let t21005 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t16065;
    let t21007 = t7676 * t568;
    let t21008 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t21007;
    let t21012 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t6205 * t2023;
    (t21001, t21002, t21003, t21004, t21005, t21008, t21012)
}
