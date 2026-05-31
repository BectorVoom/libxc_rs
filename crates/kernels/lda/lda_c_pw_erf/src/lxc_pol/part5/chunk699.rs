//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 699/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk699<F: Float>(t493: F, t6220: F, t2134: F, t795: F, t2463: F, t656: F, t2402: F, t568: F, t1976: F, t739: F, t4829: F, t1325: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6221 = t493 * t6220;
    let t6222 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t6221;
    let t6223 = t795 * t2134;
    let t6224 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t6223;
    let t6225 = t2463 * t656;
    let t6227 = t2402 * t568;
    let t6228 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t6227;
    let t6229 = t1976 * t739;
    let t6230 = t4829 * t6229;
    let t6232 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1325 * t6230;
    (t6221, t6222, t6223, t6224, t6225, t6227, t6228, t6229, t6230, t6232)
}
