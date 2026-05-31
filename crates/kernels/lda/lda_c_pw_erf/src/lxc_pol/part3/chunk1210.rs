//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1210/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1210<F: Float>(t1318: F, t1319: F, t3563: F, t816: F, t1287: F, t1954: F, t4758: F, t3787: F, t4937: F, t519: F, t1440: F, t3677: F, t806: F, t9223: F) -> (F, F, F, F) {
    let t14271 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1318 * t1319 * t816 * t3563;
    let t14275 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1318 * t4758 * t1954 * t1287;
    let t14277 = t519 * t3787 * t4937;
    let t14278 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t14277;
    let t14283 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t519 * t1440 * t9223 * t806 * t3677;
    (t14271, t14275, t14278, t14283)
}
