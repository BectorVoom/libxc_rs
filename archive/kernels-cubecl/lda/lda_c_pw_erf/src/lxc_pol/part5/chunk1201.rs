//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1201/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1201<F: Float>(t12314: F, t6743: F, t6749: F, t16863: F, t1972: F, t4488: F, t12321: F, t1967: F, t2471: F, t16606: F, t2010: F, t4506: F) -> (F, F, F, F, F) {
    let t21752 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t12314 * t6743;
    let t21754 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t12314 * t6749;
    let t21757 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t4488 * t16863 * t1972;
    let t21761 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4488 * t12321 * t2471 * t1967;
    let t21764 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4506 * t16606 * t2010;
    (t21752, t21754, t21757, t21761, t21764)
}
