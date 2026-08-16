//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 490/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk490<F: Float>(t2470: F, t493: F, t1972: F, t835: F, t2002: F, t806: F, t1962: F, t805: F) -> (F, F, F, F) {
    let t2472 = t493 * t2470 / F::cast_from(27.0_f64);
    let t2474 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1972 * t835;
    let t2476 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t2002 * t806;
    let t2477 = t1962 * t805;
    (t2472, t2474, t2476, t2477)
}
